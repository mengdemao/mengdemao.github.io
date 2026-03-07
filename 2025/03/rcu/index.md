# RCU机制


<!--more-->
linux内核rcu机制
<!--more-->

## 基础用法

读侧
```c
rcu_read_lock();                // 进入读临界区
data = rcu_dereference(ptr);    // 安全获取指针
// 读取数据...
rcu_read_unlock();              // 退出读临界区
```

写侧
```c
// 1. 分配新数据并初始化
new_data = kmalloc(sizeof(*new_data), GFP_KERNEL);
memcpy(new_data, old_data, sizeof(*new_data));
new_data->value = new_value;

// 2. 替换指针
rcu_assign_pointer(ptr, new_data);

// 3. 同步等待宽限期结束（阻塞）
synchronize_rcu();

// 4. 释放旧数据
kfree(old_data);

// 或使用异步回收（非阻塞）：
call_rcu(&old_data->rcu_head, callback_func);
```

编写一个完整的rcu测试demo
```c
#include <linux/module.h>
#include <linux/fs.h>
#include <linux/rcupdate.h>
#include <linux/slab.h>
#include <linux/uaccess.h>

#define DEVICE_NAME "rcu_demo"
#define CLASS_NAME "rcucls"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Your Name");
MODULE_DESCRIPTION("RCU Demo Driver Module");

// 定义受保护的数据结构
struct rcu_data {
    int value;
    struct rcu_head rcu;
};

static struct rcu_data __rcu *global_data = NULL; // RCU保护的全局数据
static int major_number;
static struct class *rcu_class = NULL;
static struct device *rcu_device = NULL;

// 异步回收回调函数
static void rcu_free_callback(struct rcu_head *rcu)
{
    struct rcu_data *data = container_of(rcu, struct rcu_data, rcu);
    kfree(data);
    printk(KERN_INFO "RCU: Old data freed\n");
}

// 写操作处理
static ssize_t dev_write(struct file *filp, const char __user *buf,
                        size_t count, loff_t *pos)
{
    int new_value, ret;
    struct rcu_data *new_data, *old_data;

    // 从用户空间获取新值
    if (copy_from_user(&new_value, buf, sizeof(int)))
        return -EFAULT;

    // 分配新数据结构
    new_data = kmalloc(sizeof(*new_data), GFP_KERNEL);
    if (!new_data)
        return -ENOMEM;

    // 初始化新数据
    new_data->value = new_value;

    // RCU指针替换
    old_data = rcu_replace_pointer(global_data, new_data, true);

    // 同步等待宽限期结束
    synchronize_rcu();

    // 释放旧数据（异步方式可使用call_rcu）
    if (old_data) {
        kfree_rcu(old_data, rcu);
        printk(KERN_INFO "RCU: Updated value to %d\n", new_value);
    }

    return sizeof(int);
}

// 读操作处理
static ssize_t dev_read(struct file *filp, char __user *buf,
                       size_t count, loff_t *pos)
{
    struct rcu_data *data;
    int ret_val = 0;

    rcu_read_lock();
    data = rcu_dereference(global_data);
    if (data) {
        ret_val = data->value;
        if (copy_to_user(buf, &ret_val, sizeof(int)))
            ret_val = -EFAULT;
    }
    rcu_read_unlock();

    return (ret_val < 0) ? ret_val : sizeof(int);
}

static struct file_operations fops = {
    .owner = THIS_MODULE,
    .read = dev_read,
    .write = dev_write,
};

// 模块初始化
static int __init rcu_init(void)
{
    // 初始化默认数据
    struct rcu_data *data = kmalloc(sizeof(*data), GFP_KERNEL);
    if (!data)
        return -ENOMEM;

    data->value = 0;
    rcu_assign_pointer(global_data, data);

    // 注册字符设备
    major_number = register_chrdev(0, DEVICE_NAME, &fops);
    if (major_number < 0) {
        kfree(data);
        return major_number;
    }

    // 创建设备文件
    rcu_class = class_create(THIS_MODULE, CLASS_NAME);
    if (IS_ERR(rcu_class)) {
        unregister_chrdev(major_number, DEVICE_NAME);
        kfree(data);
        return PTR_ERR(rcu_class);
    }

    rcu_device = device_create(rcu_class, NULL,
                             MKDEV(major_number, 0), NULL, DEVICE_NAME);
    if (IS_ERR(rcu_device)) {
        class_destroy(rcu_class);
        unregister_chrdev(major_number, DEVICE_NAME);
        kfree(data);
        return PTR_ERR(rcu_device);
    }

    printk(KERN_INFO "RCU demo module loaded\n");
    return 0;
}

// 模块退出
static void __exit rcu_exit(void)
{
    struct rcu_data *data;

    device_destroy(rcu_class, MKDEV(major_number, 0));
    class_unregister(rcu_class);
    class_destroy(rcu_class);
    unregister_chrdev(major_number, DEVICE_NAME);

    // 清理全局数据
    data = rcu_dereference(global_data);
    if (data) {
        synchronize_rcu();
        kfree(data);
    }

    printk(KERN_INFO "RCU demo module unloaded\n");
}

module_init(rcu_init);
module_exit(rcu_exit);
```

## 源码解析

### `rcu`数据结构

```c
struct callback_head {
	struct callback_head *next;
	void (*func)(struct callback_head *head);
} __attribute__((aligned(sizeof(void *))));
#define rcu_head callback_head
```

### rcu读锁上锁
```c
static __always_inline void rcu_read_lock(void)
{
	__rcu_read_lock();
	__acquire(RCU);
	rcu_lock_acquire(&rcu_lock_map);
	RCU_LOCKDEP_WARN(!rcu_is_watching(),
			 "rcu_read_lock() used illegally while idle");
}
```


### rcu读锁解锁

```c
static inline void rcu_read_unlock(void)
{
	RCU_LOCKDEP_WARN(!rcu_is_watching(),
			 "rcu_read_unlock() used illegally while idle");
	rcu_lock_release(&rcu_lock_map); /* Keep acq info for rls diags. */
	__release(RCU);
	__rcu_read_unlock();
}
```

### 指针解析

```c
#define rcu_dereference(p) rcu_dereference_check(p, 0)

#define rcu_dereference_check(p, c) \
	__rcu_dereference_check((p), __UNIQUE_ID(rcu), \
				(c) || rcu_read_lock_held(), __rcu)


#define __rcu_dereference_check(p, local, c, space) \
({ \
	/* Dependency order vs. p above. */ \
	typeof(*p) *local = (typeof(*p) *__force)READ_ONCE(p); \
	RCU_LOCKDEP_WARN(!(c), "suspicious rcu_dereference_check() usage"); \
	rcu_check_sparse(p, space); \
	((typeof(*p) __force __kernel *)(local)); \
})
```

```c
#define rcu_replace_pointer(rcu_ptr, ptr, c)				\
({									\
	typeof(ptr) __tmp = rcu_dereference_protected((rcu_ptr), (c));	\
	rcu_assign_pointer((rcu_ptr), (ptr));				\
	__tmp;								\
})
```
