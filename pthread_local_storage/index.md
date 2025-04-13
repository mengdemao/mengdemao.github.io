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

## `GCC TLS`实现分析

GCC中`__thread`关键字的实现涉及多个层次，包括编译器、链接器、运行时库和操作系统的协作。以下是其实现机制的分析：

### 1. **编译阶段处理**

- **关键字识别**：GCC识别`__thread`修饰的变量，将其标记为线程局部存储（TLS）类型（`STT_TLS`符号）。
- **段分配**：
  - 已初始化的TLS变量（如`__thread int x = 42;`）被放置在`.tdata`段。
  - 未初始化的TLS变量（如`__thread int y;`）被放置在`.tbss`段。
- **代码生成**：根据目标架构和TLS模型生成访问指令。例如，x86-64使用`%fs`或`%gs`段寄存器加偏移量：
  ```asm
  movl %fs:0x0, %eax  # 访问偏移量为0的TLS变量
  ```

### 2. **链接阶段处理**
- **TLS模板**：链接器收集所有`.tdata`和`.tbss`段，生成TLS模板（在ELF中通过`PT_TLS`程序头描述）。
- **符号解析**：为每个TLS变量分配偏移量，并生成动态重定位信息（若使用动态TLS模型）。

### 3. **运行时管理**
- **线程创建**：当新线程创建时（如通过`pthread_create`），运行时库（如glibc）根据TLS模板为线程分配独立的TLS存储区域。
- **存储访问**：
  - **静态模型**（如`local-exec`）：直接通过段寄存器+固定偏移量访问，无需运行时计算。
  - **动态模型**（如`global-dynamic`）：调用`__tls_get_addr()`函数动态获取变量地址。
- **线程控制块（TCB）**：每个线程的TCB包含指向其TLS存储的指针，通常通过段寄存器（如x86的`%fs`）快速访问。

### 4. **TLS模型与性能权衡**
GCC支持多种TLS模型（通过`-ftls-model`指定）：
- **global-dynamic**：支持动态库，通过`__tls_get_addr()`获取地址，灵活性高但速度较慢。
- **local-exec**：静态链接时确定偏移量，访问最快但无法用于动态库。
- **initial-exec**：通过GOT（全局偏移表）访问，适合动态库但要求库加载在进程启动时完成。

### 5. **跨平台与架构差异**
- **x86/x86-64**：使用`%fs`或`%gs`段寄存器存储TLS区块基址。
- **ARM/AArch64**：通过TPIDR_EL0（用户态TLS指针寄存器）访问。
- **Windows**：使用`__declspec(thread)`，机制类似但实现细节不同（如`.tls`段和TLS回调函数）。

### 6. **动态库支持**
动态库中的TLS变量需要额外处理：
- 加载动态库时，为所有现有线程分配该库的TLS空间。
- 使用`dlopen`加载时，若TLS模型为`global-dynamic`，可能触发TLS扩容。

### 7. **调试与工具验证**
- **查看ELF信息**：
  ```bash
  readelf -S a.out | grep -E '(tdata|tbss)'  # 确认TLS段
  readelf -l a.out | grep TLS                # 查看PT_TLS程序头
  ```
- **汇编分析**：
  ```bash
  gcc -S tls.c -o tls.s  # 生成汇编，观察TLS变量访问指令
  ```
- **调试验证**：在GDB中查看不同线程的变量地址是否不同。

### 8. **示例代码分析**
```c
__thread int tls_var = 42;

int main() {
    tls_var = 10;  // 编译为 movl $10, %fs:0x0（假设偏移量0）
    return 0;
}
```
编译后的汇编可能直接使用段寄存器访问，或在动态模型中调用`__tls_get_addr`。

### 9. **初始化与销毁**
- **主线程**：TLS变量在程序启动时初始化。
- **新线程**：TLS空间在创建时初始化，内容从模板拷贝。
- **析构函数**：C++中带构造函数的TLS对象需注册析构，线程退出时调用。

### 总结
GCC的`__thread`实现通过协作的编译、链接和运行时机制，结合硬件特性（如段寄存器）和操作系统支持，为每个线程提供独立的变量存储。不同TLS模型在灵活性和性能之间权衡，开发者可根据场景选择最优模型。

