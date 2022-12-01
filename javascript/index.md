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

#### 定义字符串

```javascript

var a = "hello";
var b = "world";

var c = a + ',' + b;
var d = `${a}, ${b}`;

console.log(c);
console.log(d);
```

{{< script >}}
    var a = "hello";
    var b = "world";

    var c = a + ',' + b;
    var d = `${a}, ${b}`;

    console.log(c);
    console.log(d);
{{< /script >}}

根据上面的例子可以看出,
字符串的的拼接可以分成两种情况:
1. 使用`+`拼接字符串
2. 使用``拼接(前面的符号是Esc按键下面的按键)

#### 拼接字符串

要获取字符串某个指定位置的字符,类似于C语言的字符数组,
`str[0]`,`str[1]`,`str[2]`....分别可以得到字符;

#### 字符串函数

+ `toUpperCase()`把一个字符串全部变为大写
+ `toLowerCase()`把一个字符串全部变为小写
+ `indexOf()` 会搜索指定字符串出现的位置
+ `substring()`返回指定索引区间的子串

```javascript
var s = 'Hello';
console.log(s.toUpperCase());	// 'HELLO'
console.log(s.toLowerCase());	// 'HELLO'
console.log(s.indexOf('ll'));	// 2
console.log(s.substring(1,3));	// el
console.log(s.substring(1));	// ello
```

> 执行测试,可以在浏览器中看到日志

{{< script >}}
    var s = 'Hello';
    console.log(s.toUpperCase());	// 'HELLO'
    console.log(s.toLowerCase());	// 'HELLO'
    console.log(s.indexOf('ll'));	// 2
    console.log(s.substring(1,3));	// el
    console.log(s.substring(1));	// ello
{{< /script >}}

### 数组

`JavaScript`的数组可以包含任意数据类型;并通过索引来访问每个元素.

```javascript
var array = [1, 2, 'hello', "world", true]; // 定义一个数组
array.length;   // 5
array[0];       // 打印
```

### 对象

#### 定义对象
JavaScript的对象类似于`Json`,但是表示方法不同;

```javascript
var student = {
    name: "hello",
    age: 18
};
```
JavaScript用一个{...}表示一个对象,键值对以`成员名:属性值`声明;
与`Json`相同的是,最后一个元素不可以添加`,`;

#### 访问成员
> 访问成员的方式存在两种

+ C方式:`对象名.成员名`
+ 反射式:`对象名['成员名']`

```javascript
'use strict';

var student = {
    name: "hello",
    age: 18
};

console.log(student.name);         // hello
console.log(student['age']);       // 18
```

### 条件判断
> 与`C`相似,此处就不过多赘述

1. if
2. else
3. else if

### 循环语句

1. for
2. for-in
3. while
4. do-while

### Map和Set

### 遍历语句

### 函数

#### 函数模型

1. c语言类型

```javascript
function functionName(args)
{
    return retVal;
}
```

+ function:函数定义
+ functionName
+ parameter
+ retVal

2. 一种完全等价的定义

```javascript
var functionName = function(args)
{
    return retVal;
}
```

## javascript提高

### 标准对象

+ Date
+ RegExp
+ JSON

### 面向对象编程

### 浏览器

+ window
+ navigator
+ location
+ document
+ history

#### 浏览器对象

+ DOM
+ 表单
+ 文件
+ AJAX
+ Promise
+ Canvas

### 错误处理

> 程序有可能会出错,因此需要进行错误处理;高级语言`try ... catch ... finally`,

```javacript
'use strict';
var r1, r2, s = null;
try {
    r1 = s.length; // 此处应产生错误
    r2 = 100; // 该语句不会执行
} catch (e) {
    console.log('出错了：' + e);
} finally {
    console.log('finally');
}
console.log('r1 = ' + r1); // r1应为undefined
console.log('r2 = ' + r2); // r2应为undefined

// 下面的文件
// 出错了：TypeError: Cannot read properties of null (reading 'length')
// finally
// r1 = undefined
// r2 = undefined
```

+ 出错
    1. 先执行`try { ... }`的代码;
    2. 执行到出错的语句时,后续语句不再继续执行.转而执行`catch (e) { ... }`代码；
    3. 最后执行finally`{ ... }`代码。

+ 无错
    1. 先执行try { ... }的代码；
    2. 因为没有出错，catch (e) { ... }代码不会被执行；
    3. 最后执行finally { ... }代码。

## javascript进阶

### jQuery.js

### underscore.js
