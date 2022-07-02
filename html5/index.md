# HTML5基础教程

 

## 网页结构
> hugo中`markdown`可以直接渲染`html`,可以直接得到效果
> 但是网页基础结构却不可以编写,这样会破坏渲染过程

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

+ <!DOCTYPE html> www规范
+ 注释的写法 `<!-- 注释  -->`
+ html: 网页
+ head: 标题标签
+ meta: 网站信息
+ body: 网页主题

> `meta`标签处于`head`区,对用户不可见,用于对网页进行描述,一般SEO优化

```html
<head>
<meta name="description" content="前端基础">
<meta name="keywords" content="HTML,CSS,JavaScript">
<meta name="author" content="孟德茂">
<meta charset="UTF-8">
</head>
```

## 基本标签

+ 标题标签
+ 段落标签
+ 换行标签
+ 水平线标签
+ 字体样式标签
+ 注释和特殊符号标签

### 标题标签

```html
<h1>一级标签</h1>
```

<h1>一级标签</h1>

```html
<h2>二级标签</h2>
```

<h2>二级标签</h2>

```html
<h3>三级标签</h3>
```

<h3>三级标签</h3>

```html
<h4>四级标签</h4>
```

<h4>四级标签</h4>

```html
<h5>五级标签</h5>
```

<h5>五级标签</h5>

```html
<h6>六级标签</h6>
```

<h6>六级标签</h6>

### 段落标签

```html
<p>段落标签</p>
```

显示如下:
<p>我是一个段落</p>
<p>我又是一个段落</p>

### 换行标签
```html
<br/>
```

### 水平线标签
```html
<hr/>
```

<p>我在水平线标签上方</p>
<hr/>
<p>我在水平线标签下方</p>


### 字体样式标签
```html
<!-- 字体样式标签 -->
普通文本<span>无效果</span></br>

普通文本<b>加粗</b></br>
普通文本<strong>粗体</strong></br>

普通文本<i>斜体</i></br>
普通文本<em>斜体</em></br>

普通文本<u>下划线</u></br>
普通文本<ins>下划线</ins></br>

普通文本<s>删除线</s></br>
普通文本<del>删除线</del></br>

普通文本<sub>下标文本</sub></br>
普通文本<sup>上标文本</sup></br>
```
<!-- 字体样式标签 -->
普通文本<span>无效果</span></br>

普通文本<b>加粗</b></br>
普通文本<strong>粗体</strong></br>

普通文本<i>斜体</i></br>
普通文本<em>斜体</em></br>

普通文本<u>下划线</u></br>
普通文本<ins>下划线</ins></br>

普通文本<s>删除线</s></br>
普通文本<del>删除线</del></br>

普通文本<sub>下标文本</sub></br>
普通文本<sup>上标文本</sup></br>

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

<ol>
    <li>HTML</li>
    <li>CSS</li>
    <li>JavaScript</li>
</ol>

### 无序列表
```html
<ul>
    <li>HTML</li>
    <li>CSS</li>
    <li>JavaScript</li>
</ul>
```

<ul>
    <li>HTML</li>
    <li>CSS</li>
    <li>JavaScript</li>
</ul>

### 定义列表
```html
<dl>
    <dt>前端</dt>
        <dd>html</dd>
        <dd>CSS</dd>
        <dd>JavaScript</dd>
</dl>
```
<dl>
    <dt>前端</dt>
        <dd>html</dd>
        <dd>CSS</dd>
        <dd>JavaScript</dd>
</dl>

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

<iframe src="//player.bilibili.com/player.html?aid=55631961&bvid=BV1x4411V75C&cid=97257967&page=11" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe>

## 表单

```html
<form action="submit.html" method="GET/POST">
    <p>名字: <input type="text" name="name"></p>
    <p>密码: <input type="password" name="password"></p>
    <p>
        <input type="submit">        
        <input type="reset">
    </p>
</form>
```

> 出入账户和密码,点击按钮会触发相应的
> 动作**http://url/html5/submit.html?name=mengdemao&password=1234**,
> 但是此时会显示失败,因为没有处理函数.

<form action="submit.html" method="GET/POST">
    <p>名字: <input type="text" name="name"></p>
    <p>密码: <input type="password" name="password"></p>
    <p>
        <input type="submit">        
        <input type="reset">
    </p>
</form>


