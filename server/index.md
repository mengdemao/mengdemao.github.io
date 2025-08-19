# Linux工具部署功能记录


## 添加普通用户

```shell
useradd -m -G users,wheel,audio -s /bin/bash larry
passwd larry
Password: (Enter the password for larry)
Re-enter password: (Re-enter the password to verify)
```

## gentoo emerge

1. 更新sync
```shell
emerge-webrsync

emerge --sync
```

2. 更新软件包
```shell
emerge --ask --verbose --update --deep --newuse --getbinpkg @world
```

3. 安装软件包
```shell
emerge --ask --verbose --getbinpkg 软件包名
```

## 编译内核

1. 配置key

```shell
openssl req -new -nodes -utf8 -sha256 -x509 -outform PEM -out kernel_key.pem -keyout kernel_key.pem
```
修改make.conf,将路径添加到
```shell
USE="modules-sign"

# Optionally, when using custom signing keys.
MODULES_SIGN_KEY="/path/to/kernel_key.pem"
MODULES_SIGN_CERT="/path/to/kernel_key.pem" # Only required if the MODULES_SIGN_KEY does not also contain the certificate
MODULES_SIGN_HASH="sha512" # Defaults to sha512
```

```shell
zcat /proc/config.gz > /usr/src/linux

genkernel --menuconfig all
```

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

## HTTPS配置

1. 安装acme.sh工具

```shell
curl https://get.acme.sh | sh -s email=my@example.com
```

```shell
wget -O -  https://get.acme.sh | sh -s email=my@example.com
```

同时脚本会将环境变量写道bashrc中

2.设置默认CA

```shell
acme.sh --set-default-ca --server letsencrypt
```

3.生成证书

+ 直接签发

```shell
acme.sh --issue -d mydomain.com -d www.mydomain.com --webroot /home/wwwroot/mydomain.com/
```

+ 使用 Apache 模式

```shell
acme.sh --issue --apache -d example.com -d www.example.com -d cp.example.com
```

+ nginx

```shell
acme.sh --issue --nginx -d example.com -d www.example.com -d cp.example.com
```

+ 独立模式

```shell
acme.sh --issue --standalone -d example.com -d www.example.com -d cp.example.com
```

4.安装证书

```shell
acme.sh --install-cert -d example.com \
    --key-file       /path/to/keyfile/in/nginx/key.pem  \
    --fullchain-file /path/to/fullchain/nginx/cert.pem \
    --reloadcmd     "service nginx reload"
```

5. 更新证书
```shell
acme.sh --renew -d example.com --force
```

6. 配置nginx

```shell
server {
    listen       443 ssl http2;
    listen       [::]:443 ssl http2;
    server_name  mengdemao.com;
    root         /var/www/mengdemao.com;

    ssl_certificate "/etc/nginx/ssl/mengdemao.com/fullchain.cer";
    ssl_certificate_key "/etc/nginx/ssl/mengdemao.com/mengdemao.com.key";
    ssl_session_cache shared:SSL:1m;
    ssl_session_timeout  10m;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # Load configuration files for the default server block.
    include /etc/nginx/default.d/*.conf;

    error_page 404 /404.html;
        location = /40x.html {
    }

    error_page 500 502 503 504 /50x.html;
        location = /50x.html {
    }
}
```

## nfs

### 安装

``` shell
sudo apt-get install nfs-kernel-server
```

### 设置导出
``` shell
/home/exports *(rw,nohide,insecure,no_subtree_check,async,no_root_squash)
```

### 开启服务
``` shell
sudo /etc/init.d/nfs-kernel-server restart
```

### 测试
``` shell
sudo mount -t nfs -o nolock,vers=3 127.0.0.1:/home/exports /mnt
ls /mnt
```
