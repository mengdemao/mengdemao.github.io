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

## 函数(ES5)

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

```javascript
function greet(name) {
  console.log("Hello, " + name + "!");
}
greet("javascript");
```

函数还可以作为参数使用

```javascript
function add(a, b) {
  return a + b;
}

function sub(a, b) {
  return a - b;
}

function calculate(func, a, b) {
  return func(a, b);
}

console.log(calculate(add, 5, 3)); // 输出 8
console.log(calculate(sub, 5, 3)); // 输出 2
```

{{< script >}}
function add(a, b) {
  return a + b;
}

function sub(a, b) {
  return a - b;
}

function calculate(func, a, b) {
  return func(a, b);
}

console.log("5 + 3 = ", calculate(add, 5, 3)); // 输出 8
console.log("5 - 3 = ", calculate(sub, 5, 3)); // 输出 2
{{< /script >}}

## 函数(ES6)

> ES6引入了许多新的函数特性，包括箭头函数、默认参数、剩余参数、展开运算符等。下面是一些重要的内容：


### 箭头函数

箭头函数是ES6中最受欢迎的新特性之一，它简化了函数的书写方式，并且在某些情况下可以提高代码可读性。

箭头函数的语法如下：

```javascript
复制代码(param1, param2, ..., paramN) => { statements }
```

例如，以下两个函数是等价的：

```javascript
// ES5
var square = function(x) {
  return x * x;
};

// ES6
const square = (x) => {
  return x * x;
};
```

{{< script >}}
const square = (x) => {
  return x * x;
};

console.log("12 * 12 = ", square(12))
{{< /script >}}

当函数体只有一行语句时，可以省略花括号和return关键字：

```javascript
// ES6
const square = (x) => x * x;
```

如果函数只有一个参数，甚至可以省略括号：

```javascript
// ES6
const square = x => x * x;
```

### 默认参数

默认参数是指函数定义时指定参数的默认值。如果调用函数时没有传递该参数，则使用默认值。

默认参数的语法如下：

```javascript
function func(param1=default1, param2=default2) {
  // ...
}
```

例如，以下两个函数是等价的：

```javascript
// ES5
function greet(name) {
  name = name || 'world';
  console.log('Hello, ' + name + '!');
}

// ES6
function greet(name='world') {
  console.log(`Hello, ${name}!`);
}
```

在上面的例子中，如果调用`greet()`函数时不传递任何参数，则将使用默认参数值 `'world'`。

### 剩余参数

剩余参数是指一个函数可以接收多个参数，并将它们转换成一个数组。

剩余参数的语法如下：

```javascript
复制代码function func(param1, param2, ...rest) {
  // rest为包含剩余参数的数组
}
```

例如，以下函数可以将所有传递给它的参数相加：

```javascript
function sum(...args) {
  return args.reduce((acc, val) => acc + val, 0);
}

console.log(sum(1, 2, 3)); // 输出6
console.log(sum(4, 5, 6, 7)); // 输出22
```

在上面的例子中，`sum()`函数可以接受任意数量的参数，并将它们存储在一个名为`args`的数组中。

### 展开运算符

展开运算符允许将一个数组或对象拆分成单个元素，并将它们传递给函数。

展开运算符的语法如下：

```javascript
// 对象展开
const obj = { prop1: 'value1', prop2: 'value2' };
const newObj = { ...obj, prop3: 'value3' };

// 数组展开
const arr = [1, 2, 3];
const newArr = [0, ...arr, 4];

// 函数参数展开
function func(x, y, z) {
  // ...
}
const args = [1, 2, 3];
func(...args);
```

在上面的例子中，`...obj`将对象拆分成单个属性，并添加一个新的属性`prop3`。`...arr`将数组拆分成单个元素，并将它们插入到新数组中。`...args`将数组展开并传递给`func()`函数作为单独的参数。

这些都是ES6引入的一些重要的函数特性。它们使代码更加简洁、易读和灵活，因此在编写现代JavaScript应用程序时非常有用。

## 面向对象

### 创建对象(ES5)

> ES5设计的面向对象真的麻烦

#### Object对象

+ 创建对象

```javascript
// 定义对象
var student = new object();
```

+ 添加成员

```javascript
// 添加属性
student.age = 18;
student.name = "hello";

// 添加方法
student3.printStudent = function() {
	console.log('name : ' + this.name);
	console.log('age : ' + this.age);
}
```

+ 修改成员

```javascript
// 修改属性
student.age = 19;
student.name = "world";

// 修改方法
student3.printStudent = function() {
	console.log('new name : ' + this.name);
	console.log('new age : ' + this.age);
}
```

+ 删除成员

```javascript
delete(student3.name);
student3.printStudent();

// 打印结果
// new name : undefined
// new age : 19
```

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
#### 继承语法

在 ES5 中，我们可以使用原型链来实现对象的继承。具体来说，就是通过创建一个新的构造函数，并将父类的实例作为新构造函数的原型，从而实现继承。

下面是一个简单的继承示例：

```javascript
function Person(name, age) {
  this.name = name;
  this.age = age;
}

Person.prototype.sayHello = function() {
  console.log("Hello, my name is " + this.name + ", I'm " + this.age + " years old.");
};

function Student(name, age, grade) {
  Person.call(this, name, age); // 调用父类的构造函数
  this.grade = grade;
}

Student.prototype = Object.create(Person.prototype);
Student.prototype.constructor = Student;

Student.prototype.study = function() {
  console.log(this.name + " is studying in grade " + this.grade + ".");
};

var jerry = new Student("Jerry", 12, 6);
jerry.sayHello(); // 输出：Hello, my name is Jerry, I'm 12 years old.
jerry.study(); // 输出：Jerry is studying in grade 6.
```

在上面的示例中，我们定义了两个构造函数 Person 和 Student，并通过原型链实现了 Student 继承了 Person。具体来说，我们通过 Object.create() 方法创建了一个空对象，并把 Person.prototype 赋值给这个空对象的原型，然后再把这个新对象赋值给 Student.prototype，这样就建立了 Student 的原型链。最后，我们还需要重置 Student.prototype 的构造函数为 Student，以确保继承关系正确。

在子类的构造函数中，我们需要调用父类的构造函数来初始化父类的属性。这里使用了 Person.call(this, name, age)，其中的 this 指代的是子类的实例，并将父类的属性赋值给子类的实例。这样，子类就可以继承父类的属性和方法。

在子类的原型上，我们还可以定义新的方法，例如 study() 方法。

通过原型链继承虽然比较简单，但也存在一些缺点，例如：

+ 父类的引用类型属性会被所有子类实例共享，容易造成修改冲突。

+ 不能向父类的构造函数传递参数。

+ 在创建子类实例时，无法向父类构造函数中传递参数。

#### 封装语法

在 ES5 中，通过使用函数作用域和闭包，可以实现对象的封装。具体来说，我们可以使用函数作为对象的构造函数，并在构造函数内部定义变量和方法，并使用闭包来保护这些私有属性和方法，使得它们不能被外界直接访问。

下面是一个简单的封装示例：

```javascript
function Person(name, age) {
  var _name = name;
  var _age = age;

  function getName() {
    return _name;
  }

  function getAge() {
    return _age;
  }

  function setName(name) {
    _name = name;
  }

  function setAge(age) {
    _age = age;
  }

  this.sayHello = function() {
    console.log("Hello, my name is " + _name + ", I'm " + _age + " years old.");
  };

  Object.defineProperty(this, "name", {
    get: getName,
    set: setName
  });

  Object.defineProperty(this, "age", {
    get: getAge,
    set: setAge
  });
}

var tom = new Person("Tom", 29);
console.log(tom.name); // 输出：Tom
console.log(tom.age); // 输出：29

tom.sayHello(); // 输出：Hello, my name is Tom, I'm 29 years old.

tom.name = "Jerry";
tom.age = 30;

console.log(tom.name); // 输出：Jerry
console.log(tom.age); // 输出：30

tom.sayHello(); // 输出：Hello, my name is Jerry, I'm 30 years old.
```

在上面的示例中，我们使用函数作为 `Person` 的构造函数，并在构造函数内部定义了变量 `_name` 和 `_age`，以及方法 `getName()`、`getAge()`、`setName()` 和 `setAge()`。这些变量和方法通过闭包来保护，使得它们只能在构造函数内部被访问。

为了使得外部代码可以访问到对象的属性 `name` 和 `age`，我们使用 `Object.defineProperty()` 方法给对象定义了 getter 和 setter。

最后，我们还在对象的原型上定义了公共方法 `sayHello()`，用于向外界展示对象的信息。

通过封装，我们可以将对象的实现细节隐藏起来，避免外部直接访问和修改对象的内部状态，从而提高了代码的安全性和可维护性。

### 创建对象(ES6)

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

#### 继承语法

在 ES6 中，我们可以使用 `class` 关键字来定义一个类，并使用 `extends` 关键字来实现继承。具体来说，我们可以定义一个子类并通过 `extends` 关键字指定它的父类，从而实现子类继承父类的所有属性和方法。

下面是一个简单的继承示例：

```javascript
class Person {
  constructor(name, age) {
    this.name = name;
    this.age = age;
  }

  sayHello() {
    console.log("Hello, my name is " + this.name + ", I'm " + this.age + " years old.");
  }
}

class Student extends Person {
  constructor(name, age, grade) {
    super(name, age); // 调用父类的构造函数
    this.grade = grade;
  }

  study() {
    console.log(this.name + " is studying in grade " + this.grade + ".");
  }
}

var jerry = new Student("Jerry", 12, 6);
jerry.sayHello(); // 输出：Hello, my name is Jerry, I'm 12 years old.
jerry.study(); // 输出：Jerry is studying in grade 6.
```

在上面的示例中，我们定义了一个 `Person` 类作为父类，并定义了两个实例属性 `name` 和 `age`，以及一个实例方法 `sayHello()`。然后，我们定义了一个 `Student` 类作为子类，并通过 `extends` 关键字让它继承了 `Person` 类的所有属性和方法。在子类的构造函数中，我们需要调用父类的构造函数来初始化父类的属性。这里使用了 `super()` 方法，其中的 `super` 指代的是父类的构造函数。

在子类的原型上，我们还可以定义新的方法，例如 `study()` 方法。

通过类继承，不仅可以实现继承，还可以实现多态和封装等面向对象编程的基本概念，同时也更易于理解和使用，代码可读性更高。

需要注意的是，在子类中重写父类的方法时，如果要调用父类的方法，可以通过 `super` 关键字来调用父类的同名方法。例如：

```javascript
class Person {
  sayHello() {
    console.log("Hello, I'm a person.");
  }
}

class Student extends Person {
  sayHello() {
    super.sayHello(); // 调用父类的 sayHello() 方法
    console.log("I'm a student.");
  }
}

var jerry = new Student();
jerry.sayHello(); // 输出：Hello, I'm a person.   I'm a student.
```

以上就是 ES6 中继承的语法和用法。相较于 ES5 中的原型链继承，ES6 的类继承更加灵活易用，同时也符合面向对象编程的基本概念和规范。

#### 封装语法

ES6中提供了一些封装语法，可以用来实现面向对象编程中的封装。

其中包括：

1. 类(Class)：使用class关键字定义一个类，可以封装属性和方法。
2. 构造函数(Constructor)：使用constructor方法定义一个构造函数，可以在创建对象时初始化对象的属性。
3. get和set方法：可以使用get和set方法对类的属性进行读取和设置操作，从而实现隐藏属性的细节。
4. Symbol类型：可以使用Symbol类型定义一个独一无二的属性，为类添加私有变量。
5. 封装模块(Module)：使用export和import关键字将模块封装成一个独立的单元。
6. Proxy代理：使用Proxy代理可以控制对类和对象的访问，实现更加精细的权限控制。

这些封装语法可以帮助开发者更好地实现代码的封装，提高代码的可维护性和安全性。

##### 类(Class)

可以使用class关键字定义一个类，包含属性和方法。下面是一个简单的例子：

```javascript
class Person {
  constructor(name, age) {
    this.name = name;
    this.age = age;
  }

  sayHello() {
    console.log(`Hello, my name is ${this.name}, I'm ${this.age} years old.`);
  }
}
```

在这个例子中，我们定义了一个名为Person的类，它有两个属性name和age以及一个方法sayHello。构造函数constructor会在实例化对象时被调用，用来初始化属性值。方法sayHello用来输出属性值。

可以通过以下方式创建Person的实例：

```
javascript复制代码const person = new Person('Tom', 18);
person.sayHello(); // Hello, my name is Tom, I'm 18 years old.
```

##### 构造函数(Constructor)

构造函数是类中特殊的方法，在类实例化时被调用，用来初始化对象的属性。

```javascript
class Person {
  constructor(name, age) {
    this.name = name;
    this.age = age;
  }
}
```

在这个例子中，我们定义了一个名为Person的类，构造函数constructor接受两个参数name和age，并将它们分别赋值给实例的属性name和age。

##### get和set方法

可以使用get和set方法对类的属性进行读取和设置操作，从而实现隐藏属性的细节。

```javascript
class Person {
  constructor(name, age) {
    this._name = name;
    this._age = age;
  }

  get name() {
    return this._name;
  }

  set name(name) {
    this._name = name;
  }

  get age() {
    return this._age;
  }

  set age(age) {
    this._age = age;
  }
}
```

在这个例子中，我们使用get和set方法来封装属性name和age。注意：我们将实际存储数据的变量名改为了以"_"开头的变量名，避免与get和set方法同名而造成冲突。

可以通过以下方式读取或设置Person的实例的属性：

```javascript
javascript复制代码const person = new Person('Tom', 18);
console.log(person.name); // Tom
person.name = 'Jerry';
console.log(person.name); // Jerry
```

##### Symbol类型

Symbol类型是ES6新增的一种基本数据类型，可以用来定义独一无二的属性值，从而达到隐藏属性的目的。

```
javascript复制代码const _name = Symbol('name');
const _age = Symbol('age');

class Person {
  constructor(name, age) {
    this[_name] = name;
    this[_age] = age;
  }

  get name() {
    return this[_name];
  }

  set name(name) {
    this[_name] = name;
  }

  get age() {
    return this[_age];
  }

  set age(age) {
    this[_age] = age;
  }
}
```

在这个例子中，我们使用Symbol类型创建了两个唯一的属性_name和_age，并通过get和set方法来封装属性。

##### 封装模块(Module)

可以使用export和import关键字将模块封装成一个独立的单元，从而实现代码的模块化和封装。

```javascript
// person.js
const _name = Symbol('name');
const _age = Symbol('age');

export class Person {
  constructor(name, age) {
    this[_name] = name;
    this[_age] = age;
  }

  get name() {
    return this[_name];
  }

  set name(name) {
    this[_name] = name;
  }

  get age() {
    return this[_age];
  }

  set age(age) {
    this[_age] = age;
  }
}

// app.js
import { Person } from './person.js';

const person = new Person('Tom', 18);
console.log(person.name); // Tom
```

在这个例子中，我们将Person类封装到person.js中，并通过export关键字导出



### 访问成员

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

## 标准对象

> JS为我们定义的标准对象

+ Map(映射)
+ Set(集合)
+ Date(日期)
+ RegExp(正则表达式)
+ JSON(JSON)
+ Math(数学)

### Map(映射)

Map 是一种用于在 JavaScript 中存储键值对的数据结构，其中每个键唯一且与一个值关联。它是 ES6 中引入的新类型之一。以下是使用 Map 的一些基本操作：

1. 创建 Map 对象

可以通过以下方式创建一个空 Map 对象：

```javascript
const myMap = new Map();
```

也可以将初始键值对作为数组传递给 Map 构造函数：

```javascript
const myMap = new Map([['key1', 'value1'], ['key2', 'value2']]);
```

1. 在 Map 中添加/更新键值对

可以使用 set() 方法向 Map 中添加键值对。如果键已经存在，则会更新其对应的值。

```javascript
myMap.set('key3', 'value3');
```

1. 从 Map 中获取值

可以使用 get() 方法从 Map 中获取与给定键相关联的值。如果该键不存在，则返回 undefined。

```javascript
console.log(myMap.get('key1')); // 输出 "value1"
console.log(myMap.get('nonexistentKey')); // 输出 "undefined"
```

1. 检查 Map 中是否存在某个键

可以使用 has() 方法检查 Map 中是否存在指定的键。

```javascript
console.log(myMap.has('key1')); // 输出 "true"
console.log(myMap.has('nonexistentKey')); // 输出 "false"
```

1. 从 Map 中删除某个键值对

可以使用 delete() 方法从 Map 中删除指定的键值对。

```javascript
myMap.delete('key2');
```

1. 获取 Map 中键值对的数量

可以使用 size 属性获取 Map 中键值对的数量。

```javascript
console.log(myMap.size); // 输出 "2"
```

以上是一些基本操作，还可以使用 forEach() 方法遍历 Map 中的所有键值对。在这个方法中，每个键值对将作为参数传递给回调函数。

```javascript
myMap.forEach(function(value, key) {
    console.log(key + ' = ' + value);
});
```

希望这些基本操作能够帮助你了解如何使用 Map 在 JavaScript 中存储和操作键值对。

### Set(集合)

JavaScript中的Set对象是ES6中新增的一种数据结构，它可以帮助我们快速地存储和检索一些独特的值。本文将简要介绍Set对象以及如何使用它。

#### 创建Set

我们可以使用`new`关键字来创建一个新的Set对象，例如：

```javascript
const mySet = new Set()
```

这将创建一个名为`mySet`的空Set对象。如果我们想初始化Set对象并传入一些值，则可以在创建时将一个可迭代对象作为参数传递，例如：

```javascript
const mySet = new Set([1, 2, 3])
```

这将创建一个名为`mySet`的Set对象，并将1、2和3添加到其中。

#### 向Set中添加元素

我们可以使用`add()`方法向Set对象中添加元素，例如：

```javascript
mySet.add(4)
```

这将向`mySet`中添加数字4。

请注意，Set对象只能包含唯一的值，因此如果我们尝试向Set中添加一个已经存在的值，它将被忽略。

#### 从Set中删除元素

我们可以使用`delete()`方法从Set对象中删除元素，例如：

```javascript
mySet.delete(4)
```

这将从`mySet`中删除数字4。

#### 检查Set中是否存在元素

我们可以使用`has()`方法来检查Set对象中是否存在某个元素，例如：

```javascript
mySet.has(3) // true
mySet.has(4) // false
```

#### 获取Set中的元素数量

我们可以使用`size`属性来获取Set对象中元素的数量，例如：

```javascript
mySet.size // 3
```

#### 遍历Set对象

我们可以使用`forEach()`方法或者`for...of`循环来遍历Set对象中的元素，例如：

```javascript
// 使用forEach()
mySet.forEach(function(value) {
  console.log(value)
})

// 使用for...of
for (const value of mySet) {
  console.log(value)
}
```

这些代码将输出Set对象中的每个元素。

#### 将Set转换为数组

我们可以使用扩展运算符(`...`)或者`Array.from()`方法将Set对象转换为数组，例如：

```javascript
// 使用扩展运算符
const myArray = [...mySet]

// 使用Array.from()
const myArray = Array.from(mySet)
```

这些代码将创建一个名为`myArray`的数组，并将Set对象中的所有元素添加到其中。

### Date(日期)

在JavaScript中，Date对象是一种用于处理日期和时间的内置对象。我们可以使用它来获取当前日期和时间、创建指定日期的实例、执行日期和时间计算等操作。本文将介绍如何使用Date对象。

#### 获取当前日期和时间

我们可以使用`new Date()`构造函数来创建一个Date对象，该对象将包含当前日期和时间。例如：

```javascript
const now = new Date()
console.log(now) // 输出当前日期和时间
```

#### 创建指定日期的实例

我们可以使用`new Date(year, month, day, hours, minutes, seconds, milliseconds)`构造函数来创建一个指定日期和时间的Date对象。其中，year表示年份（从1900开始），month表示月份（0表示1月，11表示12月），day表示日期（从1开始）。hours、minutes、seconds和milliseconds分别表示小时、分钟、秒和毫秒数。如果这些参数中任意一个省略，则默认为0。例如：

```javascript
const someDate = new Date(2023, 5, 15, 8, 30)
console.log(someDate) // 输出2023年6月15日 08:30:00
```

#### 获取Date对象的值

我们可以使用Date对象的各种方法来获取其表示的日期和时间。例如：

```javascript
const date = new Date()

// 获取年份（四位数）
const year = date.getFullYear()
console.log(year)

// 获取月份（0-11）
const month = date.getMonth()
console.log(month)

// 获取日期（1-31）
const day = date.getDate()
console.log(day)

// 获取星期几（0-6，0表示星期日）
const weekday = date.getDay()
console.log(weekday)

// 获取小时数（0-23）
const hours = date.getHours()
console.log(hours)

// 获取分钟数（0-59）
const minutes = date.getMinutes()
console.log(minutes)

// 获取秒数（0-59）
const seconds = date.getSeconds()
console.log(seconds)

// 获取毫秒数
const milliseconds = date.getMilliseconds()
console.log(milliseconds)
```

#### 设置Date对象的值

我们可以使用各种`set`方法来设置Date对象表示的日期和时间。例如：

```javascript
const date = new Date()

// 设置年份
date.setFullYear(2023)

// 设置月份
date.setMonth(5) // 月份从0开始计数

// 设置日期
date.setDate(15)

// 设置小时数
date.setHours(8)

// 设置分钟数
date.setMinutes(30)

// 设置秒数
date.setSeconds(0)

// 设置毫秒数
date.setMilliseconds(0)

console.log(date) // 输出2023年6月15日 08:30:00
```

#### 执行日期和时间计算

我们可以使用各种算术运算符和函数来执行日期和时间计算。例如：

```javascript
const now = new Date()

// 将日期增加1天
const tomorrow = new Date(now.getTime() + 24 * 60 * 60 * 1000)
console.log(tomorrow)

// 计算两个日期之间的天数差
const start = new Date(2023, 5, 1)
const end = new Date(2023, 5, 15)
const days = Math.floor((end - start) / (24 * 60 * 60 * 1000))
console.log(days)
```

这些代码将增加当前日期1天，并计算6月1日和6月15日之间的天数差。

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

## 宿主对象

> 由浏览器提供的对象

+ window
+ navigator
+ location
+ document
+ history

### window

> window 对象表示一个包含 DOM 文档的窗口，其 document 属性指向窗口中载入的 DOM 文档。
> 使用 document.defaultView 属性可以获取指定文档所在窗口。

> window作为全局变量，代表了脚本正在运行的窗口，暴露给 Javascript 代码。

### navigator

### location

### document

### history

### cookie

## DOM

> 文档对象模型(DOM)将**web**页面与到脚本或编程语言连接起来.
> JS通过DOM访问文档;

DOM（Document Object Model）指的是文档对象模型，它是一种用来访问和操作HTML文档的API。在JavaScript中，可以使用DOM API来访问和修改HTML文档中的元素、属性和样式等内容。

以下是一个简单的DOM操作示例：

```html
<!DOCTYPE html>
<html>
<head>
	<title>DOM示例</title>
	<script>
		window.onload = function() {
			var element = document.getElementById("myElement");
			element.innerHTML = "Hello DOM!";
		};
	</script>
</head>
<body>
	<div id="myElement"></div>
</body>
</html>
```

上述代码将在页面加载完成后，使用document对象的getElementById方法获取id为"myElement"的元素，并将其innerHTML属性设置为"Hello DOM!"。

下面是一些常见的DOM操作：

1. 获取元素： 可以使用document对象的getElementById、getElementsByClassName、getElementsByTagName等方法获取HTML元素，例如：

```javascript
var element = document.getElementById("myElement");
var elements = document.getElementsByClassName("myClass");
var elements = document.getElementsByTagName("div");
```

1. 修改元素： 可以使用元素的innerHTML、textContent、setAttribute等属性或方法来修改元素的内容和属性，例如：

```javascript
element.innerHTML = "New content";
element.setAttribute("class", "newClass");
```

1. 添加新元素： 可以使用document.createElement方法创建新元素，然后使用appendChild或insertBefore方法将其添加到文档中，例如：

```javascript
var newElement = document.createElement("div");
newElement.innerHTML = "New element";
document.body.appendChild(newElement);
```

1. 删除元素： 可以使用元素的removeChild方法或parentElement.removeChild方法将元素从文档中删除，例如：

```javascript
var element = document.getElementById("myElement");
element.parentElement.removeChild(element);
```

除了以上列出的DOM操作，还有许多其他的操作，如动态修改样式、注册事件处理程序等。总之，DOM提供了一些强大的API，使得JavaScript能够更好地控制和访问HTML文档中的元素、属性和样式等内容。

测试程序

<div id="myElement">这里有一个div标签</div>

{{< script >}}
var element = document.getElementById("myElement");
element.innerHTML = "Hello DOM!";
{{< /script >}}

程序会将上面的`这里有一个div标签`改写成`Hello DOM!`

## 表单

JavaScript可以用于操作HTML表单元素，包括获取和设置表单元素的值、验证表单输入等。

以下是一些常见的表单操作示例：

1. 获取表单元素的值： 可以通过document对象的getElementById方法或getElementsByTagName方法获取表单元素，并使用其value属性获取元素的值，例如：

```javascript
var input = document.getElementById("myInput");
var value = input.value;
```

1. 设置表单元素的值： 可以通过setAttribute方法或直接修改元素的value属性来设置表单元素的值，例如：

```javascript
var input = document.getElementById("myInput");
input.setAttribute("value", "new value");

// 或者
input.value = "new value";
```

1. 验证表单输入： 可以使用正则表达式或其他逻辑来验证表单输入是否符合要求，并在必要时向用户显示错误消息，例如：

```javascript
var input = document.getElementById("myInput");
if (!/^[a-zA-Z]+$/.test(input.value)) {
    alert("请输入字母！");
}
```

1. 提交表单数据： 可以使用form元素的submit方法提交表单数据，例如：

```javascript
var form = document.getElementById("myForm");
form.submit();
```

除了以上列出的表单操作，还有其他一些常见的操作，如重置表单数据、动态添加表单元素等。总之，JavaScript可以帮助我们更好地控制和操作HTML表单元素。

## 文件

JavaScript 可以操作文件，但是在浏览器端的 JavaScript 有一些限制，不能直接访问本地文件系统。如果您需要在浏览器中读取或写入文件，可以使用 HTML5 的 File API 来实现。

以下是一个使用 File API 读取本地文件的例子：

```html
<!DOCTYPE html>
<html>
<head>
  <title>File Reader Example</title>
</head>
<body>
  <input type="file" id="file-input">
  <div id="file-content"></div>

  <script>
    const fileInput = document.getElementById('file-input');
    const fileContent = document.getElementById('file-content');

    fileInput.addEventListener('change', (event) => {
      const file = event.target.files[0];
      const reader = new FileReader();
      reader.readAsText(file);
      reader.onload = (event) => {
        fileContent.textContent = event.target.result;
      };
    });
  </script>
</body>
</html>
```

这个例子创建了一个文件输入框和一个用于显示文件内容的 div 元素。当用户选择一个文件后，它将被读取并显示在 div 中。

在 Node.js 服务器环境中，可以使用 fs 模块来进行文件操作。以下是一个使用 fs 模块读取文件的例子：

```javascript
const fs = require('fs');
fs.readFile('/path/to/file', 'utf8', (err, data) => {
  if (err) throw err;
  console.log(data);
});
```

这个例子使用 `fs.readFile` 方法读取指定路径的文件，并在控制台打印出文件内容。

## AJAX

AJAX（Asynchronous JavaScript And XML）是一种用于创建动态 Web 应用程序的技术。它可以在不刷新整个页面的情况下，从服务器端异步加载数据，更新部分页面内容，提高用户体验。

以下是使用原生 JavaScript 实现 AJAX 的基本步骤：

1. 创建 XMLHttpRequest 对象

```
javascript复制代码const xhr = new XMLHttpRequest();
```

1. 指定请求处理函数

```javascript
xhr.onreadystatechange = function() {
  if (xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
    console.log(xhr.responseText);
  }
};
```

1. 发送请求

```
javascript复制代码xhr.open('GET', 'https://example.com/data');
xhr.send();
```

以上代码创建了一个 XMLHttpRequest 对象，并指定了 `onreadystatechange` 函数来处理服务器返回的响应。然后发送一个 GET 请求到指定的 URL。

当 readyState 的值为 4 时，表示请求已完成并接收到了响应。如果 status 的值为 200，则说明响应成功。此时可以通过 `responseText` 属性来获取服务器返回的数据。

实际上，AJAX 可以使用多种 HTTP 方法（例如 GET、POST、DELETE、PUT 等），也可以发送任何类型的数据（例如表单数据、JSON 数据等）。根据实际需求选择相应的方法和数据格式即可。

需要注意的是，在跨域请求时，需要设置 CORS 或 JSONP 等方式来允许跨域访问。

## Promise

Promise 是一种处理异步操作的技术，它可以让我们更优雅地编写异步代码，并避免了回调地狱等问题。

以下是 Promise 的基本使用方法：

1. 创建 Promise 对象

```
javascript复制代码const promise = new Promise((resolve, reject) => {
  // 异步操作
});
```

Promise 构造函数接受一个函数作为参数，该函数有两个参数：`resolve` 和 `reject`。当异步操作成功时，调用 `resolve` 函数并传递数据；当异步操作失败时，调用 `reject` 函数并传递错误信息。

1. 处理 Promise 结果

```
javascript复制代码promise.then(data => {
  // 成功处理
}).catch(error => {
  // 失败处理
});
```

`then` 方法用于处理异步操作成功时的结果，接收一个回调函数作为参数，该函数的参数即为 `resolve` 函数传递的数据。`catch` 方法用于处理异步操作失败时的结果，接收一个回调函数作为参数，该函数的参数即为 `reject` 函数传递的错误信息。

1. Promise 链式调用

```javascript
promise.then(data => {
  // 处理第一次异步操作
  return anotherPromise;
}).then(data => {
  // 处理第二次异步操作
}).catch(error => {
  // 错误处理
});
```

Promise 对象可以链式调用，每个 then 方法返回的都是一个新的 Promise 对象。这样可以依次执行多个异步操作，并在最后处理成功或失败的结果。

需要注意的是，Promise 可以使用异步操作、回调函数等技术来实现，但它本身并不是异步的。Promise 的作用在于封装和组合异步操作，使代码更加可读和易维护。

当然，如果需要兼容一些比较老的浏览器，可以使用 Promise 的 polyfill 库来实现 Promise 相关的功能。

## Canvas

Canvas 是 HTML5 中新增的一个用于绘制图形的技术，它可以实现动画效果、游戏开发等功能。以下是 Canvas 的基本使用方法：

1. 获取 Canvas 元素

```
javascript复制代码const canvas = document.getElementById('my-canvas');
const ctx = canvas.getContext('2d');
```

Canvas 元素是一个 HTML 元素，在 JavaScript 中可以通过 `document.getElementById` 方法获取到。获取到元素后，还需要调用 `getContext` 方法来获取绘图上下文对象。

1. 绘制图形

```
javascript复制代码ctx.fillStyle = 'red';
ctx.fillRect(10, 10, 100, 100);
```

这个例子使用 `fillStyle` 属性设置填充颜色为红色，然后使用 `fillRect` 方法在 Canvas 上绘制一个矩形。

1. 清空 Canvas

```
javascript复制代码ctx.clearRect(0, 0, canvas.width, canvas.height);
```

使用 `clearRect` 方法可以清空整个 Canvas。

1. 绘制路径

```
javascript复制代码ctx.beginPath();
ctx.moveTo(50, 50);
ctx.lineTo(100, 100);
ctx.lineTo(50, 150);
ctx.closePath();
ctx.stroke();
```

这个例子使用 `beginPath` 方法开始一条新路径，然后依次调用 `moveTo` 和 `lineTo` 方法来绘制路径，最后使用 `closePath` 方法闭合路径，并使用 `stroke` 方法描边。

1. 绘制图片

```
javascript复制代码const img = new Image();
img.onload = function() {
  ctx.drawImage(img, 0, 0);
};
img.src = 'image.jpg';
```

使用 Image 对象加载图片后，可以在 `onload` 回调函数中使用 `drawImage` 方法将图片绘制到 Canvas 中。

Canvas 还有很多其他的功能，例如设置字体、旋转变换等。需要根据实际需求选择相应的 API。

需要注意的是，Canvas 是一个位图技术，它不支持矢量图形。因此，在设计图形时需要注意分辨率和像素密度等问题。

## WASM

在 JavaScript 中，可以通过以下方法与 WebAssembly 模块进行交互：

1. 加载 WebAssembly 模块：使用 `WebAssembly.instantiate()` 或 `WebAssembly.instantiateStreaming()` 函数加载并实例化 WebAssembly 模块。
2. 导入和导出函数：WebAssembly 模块可以从 JavaScript 中导入函数，并且可以将函数从 WebAssembly 模块导出到 JavaScript 中。
3. 使用内存：WebAssembly 模块可以分配内存，并通过指针引用内存中的数据。在 JavaScript 中，可以通过 `WebAssembly.Memory` 对象来访问 WebAssembly 内存。
4. 调用函数：在 JavaScript 中，可以通过 `WebAssembly.Instance` 对象来调用 WebAssembly 模块中导出的函数。可以将参数传递给这些函数，并获取函数的返回值。

下面是一个简单的示例，展示了如何在 JavaScript 中加载、实例化和调用 WebAssembly 模块：

```javascript
// 从 URL 加载 WebAssembly 模块
fetch('example.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {}))
  .then(obj => {
    // 获取导出的函数
    const { add } = obj.instance.exports;

    // 调用函数并获取返回值
    const result = add(1, 2);
    console.log(result); // 输出: 3
  });
```

在这个示例中，我们首先从 URL 加载 WebAssembly 模块，并使用 `WebAssembly.instantiate()` 函数实例化它。然后，我们获取导出的函数 `add` 并调用它，传递两个整数参数。最后，我们将函数的返回值打印到控制台上。

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

