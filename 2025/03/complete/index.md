# Complete完成量笔记


<!--more-->
linux内核活动--完成量
<!--more-->

## 内核API

| 函数 |	作用 |
| --- | --- |
| init_completion(comp)	| 初始化完成量（done=0） |
| wait_for_completion(comp)	| 等待完成量，不可中断（会阻塞当前线程） |
| wait_for_completion_interruptible(comp) |	可中断的等待（允许信号唤醒） |
| complete(comp) |	唤醒一个等待者（done++，若done>0则直接返回）|
| complete_all(comp) |	唤醒所有等待者（done设置为UINT_MAX/2） |
| try_wait_for_completion(comp) |	非阻塞尝试等待，成功返回1，否则返回0 |

## `init_completion`初始化

我们先看到这个结构体,其中有一个swait结构体

```c
struct completion {
	unsigned int done;
	struct swait_queue_head wait;
};
```


```c
static inline void init_completion(struct completion *x)
{
	x->done = 0;
	init_swait_queue_head(&x->wait);
}
```

## `wait_for_completion`等待完成量

```c
void __sched wait_for_completion(struct completion *x)
{
	wait_for_common(x, MAX_SCHEDULE_TIMEOUT, TASK_UNINTERRUPTIBLE);
}
```

## `complete`唤醒完成量

```c
void complete(struct completion *x)
{
	complete_with_flags(x, 0);
}
```

