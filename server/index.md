# Server


个人服务器安装记录
====
## 配置交换文件
```shell
# 创建交换文件
sudo fallocate -l 2G /swapfile

# 修改权限只允许ROOT使用
sudo chmod 600 /swapfile

# 创建swapfile
sudo mkswap /swapfile

# 开启swapfile
sudo swapon /swapfile

# 写入fstab
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

## `jenkins`安装

### 安装jdk

```shell
sudo apt install default-jdk
```
### 安装jenkins

```shell
wget -q -O - https://pkg.jenkins.io/debian-stable/jenkins.io.key | sudo apt-key add -
sudo sh -c 'echo deb https://pkg.jenkins.io/debian-stable binary/ > /etc/apt/sources.list.d/jenkins.list'
sudo apt-get update
sudo apt-get install jenkins
```
## `docker`安装

### 安装依赖

```shell
sudo apt install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common
```
### 安装公钥

```shell
curl -fsSL https://mirrors.ustc.edu.cn/docker-ce/linux/ubuntu/gpg | sudo apt-key add -
```

### 设置仓库
```shell
sudo add-apt-repository \
   "deb [arch=amd64] https://mirrors.ustc.edu.cn/docker-ce/linux/ubuntu/ \
  $(lsb_release -cs) \
  stable"
```

### 安装Docker

```shell
sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io
```

### 添加root组

```shell
sudo usermod -aG docker `whoami`
```

### docker辅助工具
+ docker-compose
+ lazydocker

## 嵌入式开发工具链

### arm编译工具链

```shell
sudo apt install gcc-arm-linux-gnueabihf
sudo apt install gcc-arm-linux-gnueabi
sudo apt install gcc-arm-none-eabi
```

### 测试工具

```shell
sudo apt install qemu
sudo apt install qemu-system-arm
sudo apt install qemu-user-static
sudo apt install qemu-user
sudo apt install qemu-user-binfmt
```

### 打包工具
```shell
sudo apt install u-boot-tools
```


