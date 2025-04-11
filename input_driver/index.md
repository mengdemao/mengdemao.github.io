# 输入子系统


<!--more-->
Linux输入子系统笔记
<!--more-->

输入子设备分为三层
1. handle
2. core
3. device

## input的相关结构体
```c
  struct input_dev {				/* 输入设备的描述 */
  const char *name;				/* 设备名称 */
	  const char *phys;
	  const char *uniq;
	  struct input_id id;

	  unsigned long propbit[BITS_TO_LONGS(INPUT_PROP_CNT)];

	  unsigned long evbit[BITS_TO_LONGS(EV_CNT)];
	  unsigned long keybit[BITS_TO_LONGS(KEY_CNT)];
	  unsigned long relbit[BITS_TO_LONGS(REL_CNT)];
	  unsigned long absbit[BITS_TO_LONGS(ABS_CNT)];
	  unsigned long mscbit[BITS_TO_LONGS(MSC_CNT)];
	  unsigned long ledbit[BITS_TO_LONGS(LED_CNT)];
	  unsigned long sndbit[BITS_TO_LONGS(SND_CNT)];
	  unsigned long ffbit[BITS_TO_LONGS(FF_CNT)];
	  unsigned long swbit[BITS_TO_LONGS(SW_CNT)];

	  unsigned int hint_events_per_packet;

	  unsigned int keycodemax;
	  unsigned int keycodesize;
	  void *keycode;

	  int (*setkeycode)(struct input_dev *dev,
				const struct input_keymap_entry *ke,
				unsigned int *old_keycode);
	  int (*getkeycode)(struct input_dev *dev,
				struct input_keymap_entry *ke);

	  struct ff_device *ff;

	  unsigned int repeat_key;
	  struct timer_list timer;

	  int rep[REP_CNT];

	  struct input_mt *mt;

	  struct input_absinfo *absinfo;

	  unsigned long key[BITS_TO_LONGS(KEY_CNT)];
	  unsigned long led[BITS_TO_LONGS(LED_CNT)];
	  unsigned long snd[BITS_TO_LONGS(SND_CNT)];
	  unsigned long sw[BITS_TO_LONGS(SW_CNT)];

	  int (*open)(struct input_dev *dev);
	  void (*close)(struct input_dev *dev);
	  int (*flush)(struct input_dev *dev, struct file *file);
	  int (*event)(struct input_dev *dev, unsigned int type, unsigned int code, int value);

	  struct input_handle __rcu *grab;

	  spinlock_t event_lock;
	  struct mutex mutex;

	  unsigned int users;
	  bool going_away;

	  struct device dev;

	  struct list_head	h_list;
	  struct list_head	node;

	  unsigned int num_vals;
	  unsigned int max_vals;
	  struct input_value *vals;

	  bool devres_managed;
  };
#define to_input_dev(d) container_of(d, struct input_dev, dev)
```

## input子系统使用

## input子系统分析

1. Makefile编写

```Makfile
obj-$(CONFIG_INPUT)		+= input-core.o
input-core-y := input.o input-compat.o input-mt.o ff-core.o
```
2. 开始判断下面的第一个文件 *input.c*
```c
subsys_initcall(input_init);
module_exit(input_exit);
```

输入子系统的设备号
```c
#define INPUT_MAJOR  13
```

安装驱动
```c
  static int __init input_init(void)
  {
	  int err;
	  /* 注册设备类 */
	  err = class_register(&input_class);
	  if (err) {
		  pr_err("unable to register input_dev class\n");
		  return err;
	  }

	  /* 注册proc文件系统 */
	  err = input_proc_init();
	  if (err)
		  goto fail1;
	  /* 注册设备 */
	  err = register_chrdev_region(MKDEV(INPUT_MAJOR, 0),
					   INPUT_MAX_CHAR_DEVICES, "input");
	  if (err) {
		  pr_err("unable to register char major %d", INPUT_MAJOR);
		  goto fail2;
	  }

	  return 0;

   fail2:	input_proc_exit();
   fail1:	class_unregister(&input_class);
	  return err;
  }
```

卸载驱动
```c
  static void __exit input_exit(void)
  {
	  /* 卸载proc文件系统 */
	  input_proc_exit();

	  /* 注销设备号 */
	  unregister_chrdev_region(MKDEV(INPUT_MAJOR, 0),
				   INPUT_MAX_CHAR_DEVICES);

	  /* 注销CLass */
	  class_unregister(&input_class);
  }
```

设备类操作
```c
	/* 设备类型 */
	struct class input_class = {
		.name		= "input",
		.devnode	= input_devnode,
	};
	EXPORT_SYMBOL_GPL(input_class);

	/* 注册设备 */
	  err = class_register(&input_class);
	  if (err) {
		  pr_err("unable to register input_dev class\n");
		  return err;
	  }

								  /* 卸载设备 */
  class_unregister(&input_class);
```

Proc文件系统操作

Proc文件系统添加
```c
static int __init input_proc_init(void)
{
	struct proc_dir_entry *entry;

	proc_bus_input_dir = proc_mkdir("bus/input", NULL);
	if (!proc_bus_input_dir)
		return -ENOMEM;

	entry = proc_create("devices", 0, proc_bus_input_dir,
				&input_devices_fileops);
	if (!entry)
		goto fail1;

	entry = proc_create("handlers", 0, proc_bus_input_dir,
				&input_handlers_fileops);
	if (!entry)
		goto fail2;

	return 0;

 fail2:	remove_proc_entry("devices", proc_bus_input_dir);
 fail1: remove_proc_entry("bus/input", NULL);
	return -ENOMEM;
}
```

Proc文件系统卸载
```c
static void input_proc_exit(void)
{
	remove_proc_entry("devices", proc_bus_input_dir);
	remove_proc_entry("handlers", proc_bus_input_dir);
	remove_proc_entry("bus/input", NULL);
}
```
## 接口部分

### Handler操作

```c
/**
 * 注册 input handler
 * input_register_handler - register a new input handler
 * @handler: handler to be registered
 *
 * This function registers a new input handler (interface) for input
 * devices in the system and attaches it to all input devices that
 * are compatible with the handler.
 */
int input_register_handler(struct input_handler *handler)
{
	struct input_dev *dev;
	int error;

	error = mutex_lock_interruptible(&input_mutex);
	if (error)
		return error;

	INIT_LIST_HEAD(&handler->h_list);

	list_add_tail(&handler->node, &input_handler_list);

	list_for_each_entry(dev, &input_dev_list, node)
		input_attach_handler(dev, handler);

	input_wakeup_procfs_readers();

	mutex_unlock(&input_mutex);
	return 0;
}
EXPORT_SYMBOL(input_register_handler);

/**
 * 解除注册 input handler
 * input_unregister_handler - unregisters an input handler
 * @handler: handler to be unregistered
 *
 * This function disconnects a handler from its input devices and
 * removes it from lists of known handlers.
 */
void input_unregister_handler(struct input_handler *handler)
{
	struct input_handle *handle, *next;

	mutex_lock(&input_mutex);

	list_for_each_entry_safe(handle, next, &handler->h_list, h_node)
		handler->disconnect(handle);
	WARN_ON(!list_empty(&handler->h_list));

	list_del_init(&handler->node);

	input_wakeup_procfs_readers();

	mutex_unlock(&input_mutex);
}
EXPORT_SYMBOL(input_unregister_handler);
```

### 注册设备
```c
/**
 * 注册一个设备
 * input_register_device - register device with input core
 * @dev: device to be registered
 *
 * This function registers device with input core. The device must be
 * allocated with input_allocate_device() and all it's capabilities
 * set up before registering.
 * If function fails the device must be freed with input_free_device().
 * Once device has been successfully registered it can be unregistered
 * with input_unregister_device(); input_free_device() should not be
 * called in this case.
 *
 * Note that this function is also used to register managed input devices
 * (ones allocated with devm_input_allocate_device()). Such managed input
 * devices need not be explicitly unregistered or freed, their tear down
 * is controlled by the devres infrastructure. It is also worth noting
 * that tear down of managed input devices is internally a 2-step process:
 * registered managed input device is first unregistered, but stays in
 * memory and can still handle input_event() calls (although events will
 * not be delivered anywhere). The freeing of managed input device will
 * happen later, when devres stack is unwound to the point where device
 * allocation was made.
 */
int input_register_device(struct input_dev *dev)
{
	struct input_devres *devres = NULL;
	struct input_handler *handler;
	unsigned int packet_size;
	const char *path;
	int error;

	if (dev->devres_managed) {
		devres = devres_alloc(devm_input_device_unregister,
					  sizeof(struct input_devres), GFP_KERNEL);
		if (!devres)
			return -ENOMEM;

		devres->input = dev;
	}

	/* Every input device generates EV_SYN/SYN_REPORT events. */
	__set_bit(EV_SYN, dev->evbit);

	/* KEY_RESERVED is not supposed to be transmitted to userspace. */
	__clear_bit(KEY_RESERVED, dev->keybit);

	/* Make sure that bitmasks not mentioned in dev->evbit are clean. */
	input_cleanse_bitmasks(dev);

	packet_size = input_estimate_events_per_packet(dev);
	if (dev->hint_events_per_packet < packet_size)
		dev->hint_events_per_packet = packet_size;

	dev->max_vals = dev->hint_events_per_packet + 2;
	dev->vals = kcalloc(dev->max_vals, sizeof(*dev->vals), GFP_KERNEL);
	if (!dev->vals) {
		error = -ENOMEM;
		goto err_devres_free;
	}

	/*
	 * If delay and period are pre-set by the driver, then autorepeating
	 * is handled by the driver itself and we don't do it in input.c.
	 */
	if (!dev->rep[REP_DELAY] && !dev->rep[REP_PERIOD]) {
		dev->timer.data = (long) dev;
		dev->timer.function = input_repeat_key;
		dev->rep[REP_DELAY] = 250;
		dev->rep[REP_PERIOD] = 33;
	}

	if (!dev->getkeycode)
		dev->getkeycode = input_default_getkeycode;

	if (!dev->setkeycode)
		dev->setkeycode = input_default_setkeycode;

	error = device_add(&dev->dev);
	if (error)
		goto err_free_vals;

	path = kobject_get_path(&dev->dev.kobj, GFP_KERNEL);
	pr_info("%s as %s\n",
		dev->name ? dev->name : "Unspecified device",
		path ? path : "N/A");
	kfree(path);

	error = mutex_lock_interruptible(&input_mutex);
	if (error)
		goto err_device_del;

	list_add_tail(&dev->node, &input_dev_list);

	list_for_each_entry(handler, &input_handler_list, node)
		input_attach_handler(dev, handler);

	input_wakeup_procfs_readers();

	mutex_unlock(&input_mutex);

	if (dev->devres_managed) {
		dev_dbg(dev->dev.parent, "%s: registering %s with devres.\n",
			__func__, dev_name(&dev->dev));
		devres_add(dev->dev.parent, devres);
	}
	return 0;

err_device_del:
	device_del(&dev->dev);
err_free_vals:
	kfree(dev->vals);
	dev->vals = NULL;
err_devres_free:
	devres_free(devres);
	return error;
}
EXPORT_SYMBOL(input_register_device);

/**
 * 解除注册设备
 * input_unregister_device - unregister previously registered device
 * @dev: device to be unregistered
 *
 * This function unregisters an input device. Once device is unregistered
 * the caller should not try to access it as it may get freed at any moment.
 */
void input_unregister_device(struct input_dev *dev)
{
	if (dev->devres_managed) {
		WARN_ON(devres_destroy(dev->dev.parent,
					devm_input_device_unregister,
					devm_input_device_match,
					dev));
		__input_unregister_device(dev);
		/*
		 * We do not do input_put_device() here because it will be done
		 * when 2nd devres fires up.
		 */
	} else {
		__input_unregister_device(dev);
		input_put_device(dev);
	}
}
EXPORT_SYMBOL(input_unregister_device);
```

