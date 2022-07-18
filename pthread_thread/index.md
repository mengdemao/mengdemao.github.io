# pthread线程源码分析


> 基于musl源码库与glibc库,其中musl提供分析的思路,glibc分析具体实现

## pthread句柄

### musl实现

```c
struct pthread {

	struct pthread *self;

    #ifndef TLS_ABOVE_TP
	uintptr_t *dtv;
	#endif
	
    /* 线程链表项 */
    struct pthread *prev, *next; /* non-ABI */
    
    /* 系统信息 */
	uintptr_t sysinfo;
	
    #ifndef TLS_ABOVE_TP
	#ifdef CANARY_PAD
	uintptr_t canary_pad;
	#endif 
	uintptr_t canary;
	#endif /* TLS_ABOVE_TP */

	int tid; // 线程ID
	
    int errno_val;
	
    volatile int detach_state;  // 分离状态
	
    volatile int cancel;					// cancle启动标志
    volatile unsigned char canceldisable; 	// cancle控制
    volatile unsigned char cancelasync;   	// cancle同步标志
	
    unsigned char tsd_used:1;
	unsigned char dlerror_flag:1;
	
    unsigned char *map_base;	// mmap
	size_t map_size;
	
    void *stack;	// 堆栈
	size_t stack_size;
	
    size_t guard_size; 
	
    void *result; // 返回结果,
    
    /* 线程清理回调函数 pthread_cleanup_push、pthread_cleanup_pop */
	struct __ptcb *cancelbuf; 
	
    void **tsd;
	
    struct {
		volatile void *volatile head;
		long off;
		volatile void *volatile pending;
	} robust_list;
	
    int h_errno_val;
    
	volatile int timer_id;
	
    locale_t locale;
	
    volatile int killlock[1];		// 退出锁
	
    char *dlerror_buf;
	
    void *stdio_locks;

	#ifdef TLS_ABOVE_TP
	uintptr_t canary;
	uintptr_t *dtv;
	#endif
    
};
```

### glibc实现

```c
struct __pthread
{
  /* 线程ID: typedef unsigned long int pthread_t; */
  pthread_t thread;

  unsigned int nr_refs;		
  /* Detached threads have a self reference only,
	while joinable threads have two references.
	These are used to keep the structure valid at
	thread destruction.  Detaching/joining a thread
	drops a reference.  */

  /* Cancellation.  */
  pthread_mutex_t cancel_lock;	/* Protect cancel_xxx members.  */
  void (*cancel_hook) (void *);	
  /* Called to unblock a thread blocking
	in a cancellation point (namely,
	__pthread_cond_timedwait_internal).  */
  void *cancel_hook_arg;
  int cancel_state;
  int cancel_type;
  int cancel_pending;

	/* Thread stack.  */
	void *stackaddr;	/* 堆栈地址 */
	size_t stacksize;	/* 堆栈大小 */
	size_t guardsize;	/* 预留用来保护堆栈大小的字节 */
	int stack;			/* Nonzero if the stack was allocated.  */

	/* Exit status.  */
	void *status;

	/* Thread state.  */
	enum pthread_state state;
	pthread_mutex_t state_lock;	/* Locks the state.  */
	pthread_cond_t state_cond;	/* Signalled when the state changes.  */
	bool terminated;		/* Whether the kernel thread is over
							and we can reuse this structure.  */

	/* Resolver state.  */
	struct __res_state res_state;

	/* Indicates whether is a C11 thread created by thrd_creat.  */
	bool c11;

	/* Initial sigset for the thread.  */
	sigset_t init_sigset;

	/* Thread context.  */
	struct pthread_mcontext mcontext;

	PTHREAD_KEY_MEMBERS
	/*
	void **thread_specifics;		
  	// This is only resized by the thread, and always growing
	unsigned thread_specifics_size;	
	// Number of entries in thread_specifics
	*/

	PTHREAD_SYSDEP_MEMBERS
  	/*
	thread_t kernel_thread;
	mach_msg_header_t wakeupmsg;
	*/
	
	/* 线程控制块:与系统进行沟通 */
 	tcbhead_t *tcb;

	/* Queue links.  Since PREVP is used to determine 
	if a thread has been awaken, 
	it must be protected by the queue lock.  */
	struct __pthread *next, **prevp;
};

```

但是在我们使用的时候发现与我们正常的使用不太一致,在用户层,我们一般认为pthread为线程ID，但是内部实现好像都是指针,因此出现了什么特殊的原因；

在musl中,直接抹掉了内部结构;
```c
// 因此在此处使用了技巧,在内部和外部使用的定义形式不一致
#ifdef __cplusplus
typedef unsigned long pthread_t;
#else
typedef struct __pthread* pthread_t;
#endif

```
而在glibc中,线程ID仅仅是线程结构体中的一个成员,因此,glibc的处理更加安全,
下面分析一下id的实现,

```c
__pthread_create (pthread_t * thread, const pthread_attr_t * attr,
		  void *(*start_routine) (void *), void *arg)
{
  int err;
  struct __pthread *pthread;

  err = __pthread_create_internal (&pthread, attr, start_routine, arg);
  if (!err)
    *thread = pthread->thread;
  else if (err == ENOMEM)
    err = EAGAIN;

  return err;
}

/* 那么可以明白,线程ID */
int _dl_pthread_num_threads;
struct __pthread **_dl_pthread_threads;
__libc_rwlock_define_initialized (, _dl_pthread_threads_lock)

/* 下面的代码实现线程ID的分配:只保留了成功的部分,没有考虑意外情况 */
__libc_rwlock_wrlock (GL(dl_pthread_threads_lock));
if (GL(dl_pthread_num_threads) < __pthread_max_threads)
{
	/* We have a free slot.  Use the slot number plus one as the
	thread ID for the new thread.  */
	new->thread = 1 + GL(dl_pthread_num_threads)++;
	GL(dl_pthread_threads)[new->thread - 1] = NULL;

	__libc_rwlock_unlock (GL(dl_pthread_threads_lock));

	*pthread = new;
	return 0;
}
```
但是GL是什么呢?hehe
`#define GL(x) _##x`

## 所有的函数

### 线程基础

```c
// 线程创建
int pthread_create(pthread_t *__restrict, 
                   const pthread_attr_t *__restrict, 
                   void *(*)(void *), void *__restrict);

// 线程退出
void pthread_exit(void *);

// 线程属性
int pthread_attr_init(pthread_attr_t *);
int pthread_attr_destroy(pthread_attr_t *);

int pthread_attr_getguardsize(const pthread_attr_t *__restrict, size_t *__restrict);
int pthread_attr_setguardsize(pthread_attr_t *, size_t);
int pthread_attr_getstacksize(const pthread_attr_t *__restrict, size_t *__restrict);
int pthread_attr_setstacksize(pthread_attr_t *, size_t);
int pthread_attr_getdetachstate(const pthread_attr_t *, int *);
int pthread_attr_setdetachstate(pthread_attr_t *, int);
int pthread_attr_getstack(const pthread_attr_t *__restrict, void **__restrict, size_t *__restrict);
int pthread_attr_setstack(pthread_attr_t *, void *, size_t);
int pthread_attr_getscope(const pthread_attr_t *__restrict, int *__restrict);
int pthread_attr_setscope(pthread_attr_t *, int);
int pthread_attr_getschedpolicy(const pthread_attr_t *__restrict, int *__restrict);
int pthread_attr_setschedpolicy(pthread_attr_t *, int);
int pthread_attr_getschedparam(const pthread_attr_t *__restrict, struct sched_param *__restrict);
int pthread_attr_setschedparam(pthread_attr_t *__restrict, const struct sched_param *__restrict);
int pthread_attr_getinheritsched(const pthread_attr_t *__restrict, int *__restrict);
int pthread_attr_setinheritsched(pthread_attr_t *, int);
```

### 线程分离
```c
// 线程分离
int pthread_detach(pthread_t);

// 线程等待
int pthread_join(pthread_t, void **);
```

```c
// 获取线程自己的ID
pthread_t pthread_self(void);

// 判断线程是否相等
int pthread_equal(pthread_t, pthread_t); // 其实,之间简单的比较id就可以了
```

### 线程取消
```c
int pthread_setcancelstate(int, int *);
int pthread_setcanceltype(int, int *);
void pthread_testcancel(void);
int pthread_cancel(pthread_t);
```

### 调度相关
```c
int pthread_getschedparam(pthread_t t, 
                          int *restrict policy, 
                          struct sched_param *restrict param);
__syscall(SYS_sched_getparam, pthread_t->tid, sched_param);
__syscall(SYS_sched_getscheduler, pthread_t->tid);

int pthread_setschedparam(pthread_t t, 
                          int policy, 
                          const struct sched_param *param);
__syscall(SYS_sched_setscheduler, pthread_t->tid, policy, sched_param);
// sched_param保存着优先级参数

int pthread_setschedprio(pthread_t t, int prio);
__syscall(SYS_sched_setparam, pthread_t->tid, &prio);
```

## 补充知识

### Linux线程实现

### 系统调用实现

```c
#define __asm_syscall(...) 
do {
	__asm__ __volatile__ ( "svc 0" : "=r"(r0) : __VA_ARGS__ : "memory");
	return r0;
} while (0);

#define R7_OPERAND "r"(r7)
static inline long __syscall0(long n)
{
	register long r7 __ASM____R7__ = n; //使用R7传递个数
	register long r0 __asm__("r0");
	__asm_syscall(R7_OPERAND);
}

static inline long __syscall1(long n, long a)
{
	register long r7 __ASM____R7__ = n;
	register long r0 __asm__("r0") = a;
	__asm_syscall(R7_OPERAND, "0"(r0));
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
	register long r7 __ASM____R7__ = n;
	register long r0 __asm__("r0") = a;
	register long r1 __asm__("r1") = b;
	register long r2 __asm__("r2") = c;
	register long r3 __asm__("r3") = d;
	register long r4 __asm__("r4") = e;
	register long r5 __asm__("r5") = f;
	__asm_syscall(R7_OPERAND, "0"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5));
}

// 一个相当巧妙的宏定义的实现
#define __SYSCALL_NARGS_X(a,b,c,d,e,f,g,h,n,...) n
#define __SYSCALL_NARGS(...) __SYSCALL_NARGS_X(__VA_ARGS__,7,6,5,4,3,2,1,0,)
// 每次添加一个参数,就会将数字向后面推一个位置,形成参数个数

// 拼装函数调用
#define __SYSCALL_CONCAT_X(a,b) a##b
#define __SYSCALL_CONCAT(a,b) __SYSCALL_CONCAT_X(a,b)
#define __SYSCALL_DISP(b,...) __SYSCALL_CONCAT(b,__SYSCALL_NARGS(__VA_ARGS__))(__VA_ARGS__)

// __syscall_ret仅仅检查了系统调用号
#define __syscall(...) __SYSCALL_DISP(__syscall,__VA_ARGS__)
#define syscall(...) __syscall_ret(__syscall(__VA_ARGS__))
```

1. `syscall(SYS_close, fd)` 原始函数
2. `__syscall_ret(__syscall(SYS_close, fd))` syscall宏定义展开
3. `__syscall(SYS_close, fd)` 直接拿掉`syscall_ret`
4. `__SYSCALL_DISP(__syscall, SYS_close, fd)` 展开__syscall
5. `__SYSCALL_CONCAT(__syscall, __SYSCALL_NARGS(SYS_close, fd))(SYS_close, fd)`
6. `__SYSCALL_CONCAT(_syscall, 1, (SYS_close, fd))`
7. `__syscall1(SYS_close, fd)` 生成结束


系统调用号实现
```c
#ifndef _UAPI_ASM_ARM_UNISTD_COMMON_H
#define _UAPI_ASM_ARM_UNISTD_COMMON_H 1
#define __NR_restart_syscall (__NR_SYSCALL_BASE + 0)
#define __NR_exit (__NR_SYSCALL_BASE + 1)
#define __NR_fork (__NR_SYSCALL_BASE + 2)
#define __NR_read (__NR_SYSCALL_BASE + 3)
#define __NR_write (__NR_SYSCALL_BASE + 4)
#define __NR_open (__NR_SYSCALL_BASE + 5)
#define __NR_close (__NR_SYSCALL_BASE + 6)
#define __NR_creat (__NR_SYSCALL_BASE + 8)
...
#define __NR_io_pgetevents (__NR_SYSCALL_BASE + 399)
#endif /* _UAPI_ASM_ARM_UNISTD_COMMON_H */
```
### glibc系统调用
同样的道理,可以分析一下glibc的系统调用时如何进行
但是在一般的情况下此时就可以明白,与musl相同的调用方式时一样的

```c
INLINE_SYSCALL_CALL --> __INLINE_SYSCALL_DISP
__INLINE_SYSCALL_DISP -->  __SYSCALL_CONCAT
```

生成一条调用指令的语言

```c
# define INTERNAL_SYSCALL_RAW(name, nr, args...)		\
  ({								\
       register int _a1 asm ("r0"), _nr asm ("r7");		\
       LOAD_ARGS_##nr (args)					\
       _nr = name;						\
       asm volatile ("swi	0x0	@ syscall " #name	\
		     : "=r" (_a1)				\
		     : "r" (_nr) ASM_ARGS_##nr			\
		     : "memory");				\
       _a1; })
```

### clone系统调用
```c
int __clone(int (*func)(void *), void *stack, int flags, void *arg, ...);

// 但是实现的地方却存在着异常

__clone:
	stmfd sp!,{r4,r5,r6,r7}

	mov r7,#120
        
	mov r6,r3
	mov r5,r0
	mov r0,r2

	and r1,r1,#-16
	ldr r2,[sp,#16]
	ldr r3,[sp,#20]
	ldr r4,[sp,#24]
	svc 0
	
    tst r0,r0
	beq 1f

	ldmfd sp!,{r4,r5,r6,r7}
	bx lr
```

## 线程基础

### 线程创建

```c
typedef struct { 
    union { 
        int __i[sizeof(long)==8?14:9]; 
        volatile int __vi[sizeof(long)==8?14:9]; 
        unsigned long __s[sizeof(long)==8?7:9]; 
    } __u; 
} pthread_attr_t;

#define __SU (sizeof(size_t)/sizeof(int))

#define _a_stacksize __u.__s[0]
#define _a_guardsize __u.__s[1]
#define _a_stackaddr __u.__s[2]
#define _a_detach __u.__i[3*__SU+0]
#define _a_sched __u.__i[3*__SU+1]
#define _a_policy __u.__i[3*__SU+2]
#define _a_prio __u.__i[3*__SU+3]
#define _m_type __u.__i[0]
#define _m_lock __u.__vi[1]
#define _m_waiters __u.__vi[2]
#define _m_prev __u.__p[3]
#define _m_next __u.__p[4]
#define _m_count __u.__i[5]
#define _c_shared __u.__p[0]
#define _c_seq __u.__vi[2]
#define _c_waiters __u.__vi[3]
#define _c_clock __u.__i[4]
#define _c_lock __u.__vi[8]
#define _c_head __u.__p[1]
#define _c_tail __u.__p[5]
#define _rw_lock __u.__vi[0]
#define _rw_waiters __u.__vi[1]
#define _rw_shared __u.__i[2]
#define _b_lock __u.__vi[0]
#define _b_waiters __u.__vi[1]
#define _b_limit __u.__i[2]
#define _b_count __u.__vi[3]
#define _b_waiters2 __u.__vi[4]
#define _b_inst __u.__p[3]

/**
 * @fn int pthread_create(pthread_t* restrict, const pthread_attr_t* restrict, void*(*)(void*), void* restrict)
 * @brief 创建线程
 *
 * @param res 		pthread指针
 * @param attrp		pthread属性
 * @param entry		入口地址
 * @param arg		入口属性
 * @return			创建结果
 */
int pthread_create(pthread_t *restrict res,
				   const pthread_attr_t *restrict attrp,
				   void *(*entry)(void *), 
                   void *restrict arg)
{
    int ret, c11 = (attrp == __ATTRP_C11_THREAD);
	size_t size;
    size_t guard;
	struct pthread *self;
    struct pthread *new;
	unsigned char *map = 0, *stack = 0, *tsd = 0, *stack_limit;
	unsigned flags = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND
		| CLONE_THREAD | CLONE_SYSVSEM | CLONE_SETTLS
		| CLONE_PARENT_SETTID | CLONE_CHILD_CLEARTID | CLONE_DETACHED;
	
    // 通过一定的规则将attrp-->attr
    pthread_attr_t attr = { 0 };
	sigset_t set;
    
    /* 如果没有设置堆栈大小,先设置默认的堆栈大小 */
    attr._a_stacksize = __default_stacksize;
	attr._a_guardsize = __default_guardsize;
    
    /* 填充pthread成员 */
    new = __copy_tls(tsd - libc.tls_size);
	new->map_base = map;
	new->map_size = size;
	new->stack = stack;
	new->stack_size = stack - stack_limit;
	new->guard_size = guard;
	new->self = new;
	new->tsd = (void *)tsd;
	new->locale = &libc.global_locale;
	if (attr._a_detach) {
		new->detach_state = DT_DETACHED;
	} else {
		new->detach_state = DT_JOINABLE;
	}
	new->robust_list.head = &new->robust_list.head;
	new->canary = self->canary;
	new->sysinfo = self->sysinfo;
    
    /* 移动stack指针,保存启动参数 */
    stack -= (uintptr_t)stack % sizeof(uintptr_t);
	stack -= sizeof(struct start_args);
	
    /* 设置启动参数 */
    struct start_args *args = (void *)stack;
	args->start_func = entry;
	args->start_arg = arg;
	args->control = attr._a_sched ? 1 : 0;
    
    /* 调用clone创建线程  */
    clone(start, stack, flags, args, &new->tid, TP_ADJ(new), &__thread_list_lock);
	
    /* 设置调度器 */
    ret = __syscall(SYS_sched_setscheduler, new->tid, attr._a_policy, &attr._a_prio);
	if (a_swap(&args->control, ret ? 3 : 0)==2)
		__wake(&args->control, 1, 1);
	if (ret)
		__wait(&args->control, 0, 3, 0);
    
    /* 初始化链表 */
    new->next = self->next;
	new->prev = self;
	new->next->prev = new;
	new->prev->next = new;
    
    /* 返回new作为线程ID */
}

void __pthread_exit(void *result)
{
	pthread_t self = __pthread_self();
	sigset_t set;
    
    /*  设置退出标志 */
    self->canceldisable = 1;
	self->cancelasync = 0;
	self->result = result;
	
    /* 执行线程清理函数 */
	while (self->cancelbuf) {
		void (*f)(void *) = self->cancelbuf->__f;
		void *x = self->cancelbuf->__x;
		self->cancelbuf = self->cancelbuf->__next;
		f(x);
	}

    int state = a_cas(&self->detach_state, DT_JOINABLE, DT_EXITING);
	
    /* 如果线程分离状态,则代表需要自己手动释放内存 */
	if (state==DT_DETACHED && self->map_base) {
        __vm_wait();
	}
    
    volatile void *volatile *rp;
	while ((rp=self->robust_list.head) && rp != &self->robust_list.head) {
		pthread_mutex_t *m = (void *)((char *)rp
			- offsetof(pthread_mutex_t, _m_next));
		int waiters = m->_m_waiters;
		int priv = (m->_m_type & 128) ^ 128;
		self->robust_list.pending = rp;
		self->robust_list.head = *rp;
		int cont = a_swap(&m->_m_lock, 0x40000000);
		self->robust_list.pending = 0;
		if (cont < 0 || waiters)
			__wake(&m->_m_lock, 1, priv);
	}

    self->next->prev = self->prev;
	self->prev->next = self->next;
	self->prev = self->next = self;

	if (state==DT_DETACHED && self->map_base) {
		if (self->robust_list.off) {
			__syscall(SYS_set_robust_list, 0, 3*sizeof(long));
        }
        
		__unmapself(self->map_base, self->map_size);
	}
    
    /* 设置线程为退出 */
    a_store(&self->detach_state, DT_EXITED);
	
    /* 唤醒 */
    __wake(&self->detach_state, 1, 1);

    /* 清除线程ID  */
	self->tid = 0;

    /* 杀死线程 */
	for (;;) __syscall(SYS_exit, 0);
}
```

#### glibc实现的线程函数

```c

int pthread_create (pthread_t * thread, const pthread_attr_t * attr,
				    void *(*start_routine) (void *), void *arg)
{
	struct __pthread *pthread;
	const struct __pthread_attr *setup;
	sigset_t sigset;
	size_t stacksize;
	int err;	
	
	err = __pthread_alloc (&pthread); // 申请pthread
	
	/* 填充堆栈大小,优先级从上到下 */
	
	/* 1. 用户设定 */
	stacksize = setup->__stacksize;
	
	/* 2. 系统限制 */
    __getrlimit (RLIMIT_STACK, &rlim);
	stacksize = rlim.rlim_cur;
	
	/* 3. 默认大小8M */
	stacksize = PTHREAD_STACK_DEFAULT;
	
	/* 最后将结果回填 */
	pthread->stacksize = stacksize;
	
	/* 确定PTHREAD_DETACHED/PTHREAD_JOINABLE */
	pthread->state = (setup->__detachstate == PTHREAD_CREATE_DETACHED
					  ? PTHREAD_DETACHED : PTHREAD_JOINABLE);
	
	/* 1. 填充堆栈:用户提供堆栈 */
	pthread->stackaddr = setup->__stackaddr;
	pthread->guardsize = 0;
	pthread->stack = 0;

	/* 2. 用户没有提供堆栈 */
	__pthread_stack_alloc (&pthread->stackaddr,
				   ((setup->__guardsize + __vm_page_size - 1)
				    / __vm_page_size) * __vm_page_size
				   + stacksize);
	pthread->guardsize = setup->__guardsize;
	pthread->stack = 1;
    
    /* 申请内核线程 */
    __pthread_thread_alloc (pthread);
    
    /* 申请线程控制块 */
    pthread->tcb = _dl_allocate_tls (NULL);
    pthread->tcb->tcb = pthread->tcb;
    
    /* 设置入口地址等相关参数 */
	__pthread_setup (pthread, entry_point, start_routine, arg);
    
    /* 初始化信号 */
    __pthread_sigstate_init (pthread);
    __pthread_sigstate (_pthread_self(), 0, 0, &pthread->init_sigset, 0);
    
    /* 增加计数 */
    atomic_increment (&__pthread_total);
    __libc_rwlock_rdlock (GL (dl_pthread_threads_lock));
  	GL (dl_pthread_threads)[pthread->thread - 1] = pthread;
  	__libc_rwlock_unlock (GL (dl_pthread_threads_lock));
    
    /* 创建结束,返回线程ID  */
	*thread = pthread->thread;
    
    /* 启动调度 */
    __pthread_thread_start(pthread);
}

```

将其中的函数拆解开进行分析

#### pthread管理单元申请与释放

1. 全局free线程链表中获取
```c
// __pthread_alloc
struct __pthread *__pthread_free_threads;
pthread_mutex_t __pthread_free_threads_lock;

__pthread_mutex_lock (&__pthread_free_threads_lock);
for (new = __pthread_free_threads; new; new = new->next)
{
	/* 此标志为1才代表完全退出 */
	if (new->terminated)
	{
		__pthread_dequeue(new);
		break;
	}
}
__pthread_mutex_unlock(&__pthread_free_threads_lock);

// __pthread_dealloc
__pthread_mutex_lock (&__pthread_free_threads_lock);
__pthread_enqueue (&__pthread_free_threads, pthread);
__pthread_mutex_unlock (&__pthread_free_threads_lock);

/* 添加和删除的操作较为简单,可自行阅读 */
__pthread_enqueue (struct __pthread **head, struct __pthread *thread);
__pthread_dequeue (struct __pthread *thread);
// 操作的成员分别是 prev/next
```
2. 直接调用malloc进行申请
3. 无论是复用还是 重新申请,最后需要调用`initialize_pthread`进行初始化,此函数初始化默认参数

#### 内核线程创建

```c
int __pthread_thread_alloc (struct __pthread *thread)
{
    /// 创建唤醒消息
	create_wakeupmsg(thread);
    
    /// 创建内核线程
    __thread_create(__mach_task_self(), &thread->kernel_thread);
}
```

#### TLS创建

```c
void *_dl_allocate_tls (void *mem)
{
    void *result = NULLL;
    
    if (NULL == mem)
    {
        result = _dl_allocate_tls_storage();
    }
    else
    {
        result = allocate_dtv(mem);
    }
    
    return _dl_allocate_tls_init(result);
}
```

#### 设置启动参数

```c
// 1. 线程实体
// 2. 入口函数 entry_point --> start_routine(arg); 
// 3. 用户线程
// 4. 用户参数
int __pthread_setup (struct __pthread *thread,
	void (*entry_point) (struct __pthread *, void *(*)(void *), void *), 
	void *(*start_routine) (void *), void *arg)
{	
    /* 设置线程上下文:此时应该思考一个问题，什么叫做上下文 */
    thread->mcontext.pc = entry_point;
  	thread->mcontext.sp = stack_setup (thread, start_routine, arg);
}

```

### ptrhead进程属性机制
```c

int pthread_attr_init(pthread_attr_t *a);
pthread_attr_t->_a_stacksize = __default_stacksize;
pthread_attr_t->_a_guardsize = __default_guardsize;

int pthread_attr_setdetachstate(pthread_attr_t *a, int state);
pthread_attr_t->_a_detach = state;

int pthread_attr_setguardsize(pthread_attr_t *a, size_t size);
pthread_attr_t->_a_guardsize = size;

int pthread_attr_setinheritsched(pthread_attr_t *a, int inherit);
pthread_attr_t->_a_sched = inherit;

int pthread_attr_setschedparam(pthread_attr_t *restrict a, 
const struct sched_param *restrict param);
pthread_attr_t->_a_prio = param->sched_priority;

int pthread_attr_setschedpolicy(pthread_attr_t *a, int policy);
pthread_attr_t->_a_policy = policy;

int pthread_attr_setstack(pthread_attr_t *a, void *addr, size_t size);
pthread_attr_t->_a_stackaddr = (size_t)addr + size;
pthread_attr_t->_a_stacksize = size;

int pthread_attr_setstacksize(pthread_attr_t *a, size_t size);
pthread_attr_t->_a_stackaddr = 0;
pthread_attr_t->_a_stacksize = size;

```
### 自身线程ID
```c
// 在musl和glibc的实现上二者不太一样

static inline uintptr_t __get_tp()
{
	uintptr_t tp;
	__asm__ ( "mrc p15,0,%0,c13,c0,3" : "=r"(tp) );
	return tp;
}

// 获取线程自身的方法
#define __pthread_self() ((pthread_t)(__get_tp() - sizeof(struct __pthread) - TP_OFFSET))

// 二者实现不太一样
/* Return the thread descriptor for the current thread.  */
# define THREAD_SELF  ((struct pthread *)__builtin_thread_pointer () - 1)
pthread_t __pthread_self (void)
{
  return (pthread_t) THREAD_SELF;
}
```

### 线程清理函数

一句话:就是设置`pthread->cancelbuf`成员函数

```c
#define pthread_cleanup_push(f, x) \
do { \
	struct __ptcb __cb; \
    _pthread_cleanup_push(&__cb, f, x); \

#define pthread_cleanup_pop(r) \
	_pthread_cleanup_pop(&__cb, (r)); \
} while(0)

void _pthread_cleanup_push(struct __ptcb *cb, void (*f)(void *), void *x)
{
	cb->__f = f;
	cb->__x = x;
	__do_cleanup_push(cb);
}

void _pthread_cleanup_pop(struct __ptcb *cb, int run)
{
	__do_cleanup_pop(cb);
	if (run) cb->__f(cb->__x);
}

void __do_cleanup_push(struct __ptcb *cb)
{
	struct pthread *self = __pthread_self();
	cb->__next = self->cancelbuf;
	self->cancelbuf = cb;
}

void __do_cleanup_pop(struct __ptcb *cb)
{
	__pthread_self()->cancelbuf = cb->__next;
}
```

#### cancle设置
```c
int __pthread_setcancelstate(int new, int *old);
self->canceldisable --> old;
self->canceldisable <-- new;

int pthread_setcanceltype(int new, int *old);
self->cancelasync --> old;
self->cancelasync <-- new;
// 同时还会执行__pthread_testcancel
	
int pthread_cancel(pthread_t t)
{
	// 自己的线程,直接退出
	pthread_exit(PTHREAD_CANCELED);
	
	// 不是自己,发送退出信息
	pthread_kill(t, SIGCANCEL)
}

// 设置线程取消点
__pthread_testcancel --> __testcancel --> __cancel()
void __pthread_testcancel()
{
	__testcancel();
}

void __testcancel()
{
	pthread_t self = __pthread_self();
	if (self->cancel && !self->canceldisable)
		__cancel();
}

/* 允许进行cancle才可以 */
long __cancel()
{
	pthread_t self = __pthread_self();
	if (self->canceldisable == PTHREAD_CANCEL_ENABLE ||
    	self->cancelasync)
		pthread_exit(PTHREAD_CANCELED);
	self->canceldisable = PTHREAD_CANCEL_DISABLE;
	return -ECANCELED;
}
// 此处产生效果,强行将pthread_testcancel退出点设置到此处
```

但是此时还是一个问题,从信号到处理究竟发生了什么?

因此需要分析一下pthread_kill发生了什么?

```c
int pthread_kill(pthread_t t, int sig)
{
	__block_all_sigs(&set);
	LOCK(t->killlock);
    
    // 仅仅对线程发送了一个信号
	__syscall(SYS_tkill, t->tid, sig);
	
    UNLOCK(t->killlock);
	__restore_sigs(&set);
}

// 那么在此回到pthread_cancle函数
static void init_cancellation()
{
	struct sigaction sa = {
		.sa_flags = SA_SIGINFO | SA_RESTART,
		.sa_sigaction = cancel_handler
	};
	memset(&sa.sa_mask, -1, _NSIG/8);
	__libc_sigaction(SIGCANCEL, &sa, 0);
}

int pthread_cancel(pthread_t t)
{
    /* 此处存在这一个初始化 */
	static int init;
	if (!init) {
		init_cancellation();
		init = 1;
	}
}

static void cancel_handler(int sig, siginfo_t *si, void *ctx)
{
	pthread_t self = __pthread_self();
	ucontext_t *uc = ctx;
	uintptr_t pc = uc->uc_mcontext.MC_PC;

	a_barrier();
	if (!self->cancel || self->canceldisable == PTHREAD_CANCEL_DISABLE) return;

	_sigaddset(&uc->uc_sigmask, SIGCANCEL);

	if (self->cancelasync || pc >= (uintptr_t)__cp_begin && pc < (uintptr_t)__cp_end) {
		uc->uc_mcontext.MC_PC = (uintptr_t)__cp_cancel;
#ifdef CANCEL_GOT
		uc->uc_mcontext.MC_GOT = CANCEL_GOT;
#endif
		return;
	}

	__syscall(SYS_tkill, self->tid, SIGCANCEL);
}
```

在musl中分析结束,现在分析一下glibc是如何实现的？
```c
// 确实,在glibc中更加清晰,但是我还是没有明白,为什么会在此处进行退出呢
void __pthread_testcancel (void)
{
  struct __pthread *p = _pthread_self ();
  int cancelled;

  __pthread_mutex_lock (&p->cancel_lock);
  cancelled = (p->cancel_state == PTHREAD_CANCEL_ENABLE) && p->cancel_pending;
  __pthread_mutex_unlock (&p->cancel_lock);

  if (cancelled)
    __pthread_exit (PTHREAD_CANCELED);
}

// 但是glibc没有什么特别的差别

```


### 线程分离

```c
// 线程分离
int pthread_detach(pthread_t)
--> __pthread_join(pthread_t, 0)
--> __pthread_timedjoin_np(pthread_t, 0, 0)


// 设置线程等待
int pthread_join(pthread_t, void **);
-->__pthread_timedjoin_np(pthread_t, res, 0)

// 那么也就是说明都调用了相同的函数

static int __pthread_timedjoin_np(pthread_t t, void **res, const struct timespec *at)
{
	/* 设置线程分离状态 */
}
```

