# Buildroot


## 下载buildroot

```shell
git clone --depth=1 https://git.busybox.net/buildroot
git branch -a [new branch]
git checkout [new branch]
```

## 快速开始

1. 可以先找到一个类似的配置文件,配置文件在下载的根目录的configs文件夹,下面
2. 复制成为自己的配置文件`cp configs/qemu_arm_vexpress_defconfig configs/qemu_arm_mengdemao_defconfig`
3. 测试环境是否正常,执行`make qemu_arm_mengdemao_defconfig && make`,此时就是漫长的等待时间
4. 运行测试程序`cd output/images`,并且执行start-qemu.sh

## 基础配置

### 使用自定义的内核

在defconfig文件中添加下面的配置

```c
BR2_LINUX_KERNEL=y # 是否编译内核
BR2_LINUX_KERNEL_CUSTOM_GIT=y # 是否使用git版本管理
BR2_LINUX_KERNEL_CUSTOM_REPO_URL="git@github.com:mengdemao/kernel.git" # kernel的地址
BR2_LINUX_KERNEL_CUSTOM_REPO_VERSION="master" # 版本
BR2_LINUX_KERNEL_VERSION="master"
BR2_LINUX_KERNEL_USE_DEFCONFIG=y
BR2_LINUX_KERNEL_DEFCONFIG="debian"
BR2_LINUX_KERNEL_ZIMAGE=y
BR2_LINUX_KERNEL_GZIP=y
```

