# 线程局部存储分析


## 前言

> 在Linux C/C++编程时不可避免的会遇到以下的需求,全局变量线程共享;最为典型的功能则是`errno`,变量
> 在程序的任何地方都可以访问,但是不会影响到其他线程,这就是本文档说明的`TLS`(线程局部存储变量)

**如何创建并且使用TLS? 存在下面两种方法**

+ 线程库函数
+ 编译器提供

下面分别进行说明分析

## 线程库函数
> pthread提供了函数用来处理`TLS`,
> 分别管理键值和数据

### 键值
```c
typedef unsigned pthread_key_t;
int pthread_key_create(pthread_key_t *, void (*)(void *));
int pthread_key_delete(pthread_key_t);
```

```c
// 进程最多可以创建128个键值
#define PTHREAD_KEYS_MAX 128
```

#### 创建键值
`pthread_key_create`
1. pthread_key_t 键值变量
2. 析构函数(线程退出时,自动调用)

下面的代码为了减少篇幅和增加可读性,我删除了不少辅助代码;

```c
pthread_key_t j = next_key;
do {
	if (!keys[j]) {
		keys[next_key = *k = j] = dtor;
		return 0;
	}
} while ((j=(j+1)%PTHREAD_KEYS_MAX) != next_key);
```

#### 销毁键值
`pthread_key_delete`

```c
do {
	td->tsd[k] = 0;
} while ((td=td->next)!=self);
keys[k] = 0;
```

#### 析构调用
`pthread_tsd_run_dtors`
线程退出`__pthread_exit`函数调用;

```c
for (j=0; self->tsd_used && j<PTHREAD_DESTRUCTOR_ITERATIONS; j++) {
	self->tsd_used = 0;

	for (i=0; i < PTHREAD_KEYS_MAX; i++) {
		void *val = self->tsd[i];
		void (*dtor)(void *) = keys[i];
		self->tsd[i] = 0;

		if (val && dtor && dtor != nodtor) {
			dtor(val);
		}
	}
}
```

### 数据
```c
void *pthread_getspecific(pthread_key_t);
int pthread_setspecific(pthread_key_t, const void *);

// 设置
self->tsd[k] = 私有数据;
self->tsd_used = 1;

// 读取
return self->tsd[k];
```

## 编译器提供

> musl线程库提供的私有数据还可以理解,但是`GCC`的`__thread`变量就比较难以分析了,因此此时只能通过
> 汇编文件进行分析

給出测试程序

```c
__thread int num;

int test(void) 
{
    return num;
}
```

GCC生成的汇编语言
```assembly
num:
test:
	push    {r7}						@ 进入函数,保存现场
	add     r7, sp, #0					@ R7 = SP

	mrc     p15, 0, r3, c13, c0, 3    	@ R3 = 线程号
	ldr     r2, .L3						@ R2 = &num
	ldr     r3, [r3, r2]				@ R3 = *((int *)(R3 + R2))
	mov     r0, r3						@ R0 = R3
	
	mov     sp, r7						@ 恢复SP
	ldr     r7, [sp], #4				@ 恢复R7
	
	bx      lr							@ return 

.L3:
        .word   num(tpoff)
```

但是到现在我们还是没有通过汇编理解原因,但是我们可以注意到一个可疑点`.word   num(tpoff)`,这个表达式中tpoff是什么?那么只好到GCC官网上看看是怎么处理的.
[GCC Thread-Local Storage](https://gcc.gnu.org/onlinedocs/gcc-12.1.0/gcc/Thread-Local.html)
同时可以得到一份文档[ELF Handling For Thread-Local Storage](https://www.akkadia.org/drepper/tls.pdf);

那么就开始分析此文档


