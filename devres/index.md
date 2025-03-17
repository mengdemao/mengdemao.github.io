# Devres内存管理


**Devres（Device Resource Management）分析**

## **1. 概念与背景**
Devres 是 Linux 内核中用于**自动化设备资源管理**的机制，旨在简化驱动开发中的资源分配与释放流程。通过 Devres，开发者无需手动跟踪和释放资源（如内存、中断、DMA 缓冲区等），而是将资源绑定到设备生命周期（`struct device`），当设备注销或驱动卸载时，内核自动释放相关资源，从而减少内存泄漏风险。

---

## **2. Devres 的核心原理**
- **资源绑定**：通过 `devm_*` 系列函数（如 `devm_kzalloc()`, `devm_request_irq()`）分配资源时，内核会记录资源与设备的关联。
- **自动释放**：当设备被移除（调用 `device_del()`）或驱动卸载时，内核遍历与该设备关联的资源链表，逐一释放资源。
- **数据结构**：资源通过链表（`struct devres_node`）管理，每个节点包含释放资源的回调函数。

---

## **3. 传统方法与 Devres 对比**
**传统驱动开发**：
```c
probe() {
    res1 = kmalloc(...);
    res2 = request_irq(...);
    if (error) {
        free(res1);
        free_irq(res2);
        return -ERR;
    }
}

remove() {
    free(res1);
    free_irq(res2);
}
```

**使用 Devres**：
```c
probe() {
    res1 = devm_kzalloc(dev, ...);
    res2 = devm_request_irq(dev, ...);
    // 无需手动释放
    if (error)
        return -ERR; // 内核自动回滚已分配资源
}

remove() {
    // 无需操作，资源已自动释放
}
```

## 4. 源码分析

4.1 主要 Devres API

| 函数                     | 作用                          |
|--------------------------|-------------------------------|
| `devm_kzalloc()`         | 分配内存，自动释放            |
| `devm_request_irq()`     | 注册中断处理函数              |
| `devm_ioremap()`         | 映射物理地址到内核虚拟空间    |
| `devm_gpio_request()`    | 请求 GPIO 引脚                |
| `devm_clk_get()`         | 获取时钟资源                  |

4.2  `devm_kmalloc`函数分析

```c
struct devres_node {
	struct list_head		entry;
	dr_release_t			release;
	const char				*name;
	size_t					size;
};

struct devres {
	struct devres_node		node;
	u8 __aligned(ARCH_KMALLOC_MINALIGN) data[];
};
```


```c
void *devm_kmalloc(struct device *dev, size_t size, gfp_t gfp)
{
	struct devres *dr;

	if (unlikely(!size))
		return ZERO_SIZE_PTR;

	/* use raw alloc_dr for kmalloc caller tracing */
	dr = alloc_dr(devm_kmalloc_release, size, gfp, dev_to_node(dev));
	if (unlikely(!dr))
		return NULL;

	set_node_dbginfo(&dr->node, "devm_kzalloc_release", size);
	devres_add(dev, dr->data);
	return dr->data;
}
```

在申请资源的时候将释放也加入其中

```c
static __always_inline struct devres * alloc_dr(dr_release_t release,
						size_t size, gfp_t gfp, int nid)
{
	size_t tot_size;
	struct devres *dr;

	if (!check_dr_size(size, &tot_size))
		return NULL;

	dr = kmalloc_node_track_caller(tot_size, gfp, nid);
	if (unlikely(!dr))
		return NULL;

	/* No need to clear memory twice */
	if (!(gfp & __GFP_ZERO))
		memset(dr, 0, offsetof(struct devres, data));

	INIT_LIST_HEAD(&dr->node.entry);
	dr->node.release = release;
	return dr;
}
```



**devres_add**

`devres_add` ==> `add_dr`

```c
void devres_add(struct device *dev, void *res)
{
	struct devres *dr = container_of(res, struct devres, data);
	unsigned long flags;

	spin_lock_irqsave(&dev->devres_lock, flags);
	add_dr(dev, &dr->node);
	spin_unlock_irqrestore(&dev->devres_lock, flags);
}
```

将`dr`加入到设备的`devres_head`中

**4.3 释放所有资源**

```c
int devres_release_all(struct device *dev)
{
	unsigned long flags;
	LIST_HEAD(todo);
	int cnt;

	if (list_empty(&dev->devres_head))
		return 0;

	spin_lock_irqsave(&dev->devres_lock, flags);
	cnt = remove_nodes(dev, dev->devres_head.next, &dev->devres_head, &todo);
	spin_unlock_irqrestore(&dev->devres_lock, flags);

	release_nodes(dev, &todo);
	return cnt;
}
```

那么为什么资源可以自动释放呢？

经过代码分析可知,那就可以说明驱动卸载的时候自动释放资源

```
device_release --> devres_release_all
```

**4.4 提前释放资源**

可以很清楚的弄明白提前释放的原理

```c
void devm_remove_action(struct device *dev, void (*action)(void *), void *data)
{
	struct action_devres devres = {
		.data = data,
		.action = action,
	};

	WARN_ON(devres_destroy(dev, devm_action_release, devm_action_match,
			       &devres));

}
```

```c
int devres_destroy(struct device *dev, dr_release_t release,
		   dr_match_t match, void *match_data)
{
	void *res;

	res = devres_remove(dev, release, match, match_data);
	if (unlikely(!res))
		return -ENOENT;

	devres_free(res);
	return 0;
}

void * devres_remove(struct device *dev, dr_release_t release,
		     dr_match_t match, void *match_data)
{
	struct devres *dr;
	unsigned long flags;

	spin_lock_irqsave(&dev->devres_lock, flags);
	dr = find_dr(dev, release, match, match_data);
	if (dr) {
		list_del_init(&dr->node.entry);
		devres_log(dev, &dr->node, "REM");
	}
	spin_unlock_irqrestore(&dev->devres_lock, flags);

	if (dr)
		return dr->data;
	return NULL;
}

void devres_free(void *res)
{
	if (res) {
		struct devres *dr = container_of(res, struct devres, data);

		BUG_ON(!list_empty(&dr->node.entry));
		kfree(dr);
	}
}
```


