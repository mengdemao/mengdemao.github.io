# Css


## CSS开始(层叠样式表)

HTML + CSS + JavaScript
名词 + 形容词 + 动词
相当于对原始的HTML进行美化

### 快速入门
1. CSS是什么
2. CSS怎么用
3. CSS选择器
4. 美化网页
5. 盒子模型
6. 浮动
7. 定位
8. 网页动画

### 什么是CSS
美化:字体, 颜色,高度,宽度, 背景图片

### CSS的优势:
+ 内容和表现分离
+ CSS文件可以复用
+ 样式十分丰富
+ 建议使用独立的CSS文件

### CSS导入的方法

1. 行内样式
```html
<h1 style="color: red">一级标题</h1>
```
2. style标签
```html
<style></style>
```
3. 外部样式

+ 链接方式
```html
<link rel="stylesheet" href="style.css">
```
+ 导入式
```html
<style>
	@import url("css/style.css");
</style>
```
### 基本语法
```css
/* 注释语法 */
selector {
	/* 声明 */
	attr:value;
}
```

## 选择器

### 基本选择器

+ 标签选择器

+ 类选择器

+ ID选择器

#### 标签选择器
```css
h1 {
    color: red;
}
h2 {
    color: black;
}
h3 {
    color: yellow;
}
h4 {
    color: red;
}
```
#### 类选择器
```html
<h1 class="test">测试</h1>
```
此时,可以讲HTML选中
```css
.test {
    color: black;
}
```
#### ID选择器
```html
<h1 id="test">测试</h1>
```
```css
#test {
	color: black;
}
```
ID唯一确定,不可以共享;

### 层次选择器 

