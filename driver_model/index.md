# Linux驱动模型


## 调试环境

### 编写`hello world`程序

**Makefile**描述编译指令

```Makefile
# 如果kernel环境没有设置,那么则使用主机系统的kernel文件
kernel ?= /lib/modules/$(shell uname -r)/build

# 构建
modules:
	make -C ${kernel} M=$(shell pwd) modules

# 清理
clean:
	make -C ${kernel} M=$(shell pwd) clean
```

**Kbuild**描述具体文件

```Makefile
# 添加头文件
subdir-ccflags-y = -I$(src)/include

# 添加文件
obj-m += hello.o
```

**hello.c**
```c
/**
 * @file hello.c
 * @author mengdemao (mengdemao19951021@163.com)
 * @version 1.0
 * @date 2021-04-24
 *
 * @brief 驱动开发环境测试
 *
 * @copyright Copyright (c) 2021  mengdemao
 *
 */
#include <linux/init.h>
#include <linux/module.h>

static int __init hello_init(void)
{
	printk(KERN_INFO"Hello Init\r\n");
	return 0;
}
module_init(hello_init);

static void __exit hello_exit(void)
{
	printk(KERN_INFO"Hello Exit\r\n");
}
module_exit(hello_exit);

MODULE_AUTHOR("mengdemao19951021@163.com");
MODULE_LICENSE("GPL v2");
MODULE_VERSION("1.0");
```

```shell
$ modinfo hello.ko
	filename:       /work/linux/module/hello/hello.ko
	version:        1.0
	license:        GPL v2
	author:         mengdemao
	srcversion:     EBF8A02F0408BC4D9789ED7
	depends:
	retpoline:      Y
	name:           hello
	vermagic:       6.1.12-arch1-1 SMP preempt mod_unload

$ sudo insmod hello.ko
$ sudo rmmod hello
[ 6522.572194] Hello Init
[ 6546.892588] Hello Exit
```

### 驱动签名

但是，在安装驱动的时候;

```shell
[ 6522.571883] hello: loading out-of-tree module taints kernel.
[ 6522.571933] hello: module verification failed: signature and/or required key missing - tainting kernel
```
需要进行签名,

## 驱动原理

### 模块加载

+ insmod 	加载驱动
+ rmmod	 	删除驱动
+ modprobe	依赖加载

那么，则需要先分析[module实现](#module)

**[ sys_init_module ]**

```c
SYSCALL_DEFINE3(init_module, void __user *, umod,
		unsigned long, len, const char __user *, uargs)
{
	int err;
	struct load_info info = { };

	// 初始化异常?
	err = may_init_module();
	if (err)
		return err;

	pr_debug("init_module: umod=%p, len=%lu, uargs=%p\n",
	       umod, len, uargs);

	err = copy_module_from_user(umod, len, &info);
	if (err)
		return err;

	return load_module(&info, uargs, 0);
}
```

**[ finit_module ]**

```c
SYSCALL_DEFINE3(finit_module, int, fd, const char __user *, uargs, int, flags)
{
	int err;
	struct load_info info = { };

	err = may_init_module();
	if (err)
		return err;

	pr_debug("finit_module: fd=%d, uargs=%p, flags=%i\n", fd, uargs, flags);

	if (flags & ~(MODULE_INIT_IGNORE_MODVERSIONS
		      |MODULE_INIT_IGNORE_VERMAGIC))
		return -EINVAL;

	err = copy_module_from_fd(fd, &info);
	if (err)
		return err;

	return load_module(&info, uargs, flags);
}
```

**[ delete_module ]**

```c
SYSCALL_DEFINE2(delete_module, const char __user *, name_user,
		unsigned int, flags)
{
	struct module *mod;
	char name[MODULE_NAME_LEN];
	int ret, forced = 0;

	if (!capable(CAP_SYS_MODULE) || modules_disabled)
		return -EPERM;

	if (strncpy_from_user(name, name_user, MODULE_NAME_LEN-1) < 0)
		return -EFAULT;
	name[MODULE_NAME_LEN-1] = '\0';

	if (mutex_lock_interruptible(&module_mutex) != 0)
		return -EINTR;

	mod = find_module(name);
	if (!mod) {
		ret = -ENOENT;
		goto out;
	}

	if (!list_empty(&mod->source_list)) {
		/* Other modules depend on us: get rid of them first. */
		ret = -EWOULDBLOCK;
		goto out;
	}

	/* Doing init or already dying? */
	if (mod->state != MODULE_STATE_LIVE) {
		/* FIXME: if (force), slam module count damn the torpedoes */
		pr_debug("%s already dying\n", mod->name);
		ret = -EBUSY;
		goto out;
	}

	/* If it has an init func, it must have an exit func to unload */
	if (mod->init && !mod->exit) {
		forced = try_force_unload(flags);
		if (!forced) {
			/* This module can't be removed */
			ret = -EBUSY;
			goto out;
		}
	}

	/* Stop the machine so refcounts can't move and disable module. */
	ret = try_stop_module(mod, flags, &forced);
	if (ret != 0)
		goto out;

	mutex_unlock(&module_mutex);
	/* Final destruction now no one is using it. */
	if (mod->exit != NULL)
		mod->exit();
	blocking_notifier_call_chain(&module_notify_list,
				     MODULE_STATE_GOING, mod);
	async_synchronize_full();

	/* Store the name of the last unloaded module for diagnostic purposes */
	strlcpy(last_unloaded_module, mod->name, sizeof(last_unloaded_module));

	free_module(mod);
	return 0;
out:
	mutex_unlock(&module_mutex);
	return ret;
}
```

## 内存传递

```c
static inline unsigned long __must_check copy_from_user(void *to, const void __user *from, unsigned long n)
{
	if (access_ok(VERIFY_READ, from, n))
		n = __copy_from_user(to, from, n);
	else /* security hole - plug it */
		memset(to, 0, n);
	return n;
}

static inline unsigned long __must_check copy_to_user(void __user *to, const void *from, unsigned long n)
{
	if (access_ok(VERIFY_WRITE, to, n))
		n = __copy_to_user(to, from, n);
	return n;
}
```

__copy_from_user
__copy_to_user
都是汇编实现的,暂时不进行分析

## 驱动模型

| 顶级kobject | 解释                                              |
| ----------- | ------------------------------------------------- |
| block       | 块设备链接-->/sys/deives相关文件                  |
| bus         | 存放各种总线文件                                  |
| class       | 各种设备类                                        |
| dev         | 存放(字符/块)设备主副设备号链接文件-->/sys/deives |
| devices     | 设备的具体存放文件                                |
| firmware    | 固件存放                                          |
| fs          | 文件类型                                          |
| kernel      | kernel子系统                                      |
| module      | 模块信息                                          |
| power       | 能源管理                                          |

### kobject

> 内核对象:kobject/kobject_type/kset
> 为模块提供一个底层抽象,其中文件存放于/sys文件下面

####  数据结构

```c
struct kobject {
	const char			*name;		/* 名字 */
	struct list_head	entry;		/* 链表:链接进入kset */
	struct kobject		*parent;	/* 指向父对象,建立层次结构 */
	struct kset			*kset;		/* 对象集合 */
	struct kobj_type	*ktype;		/* 对象类型 */
	struct kernfs_node	*sd; 		/* sysfs directory entry */
	struct kref			kref;		/* 引用计数 */

    #ifdef CONFIG_DEBUG_KOBJECT_RELEASE
	struct delayed_work	release;
	#endif

    unsigned int state_initialized:1;			/* 标志位:初始化 */
	unsigned int state_in_sysfs:1;				/* 标志位:在sysfs中 */
	unsigned int state_add_uevent_sent:1;		/* 标志位:发出KOBJ_ADD uevent */
	unsigned int state_remove_uevent_sent:1;	/* 标志位:发出KOBJ_REMOVE uevent */
	unsigned int uevent_suppress:1;				/* 标志位:禁止发出uevent */
};
```
####  初始化
```c
/**
 * kobject_init - initialize a kobject structure
 * @kobj: pointer to the kobject to initialize
 * @ktype: pointer to the ktype for this kobject.
 *
 * This function will properly initialize a kobject such that it can then
 * be passed to the kobject_add() call.
 *
 * After this function is called, the kobject MUST be cleaned up by a call
 * to kobject_put(), not by a call to kfree directly to ensure that all of
 * the memory is cleaned up properly.
 */
void kobject_init(struct kobject *kobj, struct kobj_type *ktype)
{
	char *err_str;			/** 错误信息 */

	/** 校验参数NULL */
	if (!kobj) {
		err_str = "invalid kobject pointer!";
		goto error;
	}
	if (!ktype) {
		err_str = "must have a ktype to be initialized properly!\n";
		goto error;
	}

	/** kobject是否已经初始化 */
	if (kobj->state_initialized) {
		/* do not error out as sometimes we can recover */
		pr_err("kobject (%p): tried to init an initialized object, something is seriously wrong.\n",
		       kobj);
		dump_stack(); /** 回溯堆栈 */
	}

	/** 调用具体初始化函数 */
	kobject_init_internal(kobj);

	/* 设置类型 */
	kobj->ktype = ktype;
	return;

error:
	pr_err("kobject (%p): %s\n", kobj, err_str);
	dump_stack();
}
EXPORT_SYMBOL(kobject_init);
```

#### 添加
```c
int kobject_add(struct kobject *kobj, 	/* 需要添加kobject */
				struct kobject *parent, /* 父指针 */
				const char *fmt, ...)   /* 命名 */
{
	va_list args;
	int retval;

	/* 校验kobject */
	if (!kobj)
		return -EINVAL;

	/* 是否已经初始化 */
	if (!kobj->state_initialized) {
		pr_err("kobject '%s' (%p): tried to add an uninitialized object, something is seriously wrong.\n",
		       kobject_name(kobj), kobj);
		dump_stack();
		return -EINVAL;
	}
	va_start(args, fmt);
	/* 设置名字并且将父指针添加到parent */
	retval = kobject_add_varg(kobj, parent, fmt, args);
	va_end(args);

	return retval;
}
```

最终调用添加函数
```c
static int kobject_add_internal(struct kobject *kobj)
{
	int error = 0;
	struct kobject *parent;

    /* 判断参数NULL */
	if (!kobj)
		return -ENOENT;

    /* 判断名字是否有效 */
	if (!kobj->name || !kobj->name[0]) {
		WARN(1,
		     "kobject: (%p): attempted to be registered with empty name!\n",
		     kobj);
		return -EINVAL;
	}

    /** 获取父指针 */
	parent = kobject_get(kobj->parent);

	/* join kset if set, use it as parent if we do not already have one */
	if (kobj->kset) {		/* kset已经设置 */
		if (!parent)		/* 不存在父指针 */
			/* kset的kobject作为父指针 */
            parent = kobject_get(&kobj->kset->kobj);
		/* 将kobject加入kset */
        kobj_kset_join(kobj);

        /* 保存父指针 */
        kobj->parent = parent;
	}

	pr_debug("kobject: '%s' (%p): %s: parent: '%s', set: '%s'\n",
		 kobject_name(kobj), kobj, __func__,
		 parent ? kobject_name(parent) : "<NULL>",
		 kobj->kset ? kobject_name(&kobj->kset->kobj) : "<NULL>");

    /* 创建dir */
	error = create_dir(kobj);
	if (error) { /* 出错,清理 */
		kobj_kset_leave(kobj);
		kobject_put(parent);
		kobj->parent = NULL;

		/* be noisy on error issues */
		if (error == -EEXIST)
			pr_err("%s failed for %s with -EEXIST, don't try to register things with the same name in the same directory.\n",
			       __func__, kobject_name(kobj));
		else
			pr_err("%s failed for %s (error: %d parent: %s)\n",
			       __func__, kobject_name(kobj), error,
			       parent ? kobject_name(parent) : "'none'");
	} else
		kobj->state_in_sysfs = 1;  /* 添加到sysfs中 */

	return error;
}
```

##### sysfs文件夹生成

```c
static int create_dir(struct kobject *kobj)
{
	const struct kobj_ns_type_operations *ops;
	int error;

	error = sysfs_create_dir_ns(kobj, kobject_namespace(kobj));
	if (error)
		return error;

	error = populate_dir(kobj);
	if (error) {
		sysfs_remove_dir(kobj);
		return error;
	}

	/*
	 * @kobj->sd may be deleted by an ancestor going away.  Hold an
	 * extra reference so that it stays until @kobj is gone.
	 */
	sysfs_get(kobj->sd);

	/*
	 * If @kobj has ns_ops, its children need to be filtered based on
	 * their namespace tags.  Enable namespace support on @kobj->sd.
	 */
	ops = kobj_child_ns_ops(kobj);
	if (ops) {
		BUG_ON(ops->type <= KOBJ_NS_TYPE_NONE);
		BUG_ON(ops->type >= KOBJ_NS_TYPES);
		BUG_ON(!kobj_ns_type_registered(ops->type));

		sysfs_enable_ns(kobj->sd);
	}

	return 0;
}
```

#### 删除

```c
void kobject_del(struct kobject *kobj)
{
	struct kernfs_node *sd;

	if (!kobj)
		return;

	sd = kobj->sd;
	sysfs_remove_dir(kobj);
	sysfs_put(sd);

	kobj->state_in_sysfs = 0;
	kobj_kset_leave(kobj);
	kobject_put(kobj->parent);
	kobj->parent = NULL;
}
```

#### 引用计数
```c
struct kobject *kobject_get(struct kobject *kobj)
{
	if (kobj) {
		if (!kobj->state_initialized)
			WARN(1, KERN_WARNING
				"kobject: '%s' (%p): is not initialized, yet kobject_get() is being called.\n",
			     kobject_name(kobj), kobj);
		kref_get(&kobj->kref);
	}
	return kobj;
}

void kobject_put(struct kobject *kobj)
{
	if (kobj) {
		if (!kobj->state_initialized)
			WARN(1, KERN_WARNING
				"kobject: '%s' (%p): is not initialized, yet kobject_put() is being called.\n",
			     kobject_name(kobj), kobj);
		kref_put(&kobj->kref, kobject_release);
	}
}
```

###  kset

#### 数据结构
```c
struct kset {
	struct list_head list;
	spinlock_t list_lock;
	struct kobject kobj;
	const struct kset_uevent_ops *uevent_ops;
} __randomize_layout;
```

###  ktype
#### 数据结构
```c
struct kobj_type {
	void (*release)(struct kobject *kobj);
	const struct sysfs_ops *sysfs_ops;
	struct attribute **default_attrs;
	const struct kobj_ns_type_operations *(*child_ns_type)(struct kobject *kobj);
	const void *(*namespace)(struct kobject *kobj);
	void (*get_ownership)(struct kobject *kobj, kuid_t *uid, kgid_t *gid);
};
```
## class
> 设备类描述

```c
struct class {
	const char		*name;
	struct module		*owner;

	const struct attribute_group	**class_groups;
	const struct attribute_group	**dev_groups;
	struct kobject			*dev_kobj;

	int (*dev_uevent)(struct device *dev, struct kobj_uevent_env *env);
	char *(*devnode)(struct device *dev, umode_t *mode);

	void (*class_release)(struct class *class);
	void (*dev_release)(struct device *dev);

	int (*shutdown_pre)(struct device *dev);

	const struct kobj_ns_type_operations *ns_type;
	const void *(*namespace)(struct device *dev);

	void (*get_ownership)(struct device *dev, kuid_t *uid, kgid_t *gid);

	const struct dev_pm_ops *pm;

	struct subsys_private *p;
};
```

## bus

> 设备总线描述

### 总线类型

```shell
# ls
amba          cpu           nvmem         platform      virtio
clockevents   event_source  pci           scsi          workqueue
clocksource   gpio          pci_express   serio
container     hid           pcmcia        spi
```

其中每一个总线具有如下信息
```shell
# ls
devices            drivers_autoprobe  uevent
drivers            drivers_probe
```

```c
struct bus_type {
	const char		*name;
	const char		*dev_name;
	struct device		*dev_root;
	const struct attribute_group **bus_groups;
	const struct attribute_group **dev_groups;
	const struct attribute_group **drv_groups;

	int (*match)(struct device *dev, struct device_driver *drv);
	int (*uevent)(struct device *dev, struct kobj_uevent_env *env);
	int (*probe)(struct device *dev);
	int (*remove)(struct device *dev);
	void (*shutdown)(struct device *dev);

	int (*online)(struct device *dev);
	int (*offline)(struct device *dev);

	int (*suspend)(struct device *dev, pm_message_t state);
	int (*resume)(struct device *dev);

	int (*num_vf)(struct device *dev);

	int (*dma_configure)(struct device *dev);

	const struct dev_pm_ops *pm;

	const struct iommu_ops *iommu_ops;

	struct subsys_private *p;
	struct lock_class_key lock_key;

	bool need_parent_lock;
};
```

#### 设备总线注册
```c
int bus_register(struct bus_type *bus)
{
	int retval;
	struct subsys_private *priv;
	struct lock_class_key *key = &bus->lock_key;

	priv = kzalloc(sizeof(struct subsys_private), GFP_KERNEL);
	if (!priv)
		return -ENOMEM;

	priv->bus = bus;
	bus->p = priv;

	BLOCKING_INIT_NOTIFIER_HEAD(&priv->bus_notifier);

	retval = kobject_set_name(&priv->subsys.kobj, "%s", bus->name);
	if (retval)
		goto out;

	priv->subsys.kobj.kset = bus_kset;
	priv->subsys.kobj.ktype = &bus_ktype;
	priv->drivers_autoprobe = 1;

	retval = kset_register(&priv->subsys);
	if (retval)
		goto out;

	retval = bus_create_file(bus, &bus_attr_uevent);
	if (retval)
		goto bus_uevent_fail;

	priv->devices_kset = kset_create_and_add("devices", NULL,
						 &priv->subsys.kobj);
	if (!priv->devices_kset) {
		retval = -ENOMEM;
		goto bus_devices_fail;
	}

	priv->drivers_kset = kset_create_and_add("drivers", NULL,
						 &priv->subsys.kobj);
	if (!priv->drivers_kset) {
		retval = -ENOMEM;
		goto bus_drivers_fail;
	}

	INIT_LIST_HEAD(&priv->interfaces);
	__mutex_init(&priv->mutex, "subsys mutex", key);
	klist_init(&priv->klist_devices, klist_devices_get, klist_devices_put);
	klist_init(&priv->klist_drivers, NULL, NULL);

	retval = add_probe_files(bus);
	if (retval)
		goto bus_probe_files_fail;

	retval = bus_add_groups(bus, bus->bus_groups);
	if (retval)
		goto bus_groups_fail;

	pr_debug("bus: '%s': registered\n", bus->name);
	return 0;

bus_groups_fail:
	remove_probe_files(bus);
bus_probe_files_fail:
	kset_unregister(bus->p->drivers_kset);
bus_drivers_fail:
	kset_unregister(bus->p->devices_kset);
bus_devices_fail:
	bus_remove_file(bus, &bus_attr_uevent);
bus_uevent_fail:
	kset_unregister(&bus->p->subsys);
out:
	kfree(bus->p);
	bus->p = NULL;
	return retval;
}

```

#### 设备总线卸载
```c
void bus_unregister(struct bus_type *bus)
{
	pr_debug("bus: '%s': unregistering\n", bus->name);
	if (bus->dev_root)
		device_unregister(bus->dev_root);
	bus_remove_groups(bus, bus->bus_groups);
	remove_probe_files(bus);
	kset_unregister(bus->p->drivers_kset);
	kset_unregister(bus->p->devices_kset);
	bus_remove_file(bus, &bus_attr_uevent);
	kset_unregister(&bus->p->subsys);
}
```

## devices

> 设备文件具体描述:
> device(设备描述)
> device_driver(驱动描述)
> bus_type(总线信息)

```shell
|-- breakpoint
|-- kprobe
|-- platform
|-- software
|-- system
|-- tracepoint
|-- uprobe
`-- virtual
```

### device
> 描述设备


```c
struct device {
	struct device		*parent;

	struct device_private	*p;

	struct kobject kobj;
	const char		*init_name; /* initial name of the device */
	const struct device_type *type;

	struct mutex		mutex;	/* mutex to synchronize calls to
					 * its driver.
					 */

	struct bus_type	*bus;		/* type of bus device is on */
	struct device_driver *driver;	/* which driver has allocated this
					   device */
	void		*platform_data;	/* Platform specific data, device
					   core doesn't touch it */
	void		*driver_data;	/* Driver data, set and get with
					   dev_set/get_drvdata */
	struct dev_links_info	links;
	struct dev_pm_info	power;
	struct dev_pm_domain	*pm_domain;

#ifdef CONFIG_GENERIC_MSI_IRQ_DOMAIN
	struct irq_domain	*msi_domain;
#endif
#ifdef CONFIG_PINCTRL
	struct dev_pin_info	*pins;
#endif
#ifdef CONFIG_GENERIC_MSI_IRQ
	struct list_head	msi_list;
#endif

#ifdef CONFIG_NUMA
	int		numa_node;	/* NUMA node this device is close to */
#endif
	const struct dma_map_ops *dma_ops;
	u64		*dma_mask;	/* dma mask (if dma'able device) */
	u64		coherent_dma_mask;/* Like dma_mask, but for
					     alloc_coherent mappings as
					     not all hardware supports
					     64 bit addresses for consistent
					     allocations such descriptors. */
	u64		bus_dma_mask;	/* upstream dma_mask constraint */
	unsigned long	dma_pfn_offset;

	struct device_dma_parameters *dma_parms;

	struct list_head	dma_pools;	/* dma pools (if dma'ble) */

	struct dma_coherent_mem	*dma_mem; /* internal for coherent mem
					     override */
#ifdef CONFIG_DMA_CMA
	struct cma *cma_area;		/* contiguous memory area for dma
					   allocations */
#endif
	/* arch specific additions */
	struct dev_archdata	archdata;

	struct device_node	*of_node; /* associated device tree node */
	struct fwnode_handle	*fwnode; /* firmware device node */

	dev_t			devt;	/* dev_t, creates the sysfs "dev" */
	u32			id;	/* device instance */

	spinlock_t		devres_lock;
	struct list_head	devres_head;

	struct klist_node	knode_class;
	struct class		*class;
	const struct attribute_group **groups;	/* optional groups */

	void	(*release)(struct device *dev);
	struct iommu_group	*iommu_group;
	struct iommu_fwspec	*iommu_fwspec;

	bool			offline_disabled:1;
	bool			offline:1;
	bool			of_node_reused:1;
};
```

### driver
> 描述驱动

```c
struct device_driver {
	const char		*name;
	struct bus_type		*bus;

	struct module		*owner;
	const char		*mod_name;	/* used for built-in modules */

	bool suppress_bind_attrs;	/* disables bind/unbind via sysfs */
	enum probe_type probe_type;

	const struct of_device_id	*of_match_table;
	const struct acpi_device_id	*acpi_match_table;

	int (*probe) (struct device *dev);
	int (*remove) (struct device *dev);
	void (*shutdown) (struct device *dev);
	int (*suspend) (struct device *dev, pm_message_t state);
	int (*resume) (struct device *dev);
	const struct attribute_group **groups;

	const struct dev_pm_ops *pm;
	void (*coredump) (struct device *dev);

	struct driver_private *p;
};
```

## kernel
> kernel子系统

## module
> 模块信息

module实现
```c
struct module {
	enum module_state state;

	/* Member of list of modules */
	struct list_head list;

	/* Unique handle for this module */
	char name[MODULE_NAME_LEN];

	/* Sysfs stuff. */
	struct module_kobject mkobj;
	struct module_attribute *modinfo_attrs;
	const char *version;
	const char *srcversion;
	struct kobject *holders_dir;

	/* Exported symbols */
	const struct kernel_symbol *syms;
	const unsigned long *crcs;
	unsigned int num_syms;

	/* Kernel parameters. */
	struct kernel_param *kp;
	unsigned int num_kp;

	/* GPL-only exported symbols. */
	unsigned int num_gpl_syms;
	const struct kernel_symbol *gpl_syms;
	const unsigned long *gpl_crcs;

#ifdef CONFIG_UNUSED_SYMBOLS
	/* unused exported symbols. */
	const struct kernel_symbol *unused_syms;
	const unsigned long *unused_crcs;
	unsigned int num_unused_syms;

	/* GPL-only, unused exported symbols. */
	unsigned int num_unused_gpl_syms;
	const struct kernel_symbol *unused_gpl_syms;
	const unsigned long *unused_gpl_crcs;
#endif

#ifdef CONFIG_MODULE_SIG
	/* Signature was verified. */
	bool sig_ok;
#endif

	/* symbols that will be GPL-only in the near future. */
	const struct kernel_symbol *gpl_future_syms;
	const unsigned long *gpl_future_crcs;
	unsigned int num_gpl_future_syms;

	/* Exception table */
	unsigned int num_exentries;
	struct exception_table_entry *extable;

	/* Startup function. */
	int (*init)(void);

	/* If this is non-NULL, vfree after init() returns */
	void *module_init;

	/* Here is the actual code + data, vfree'd on unload. */
	void *module_core;

	/* Here are the sizes of the init and core sections */
	unsigned int init_size, core_size;

	/* The size of the executable code in each section.  */
	unsigned int init_text_size, core_text_size;

	/* Size of RO sections of the module (text+rodata) */
	unsigned int init_ro_size, core_ro_size;

	/* Arch-specific module values */
	struct mod_arch_specific arch;

	unsigned int taints;	/* same bits as kernel:tainted */

#ifdef CONFIG_GENERIC_BUG
	/* Support for BUG */
	unsigned num_bugs;
	struct list_head bug_list;
	struct bug_entry *bug_table;
#endif

#ifdef CONFIG_KALLSYMS
	/*
	 * We keep the symbol and string tables for kallsyms.
	 * The core_* fields below are temporary, loader-only (they
	 * could really be discarded after module init).
	 */
	Elf_Sym *symtab, *core_symtab;
	unsigned int num_symtab, core_num_syms;
	char *strtab, *core_strtab;

	/* Section attributes */
	struct module_sect_attrs *sect_attrs;

	/* Notes attributes */
	struct module_notes_attrs *notes_attrs;
#endif

	/* The command line arguments (may be mangled).  People like
	   keeping pointers to this stuff */
	char *args;

#ifdef CONFIG_SMP
	/* Per-cpu data. */
	void __percpu *percpu;
	unsigned int percpu_size;
#endif

#ifdef CONFIG_TRACEPOINTS
	unsigned int num_tracepoints;
	struct tracepoint * const *tracepoints_ptrs;
#endif
#ifdef HAVE_JUMP_LABEL
	struct jump_entry *jump_entries;
	unsigned int num_jump_entries;
#endif
#ifdef CONFIG_TRACING
	unsigned int num_trace_bprintk_fmt;
	const char **trace_bprintk_fmt_start;
#endif
#ifdef CONFIG_EVENT_TRACING
	struct ftrace_event_call **trace_events;
	unsigned int num_trace_events;
#endif
#ifdef CONFIG_FTRACE_MCOUNT_RECORD
	unsigned int num_ftrace_callsites;
	unsigned long *ftrace_callsites;
#endif

#ifdef CONFIG_LIVEPATCH
	bool klp_alive;
#endif

#ifdef CONFIG_MODULE_UNLOAD
	/* What modules depend on me? */
	struct list_head source_list;
	/* What modules do I depend on? */
	struct list_head target_list;

	/* Destruction function. */
	void (*exit)(void);

	atomic_t refcnt;
#endif

#ifdef CONFIG_CONSTRUCTORS
	/* Constructor functions. */
	ctor_fn_t *ctors;
	unsigned int num_ctors;
#endif
};
```

+ list_head: 驱动组成的链表

### 加载模块

本质上ko文件就是一个携带调试信息的ELF文件,因此需要提前学习ELF文件结构

```c
struct load_info {
	Elf_Ehdr *hdr;
	unsigned long len;
	Elf_Shdr *sechdrs;
	char *secstrings, *strtab;
	unsigned long symoffs, stroffs;
	struct _ddebug *debug;
	unsigned int num_debug;
	bool sig_ok;
	struct {
		unsigned int sym, str, mod, vers, info, pcpu;
	} index;
};

static int load_module(struct load_info *info, const char __user *uargs,
		       int flags)
{
	struct module *mod;
	long err;
	char *after_dashes;

	// 签名校验
	err = module_sig_check(info);
	if (err)
		goto free_copy;

	// ELF头检查
	err = elf_header_check(info);
	if (err)
		goto free_copy;

	/* Figure out module layout, and allocate all the memory. */
	mod = layout_and_allocate(info, flags);
	if (IS_ERR(mod)) {
		err = PTR_ERR(mod);
		goto free_copy;
	}

	/* Reserve our place in the list. */
	err = add_unformed_module(mod);
	if (err)
		goto free_module;

#ifdef CONFIG_MODULE_SIG
	mod->sig_ok = info->sig_ok;
	if (!mod->sig_ok) {
		pr_notice_once("%s: module verification failed: signature "
			       "and/or required key missing - tainting "
			       "kernel\n", mod->name);
		add_taint_module(mod, TAINT_UNSIGNED_MODULE, LOCKDEP_STILL_OK);
	}
#endif

	/* To avoid stressing percpu allocator, do this once we're unique. */
	err = percpu_modalloc(mod, info);
	if (err)
		goto unlink_mod;

	/* Now module is in final location, initialize linked lists, etc. */
	err = module_unload_init(mod);
	if (err)
		goto unlink_mod;

	/* Now we've got everything in the final locations, we can
	 * find optional sections. */
	err = find_module_sections(mod, info);
	if (err)
		goto free_unload;

	err = check_module_license_and_versions(mod);
	if (err)
		goto free_unload;

	/* Set up MODINFO_ATTR fields */
	setup_modinfo(mod, info);

	/* Fix up syms, so that st_value is a pointer to location. */
	err = simplify_symbols(mod, info);
	if (err < 0)
		goto free_modinfo;

	err = apply_relocations(mod, info);
	if (err < 0)
		goto free_modinfo;

	err = post_relocation(mod, info);
	if (err < 0)
		goto free_modinfo;

	flush_module_icache(mod);

	/* Now copy in args */
	mod->args = strndup_user(uargs, ~0UL >> 1);
	if (IS_ERR(mod->args)) {
		err = PTR_ERR(mod->args);
		goto free_arch_cleanup;
	}

	dynamic_debug_setup(info->debug, info->num_debug);

	/* Ftrace init must be called in the MODULE_STATE_UNFORMED state */
	ftrace_module_init(mod);

	/* Finally it's fully formed, ready to start executing. */
	err = complete_formation(mod, info);
	if (err)
		goto ddebug_cleanup;

	/* Module is ready to execute: parsing args may do that. */
	after_dashes = parse_args(mod->name, mod->args, mod->kp, mod->num_kp,
				  -32768, 32767, unknown_module_param_cb);
	if (IS_ERR(after_dashes)) {
		err = PTR_ERR(after_dashes);
		goto bug_cleanup;
	} else if (after_dashes) {
		pr_warn("%s: parameters '%s' after `--' ignored\n",
		       mod->name, after_dashes);
	}

	/* Link in to syfs. */
	err = mod_sysfs_setup(mod, info, mod->kp, mod->num_kp);
	if (err < 0)
		goto bug_cleanup;

	/* Get rid of temporary copy. */
	free_copy(info);

	/* Done! */
	trace_module_load(mod);

	return do_init_module(mod);

 bug_cleanup:
	/* module_bug_cleanup needs module_mutex protection */
	mutex_lock(&module_mutex);
	module_bug_cleanup(mod);
	mutex_unlock(&module_mutex);

	/* we can't deallocate the module until we clear memory protection */
	unset_module_init_ro_nx(mod);
	unset_module_core_ro_nx(mod);

 ddebug_cleanup:
	dynamic_debug_remove(info->debug);
	synchronize_sched();
	kfree(mod->args);
 free_arch_cleanup:
	module_arch_cleanup(mod);
 free_modinfo:
	free_modinfo(mod);
 free_unload:
	module_unload_free(mod);
 unlink_mod:
	mutex_lock(&module_mutex);
	/* Unlink carefully: kallsyms could be walking list. */
	list_del_rcu(&mod->list);
	wake_up_all(&module_wq);
	/* Wait for RCU synchronizing before releasing mod->list. */
	synchronize_rcu();
	mutex_unlock(&module_mutex);
 free_module:
	/* Free lock-classes; relies on the preceding sync_rcu() */
	lockdep_free_key_range(mod->module_core, mod->core_size);

	module_deallocate(mod, info);
 free_copy:
	free_copy(info);
	return err;
}
```

## 字符设备驱动

```c
struct cdev {
	struct kobject kobj;						// kobject
	struct module *owner;						// this_module
	const struct file_operations *ops;			// ops
	struct list_head list;						// 链表
	dev_t dev;									// 设备号
	unsigned int count;							// 引用计数
};
```

### 全局变量

```c
#define CHRDEV_MAJOR_HASH_SIZE	255

static struct kobj_map *cdev_map;

static DEFINE_MUTEX(chrdevs_lock);

static struct char_device_struct {
	struct char_device_struct *next;
	unsigned int major;
	unsigned int baseminor;
	int minorct;
	char name[64];
	struct cdev *cdev;		/* will die */
} *chrdevs[CHRDEV_MAJOR_HASH_SIZE];
```

### 操作函数

+ 设备号
  + `register_chrdev_region`
  + `unregister_chrdev_region`
  + `alloc_chrdev_region`

```c
int register_chrdev_region(dev_t from, unsigned count, const char *name)
{
	struct char_device_struct *cd;
	dev_t to = from + count;
	dev_t n, next;

	for (n = from; n < to; n = next) {
		next = MKDEV(MAJOR(n)+1, 0);
		if (next > to)
			next = to;
		cd = __register_chrdev_region(MAJOR(n), MINOR(n),
			       next - n, name);
		if (IS_ERR(cd))
			goto fail;
	}
	return 0;
fail:
	to = n;
	for (n = from; n < to; n = next) {
		next = MKDEV(MAJOR(n)+1, 0);
		kfree(__unregister_chrdev_region(MAJOR(n), MINOR(n), next - n));
	}
	return PTR_ERR(cd);
}

static struct char_device_struct *
__register_chrdev_region(unsigned int major, unsigned int baseminor,
			   int minorct, const char *name)
{
	struct char_device_struct *cd, **cp;
	int ret = 0;
	int i;

	cd = kzalloc(sizeof(struct char_device_struct), GFP_KERNEL);
	if (cd == NULL)
		return ERR_PTR(-ENOMEM);

	mutex_lock(&chrdevs_lock);

	/* temporary */
	if (major == 0) {
		for (i = ARRAY_SIZE(chrdevs)-1; i > 0; i--) {
			if (chrdevs[i] == NULL)
				break;
		}

		if (i == 0) {
			ret = -EBUSY;
			goto out;
		}
		major = i;
	}

	cd->major = major;
	cd->baseminor = baseminor;
	cd->minorct = minorct;
	strlcpy(cd->name, name, sizeof(cd->name));

	i = major_to_index(major);

	for (cp = &chrdevs[i]; *cp; cp = &(*cp)->next)
		if ((*cp)->major > major ||
		    ((*cp)->major == major &&
		     (((*cp)->baseminor >= baseminor) ||
		      ((*cp)->baseminor + (*cp)->minorct > baseminor))))
			break;

	/* Check for overlapping minor ranges.  */
	if (*cp && (*cp)->major == major) {
		int old_min = (*cp)->baseminor;
		int old_max = (*cp)->baseminor + (*cp)->minorct - 1;
		int new_min = baseminor;
		int new_max = baseminor + minorct - 1;

		/* New driver overlaps from the left.  */
		if (new_max >= old_min && new_max <= old_max) {
			ret = -EBUSY;
			goto out;
		}

		/* New driver overlaps from the right.  */
		if (new_min <= old_max && new_min >= old_min) {
			ret = -EBUSY;
			goto out;
		}
	}

	cd->next = *cp;
	*cp = cd;
	mutex_unlock(&chrdevs_lock);
	return cd;
out:
	mutex_unlock(&chrdevs_lock);
	kfree(cd);
	return ERR_PTR(ret);
}
```

+ cdev
  + `cdev_init`
  + `cdev_alloc`
  + `cdev_del`
  + `cdev_add`

```c
struct cdev *cdev_alloc(void)
{
	struct cdev *p = kzalloc(sizeof(struct cdev), GFP_KERNEL);
	if (p) {
		INIT_LIST_HEAD(&p->list);
		kobject_init(&p->kobj, &ktype_cdev_dynamic);
	}
	return p;
}

void cdev_init(struct cdev *cdev, const struct file_operations *fops)
{
	memset(cdev, 0, sizeof *cdev);
	INIT_LIST_HEAD(&cdev->list);
	kobject_init(&cdev->kobj, &ktype_cdev_default);
	cdev->ops = fops;
}

int cdev_add(struct cdev *p, dev_t dev, unsigned count)
{
	int error;

	p->dev = dev;
	p->count = count;

	error = kobj_map(cdev_map, dev, count, NULL,
			 exact_match, exact_lock, p);
	if (error)
		return error;

	kobject_get(p->kobj.parent);

	return 0;
}

void cdev_del(struct cdev *p)
{
	cdev_unmap(p->dev, p->count);
	kobject_put(&p->kobj);
}
```

字符设备集中管理

+ `__register_chrdev`
+ `__unregister_chrdev`

```c
int __register_chrdev(unsigned int major, unsigned int baseminor,
		      unsigned int count, const char *name,
		      const struct file_operations *fops)
{
	struct char_device_struct *cd;
	struct cdev *cdev;
	int err = -ENOMEM;

	cd = __register_chrdev_region(major, baseminor, count, name);
	if (IS_ERR(cd))
		return PTR_ERR(cd);

	cdev = cdev_alloc();
	if (!cdev)
		goto out2;

	cdev->owner = fops->owner;
	cdev->ops = fops;
	kobject_set_name(&cdev->kobj, "%s", name);

	err = cdev_add(cdev, MKDEV(cd->major, baseminor), count);
	if (err)
		goto out;

	cd->cdev = cdev;

	return major ? 0 : cd->major;
out:
	kobject_put(&cdev->kobj);
out2:
	kfree(__unregister_chrdev_region(cd->major, baseminor, count));
	return err;
}

void __unregister_chrdev(unsigned int major, unsigned int baseminor,
			 unsigned int count, const char *name)
{
	struct char_device_struct *cd;

	cd = __unregister_chrdev_region(major, baseminor, count);
	if (cd && cd->cdev)
		cdev_del(cd->cdev);
	kfree(cd);
}
static struct char_device_struct *__unregister_chrdev_region(unsigned major, unsigned baseminor, int minorct)
{
	struct char_device_struct *cd = NULL, **cp;
	int i = major_to_index(major);

	mutex_lock(&chrdevs_lock);
	for (cp = &chrdevs[i]; *cp; cp = &(*cp)->next)
		if ((*cp)->major == major &&
		    (*cp)->baseminor == baseminor &&
		    (*cp)->minorct == minorct)
			break;
	if (*cp) {
		cd = *cp;
		*cp = cd->next;
	}
	mutex_unlock(&chrdevs_lock);
	return cd;
}
```

