# Html5


## 开始

**网页基础结构**

```html
<!-- 告诉浏览器,需要使用的规范 -->
<!DOCTYPE html>
<html lang="en">

<!-- 网页标题 -->
<head>
    <!-- 描述标签 -->
    <meta charset="UTF-8">

    <!-- 网页标题 -->
    <title>Document</title>
</head>

<!-- 网页主体 -->
<body>
</body>
</html>
```
## 网页基本标签

### 标题标签
```html
<h1>一级标签</h1>
<h2>二级标签</h2>
<h3>三级标签</h3>
<h4>四级标签</h4>
<h5>五级标签</h5>
<h6>六级标签</h6>
```
### 段落标签
```html
<p>段落标签</p>
```
### 换行标签
```html
<br/>
```
### 水平线标签
```html
<hr/>
```
### 字体样式标签
```html
<!-- 字体样式标签 -->
<strong>粗体</strong><br/>
<em>斜体</em><br/>
```
## 图片标签
```html
<img src="测试.png" alt="测试" title="测试"/>
```
## 链接
```html
<!-- 当前页打开 -->
<a href="http://www.baidu.com" target="_self">百度一下</a><br/>
<!-- 新建页打开 -->
<a href="http://www.baidu.com" target="_blank">百度一下</a><br/>
```
## 行内元素和块元素

## 列表标签
### 有序列表
```html
<ol>
    <li>HTML</li>
    <li>CSS</li>
    <li>JavaScript</li>
</ol>
```

### 无序列表
```html
<ul>
    <li>HTML</li>
    <li>CSS</li>
    <li>JavaScript</li>
</ul>
```

### 定义列表
```html
<dl>
    <dt>前端</dt>
        <dd>html</dd>
        <dd>CSS</dd>
        <dd>JavaScript</dd>
</dl>
```

## 表格
```html
<table border="1px">
	<tr>
		<td>1-1</td>
		<td>1-2</td>
	</tr>
	<tr>
		<td>2-1</td>
		<td>2-2</td>
	</tr>
</table>
```

## 页面结构分析

| 元素名  | 描述                    |
| ------- | ----------------------- |
| header  | 标题头部区域            |
| footer  | 标记尾部内容            |
| section | web页面中一块独立的区域 |
| article | 独立文章内容            |
| aside   | 相关页面或者内容        |
| nav     | 导航类辅助内容          |

## iframe内联框架
```html
<iframe src="path" name="mainFrame"></frame>
```
bilibili的例子

![image-20211007160025047](https://raw.githubusercontent.com/mengdemao/picture/master/image-20211007160025047.png)

```html
<iframe src="//player.bilibili.com/player.html?aid=55631961&bvid=BV1x4411V75C&cid=97257967&page=11" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>
```

## 表单
表单form

```html
<form action="开始.html" method="GET/POST">
    <p>名字: <input type="text" name="name"></p>
    <p>密码: <input type="password" name="password"></p>
    <p>
        <input type="submit">        
        <input type="reset">
    </p>
</form>
```
产生的效果
```text
?name=111&password=
```
