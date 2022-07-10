# linux进程管理


## 进程状态

```mermaid
graph LR

fork --> TASK_RUNNING
```

## 进程组成
> 进程的基本单位是`task_struct`,Linux的进程管理都是一句此结构进行
> 但是在用户层界面存在这另外的结构PID,我们都是根据PID对进程进行操作

**PID是`task_struct`的界面**,那么可以确定进程的描述由两种方式;

### 进程描述

1. `PID`
2. `task_struct`

> 如何根据`PID`得到`task_struct`?
> 如何根据`task_struct`得到`PID`?

#### PID

#### task_struct

```c
#ifndef randomized_struct_fields_start
# define randomized_struct_fields_start
# define randomized_struct_fields_end
#endif
```

```c
struct task_struct {
    /* 1. 线程信息 */
    struct thread_info		    thread_info;

    /* 2. 优先级 */
    int                         prio;
	int				            static_prio;
	int				            normal_prio;
	unsigned int			    rt_priority;

    /* 调度器累 */
    const struct sched_class	*sched_class;
	
    /* 调度实体 */
    struct sched_entity		    se;
	struct sched_rt_entity		rt;
    struct sched_dl_entity		dl;

    unsigned int			    policy;
	int				            nr_cpus_allowed;
	cpumask_t			        cpus_allowed;

    struct sched_info		    sched_info;

    struct thread_struct		thread;
}
```

## 进程调度
