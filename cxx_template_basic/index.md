# C++模板基础


## 相关知识
**gcc**中`typeof`关键字用来检查变量类型,那么则可以用来判断魔板生成中的数据类型,
但是在C++中存在这另外的运算符`typeid`,但是我个人认为`typeof`更加优秀，但是二者并不相同,
`typeid`返回类型对象,`typeof`只可以判断类型;

代码如下:

```c
#define __toStr(x) #x
#define toStr(x) __toStr(x)

#define check_type_item(_x, type) \
if (typeid(_x) == typeid(type)) { \
	std::cout << toStr(_x) << " is " << toStr(type) << std::endl; \
} else

#define check_type_tail(_x) \
{ \
	std::cout << toStr(_x) << " is unknow" << std::endl; \
}

#define check_type(_x) \
({ \
	check_type_item(_x, bool) 			\
	check_type_item(_x, char) 			\
	check_type_item(_x, short) 			\
	check_type_item(_x, int) 			\
	check_type_item(_x, long) 			\
	check_type_item(_x, wchar_t)		\
	check_type_item(_x, unsigned char) 	\
	check_type_item(_x, unsigned short) \
	check_type_item(_x, unsigned int) 	\
	check_type_item(_x, unsigned long) 	\
	check_type_item(_x, float) 			\
	check_type_item(_x, double) 		\
	check_type_item(_x, std::string) 	\
	check_type_tail(_x)					\
})
```

从内核中学到的一种用来编译期间校验的宏函数,用来确定推导过程是否正确

```c
#define BUILD_BUG_ON(cond) ((void)sizeof(int[1-2*(!!(cond))]))
```

C++还提供了一种运算符*static_assert*,用作编译期间静态静态检查;

```C++
static_assert(true);	// 正确: 编译通过
static_assert(false);	// 错误: static assertion failed
```

那么就可以通过此工具来分析模板推导过程是否是正确的

## 模板基础

> 模板和宏定义区别:模板在编译期进行,宏在预编译期间进行

建立通用的模板,提高复用率

C++提供两种模版机制:**函数模版**和**类模板**

### 函数模版

```c++
template <typename T>
函数声明和定义
```

+ template  -- 声明创建模版
+ typename 	-- 表明后面的符号是数据类型可以用class代替
+ T 	    -- 通用的数据类型

#### 实例

```c++
/* 两个数据交换 */
template <typename T>
void swap(T &a, T &b)
{
    T t= a; a = b; b = t;
}
```

#### 注意事项
+ 自动类型推导,必须导出类型一致的T才可以使用
+ 模版必须要确定T的数据类型,才可以使用

#### 普通函数和函数模版的区别
+ 普通函数可以发生隐式类型转换
+ 函数模板: 用自动类型推导，不可以发生隐式转换
+ 函数模板: 用显示类型推导，可以发生隐式转换

模版函数
```c++
template <typename T>
T add(T a, T b)
{
    return a + b;
}
```
调用方法
```c++
/* 自动推导 */
std::cout << add(10, 20) << std::endl;
/* 显示指定 */
std::cout << add<int>(10, 3.14) << std::endl;
```
#### 普通函数和模版函数调用规则
+ 普通函数和模版函数都可以调用,有限调用普通函数
+ 强制通过空模版参数强制调用函数模版:**函数名<>(参数列表)**
+ 函数模版也可以重载
+ 函数模版更好的匹配,选择函数模版

#### 变参模板

##### 两阶段编译检查(Two-Phase Translation) 
1. 在模板定义阶段，模板的检查并不包含类型参数的检查。只包含下面几个方面：
  + 语法检查。比如少了分号。
  + 使用了未定义的不依赖于模板参数的名称（类型名，函数名，......）
  + 未使用模板参数的static assertions。

2. 在模板实例化阶段，为确保所有代码都是有效的，模板会再次被检查，尤其是那些依赖于类型参数的部分。

总结来说

+ 模板实例化(不同于预编译)
+ 程序编译

#### 类型推断中的类型转换

在类型推断的时候自动的类型转换是受限制的：

+ 如果调用参数是按引用传递的，任何类型转换都不被允许。通过模板类型参数T 定义的
两个参数，它们实参的类型必须完全一样。
+ 如果调用参数是按值传递的，那么只有退化（decay）这一类简单转换是被允许的：`const`
和`volatile `限制符会被忽略，引用被转换成被引用的类型，raw array 和函数被转换为相
应的指针类型。通过模板类型参数T 定义的两个参数，它们实参的类型在退化（decay）
后必须一样。

#### 多模板参数调用

主要是为了完成`max(1, 21)`,但是

```c
template<typename T>
T max (T a, T b)
{
	return b < a ? a : b;
}
```
如果直接调用`error: no matching function for call to 'max(int, double)',会报错,因此可以使用如下方法:

##### 显式指定
将模板显式指定,从而可以保证编译进行
```c
std::cout << max<double>(1, 2.1) << std::endl;
```
但是总感觉不太优雅,因此一定存在更加优秀的方案,但是此时也可以称之为一个方案

##### 返回值指定

```c
template<typename T1, typename T2, typename RT = 
		std::decay_t<decltype(true ? T1() : T2())> >
RT max (T1 a, T2 b)
{
	return b < a ? a : b;
}

int main(int argc, char *argv[])
{
	float a = 12, b = 12;
	check_type(max(1, 2)); 		// int
	check_type(max(a, 2));		// float
	check_type(max(b, 2.0));	// double
}
```

其结果满足我的基本知识

```shell
max(1, 2) is int
max(a, 2) is float
max(b, 2.0) is double
```

{{< admonition type=tip title="补充知识1" open=true >}}
`auto`和`decltype`关键字都可以自动推导出变量的类型,但它们的用法是有区别的;
`auto`是根据初始化的时候变量或者表达式的类型来处理,`auto a = value`
`decltype`使用需要表达式,`decltype(exp) varname = value;`通过计算exp的类型来确定类型
因此,`decltype`可以没有初始化的参数,额`auto`则不可以
{{< /admonition >}}

`std::decay_t`可以认为是剥离引用类型, T:某种类型.
+ 当T是引用类型,decay<T>::type返回T引用的元素类型;
+ 当T是非引用类型,decay<T>::type返回T的类型.

##### 自动推导

> 不指定类型,而是由编译器自行判断,`auto`关键字判断

```c
template<typename T1, typename T2>
auto max(T1 a, T2 b)
{
	return b < a ? a : b;
}
```

##### 公共类型推导

> 类型萃取std::common_type<>作为返回类型的默认值

```c++
template<typename T1, typename T2>
std::common_type_t<T1,T2> max(T1 a, T2 b)
{
	return b < a ? a : b;
}
```

#### 默认模板参数

> 你也可以给模板参数指定默认值。这些默认值被称为默认模板参数并且可以用于任意类型的
> 模板。它们甚至可以根据其前面的模板参数来决定自己的类型

简单执行默认参数,直接在`T=类型`即可,同时可以填入相同类型的默认参数

```c
template<typename T = std::string>
T HelloWorld(T f = "HelloWorld")
{
	return f;
}

int main(int argc, char *argv[])
{
	check_type(HelloWorld());
	check_type(HelloWorld(123));
	return 0;
}
```

那么显式如下:

```shell
HelloWorld() is std::string
HelloWorld(123) is int
```

模板类型和默认参数必须是同一种类型

#### 函数模板重载

### 类模板
```c++
template <class T>
类
```

例子
```c++
template <class NameType, class AgeType>
class Person {
public:
    Person(NameType Name, AgeType Age)
    {
        m_Name = Name;
        m_Age  = Age;
    }
    NameType m_Name;
    AgeType m_Age;
};
```

实例化
```c++
Person<std::string, int> p("Hello", 99);
```

### 非类型模板参数

> 对于之前介绍的函数模板和类模板,其模板参数不一定非得是某种具体的类型,也可以是常
> 规数值.和类模板使用类型作为参数类似,可以使代码的另一些细节留到被使用时再确定，
> 只是对非类型模板参数,待定的不再是类型,而是某个数值.在使用这种模板时需要显式的
> 指出待定数值的具体值,之后代码会被实例化.

简言之</br>
**模板**不仅可以作为**类型**,还可以作为**数值**

#### 函数模板非类型参数
```c
template<int Val, typename T>
T max(T x)
{
	return std::max(x, Val);
}

int main(int argc, char *argv[])
{
	std::cout << max<5, int>(2);
	return 0;
}
```

#### 非类型模板参数的限制

+ 整形常量(包含枚举)
+ 指针`objects/functions/members`
+ `objects`或者`functions`的左值引用
+ `std::nullptr_t`

> 浮点型数值或者`class`类型的对象都不能作为非类型模板参数使用

```c
template<double VAT> // ERROR: floating-point values are not
void process (double v){}

template<std::string name> // ERROR: class-type objects are not
class MyClass {};
```

使用auto作为变量类型,
```c
template<typename T, auto Maxsize>
class MyClass {};
```

需要修改C++标准:
+ [ ] c++98 --> 无法编译
+ [ ] c++11 --> 无法编译
+ [ ] c++14 --> 无法编译
+ [X] c++17 --> 可以编译
+ [X] c++20 --> 可以编译

1. 模板的参数不只可以是类型,也可以是数值.
2. 不可以将浮点型或者`class`类型的对象用于非类型模板参数.使用指向字符串常量,临时变量和子对象的指针或引用也有一些限制.
3. 通过使用关键字auto,可以使非类型模板参数的类型更为泛化

### 变参模板

> 类似于C语言可变参数相似,那么,C++也存在这个类似的功能.
> 可以将模板参数定义成能够接受任意多个模板参数的情况,这一类模板被称为变参模板.

+ 通过使用参数包，模板可以有任意多个任意类型的参数。
+ 为了处理这些参数，需要使用递归，而且需要一个非变参函数终结递归（如果使用编译
期判断，则不需要非变参函数来终结递归）。
+ 运算符sizeof...用来计算参数包中模板参数的数目。
+ 变参模板的一个典型应用是用来发送（forward）任意多个任意类型的模板参数。
+ 通过使用折叠表达式,可以将某种运算应用于参数包中的所有参数.

#### 变参模板实例

```c++
template<typename T, typename... Types>
void print_var_args(T firstArg, Types... args)
{
	std::cout << firstArg << '\n'; //print first argument
	print_var_args(args...);
}
```
通过上面的实例我们可以得到,声明可变参数模板的方法
**typename... Types**或者**class... Types**,其中Types就可以用来声明可变参数
**Types... args**,但是为什么还需要在写一次`...`,我个人是不太理解的.

以args的剩余参数则称之为**函数参数包**;

但是只有上述的是编译不过的,因为存在参数消耗殆尽的情况,因此添加一个空的函数,
用作递归结束;

这就是编译期间编程的概念

```C++
void print_var_args()
{
	std::cout << "变参模板结束" << std::endl;
}
```

测试函数

```C++
int main(int argc, char *argv[])
{
	int a = 1;
	int b = 2;
	int c = 3;
	print_var_args(a, b, c);
	return 0;
}
```
最后运行情况的打印就是
{{< highlight shell >}}
# 1
# 2
# 3
# 变参模板结束
{{< /highlight >}}

#### 变参个运算符**sizeof...**

> C++11为变参模板引入了一种新的`sizeof`运算符:`sizeof...`
> 它会被扩展成参数包中所包含的参数数目

```C++
	// void print_var_args(T firstArg, Types... args)
	std::cout << "sizeof...(Types)\t" << sizeof...(Types) << std::endl;	// 模板参数包
	std::cout << "sizeof...(args) \t" << sizeof...(args)  << std::endl;	// 函数参数包
```

运行结果
```bash
# sizeof...(Types)	2
# sizeof...(args)	2
```

因此,可以得出结论
+ sizeof...可以计算每一次展开的个数
+ 既可以用于模板参数包,也可以用于函数参数包

#### 折叠表达式

> 从C++17开始,提供了一种可以用来计算参数包(可以有初始值)中所有参数运算结果的二
> 元运算符.

测试程序

```c++
template<class... T>
auto foldSun(T... s)
{
	// s1 + s2 + s3 + sn (其中n参数的个数) 
	return (... + s);
}
```

#### 变参模板的使用

## 模板提高

### 移动语义和enable_if<>

### 模板参数传递

### 编译期编程

## 模板进阶

