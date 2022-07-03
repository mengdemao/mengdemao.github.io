# javascript基础教程


## 基础知识

> JavaScript是一种运行在浏览器中的解释型的编程语言
> 在hugo中运行javacsript脚本,可以使用`hugo`的`script shortcode`在文章中插入`Javascript`脚本

```markdown
{{</* script */>}}
    console.log('javascript基础教程!');
{{</* /script */>}}
```

可以在浏览器的后台中看到日志`javascript基础教程!`

{{< script >}}
    console.log('javascript基础教程!');
{{< /script >}}

## javascript基础

> JavaScript严格区分大小写

作为一个新的语言,我们学的第一个程序
打印`hello world`

```javascript
/* 打印弹窗 */
alert("hello world");

/* 打印在html */
document.write("hello world");

/* 打印在调试窗口 */
console.log("Hello world")
```

### 基本语句

1. 赋值语句

```javascript
var x = 1;
```

2. 注释语句

```javascript
// 单行注释

/**
 * 多行注释
 */
```

3. 变量

+ 变量必须以字母开头
+ 变量也能以`$`和`_`符号开头
+ 变量名称对大小写敏感

### 数据类型

#### 值类型(基本类型)

1. 字符串(String)
2. 数字(Number)
3. 布尔(Boolean)
4. 空(Null)
5. 未定义(Undefined)
6. Symbol

#### 引用数据类型(对象类型)

1. 对象(Object)
2. 数组(Array)
3. 函数(Function)
4. 正则(RegExp)
5. 日期(Date)

### 字符串

### 数组

### 对象

### 条件判断

### 循环语句

### Map和Set

### 遍历语句

### 函数

## javascript提高

### 标准队形

### 面向对象编程

### 浏览器对象

### 错误处理

## javascript进阶

### jQuery

