# Hugo教程


## 基本操作

### 安装hugo

在linux/windows上只能通过直接[release](https://github.com/gohugoio/hugo)下载,

### 创建网站
``` shell
hugo new site 路径
```

### 添加主题

* 将主题直接添加到theme文件下面
* 将主题作为一个submodule

### 创建文档
``` shell
hugo new posts/hugo.md
```


### 设置预览
``` shell
 hugo server -D --disableFastRender
```

## 文件结构
```text
.
├── archetypes
├── config
├── content
├── data
├── layouts
├── static
├── themes
├── static
└── resources
```
目录结构说明
以下是每个目录的高级概述，其中包含指向 Hugo 文档中每个相应部分的链接。

### archetypes
hugo模板,在创建文件时作为模板自动生成

### assets
存储所有需要HugoPipes处理的文件;只有使用了.Permalink 或 .RelPermalink的文件才会发布到公共目录.
注意：默认情况下不创建该目录

### config
Hugo配置目录

### content
此目录存在所有的网站内容,Hugo中的每个顶级文件夹都被视为一个内容部分.

### data
该目录用于存储 Hugo 在生成网站时可以使用的配置文件

### layouts
以 .html文件的形式存储模板.

### static
存储所有静态内容:图像、CSS、JavaScript等。当Hugo构建您的站点时,静态目录中的所有资产都按原样复制

## 编写工具
### typora
使用typora作为markdown编写工具

### picgo
![image-20211003093334007](https://raw.githubusercontent.com/mengdemao/picture/master/image-20211003093334007.png)
