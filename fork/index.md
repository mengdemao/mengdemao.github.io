# Fork


fork
====
> linux创建线程的函数
> fork --> do_fork

do_fork的执行线路
`do_fork` --> `copy_process` --> `get_task_pid` --> `wake_up_new_task` --> `put_pid`

do_fork函数原型
```c
long _do_fork(unsigned long clone_flags,
	      unsigned long stack_start,
	      unsigned long stack_size,
	      int __user *parent_tidptr,
	      int __user *child_tidptr,
	      unsigned long tls);
```
+ clone_flags
+ stack_start
+ stack_size
+ parent_tidptr
+ child_tidptr
+ tls

## copy_process

## get_task_pid

## wake_up_new_task


