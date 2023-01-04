# javascript基础教程


## 基础知识

> JavaScript是一种运行在浏览器中的解释型的编程语言
> JavaScript（JS）是一种具有函数优先特性的轻量级、解释型或者说即时编译型的编程语言。
> 虽然作为 Web 页面中的脚本语言被人所熟知，但是它也被用到了很多非浏览器环境中，
> 例如 Node.js、Apache CouchDB、Adobe Acrobat 等。
> 进一步说，JavaScript 是一种基于原型、多范式、单线程的动态 (en-US)语言，
> 并且支持面向对象、命令式和声明式（如函数式编程）风格。

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


同样的道理,也可以直接写在html中;
```html
<button onclick='alert(Date())'>现在的时间是? </button>
```
点击下面的按钮，**显示效果**
<button onclick='alert(Date())'>现在的时间是? </button>

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

但是直接在浏览器中运行显得特别麻烦,此时我们可以使用`NodeJS`在命令行中运行程序

1. Nodejs中的内容

    {{< typeit code=javascript group=nodejs_print_first >}}
console.log("hello NodeJS");
    {{< /typeit >}}

2. 执行js脚本

    {{< typeit group=nodejs_print_first >}}
    $ node 1.NodeJS.js
    {{< /typeit >}}
    {{< typeit group=nodejs_print_first >}}
    $ hello NodeJS
    {{< /typeit >}}

## 基本语句

1. 赋值语句

```javascript
var x = 1;		// 定义全局变量
let y = 12;		// 定义局部变量
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

## 数据类型

> 九种数据类型

+ 值类型(基本类型)
    1. 字符串(String)
    2. 数字(Number)
    3. 布尔(Boolean)
    4. 空(Null)
    5. 未定义(Undefined)
    6. Symbol

+ 引用数据类型(对象类型)
    1. 对象(Object)
    2. 数组(Array)
    3. 函数(Function)

### 定义数据

```javascript
var a = 'Hello';			// 字符串(String)		
var b = 10;					// 数字(Number)
var c = true;				// 布尔(Boolean)
var d = function() {		// 函数(Function)
	console.log("Hello");
};
var e = [1, 2, 3];			// 数组(Array)
var f = null;				// 空(Null)
var g = Symbol();			// Symbol
var h;						// 未定义(Undefined)
var i = Object();			// 对象(Object)
```

### 检测数据

+ typeof 只可以检测基本数据类型
+ constructor返回实例的构造函数
+ instanceof 原型查找
+ Object.prototype.toString

```javascript
// 1. typeof检测类型
console.log("\r\n1. typeof检测类型")
console.log('type of a is ' + typeof(a));
console.log('type of b is ' + typeof(b));
console.log('type of c is ' + typeof(c));
console.log('type of d is ' + typeof(d));
console.log('type of e is ' + typeof(e));
console.log('type of f is ' + typeof(f));
console.log('type of g is ' + typeof(g));
console.log('type of h is ' + typeof(h));
console.log('type of i is ' + typeof(i));

// 2. constructor返回实例的构造函数
console.log("\r\n2. constructor返回实例的构造函数")
console.log(a.constructor == String);
console.log(a.constructor == Number);

console.log(e.constructor == Object);
console.log(e.constructor == Array);

// 3. instanceof 原型查找
console.log("\r\n3. instanceof 原型查找")
console.log(a instanceof String);
console.log(a instanceof Number);

console.log(e instanceof Object);
console.log(e instanceof Array);

// 4. Object.prototype.toString
console.log("\r\n4. Object.prototype.toString")
console.log('type of a is ' + toString.call(a));
console.log('type of b is ' + toString.call(b));
console.log('type of c is ' + toString.call(c));
console.log('type of d is ' + toString.call(d));
console.log('type of e is ' + toString.call(e));
console.log('type of f is ' + toString.call(f));
console.log('type of g is ' + toString.call(g));
console.log('type of h is ' + toString.call(h));
console.log('type of i is ' + toString.call(i));
```

{{< script >}}
var a = 'Hello';			// 字符串(String)		
var b = 10;					// 数字(Number)
var c = true;				// 布尔(Boolean)
var d = function() {		// 函数(Function)
	console.log("Hello");
};
var e = [1, 2, 3];			// 数组(Array)
var f = null;				// 空(Null)
var g = Symbol();			// Symbol
var h;						// 未定义(Undefined)
var i = Object();			// 对象(Object)

console.log("\r\n3.2 数据类型检测类型")

// 1. typeof检测类型
console.log("\r\n1. typeof检测类型")
console.log('type of a is ' + typeof(a));
console.log('type of b is ' + typeof(b));
console.log('type of c is ' + typeof(c));
console.log('type of d is ' + typeof(d));
console.log('type of e is ' + typeof(e));
console.log('type of f is ' + typeof(f));
console.log('type of g is ' + typeof(g));
console.log('type of h is ' + typeof(h));
console.log('type of i is ' + typeof(i));

// 2. constructor返回实例的构造函数
console.log("\r\n2. constructor返回实例的构造函数")
console.log(a.constructor == String);
console.log(a.constructor == Number);

console.log(e.constructor == Object);
console.log(e.constructor == Array);

// 3. instanceof 原型查找
console.log("\r\n3. instanceof 原型查找")
console.log(a instanceof String);
console.log(a instanceof Number);

console.log(e instanceof Object);
console.log(e instanceof Array);

// 4. Object.prototype.toString
console.log("\r\n4. Object.prototype.toString")
console.log('type of a is ' + toString.call(a));
console.log('type of b is ' + toString.call(b));
console.log('type of c is ' + toString.call(c));
console.log('type of d is ' + toString.call(d));
console.log('type of e is ' + toString.call(e));
console.log('type of f is ' + toString.call(f));
console.log('type of g is ' + toString.call(g));
console.log('type of h is ' + toString.call(h));
console.log('type of i is ' + toString.call(i));

console.log("\r\n\r\n");
{{< /script >}}

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

## 条件判断
> 与`C`相似,此处就不过多赘述

1. if
2. else
3. else if
4. switch

```javascript
if (1 < 3) {
	console.log("1 < 3 \r\n");
}

if (false) {
	console.log("false \r\n");
} else {
	console.log("true \r\n");
}

if (false) {
	console.log("false \r\n");
} else if(false) {
	console.log("false \r\n");
} else {
	console.log("true  \r\n");
}
```

同样的道理,js也存在这多路选择

```javascript
switch (new Date().getDay()) {
    case 0:
        day = "星期天";
        break;
    case 1:
        day = "星期一";
         break;
    case 2:
        day = "星期二";
         break;
    case 3:
        day = "星期三";
         break;
    case 4:
        day = "星期四";
         break;
    case 5:
        day = "星期五";
         break;
    case 6:
        day = "星期六";
}
console.log('今天是' + day);
```

## 循环语句

1. for

```javascript
for (语句 1; 语句 2; 语句 3) {
     要执行的代码块
}
```

2. for-in

```javascript
for (key in object) {
  // code block to be executed
}
```

3. for-of

4. while

5. do-while

## 遍历语句

## 函数

### 函数模型

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

### 函数调用

### 箭头函数

## 面向对象

### 定义对象(ES5)

> ES5设计的面向对象真的麻烦

#### 对象初始化器

JavaScript的对象类似于`Json`,但是表示方法不同;

```javascript
var student = {
    name: "hello",
    age: 18
};
```
JavaScript用一个{...}表示一个对象,键值对以`成员名:属性值`声明;
与`Json`相同的是,最后一个元素不可以添加`,`;

#### 构造函数

+ 通过创建一个构造函数来定义对象的类型。首字母大写是非常普遍而且很恰当的惯用法。
+ 通过 new 创建对象实例。

```javascript
function Student(name, age) {
	this.name = name;
	this.age = age;
}
var student = new student("world", 19);
```

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

#### 继承语法

#### 封装语法

### 定义对象(ES6)

```javascript
class StudentClass {
    // constructor
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
	
     // Getter
    get info() {
        return this.name + this.age;
    }
    
    // Method
    printStudent() {
        console.log('name : ' + this.name);
        console.log('age : ' + this.age);
    }
}
var student3 = new StudentClass('test', 12);

// 使用getter函数的方法
console.log(student3.info);
```

#### 访问成员

#### 继承语法

#### 封装语法

## 标准对象

> JS为我们定义的标准对象

+ Map(映射)
+ Set(集合)
+ Date(日期)
+ RegExp(正则表达式)
+ JSON(JSON)
+ Math(数学)

### Map(映射)

### Set(集合)

### Date(日期)

### RegExp(正则表达式)

### JSON(JSON)

+ 序列化: stringify()
+ 反序列化: parse() 

```javascript
var text = '{ "student" : [' +
'{ "name":"hello" , "age": 18 },' +
'{ "name":"world" , "age": 18 }]}';

var jsonObj = JSON.parse(text);
var jsonTxt = JSON.stringify(jsonObj);

console.log(jsonObj);
console.log(jsonTxt);
```

### Math(数学)

> 与其他全局对象不同的是，Math 不是一个构造器。</br>
> Math 的所有属性与方法都是静态的. </br>
> 引用圆周率的写法是 Math.PI,调用正余弦函数的写法是 Math.sin(x)，x 是要传入的参数。</br>
> Math 的常量是使用 JavaScript 中的全精度浮点数来定义的。</br>
> Math 用于 Number 类型。它不支持 BigInt </br>

```javascript
// 常数
console.log("自然常数:      " + Math.E);
console.log("2的自然对数:   " + Math.LN2);
console.log("10的自然对数:  " + Math.LN10);

// 函数
console.log("-10的绝对值:   " + Math.abs(-10));
console.log("10的正弦值:    " + Math.sin(10));
console.log("10的余弦值:    " + Math.cos(10));
```

输出值

```bash
$ node Math.js
自然常数:      2.718281828459045
2的自然对数:   0.6931471805599453
10的自然对数:  2.302585092994046
-10的绝对值:   10
10的正弦值:    -0.5440211108893698
10的余弦值:    -0.8390715290764524
```

## 浏览器

+ window
+ navigator
+ location
+ document
+ history
+ DOM
+ 表单
+ 文件
+ AJAX
+ Promise
+ Canvas

### window

> window 对象表示一个包含 DOM 文档的窗口，其 document 属性指向窗口中载入的 DOM 文档。
> 使用 document.defaultView 属性可以获取指定文档所在窗口。

> window作为全局变量，代表了脚本正在运行的窗口，暴露给 Javascript 代码。

### navigator

### location

### document

### history

### DOM

> 文档对象模型(DOM)将**web**页面与到脚本或编程语言连接起来.
> JS通过DOM访问文档;

### 表单

### 文件

### AJAX

### Promise

### Canvas

## 错误处理

> 程序有可能会出错,因此需要进行错误处理;高级语言`try ... catch ... finally`,

```javascript
'use strict';
var r1, r2, s = null;
try {
    r1 = s.length; // 此处应产生错误
    r2 = 100; // 该语句不会执行
} catch (e) {
    console.log('error: ' + e);
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

