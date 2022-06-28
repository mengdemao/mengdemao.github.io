# 调度优先级

## 优先级定义
>内核优先级使用`0-139`确定内部优先级,`0-99`分配实时进程;
>`[100, 139]`分配非实时进程,nice的映射`[-20, 19]`与非实时优先级的相对应.




## 相关数据结构

任务相关数据
```c
struct task_struct {
	
	int				prio;			// 动态优先级
	int				static_prio;	// 静态优先级
	int				normal_prio;	// 普通优先级	
	unsigned int	rt_priority;	// 实时优先级
};
```

### 计算静态优先级
```c
static int effective_prio(struct task_struct *p)
{
	/* 设置普通优先级 */
	p->normal_prio = normal_prio(p);

	/* 如果不是实时优先级,则返回普通优先级 */
	if (!rt_prio(p->prio))
		return p->normal_prio;
	
	/* 返回动态优先级 */
	return p->prio;
}
```
### 获取普通优先级
```c
static inline int __normal_prio(struct task_struct *p)
{
	return p->static_prio;
}

static inline int normal_prio(struct task_struct *p)
{
	int prio;

	if (task_has_dl_policy(p))
		prio = MAX_DL_PRIO-1;
	else if (task_has_rt_policy(p))
		prio = MAX_RT_PRIO-1 - p->rt_priority;
	else
		prio = __normal_prio(p);
	return prio;
}
```

## 负载计算
```c
// 负载权重
struct load_weight {
	unsigned long			weight;
	u32				inv_weight;
};

// 调度实体
struct sched_entity {
	struct load_weight		load;
};

struct sched_rt_entity {
	struct list_head		run_list;
	unsigned long			timeout;
	unsigned long			watchdog_stamp;
	unsigned int			time_slice;
	unsigned short			on_rq;
	unsigned short			on_list;

	struct sched_rt_entity		*back;
#ifdef CONFIG_RT_GROUP_SCHED
	struct sched_rt_entity		*parent;
	/* rq on which this entity is (to be) queued: */
	struct rt_rq			*rt_rq;
	/* rq "owned" by this entity/group: */
	struct rt_rq			*my_q;
#endif
} __randomize_layout;

struct sched_dl_entity {
	struct rb_node			rb_node;

	/*
	 * Original scheduling parameters. Copied here from sched_attr
	 * during sched_setattr(), they will remain the same until
	 * the next sched_setattr().
	 */
	u64				dl_runtime;	/* Maximum runtime for each instance	*/
	u64				dl_deadline;	/* Relative deadline of each instance	*/
	u64				dl_period;	/* Separation of two instances (period) */
	u64				dl_bw;		/* dl_runtime / dl_period		*/
	u64				dl_density;	/* dl_runtime / dl_deadline		*/

	/*
	 * Actual scheduling parameters. Initialized with the values above,
	 * they are continously updated during task execution. Note that
	 * the remaining runtime could be < 0 in case we are in overrun.
	 */
	s64				runtime;	/* Remaining runtime for this instance	*/
	u64				deadline;	/* Absolute deadline for this instance	*/
	unsigned int			flags;		/* Specifying the scheduler behaviour	*/

	/*
	 * Some bool flags:
	 *
	 * @dl_throttled tells if we exhausted the runtime. If so, the
	 * task has to wait for a replenishment to be performed at the
	 * next firing of dl_timer.
	 *
	 * @dl_boosted tells if we are boosted due to DI. If so we are
	 * outside bandwidth enforcement mechanism (but only until we
	 * exit the critical section);
	 *
	 * @dl_yielded tells if task gave up the CPU before consuming
	 * all its available runtime during the last job.
	 *
	 * @dl_non_contending tells if the task is inactive while still
	 * contributing to the active utilization. In other words, it
	 * indicates if the inactive timer has been armed and its handler
	 * has not been executed yet. This flag is useful to avoid race
	 * conditions between the inactive timer handler and the wakeup
	 * code.
	 *
	 * @dl_overrun tells if the task asked to be informed about runtime
	 * overruns.
	 */
	unsigned int			dl_throttled      : 1;
	unsigned int			dl_boosted        : 1;
	unsigned int			dl_yielded        : 1;
	unsigned int			dl_non_contending : 1;
	unsigned int			dl_overrun	  : 1;

	/*
	 * Bandwidth enforcement timer. Each -deadline task has its
	 * own bandwidth to be enforced, thus we need one timer per task.
	 */
	struct hrtimer			dl_timer;

	/*
	 * Inactive timer, responsible for decreasing the active utilization
	 * at the "0-lag time". When a -deadline task blocks, it contributes
	 * to GRUB's active utilization until the "0-lag time", hence a
	 * timer is needed to decrease the active utilization at the correct
	 * time.
	 */
	struct hrtimer inactive_timer;
};

// 调度结构体
struct task_struct {
	struct sched_entity			se;  // 调度实体
	struct sched_rt_entity		rt;  // 实时调度实体
	struct sched_dl_entity		dl;	 // 限期调度实体
};
```
### 设置负载权重

```c
const int sched_prio_to_weight[40] = {
 /* -20 */     88761,     71755,     56483,     46273,     36291,
 /* -15 */     29154,     23254,     18705,     14949,     11916,
 /* -10 */      9548,      7620,      6100,      4904,      3906,
 /*  -5 */      3121,      2501,      1991,      1586,      1277,
 /*   0 */      1024,       820,       655,       526,       423,
 /*   5 */       335,       272,       215,       172,       137,
 /*  10 */       110,        87,        70,        56,        45,
 /*  15 */        36,        29,        23,        18,        15,
};

const u32 sched_prio_to_wmult[40] = {
 /* -20 */     48388,     59856,     76040,     92818,    118348,
 /* -15 */    147320,    184698,    229616,    287308,    360437,
 /* -10 */    449829,    563644,    704093,    875809,   1099582,
 /*  -5 */   1376151,   1717300,   2157191,   2708050,   3363326,
 /*   0 */   4194304,   5237765,   6557202,   8165337,  10153587,
 /*   5 */  12820798,  15790321,  19976592,  24970740,  31350126,
 /*  10 */  39045157,  49367440,  61356676,  76695844,  95443717,
 /*  15 */ 119304647, 148102320, 186737708, 238609294, 286331153,
};

static void set_load_weight(struct task_struct *p, bool update_load)
{
	int prio = p->static_prio - MAX_RT_PRIO;
	struct load_weight *load = &p->se.load;

	// SCHED_IDLE 拥有最低权重
	if (idle_policy(p->policy)) {
		load->weight     = scale_load(WEIGHT_IDLEPRIO);
		load->inv_weight = WMULT_IDLEPRIO;
		return;
	}

	if (update_load && p->sched_class == &fair_sched_class) {
		reweight_task(p, prio);
	} else {
		load->weight = scale_load(sched_prio_to_weight[prio]);
		load->inv_weight = sched_prio_to_wmult[prio];
	}
}
```
#### 不需更新权重

```c
	load->weight = scale_load(sched_prio_to_weight[prio]);
	load->inv_weight = sched_prio_to_wmult[prio];
```

#### 需要更新权重

```c
void reweight_task(struct task_struct *p, int prio)
{
	struct sched_entity *se = &p->se;
	struct cfs_rq *cfs_rq = cfs_rq_of(se);
	struct load_weight *load = &se->load;
	unsigned long weight = scale_load(sched_prio_to_weight[prio]);

	reweight_entity(cfs_rq, se, weight, weight);
	load->inv_weight = sched_prio_to_wmult[prio];
}
static void reweight_entity(struct cfs_rq *cfs_rq, 
							struct sched_entity *se,
							unsigned long weight, 
							unsigned long runnable)
{
	if (se->on_rq) {
		/* commit outstanding execution time */
		if (cfs_rq->curr == se)
			update_curr(cfs_rq);
		account_entity_dequeue(cfs_rq, se);
		dequeue_runnable_load_avg(cfs_rq, se);
	}
	dequeue_load_avg(cfs_rq, se);

	se->runnable_weight = runnable;
	update_load_set(&se->load, weight);
	enqueue_load_avg(cfs_rq, se);
	if (se->on_rq) {
		account_entity_enqueue(cfs_rq, se);
		enqueue_runnable_load_avg(cfs_rq, se);
	}
}
```
