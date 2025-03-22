# Linux内核同步笔记


<!--more-->
Linux内核同步实现
<!--more-->

## 原子变量

### 原子变量数据结构

https://elixir.bootlin.com/linux/v4.0/source/include/linux/types.h#L175

```c
typedef struct {
	int counter;
} atomic_t;

typedef struct {
	long long counter;
} atomic64_t;
```

### 提供的操作

| **类别**       | **函数原型**                              | **功能**                          |
| :------------- | :---------------------------------------- | :-------------------------------- |
| **初始化**     | `ATOMIC_INIT(int val)`                    | 静态初始化原子变量为`val`。       |
|                | `atomic_set(atomic_t *v, int i)`          | 动态设置原子变量值为`i`。         |
| **基础操作**   | `atomic_read(const atomic_t *v)`          | 读取原子变量的值。                |
|                | `atomic_add(int i, atomic_t *v)`          | 原子地将`v`的值增加`i`。          |
|                | `atomic_sub(int i, atomic_t *v)`          | 原子地将`v`的值减少`i`。          |
|                | `atomic_inc(atomic_t *v)`                 | 原子地递增`v`（等价于`v += 1`）。 |
|                | `atomic_dec(atomic_t *v)`                 | 原子地递减`v`（等价于`v -= 1`）。 |
| **条件操作**   | `atomic_inc_and_test(atomic_t *v)`        | 递增`v`，若结果为0返回`true`。    |
|                | `atomic_dec_and_test(atomic_t *v)`        | 递减`v`，若结果为0返回`true`。    |
|                | `atomic_sub_and_test(int i, atomic_t *v)` | 减去`i`，若结果为0返回`true`。    |
| **返回值操作** | `atomic_add_return(int i, atomic_t *v)`   | 增加`i`并返回新值。               |
|                | `atomic_sub_return(int i, atomic_t *v)`   | 减少`i`并返回新值。               |



| **函数原型**                                               | **功能**                            |
| :--------------------------------------------------------- | :---------------------------------- |
| `set_bit(int nr, volatile unsigned long *addr)`            | 原子地设置地址`addr`的第`nr`位为1。 |
| `clear_bit(int nr, volatile unsigned long *addr)`          | 原子地清除地址`addr`的第`nr`位为0。 |
| `test_and_set_bit(int nr, volatile unsigned long *addr)`   | 设置第`nr`位为1，返回旧值。         |
| `test_and_clear_bit(int nr, volatile unsigned long *addr)` | 清除第`nr`位为0，返回旧值。         |

通用的模式

```c
static inline void set_bit(int nr, volatile unsigned long *addr)
{
	unsigned long mask = BIT_MASK(nr);
	unsigned long *p = ((unsigned long *)addr) + BIT_WORD(nr);
	unsigned long flags;

	_atomic_spin_lock_irqsave(p, flags);
	*p  |= mask;
	_atomic_spin_unlock_irqrestore(p, flags);
}

static inline void clear_bit(int nr, volatile unsigned long *addr)
{
	unsigned long mask = BIT_MASK(nr);
	unsigned long *p = ((unsigned long *)addr) + BIT_WORD(nr);
	unsigned long flags;

	_atomic_spin_lock_irqsave(p, flags);
	*p &= ~mask;
	_atomic_spin_unlock_irqrestore(p, flags);
}
```

如果使用高效率的方式，使用汇编



### 读取/写入原子变量

https://elixir.bootlin.com/linux/v4.0/source/arch/arm/include/asm/atomic.h#L30

```c
#define atomic_read(v)	ACCESS_ONCE((v)->counter)
#define atomic_set(v,i)	(((v)->counter) = (i))
```

[参考链接](https://elixir.bootlin.com/linux/v4.0/source/include/linux/compiler.h#L470)

```c
#define __ACCESS_ONCE(x) ({ \
	 __maybe_unused typeof(x) __var = (__force typeof(x)) 0; \
	(volatile typeof(x) *)&(x); })
#define ACCESS_ONCE(x) (*__ACCESS_ONCE(x))
```

## 自旋锁

### 数据结构

https://elixir.bootlin.com/linux/v4.0/source/include/linux/spinlock_types.h#L32

```c
typedef struct raw_spinlock {
	arch_spinlock_t raw_lock;
#ifdef CONFIG_GENERIC_LOCKBREAK
	unsigned int break_lock;
#endif
#ifdef CONFIG_DEBUG_SPINLOCK
	unsigned int magic, owner_cpu;
	void *owner;
#endif
#ifdef CONFIG_DEBUG_LOCK_ALLOC
	struct lockdep_map dep_map;
#endif
} raw_spinlock_t;
```

进入arch的实现

https://elixir.bootlin.com/linux/v4.0/source/arch/arm/include/asm/spinlock_types.h#L23

```c
typedef struct {
	union {
		u32 slock;
		struct __raw_tickets {
#ifdef __ARMEB__
			u16 next;
			u16 owner;
#else
			u16 owner;
			u16 next;
#endif
		} tickets;
	};
} arch_spinlock_t;
```

https://elixir.bootlin.com/linux/v4.0/source/include/linux/spinlock.h#L188

### 加锁

```c
static inline void spin_lock(spinlock_t *lock)
{
	raw_spin_lock(&lock->rlock);
}

#define raw_spin_lock(lock)	_raw_spin_lock(lock)
```

```c
static inline void spin_lock_bh(spinlock_t *lock)
{
	raw_spin_lock_bh(&lock->rlock);
}
```

```c
static inline int spin_trylock(spinlock_t *lock)
{
	return raw_spin_trylock(&lock->rlock);
}
```

### 解锁

```c
static inline void spin_unlock(spinlock_t *lock)
{
	raw_spin_unlock(&lock->rlock);
}
```

```c
static inline void spin_unlock_bh(spinlock_t *lock)
{
	raw_spin_unlock_bh(&lock->rlock);
}
```

```c
static inline void spin_unlock_irq(spinlock_t *lock)
{
	raw_spin_unlock_irq(&lock->rlock);
}
```



## 互斥锁

### 数据结构

https://elixir.bootlin.com/linux/v4.0/source/include/linux/mutex.h#L50

```c
struct mutex {
	/* 1: unlocked, 0: locked, negative: locked, possible waiters */
	atomic_t		count;
	spinlock_t		wait_lock;
	struct list_head	wait_list;
#if defined(CONFIG_DEBUG_MUTEXES) || defined(CONFIG_MUTEX_SPIN_ON_OWNER)
	struct task_struct	*owner;
#endif
#ifdef CONFIG_MUTEX_SPIN_ON_OWNER
	struct optimistic_spin_queue osq; /* Spinner MCS lock */
#endif
#ifdef CONFIG_DEBUG_MUTEXES
	void			*magic;
#endif
#ifdef CONFIG_DEBUG_LOCK_ALLOC
	struct lockdep_map	dep_map;
#endif
};
```

如何判断mutex加锁

```c
/**
 * mutex_is_locked - is the mutex locked
 * @lock: the mutex to be queried
 *
 * Returns 1 if the mutex is locked, 0 if unlocked.
 */
static inline int mutex_is_locked(struct mutex *lock)
{
	return atomic_read(&lock->count) != 1;
}
```

### 加锁操作

```c
void __sched mutex_lock(struct mutex *lock)
{
	might_sleep();
	/*
	 * The locking fastpath is the 1->0 transition from
	 * 'unlocked' into 'locked' state.
	 */
	__mutex_fastpath_lock(&lock->count, __mutex_lock_slowpath);
	mutex_set_owner(lock);
}

/* 直接设置owner */
static inline void mutex_set_owner(struct mutex *lock)
{
	lock->owner = current;
}
```

下面的函数分析:

1. 如果可以直接设置原子变量
2. 进入慢速的实现

```c
static inline void
__mutex_fastpath_lock(atomic_t *count, void (*fail_fn)(atomic_t *))
{
	if (unlikely(atomic_xchg(count, 0) != 1))
		if (likely(atomic_xchg(count, -1) != 1))
			fail_fn(count)
}
```

分析慢速实现
```c
__visible void __sched
__mutex_lock_slowpath(atomic_t *lock_count)
{
	struct mutex *lock = container_of(lock_count, struct mutex, count);

	__mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, 0,
			    NULL, _RET_IP_, NULL, 0);
}
```
https://elixir.bootlin.com/linux/v4.0/source/kernel/locking/mutex.c#L517

### 解锁实现

https://elixir.bootlin.com/linux/v4.0/source/kernel/locking/mutex.c#L421

```c
void __sched mutex_unlock(struct mutex *lock)
{
#ifndef CONFIG_DEBUG_MUTEXES
	mutex_clear_owner(lock);
#endif
	__mutex_fastpath_unlock(&lock->count, __mutex_unlock_slowpath);
}
```

进入快速释放函数

```c
static inline void
__mutex_fastpath_unlock(atomic_t *count, void (*fail_fn)(atomic_t *))
{
	if (unlikely(atomic_xchg(count, 1) != 0))
		fail_fn(count);
}
```

进入慢速函数实现

https://elixir.bootlin.com/linux/v4.0/source/kernel/locking/mutex.c#L724

## 信号量

### 数据结构

```c
struct semaphore {
	raw_spinlock_t		lock;
	unsigned int		count;
	struct list_head	wait_list;
};

#define __SEMAPHORE_INITIALIZER(name, n)				\
{									\
	.lock		= __RAW_SPIN_LOCK_UNLOCKED((name).lock),	\
	.count		= n,						\
	.wait_list	= LIST_HEAD_INIT((name).wait_list),		\
}
```

### down操作

```c
void down(struct semaphore *sem)
{
	unsigned long flags;

	raw_spin_lock_irqsave(&sem->lock, flags);
	if (likely(sem->count > 0))
		sem->count--;
	else
		__down(sem);
	raw_spin_unlock_irqrestore(&sem->lock, flags);
}
```

进入内核实现
https://elixir.bootlin.com/linux/v4.0/source/kernel/locking/semaphore.c#L204

### up操作

```c
void up(struct semaphore *sem)
{
	unsigned long flags;

	raw_spin_lock_irqsave(&sem->lock, flags);
	if (likely(list_empty(&sem->wait_list)))
		sem->count++;
	else
		__up(sem);
	raw_spin_unlock_irqrestore(&sem->lock, flags);
}
```

进入up的具体实现

```c
static noinline void __sched __up(struct semaphore *sem)
{
	struct semaphore_waiter *waiter = list_first_entry(&sem->wait_list,
						struct semaphore_waiter, list);
	list_del(&waiter->list);
	waiter->up = true;
	wake_up_process(waiter->task);
}
```

## 读写锁

Linux内核中的读写锁（Reader-Writer Locks）主要用于优化对共享资源的高并发访问，尤其是在读多写少的场景下。其设计目标是在保证数据一致性的前提下，最大化读取操作的并行性，同时确保写入操作的独占性。以下是Linux内核中两种主要读写锁的实现分析：

---

### **1. `rwlock_t`（自旋读写锁）**
**适用场景**：不可休眠的上下文（如中断处理程序、原子上下文），基于自旋锁实现。

#### **数据结构**
```c
// include/linux/rwlock_types.h
typedef struct {
    arch_rwlock_t raw_lock;  // 架构相关的底层锁表示
} rwlock_t;
```
- **底层实现**：依赖于架构（如x86使用原子计数和状态标志），通常通过原子指令或自旋锁机制实现。

#### **核心操作**
- **读者加锁**：
  ```c
  read_lock(rwlock_t *lock);
  ```
  - 原子操作增加读者计数，若当前无写者持有锁则立即获得访问权；否则自旋等待。

- **写者加锁**：
  ```c
  write_lock(rwlock_t *lock);
  ```
  - 等待所有读者释放锁后，独占获取锁，期间通过自旋避免上下文切换。

- **解锁**：
  ```c
  read_unlock(rwlock_t *lock);
  write_unlock(rwlock_t *lock);
  ```
  - 减少读者计数或释放写锁，唤醒等待的读者/写者。

#### **实现特点**
- **无休眠**：读者和写者均通过自旋等待，适用于非抢占式上下文。
- **低开销**：无上下文切换成本，适合极短临界区。
- **潜在问题**：长时间持有锁会导致CPU资源浪费，可能引发写者饥饿（若读者持续进入）。

---

### **2. `struct rw_semaphore`（读写信号量）**
**适用场景**：可休眠的进程上下文，基于信号量实现，支持复杂的同步策略。

#### **数据结构**
```c
// include/linux/rwsem.h
struct rw_semaphore {
    atomic_long_t count;      // 组合计数器：高32位为等待计数，低32位为活跃读者/写者标志
    struct list_head wait_list;  // 等待队列（读者或写者）
    // 其他调试和优化字段...
};
```
- **计数器设计**：
  - **写者标记**：若最低位（`0x1`）为1，表示写者持有锁。
  - **读者计数**：低32位中除最低位的其余位表示活跃读者数量。
  - **等待计数**：高32位记录等待的写者或读者数量。

#### **核心操作**
- **读者加锁**：
  ```c
  down_read(struct rw_semaphore *sem);
  ```
  - 若当前无写者持有锁且无写者等待，立即增加读者计数；否则加入等待队列并休眠。

- **写者加锁**：
  ```c
  down_write(struct rw_semaphore *sem);
  ```
  - 等待所有活跃读者/写者释放锁后独占访问，期间休眠让出CPU。

- **解锁**：
  ```c
  up_read(struct rw_semaphore *sem);
  up_write(struct rw_semaphore *sem);
  ```
  - 减少读者计数或清除写者标记，唤醒队列中的等待者（优先唤醒写者以避免饥饿）。

#### **实现特点**
- **公平性策略**：
  - **写者优先**：当写者等待时，新读者会被阻塞，防止写者饥饿。
  - **等待队列**：通过FIFO或优先级调度管理等待者，平衡读者与写者的公平性。
- **休眠支持**：在无法立即获取锁时，任务进入休眠状态，避免CPU空转。
- **优化机制**：
  - **无竞争快速路径**：无竞争时直接通过原子操作完成加锁/解锁，无需操作等待队列。
  - **乐观自旋**：在某些实现中，写者可能会短暂自旋以尝试快速获取锁，减少休眠开销。

---

### **3. 性能与设计权衡**
| **特性**     | **`rwlock_t`**                     | **`rw_semaphore`**         |
| ------------ | ---------------------------------- | -------------------------- |
| **上下文**   | 仅限非休眠上下文（中断、原子操作） | 进程上下文（可休眠）       |
| **等待机制** | 自旋等待（忙等待）                 | 休眠等待                   |
| **公平性**   | 无明确策略，可能写者饥饿           | 写者优先，减少饥饿风险     |
| **开销**     | 低（无上下文切换）                 | 较高（休眠唤醒、队列管理） |
| **适用场景** | 极短临界区、高频读操作             | 长临界区、需公平性保障     |

---

### **4. 实现细节分析**
- **原子操作与内存屏障**：
  - `rwlock_t`使用原子指令（如`atomic_add`）管理读者计数，结合内存屏障（`barrier()`）确保操作顺序。
  - `rw_semaphore`通过`atomic_long_t`的原子操作更新组合计数器，配合显式屏障（如`smp_mb__before_atomic`）保证多核一致性。

- **调试与检测**：
  - **锁依赖检测**：内核配置`CONFIG_DEBUG_LOCK_ALLOC`时，会跟踪锁的获取顺序以防止死锁。
  - **死锁预警**：通过`CONFIG_DEBUG_ATOMIC_SLEEP`检查在原子上下文中误用`rw_semaphore`。

---

### **5. 示例场景**
**场景1：网络包统计（高频读，低频写）**
- 使用`rwlock_t`保护全局统计计数器：
  ```c
  rwlock_t stats_lock;
  unsigned int packet_count;

  // 读者（中断上下文）
  read_lock(&stats_lock);
  unsigned int count = packet_count;
  read_unlock(&stats_lock);

  // 写者（定时器上下文）
  write_lock(&stats_lock);
  packet_count = 0;
  write_unlock(&stats_lock);
  ```

**场景2：文件系统元数据更新（需公平性）**
- 使用`rw_semaphore`保护目录结构：
  ```c
  struct rw_semaphore dir_sem;

  // 读者（遍历目录）
  down_read(&dir_sem);
  // 读取目录项
  up_read(&dir_sem);

  // 写者（创建文件）
  down_write(&dir_sem);
  // 修改目录结构
  up_write(&dir_sem);
  ```

