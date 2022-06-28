# Linux等待队列实现


等待事件是建立在调度的基础之上的一种同步机制

# 使用
## 等待队列头
``` c
struct __wait_queue_head {
	wq_lock_t lock;
	struct list_head task_list;
};
typedef struct __wait_queue_head wait_queue_head_t;
```
## 等待队列实体
``` c
struct __wait_queue {
	unsigned int flags;
	struct task_struct * task;
	struct list_head task_list;
};
typedef struct __wait_queue wait_queue_t;
```

## 初始化等待队列头
``` c
void __init_waitqueue_head(struct wait_queue_head *wq_head,
						   const char *name, struct lock_class_key *);
void init_waitqueue_head(struct wait_queue_head *wq_head);
```

## 初始化等待队列
```c
#define __WAITQUEUE_INITIALIZER(name, tsk) 						\
{																\
	.private	= tsk,											\
	.func		= default_wake_function,						\
	.entry		= { NULL, NULL }								\ 
}

#define DECLARE_WAITQUEUE(name, tsk)  struct wait_queue_entry name = __WAITQUEUE_INITIALIZER(name, tsk)

// 但是，一般直接
DECLARE_WAITQUEUE(wait, current);
```
+  等待队列入口
+  等待的任务

## 等待队列操作
``` c
void add_wait_queue(struct wait_queue_head *wq_head,
					struct wait_queue_entry *wq_entry);
void remove_wait_queue(struct wait_queue_head *wq_head,
					   struct wait_queue_entry *wq_entry);
```
+  等待队列头
+  等待队列实体

## 等待事件
``` c
void wait_event(wq, condition);
void wait_event_interruptible(wq, condition);
```

## 唤醒队列
+ wake_up
+ wake_up_all
+ wake_up_interruptible
+ wake_up_interruptible_all
+ wake_up_sync
+ wake_up_interruptible_sync


# 例子
## 写端

```c
ssize_t wait_write(struct file *file, const char __user *data, size_t len, loff_t *ppos)
{
	DECLARE_WAITQUEUE(wait, current);		/* 声明等待队列 */
	int ret = -1;
	PTRACE;

	mutex_lock(&wait_device.mutex);
	/* 非阻塞模式直接写入 */
	if (file->f_flags & O_NONBLOCK) {
		pr_err("write in O_NONBLOCK Mode");
		goto pure_write;
	}

	add_wait_queue(&wait_device.wait_w, &wait);
	while (wait_device.wait_flag == true) {
		pr_err("Write INTERRUPTIBLE");
		__set_current_state(TASK_INTERRUPTIBLE);
		mutex_unlock(&wait_device.mutex);
		schedule();
		if (signal_pending(current)) {
			ret = -ERESTARTSYS;
			remove_wait_queue(&wait_device.wait_w, &wait);
			__set_current_state(TASK_RUNNING);
			goto out;
		}
	}
	remove_wait_queue(&wait_device.wait_w, &wait);

pure_write:
	wait_device.wait_flag = true;
	pr_err("Write Successful");

	wake_up_interruptible(&wait_device.wait_r);
	pr_err("Wakeup Read");
	goto out;

out:
	mutex_unlock(&wait_device.mutex);
	return ret;
}
```

## 读端
``` c
 ssize_t wait_read(struct file *file, char __user *buf, size_t len, loff_t * ppos)
{
	DECLARE_WAITQUEUE(wait, current);		/* 声明等待队列 */
	int ret = 0;
	PTRACE;

	mutex_lock(&wait_device.mutex);
	/* 非阻塞模式直接写入 */
	if (file->f_flags & O_NONBLOCK) {
		pr_err("write in O_NONBLOCK Mode");
		goto pure_read;
	}

	add_wait_queue(&wait_device.wait_r, &wait);
	while (wait_device.wait_flag == false) {
		pr_err("Write INTERRUPTIBLE");
		__set_current_state(TASK_INTERRUPTIBLE);
		mutex_unlock(&wait_device.mutex);
		schedule();
		if (signal_pending(current)) {
			ret = -ERESTARTSYS;
			remove_wait_queue(&wait_device.wait_r, &wait);
			__set_current_state(TASK_RUNNING);
			goto out;
		}
	}
	remove_wait_queue(&wait_device.wait_r, &wait);

pure_read:
	wait_device.wait_flag = false;
	pr_err("Read Successful");

	wake_up_interruptible(&wait_device.wait_w);
	pr_err("Wakeup Write");

	goto out;

out:
	mutex_unlock(&wait_device.mutex);
	return 0;
}
```

# 原理
