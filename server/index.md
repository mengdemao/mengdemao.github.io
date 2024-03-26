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

## 安装nginx

1. 安装nginx
```shell
sudo apt install nginx
sudo systemctl enbale nginx
sudo systemctl restart nginx
```

2. 将网页文件放到**/var/www/html**文件夹下面

3. 执行配置

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

## 代码在线阅读工具`Opengrok`

+ 安装jdk
+ 安装tomcat
+ 安装opengrok
+ 配置opengrok

下载链接</br>
[下载tomcat8](https://tomcat.apache.org/download-80.cgi)</br>
[下载tomcat9](https://tomcat.apache.org/download-90.cgi)</br>
[下载tomcat10](https://tomcat.apache.org/download-10.cgi)</br>

解压到/opt文件夹下面

```shell
tar zxvf apache-tomcat-x.x.x.tar.gz
sudo mv apache-tomcat-x.x.x/ /opt/apache-tomcat-x.x.x
sudo ln -s /opt/apache-tomcat-x.x.x /opt/tomcat
/opt/tomcat/bin/startup.sh
```

输入ip:8080,可以得到tomcat的网页,说明服务器安装成功

安装opengrok[官方参考](https://github.com/oracle/opengrok/wiki/How-to-setup-OpenGrok)</br>
[opengrok下载链接](https://github.com/oracle/opengrok/releases),
找到最新的进行下载即可;

1. 创建工作空间

为了保证所有的空间整洁,我们可以包所有的东西全部放到'opengrok'目录下,那么

```shell
mkdir /opt/opengrok
mkdir /opt/opengrok/{src,data,dist,etc,log}
tar -C /opt/opengrok/dist --strip-components=1 -xzf opengrok-X.Y.Z.tar.gz
```

2. 拷贝日志配置

```shell
cp /opt/opengrok/dist/doc/logging.properties /opt/opengrok/etc
```

3. 拷贝源码文件

**将文件放到/opt/opengrok/src**

4. 安装管理工具(可选)

```shell
$ cd tools
$ python3 -m venv env
$ . ./env/bin/activate
$ pip install opengrok-tools.tar.gz
```

5. 部署web应用
> 拷贝应用目录`lib/source.war`到`tomcat/webapps`中,程序就会自动部署;
> 但是我们本次不进行如此处理,这种部署方式需要手动处理文件索引.

```shell
opengrok-deploy -c /opt/opengrok/etc/configuration.xml \
    /opt/opengrok/dist/lib/source.war /opt/apache-tomcat/webapps
```

6. 执行扫描
```shell
opengrok-indexer \
    -J=-Djava.util.logging.config.file=/opt/opengrok/etc/logging.properties \
    -a /opt/opengrok/dist/lib/opengrok.jar -- \
    -c /usr/local/bin/ctags \
    -s /opt/opengrok/src -d /opt/opengrok/data -H -P -S -G \
    -W /opt/opengrok/etc/configuration.xml -U http://localhost:8080/source
```

