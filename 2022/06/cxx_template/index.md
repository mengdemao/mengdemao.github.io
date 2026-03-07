# C++模板


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
 check_type_item(_x, bool)    \
 check_type_item(_x, char)    \
 check_type_item(_x, short)    \
 check_type_item(_x, int)    \
 check_type_item(_x, long)    \
 check_type_item(_x, wchar_t)  \
 check_type_item(_x, unsigned char)  \
 check_type_item(_x, unsigned short) \
 check_type_item(_x, unsigned int)  \
 check_type_item(_x, unsigned long)  \
 check_type_item(_x, float)    \
 check_type_item(_x, double)   \
 check_type_item(_x, std::string)  \
 check_type_tail(_x)     \
})
```

从内核中学到的一种用来编译期间校验的宏函数,用来确定推导过程是否正确

```c
#define BUILD_BUG_ON(cond) ((void)sizeof(int[1-2*(!!(cond))]))
```

C++还提供了一种运算符*static_assert*,用作编译期间静态静态检查;

```C++
static_assert(true); // 正确: 编译通过
static_assert(false); // 错误: static assertion failed
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
+ typename  -- 表明后面的符号是数据类型可以用class代替
+ T         -- 通用的数据类型

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
和`volatile`限制符会被忽略，引用被转换成被引用的类型，raw array 和函数被转换为相
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
 check_type(max(1, 2));   // int
 check_type(max(a, 2));  // float
 check_type(max(b, 2.0)); // double
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
 std::cout << "sizeof...(Types)\t" << sizeof...(Types) << std::endl; // 模板参数包
 std::cout << "sizeof...(args) \t" << sizeof...(args)  << std::endl; // 函数参数包
```

运行结果

```bash
# sizeof...(Types) 2
# sizeof...(args) 2
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

## [函数模板](https://en.cppreference.com/w/cpp/language/function_template)

+ 编写时不指定具体类型，直到使用时才能确定，这个概念就是泛型。模板，顾名思义，编写一次即可适用于任意类型。模板定义以关键词 template 开始，后跟一个模板参数列表，类型参数前必须使用关键字 typename 或 class，在模板参数列表中这两个关键字含义相同，可以互换使用。函数模板通常不用声明为 inline，唯一例外的是特定类型的全特化，因为编译器可能忽略 inline，函数模板是否内联取决于编译器的优化策略

```cpp
#include <cassert>
#include <string>

namespace jc {

template <typename T>
T max(const T& a, const T& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() {
  assert(jc::max<int>(1, 3) == 3);
  assert(jc::max<double>(1.0, 3.14) == 3.14);
  std::string s1 = "down";
  std::string s2 = "demo";
  assert(jc::max<std::string>(s1, s2) == "down");
}
```

## 两阶段编译（Two-Phase Translation）

+ 模板编译分为实例化前检查和实例化两个阶段。实例化前检查模板代码本身，包括
  + 检查语法是否正确，如是否遗漏分号
  + 检查是否使用不依赖于模板参数的未知名称，如未声明的类型名、函数名
  + 检查不依赖于模板参数的静态断言

```cpp
template <typename T>
void f(T x) {
  undeclared();  // 一阶段编译错误，未声明的函数
  static_assert(sizeof(int) > 10);  // 一阶段，sizeof(int) <= 10，总会编译失败
}

int main() {}
```

+ 实例化期间保证代码有效，比如对不能解引用的类型进行解引用就会实例化出错，此外会再次检查依赖于模板参数的部分

```cpp
template <typename T>
void f(T x) {
  undeclared(x);  // 调用 undeclared(T) 才会出现函数未声明的实例化错误
  static_assert(sizeof(T) > 10);  // 如果 sizeof(T) <= 10 则实例化错误
}

int main() {
  f(42);  // 调用函数才会进行实例化，不调用则不会有实例化错误
}
```

## [模板实参推断（Template Argument Deduction）](https://en.cppreference.com/w/cpp/language/template_argument_deduction)

+ 调用模板时，如果不显式指定模板参数类型，则编译器会根据传入的实参推断模板参数类型

```cpp
#include <cassert>
#include <string>

namespace jc {

template <typename T>
T max(const T& a, const T& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() {
  assert(jc::max(1, 3) == 3);          // T 推断为 int
  assert(jc::max(1.0, 3.14) == 3.14);  // T 推断为 double
  std::string s1 = "down";
  std::string s2 = "demo";
  assert(jc::max(s1, s2) == "down");  // T 推断为 std::string
}
```

+ 实参的推断要求一致，其本身不会为了编译通过自动做类型转换

```cpp
#include <cassert>

namespace jc {

template <typename T>
T max(const T& a, const T& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() {
  jc::max(1, 3.14);  // 错误，T 分别推断出 int 和 double，类型不明确
}
```

+ 字符串字面值传引用会推断为字符数组（传值则推断为 `const char*`，数组和函数会 decay 为指针）

```cpp
#include <cassert>
#include <string>

namespace jc {

template <typename T, typename U>
T max(const T& a, const U& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() {
  std::string s = "down";
  jc::max("down", s);  // 错误，T 推断为 char[5] 和 std::string
}
```

+ 对于推断不一致的情况，可以显式指定类型而不使用推断机制，或者强制转换实参为希望的类型使得推断结果一致

```cpp
#include <cassert>
#include <string>

namespace jc {

template <typename T, typename U>
T max(const T& a, const U& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() {
  std::string s = "demo";
  assert(jc::max<std::string>("down", "demo") == "down");
  assert(jc::max(std::string{"down"}, s) == "down");
}
```

+ 也可以增加一个模板参数，这样每个实参的推断都是独立的，不会出现矛盾

```cpp
#include <cassert>

namespace jc {

template <typename T, typename U>
T max(const T& a, const U& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() {
  assert(jc::max(1, 3.14) == 3);  // T 推断为 int，返回值截断为 int
  assert(jc::max<double>(1, 3.14) == 3.14);
}
```

+ 模板实参不能推断返回类型，必须显式指定

```cpp
#include <cassert>

namespace jc {

template <typename RT, typename T, typename U>
RT max(const T& a, const U& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() {
  assert(jc::max<double>(1, 3.14) == 3.14);
  assert((jc::max<double, int, int>(1, 3.14) == 3));
}
```

+ C++14 允许 auto 作为返回类型，它通过 return 语句推断返回类型，C++11 则需要额外指定尾置返回类型，对于三目运算符，其结果类型为两个操作数类型中更公用的类型，比如 int 和 double 的公用类型是 double

```cpp
#include <cassert>

namespace jc {

template <typename T, typename U>
auto max(const T& a, const U& b) -> decltype(true ? a : b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() { assert(jc::max(1, 3.14) == 3.14); }
```

+ 用 constexpr 函数可以生成编译期值

```cpp
namespace jc {

template <typename T, typename U>
constexpr auto max(const T& a, const U& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() { static_assert(jc::max(1, 3.14) == 3.14); }
```

## [type traits](https://en.cppreference.com/w/cpp/header/type_traits)

+ 对于类型进行计算的模板称为 type traits，也可以称为元函数，比如用 [std::common_type](https://en.cppreference.com/w/cpp/types/common_type) 来计算不同类型中最通用的类型

```cpp
#include <cassert>
#include <type_traits>

namespace jc {

template <typename T, typename U, typename RT = std::common_type_t<T, U>>
RT max(const T& a, const U& b) {
  return a < b ? b : a;
}

}  // namespace jc

int main() { assert(jc::max(1, 3.14) == 3.14); }
```

## 重载

+ 当类型同时匹配普通函数和模板时，优先匹配普通函数

```cpp
#include <cassert>

namespace jc {

int f(int a, int b) { return 1; }

template <typename T, typename U>
int f(const T&, const U&) {
  return 2;
}

}  // namespace jc

int main() {
  assert(jc::f(1, 3) == 1);
  assert(jc::f<double>(1, 3) == 2);
  assert(jc::f<>(1, 3) == 2);
  assert(jc::f(1, 3.14) == 2);
  assert(jc::f(3.14, 1) == 2);
}
```

+ 模板参数不同就会构成重载，如果对于给定的实参能同时匹配两个模板，重载解析会优先匹配更特殊的模板，如果同样特殊则产生二义性错误

```cpp
#include <cassert>

namespace jc {

template <typename T, typename U>
int f(const T&, const U&) {
  return 1;
}

template <typename RT, typename T, typename U>
int f(const T& a, const U& b) {
  return 2;
}

}  // namespace jc

int main() {
  assert(jc::f(1, 3.14) == 1);
  assert(jc::f<double>(1, 3.14) == 2);
  //   jc::f<int>(1, 3.14);  // 二义性错误
}
```

+ C-style 字符串的重载

```cpp
#include <cassert>
#include <cstring>
#include <string>

namespace jc {

template <typename T>
T max(T a, T b) {
  return a < b ? b : a;
}

template <typename T>
T* max(T* a, T* b) {
  return *a < *b ? b : a;
}

const char* max(const char* a, const char* b) {
  return std::strcmp(a, b) < 0 ? b : a;
}

}  // namespace jc

int main() {
  int a = 1;
  int b = 3;
  assert(jc::max(a, b) == b);
  assert(jc::max(&a, &b) == &b);

  std::string s1 = "down";
  std::string s2 = "demo";
  assert(jc::max(s1, s2) == "down");
  assert(std::strcmp(jc::max("down", "demo"), "down") == 0);
}
```

+ 注意不能返回 C-style 字符串的引用

```cpp
namespace jc {

template <typename T>
const T& f(const char* s) {
  return s;
}

}  // namespace jc

int main() {
  const char* s = "downdemo";
  jc::f<const char*>(s);  // 错误：返回临时对象的引用
}
```

+ 这种错误可能在添加代码的过程中引入

```cpp
#include <cstring>

namespace jc {

template <typename T>
const T& max(const T& a, const T& b) {
  return b < a ? a : b;
}

// 新增函数来支持 C-style 参数
const char* max(const char* a, const char* b) {
  return std::strcmp(a, b) < 0 ? b : a;
}

template <typename T>
const T& max(const T& a, const T& b, const T& c) {
  return max(max(a, b), c);  // max("down", "de") 返回临时对象的引用
}

}  // namespace jc

int main() {
  const char* a = "down";
  const char* b = "de";
  const char* c = "mo";
  jc::max<const char*>(a, b, c);  // 错误：返回临时对象的引用
}
```

+ 只有在函数调用前声明的重载才会被匹配，即使后续有更优先的匹配，由于不可见也会被忽略

```cpp
#include <cassert>

namespace jc {

template <typename T>
int f(T) {
  return 1;
}

template <typename T>
int g(T a) {
  return f(a);
}

int f(int) { return 2; }

}  // namespace jc

int main() { assert(jc::g(0) == 1); }
```

## 用于原始数组与字符串字面值（string literal）的模板

+ 字符串字面值传引用会推断为字符数组，为此需要为原始数组和字符串字面值提供特定处理的模板

```cpp
#include <cassert>
#include <cstddef>

namespace jc {

template <typename T, typename U>
constexpr bool less(const T& a, const U& b) {
  return a < b;
}

template <typename T, std::size_t M, std::size_t N>
constexpr bool less(T (&a)[M], T (&b)[N]) {
  for (std::size_t i = 0; i < M && i < N; ++i) {
    if (a[i] < b[i]) {
      return true;
    }
    if (b[i] < a[i]) {
      return false;
    }
  }
  return M < N;
}

}  // namespace jc

static_assert(jc::less(0, 42));
static_assert(!jc::less("down", "demo"));
static_assert(jc::less("demo", "down"));

int main() {}
```

+ 各种类型的数组参数对应的偏特化

```cpp
#include <cstddef>

namespace jc {

template <typename T>
struct A;

template <typename T, std::size_t N>
struct A<T[N]> {
  static constexpr int value = 1;
};

template <typename T, std::size_t N>
struct A<T (&)[N]> {
  static constexpr int value = 2;
};

template <typename T>
struct A<T[]> {
  static constexpr int value = 3;
};

template <typename T>
struct A<T (&)[]> {
  static constexpr int value = 4;
};

template <typename T>
struct A<T*> {
  static constexpr int value = 5;
};

template <typename T1, typename T2, typename T3>
constexpr void test(int a1[7], int a2[], int (&a3)[42], int (&x0)[], T1 x1,
                    T2& x2, T3&& x3) {
  static_assert(A<decltype(a1)>::value == 5);  // A<T*>
  static_assert(A<decltype(a2)>::value == 5);  // A<T*>
  static_assert(A<decltype(a3)>::value == 2);  // A<T(&)[N]>
  static_assert(A<decltype(x0)>::value == 4);  // A<T(&)[]>
  static_assert(A<decltype(x1)>::value == 5);  // A<T*>
  static_assert(A<decltype(x2)>::value == 4);  // A<T(&)[]>
  static_assert(A<decltype(x3)>::value == 4);  // A<T(&)[]>
}

}  // namespace jc

int main() {
  int a[42];
  static_assert(jc::A<decltype(a)>::value == 1);
  extern int x[];  // 传引用时将变为 int(&)[]
  static_assert(jc::A<decltype(x)>::value == 3);  // A<T[]>
  jc::test(a, a, a, x, x, x, x);
}

int x[] = {1, 2, 3};  // 定义前置声明的数组
```

## 零初始化（Zero Initialization）

+ 使用模板时常希望模板类型的变量已经用默认值初始化，但内置类型无法满足要求。解决方法是显式调用内置类型的默认构造函数

```cpp
namespace jc {

template <typename T>
constexpr T default_value() {
  T x{};
  return x;
}

template <typename T>
struct DefaultValue {
  constexpr DefaultValue() : value() {}
  T value;
};

template <typename T>
struct DefaultValue2 {
  T value{};
};

static_assert(default_value<bool>() == false);
static_assert(default_value<char>() == 0);
static_assert(default_value<int>() == 0);
static_assert(default_value<double>() == 0);

static_assert(DefaultValue<bool>{}.value == false);
static_assert(DefaultValue<char>{}.value == 0);
static_assert(DefaultValue<int>{}.value == 0);
static_assert(DefaultValue<double>{}.value == 0);

static_assert(DefaultValue2<bool>{}.value == false);
static_assert(DefaultValue2<char>{}.value == 0);
static_assert(DefaultValue2<int>{}.value == 0);
static_assert(DefaultValue2<double>{}.value == 0);

}  // namespace jc

int main() {}
```

## [类模板](https://en.cppreference.com/w/cpp/language/class_template)

+ 和函数类似，类也支持泛型，比如实现一个基于拓扑排序遍历的有向无环图的森林

```cpp
#include <algorithm>
#include <cassert>
#include <functional>
#include <map>
#include <memory>
#include <queue>
#include <set>
#include <unordered_set>
#include <vector>

namespace jc {

template <typename K, typename V>
struct DAGNode {
  K k;
  V v;
  std::set<DAGNode<K, V>*> in;
  std::set<DAGNode<K, V>*> out;
};

template <typename K, typename V>
class DAGGraph {
 public:
  bool AddEdge(const K& from, const K& to);

  V& operator[](const K& key);

  bool Exist(const K& key) const;

  void Clear();

  std::size_t Size() const;

  void Walk(std::function<void(const K& k, const V& v)> f,
            bool start_from_head = true);

  void WalkHeads(std::function<void(const K& k, const V& v)> f);

  void WalkTails(std::function<void(const K& k, const V& v)> f);

  std::unordered_set<K> NextKeys();

  std::unordered_set<K> NextKeys(const K& key);

 private:
  bool IsCyclic(const DAGNode<K, V>& from, const DAGNode<K, V>& to) const;

  void RefreshWalkSequences();

  std::vector<std::set<K>> ConnectedComponents() const;

  void DFS(const K& k, std::unordered_set<K>* visited,
           std::set<K>* connected_components) const;

  std::vector<K> TopologicalSequence(const std::set<K>& connected_components,
                                     bool start_from_head) const;

 private:
  std::map<K, DAGNode<K, V>> bucket_;
  std::unordered_set<K> heads_;
  std::unordered_set<K> tails_;
  std::vector<std::vector<K>> sequences_start_from_head_;
  std::vector<std::vector<K>> sequences_start_from_tail_;

 private:
  bool allow_modify_ = true;
  std::vector<std::vector<K>> sequences_start_from_head_for_next_;
  std::unordered_set<K> current_heads_for_next_;
};

template <typename K, typename V>
inline bool DAGGraph<K, V>::AddEdge(const K& from, const K& to) {
  assert(allow_modify_);
  if (from == to || !bucket_.count(from) || !bucket_.count(to) ||
      IsCyclic(bucket_.at(from), bucket_.at(to))) {
    return false;
  }
  bucket_.at(from).out.emplace(&bucket_.at(to));
  bucket_.at(to).in.emplace(&bucket_.at(from));
  heads_.erase(to);
  tails_.erase(from);
  sequences_start_from_head_.clear();
  sequences_start_from_tail_.clear();
  return true;
}

template <typename K, typename V>
inline V& DAGGraph<K, V>::operator[](const K& key) {
  if (!bucket_.count(key)) {
    assert(allow_modify_);
    bucket_[key].k = key;
    heads_.emplace(key);
    tails_.emplace(key);
    sequences_start_from_head_.clear();
    sequences_start_from_tail_.clear();
  }
  return bucket_.at(key).v;
}

template <typename K, typename V>
inline bool DAGGraph<K, V>::Exist(const K& key) const {
  return bucket_.count(key);
}

template <typename K, typename V>
inline void DAGGraph<K, V>::Clear() {
  allow_modify_ = true;
  bucket_.clear();
  heads_.clear();
  tails_.clear();
  sequences_start_from_head_.clear();
  sequences_start_from_tail_.clear();
}

template <typename K, typename V>
inline std::size_t DAGGraph<K, V>::Size() const {
  return bucket_.size();
}

template <typename K, typename V>
inline void DAGGraph<K, V>::Walk(std::function<void(const K& k, const V& v)> f,
                                 bool start_from_head) {
  if (sequences_start_from_head_.empty()) {
    RefreshWalkSequences();
  }
  const std::vector<std::vector<K>>& seqs_to_walk =
      start_from_head ? sequences_start_from_head_ : sequences_start_from_tail_;
  for (const std::vector<K>& seq : seqs_to_walk) {
    std::for_each(std::begin(seq), std::end(seq), [&](const K& key) {
      const DAGNode<K, V>& node = bucket_.at(key);
      f(node.k, node.v);
    });
  }
}

template <typename K, typename V>
inline void DAGGraph<K, V>::WalkHeads(
    std::function<void(const K& k, const V& v)> f) {
  if (sequences_start_from_head_.empty()) {
    RefreshWalkSequences();
  }
  for (const std::vector<K>& seq : sequences_start_from_head_) {
    std::for_each(std::begin(seq), std::end(seq), [&](const K& key) {
      if (heads_.count(key)) {
        const DAGNode<K, V>& node = bucket_.at(key);
        f(node.k, node.v);
      }
    });
  }
}

template <typename K, typename V>
inline void DAGGraph<K, V>::WalkTails(
    std::function<void(const K& k, const V& v)> f) {
  if (sequences_start_from_head_.empty()) {
    RefreshWalkSequences();
  }
  for (const std::vector<K>& seq : sequences_start_from_tail_) {
    std::for_each(std::begin(seq), std::end(seq), [&](const K& key) {
      if (tails_.count(key)) {
        const DAGNode<K, V>& node = bucket_.at(key);
        f(node.k, node.v);
      }
    });
  }
}

template <typename K, typename V>
inline std::unordered_set<K> DAGGraph<K, V>::NextKeys() {
  assert(allow_modify_);  // allowed call once unless Clear()
  allow_modify_ = false;
  current_heads_for_next_ = heads_;
  if (sequences_start_from_head_.empty()) {
    RefreshWalkSequences();
  }
  return heads_;
}

template <typename K, typename V>
inline std::unordered_set<K> DAGGraph<K, V>::NextKeys(const K& key) {
  assert(!allow_modify_);  // must call NextKeys() before
  assert(current_heads_for_next_.count(key));
  current_heads_for_next_.erase(key);

  std::unordered_set<K> res;
  for (std::vector<K>& seq : sequences_start_from_head_for_next_) {
    auto it = std::find(begin(seq), std::end(seq), key);
    if (it == std::end(seq)) {
      continue;
    }
    seq.erase(it);
    const std::set<DAGNode<K, V>*>& nodes = bucket_.at(key).out;
    for (DAGNode<K, V>* v : nodes) {
      const std::set<DAGNode<K, V>*>& prev_nodes = v->in;
      bool no_prev_node_in_seq =
          std::all_of(std::begin(prev_nodes), std::end(prev_nodes),
                      [&](DAGNode<K, V>* in_node) {
                        return std::find(std::begin(seq), std::end(seq),
                                         in_node->k) == std::end(seq);
                      });
      if (no_prev_node_in_seq) {
        current_heads_for_next_.emplace(v->k);
        res.emplace(v->k);
      }
    }
    break;
  }
  return res;
}

template <typename K, typename V>
inline bool DAGGraph<K, V>::IsCyclic(const DAGNode<K, V>& from,
                                     const DAGNode<K, V>& to) const {
  std::queue<DAGNode<K, V>*> q;
  for (DAGNode<K, V>* v : from.in) {
    q.emplace(v);
  }

  std::unordered_set<DAGNode<K, V>*> visited;
  while (!q.empty()) {
    DAGNode<K, V>* node = q.front();
    q.pop();
    if (visited.count(node)) {
      continue;
    }
    if (node == &to) {
      return true;
    }
    visited.emplace(node);
    for (DAGNode<K, V>* v : node->in) {
      q.emplace(v);
    }
  }

  return false;
}

template <typename K, typename V>
inline void DAGGraph<K, V>::RefreshWalkSequences() {
  sequences_start_from_head_.clear();
  sequences_start_from_tail_.clear();

  const std::vector<std::set<K>> connected_components = ConnectedComponents();
  for (const std::set<K>& x : connected_components) {
    const std::vector<K> seq_from_head = TopologicalSequence(x, true);
    const std::vector<K> seq_from_tail = TopologicalSequence(x, false);
    assert(!seq_from_head.empty());
    assert(!seq_from_tail.empty());
    sequences_start_from_head_.emplace_back(seq_from_head);
    sequences_start_from_tail_.emplace_back(seq_from_tail);
  }

  sequences_start_from_head_for_next_ = sequences_start_from_head_;
}

template <typename K, typename V>
inline std::vector<std::set<K>> DAGGraph<K, V>::ConnectedComponents() const {
  std::vector<std::set<K>> res;
  std::unordered_set<K> visited;
  for (auto& x : bucket_) {
    std::set<K> tmp;
    DFS(x.second.k, &visited, &tmp);
    if (!tmp.empty()) {
      res.emplace_back(tmp);
    }
  }
  std::sort(std::begin(res), std::end(res),
            [&](const std::set<K>& lhs, const std::set<K>& rhs) {
              return lhs.size() < rhs.size();
            });
  return res;
}

template <typename K, typename V>
inline void DAGGraph<K, V>::DFS(const K& k, std::unordered_set<K>* visited,
                                std::set<K>* connected_components) const {
  if (visited->count(k)) {
    return;
  }
  visited->emplace(k);
  connected_components->emplace(k);
  if (!bucket_.at(k).in.empty()) {
    for (DAGNode<K, V>* v : bucket_.at(k).in) {
      DFS(v->k, visited, connected_components);
    }
  }
  if (!bucket_.at(k).out.empty()) {
    for (DAGNode<K, V>* v : bucket_.at(k).out) {
      DFS(v->k, visited, connected_components);
    }
  }
}

template <typename K, typename V>
inline std::vector<K> DAGGraph<K, V>::TopologicalSequence(
    const std::set<K>& connected_components, bool start_from_head) const {
  std::map<K, std::vector<K>> adjacency_list;
  std::map<K, int32_t> in_degree;

  for (const K& key : connected_components) {
    if (!in_degree.count(key)) {
      in_degree.emplace(key, 0);
    }
    const std::set<DAGNode<K, V>*>& nodes =
        start_from_head ? bucket_.at(key).out : bucket_.at(key).in;
    for (DAGNode<K, V>* v : nodes) {
      adjacency_list[key].emplace_back(v->k);
      ++in_degree[v->k];
    }
  }

  std::queue<K> q;
  for (auto& x : in_degree) {
    if (x.second == 0) {
      q.emplace(x.first);
    }
  }

  std::vector<K> res;
  while (!q.empty()) {
    const K key = q.front();
    q.pop();
    res.emplace_back(key);
    for (const K& k : adjacency_list[key]) {
      if (--in_degree.at(k) == 0) {
        q.emplace(k);
      }
    }
  }

  assert(res.size() == connected_components.size());  // graph is DAG
  return res;
}

}  // namespace jc

namespace jc::test {

class MockPipelineEngine {
 public:
  void Start() {}
  void Stop() {}
  void Destroy() {}
};

void test() {
  DAGGraph<int, std::unique_ptr<MockPipelineEngine>> d;
  // Make Direct Acyclic Graph:
  //    0    6      11  13
  //   / \   |      |
  //  1   3  7  8   12
  //  | x |      \ /
  //  2   4       9
  //   \ /        |
  //    5         10
  // Traverse each child graph in order whose size smaller

  // Start Order:
  // 13
  // 6 -> 7
  // 8 -> 11 -> 12 -> 9 -> 10
  // 0 -> 1 -> 3 -> 2 -> 4 -> 5
  // Stop Order:
  // 13
  // 7 -> 6
  // 10 -> 9 -> 8 -> 12 -> 11
  // 5 -> 2 -> 4 -> 1 -> 3 -> 0

  constexpr int nodes_count = 14;
  for (int i = 0; i < nodes_count; ++i) {
    d[i].reset(new MockPipelineEngine);
  }
  assert(d.AddEdge(0, 1));
  assert(d.AddEdge(0, 3));
  assert(d.AddEdge(1, 2));
  assert(d.AddEdge(3, 4));
  assert(d.AddEdge(1, 4));
  assert(d.AddEdge(3, 2));
  assert(d.AddEdge(2, 5));
  assert(d.AddEdge(4, 5));
  assert(d.AddEdge(6, 7));
  assert(d.AddEdge(8, 9));
  assert(d.AddEdge(9, 10));
  assert(d.AddEdge(11, 12));
  assert(d.AddEdge(12, 9));

  assert(d.Size() == nodes_count);

  for (int i = 0; i < nodes_count; ++i) {
    assert(d.Exist(i));
  }

  assert(!d.AddEdge(1, 0));
  assert(!d.AddEdge(2, 0));
  assert(!d.AddEdge(4, 0));
  assert(!d.AddEdge(7, 6));
  assert(!d.AddEdge(10, 11));
  assert(!d.AddEdge(13, 13));
  assert(!d.AddEdge(13, 14));

  constexpr bool start_from_head = true;
  {
    std::vector<int> v;
    std::vector<int> start_order{13, 6, 7, 8, 11, 12, 9, 10, 0, 1, 3, 2, 4, 5};
    d.Walk(
        [&](int key, const std::unique_ptr<MockPipelineEngine>& pipeline) {
          pipeline->Start();
          v.emplace_back(key);
        },
        start_from_head);
    assert(v == start_order);
  }

  {
    std::vector<int> v;
    std::vector<int> stop_order{13, 7, 6, 10, 9, 8, 12, 11, 5, 2, 4, 1, 3, 0};
    d.Walk(
        [&](int key, const std::unique_ptr<MockPipelineEngine>& pipeline) {
          pipeline->Stop();
          v.emplace_back(key);
        },
        !start_from_head);
    assert(v == stop_order);
  }

  {
    std::vector<int> v;
    std::vector<int> heads_order{13, 6, 8, 11, 0};
    d.WalkHeads(
        [&](int key, const std::unique_ptr<MockPipelineEngine>& pipeline) {
          pipeline->Destroy();
          v.emplace_back(key);
        });
    assert(v == heads_order);
  }

  {
    std::vector<int> v;
    std::vector<int> tails_order{13, 7, 10, 5};
    d.WalkTails(
        [&](int key, const std::unique_ptr<MockPipelineEngine>& pipeline) {
          pipeline->Destroy();
          v.emplace_back(key);
        });
    assert(v == tails_order);
  }

  {
    std::vector<int> test_sequence{13, 6, 7, 0,  1,  3, 4,
                                   2,  5, 8, 11, 12, 9, 10};

    std::unordered_set<int> heads{0, 6, 8, 11, 13};
    assert(d.NextKeys() == heads);

    std::vector<std::unordered_set<int>> next_keys{
        {}, {7}, {}, {1, 3}, {}, {2, 4}, {}, {5}, {}, {}, {12}, {9}, {10}, {},
    };

    assert(test_sequence.size() == nodes_count);
    assert(next_keys.size() == nodes_count);
    for (int i = 0; i < nodes_count; ++i) {
      assert(d.NextKeys(test_sequence[i]) == next_keys[i]);
    }
  }

  d.Clear();
  assert(d.Size() == 0);
  for (int i = 0; i < nodes_count; ++i) {
    assert(!d.Exist(i));
  }
}

}  // namespace jc::test

int main() { jc::test::test(); }
```

+ 对于不同类型模板参数的类模板，会为每个类型实例化出不同的类，类的函数被调用时才实例化，类模板的 static 数据成员会分别在每个不同的类中实例化，static 数据成员和成员函数只被同一个类共享。通过 [C++ Insights](https://github.com/andreasfertig/cppinsights) 可以查看类的实例化代码，该工具提供了[在线版本](https://cppinsights.io/)

```cpp
namespace jc {

template <typename T>
class A {
 public:
  static int value();

 private:
  static int n;
};

template <typename T>
inline int A<T>::value() {
  return n;
}

template <typename T>
int A<T>::n = 0;

}  // namespace jc

int main() {
  using namespace jc;
  A<void> a;    // 实例化 A<void>::n
  A<int> b, c;  // 实例化 A<int>::n，bc 共享 A<int>::value() 和 A<int>::n
  int n = A<double>::value();  // 实例化 A<double>::value()
  n = b.value();               // 使用 A<int>::value()
  n = A<int>::value();  // 必须指定模板参数以确定实例化版本
}
```

+ 由于函数被调用时才实例化，如果不调用实例化时会出错的函数，代码也能通过编译

```cpp
namespace jc {

template <typename T>
class A {
 public:
  static int value();

 private:
  static int n;
};

template <typename T>
inline int A<T>::value() {
  return f(n);
}

template <typename T>
int A<T>::n = 0;

}  // namespace jc

int main() {
  using namespace jc;
  A<void> a;  // OK，实例化 A<void>::n
  //   a.value();         // 实例化错误，f 未声明
  //   A<void>::value();  // 实例化错误，f 未声明
}
```

## 友元

+ 类内定义友元可以省略模板参数，但友元函数在类模板实例化后才会实例化，如果类模板中的友元函数不包含模板参数，则会出现重定义的错误

```cpp
#include <iostream>
#include <typeinfo>

namespace jc {

template <typename T>
class A {
  // 类作用域内的 A 是注入类名，等价于 A<T>
  friend std::ostream& operator<<(std::ostream& os, const A& rhs) {
    return os << "A<" << typeid(T).name() << "> = " << rhs.n;
  }

  friend void f() {}

 private:
  int n = 0;
};

}  // namespace jc

int main() {
  jc::A<void> a;  // 实例化 operator<<(std::ostream&, const A<void>&) 和 f()
  std::cout << a;  // A<void> = 0
  // jc::A<int> b;    // 错误：第二次实例化 f()，重定义
}
```

+ 类外定义友元不会有重定义的问题，需要在类内声明处为类模板额外指定不同的模板参数

```cpp
#include <iostream>
#include <typeinfo>

namespace jc {

template <typename T>
class A {
  template <typename U>
  friend std::ostream& operator<<(std::ostream& os, const A<U>& rhs);

  friend void f();

 private:
  int n = 0;
};

template <typename T>
std::ostream& operator<<(std::ostream& os, const A<T>& rhs) {
  return os << "A<" << typeid(T).name() << "> = " << rhs.n;
}

void f() {}

}  // namespace jc

int main() {
  jc::A<void> a;
  std::cout << a;  // A<void> = 0
  jc::A<int> b;
  std::cout << b;  // A<int> = 0
}
```

+ 如果要在类外定义友元，又不想在类内声明额外指定模板参数，则可以将友元声明为函数模板，在类内使用模板实例作为友元

```cpp
#include <iostream>
#include <typeinfo>

namespace jc {

template <typename T>
class A;

template <typename T>
std::ostream& operator<<(std::ostream& os, const A<T>& rhs);

template <typename T>
class A {
  friend std::ostream& operator<<<T>(std::ostream& os, const A<T>& rhs);

 private:
  int n = 0;
};

template <typename T>
std::ostream& operator<<(std::ostream& os, const A<T>& rhs) {
  return os << "A<" << typeid(T).name() << "> = " << rhs.n;
}

}  // namespace jc

int main() {
  jc::A<void> a;
  std::cout << a;  // A<void> = 0
}
```

+ 如果将类模板实例声明为友元，则类模板必须已声明并可见

```cpp
namespace jc {

template <typename T>
struct Node;

template <typename T>
struct Tree {
  friend class Node<T>;  // 友元类模板必须已声明并可见
  friend class A;        // 友元类可以未声明
};

}  // namespace jc

int main() {}
```

+ 模板参数可以是友元

```cpp
namespace jc {

template <typename T>
class A {
  friend T;  // 如果 T 不是 class 则忽略

 private:
  int n = 0;
};

class B {
 public:
  static int f(const A<B>& a) { return a.n; }
};

}  // namespace jc

int main() {}
```

## [特化（Specialization）](https://en.cppreference.com/w/cpp/language/template_specialization)

+ 特化一般指的是全特化，即为一种特定类型指定一个特定实现，该类型将不使用模板的实例化版本

```cpp
#include <cassert>

namespace jc {

template <typename T>
class A {
 public:
  int f() { return 1; }
};

template <>
class A<int> {
 public:
  int f() { return 2; }
  int g() { return 3; }
};

}  // namespace jc

int main() {
  jc::A<void> a;
  assert(a.f() == 1);
  jc::A<int> b;
  assert(b.f() == 2);
  assert(b.g() == 3);
}
```

## [偏特化（Partial Specialization）](https://en.cppreference.com/w/cpp/language/partial_specialization)

+ 偏特化是为一类类型指定特定实现，是一种更通用的特化

```cpp
#include <cassert>

namespace jc {

template <typename T>
class A {
 public:
  int f() { return 1; }
};

template <typename T>
class A<T*> {
 public:
  int f() { return 2; }
  int g() { return 3; }
};

}  // namespace jc

int main() {
  jc::A<int> a;
  assert(a.f() == 1);
  jc::A<int*> b;
  assert(b.f() == 2);
  assert(b.g() == 3);
  jc::A<jc::A<int>*> c;
  assert(c.f() == 2);
  assert(c.g() == 3);
}
```

+ 偏特化可以指定多个模板参数之间的关系，如果多个偏特化匹配程度相同，将产生二义性错误。如果模板声明是一个普通声明（没有在模板名称后添加尖括号），这个声明就是一个主模板（primary template），编写偏特化通常会有一个主模板和其他偏特化模板

```cpp
namespace jc {

template <typename T, typename U>
struct A;  // primary template

template <typename T>
struct A<T, T> {
  static constexpr int i = 1;
};

template <typename T>
struct A<T, int> {
  static constexpr int j = 2;
};

template <typename T, typename U>
struct A<T*, U*> {
  static constexpr int k = 3;
};

}  // namespace jc

using namespace jc;

static_assert(A<double, double>::i == 1);
static_assert(A<double, int>::j == 2);
static_assert(A<int*, double*>::k == 3);

int main() {
  //   A<int, int>{};    // 错误，匹配 A<T, T> 和 A<T, int>
  //   A<int*, int*>{};  // 错误，匹配 A<T, T> 和 A<T*, U*>
}
```

+ 如果多个特化中，有一个匹配程度最高，则不会有二义性错误

```cpp
namespace jc {

template <typename T, typename U>
struct A;

template <typename T>
struct A<T, T> {
  static constexpr int i = 1;
};

template <typename T>
struct A<T, int> {
  static constexpr int j = 2;
};

template <typename T, typename U>
struct A<T*, U*> {
  static constexpr int k = 3;
};

template <typename T>
struct A<T*, T*> {
  static constexpr int k = 4;
};

}  // namespace jc

static_assert(jc::A<double, double>::i == 1);
static_assert(jc::A<double, int>::j == 2);
static_assert(jc::A<int*, double*>::k == 3);
static_assert(jc::A<double*, int*>::k == 3);
static_assert(jc::A<int*, int*>::k == 4);
static_assert(jc::A<double*, double*>::k == 4);

int main() {}
```

+ 偏特化常用于元编程

```cpp
#include <tuple>
#include <type_traits>

namespace jc {

template <typename T, typename Tuple>
struct is_among;

template <typename T, template <typename...> class Tuple, typename... List>
struct is_among<T, Tuple<List...>>
    : std::disjunction<std::is_same<T, List>...> {};

template <typename T, typename Tuple>
inline constexpr bool is_among_v = is_among<T, Tuple>::value;

}  // namespace jc

static_assert(jc::is_among_v<int, std::tuple<char, int, double>>);
static_assert(!jc::is_among_v<float, std::tuple<char, int, double>>);

int main() {}
```

+ 偏特化遍历 [std::tuple](https://en.cppreference.com/w/cpp/utility/tuple)

```cpp
#include <cstddef>
#include <iostream>
#include <tuple>

namespace jc {

template <std::size_t Index, std::size_t N, typename... List>
struct PrintImpl {
  static void impl(const std::tuple<List...>& t) {
    std::cout << std::get<Index>(t) << " ";
    PrintImpl<Index + 1, N, List...>::impl(t);
  }
};

template <std::size_t N, typename... List>
struct PrintImpl<N, N, List...> {
  static void impl(const std::tuple<List...>& t) {}
};

template <typename... List>
void Print(const std::tuple<List...>& t) {
  PrintImpl<0, sizeof...(List), List...>::impl(t);
}

}  // namespace jc

int main() {
  auto t = std::make_tuple(3.14, 42, "hello world");
  jc::Print(t);  // 3.14 42 hello world
}
```

+ [成员模板](https://en.cppreference.com/w/cpp/language/member_template)也能特化或偏特化

```cpp
#include <cassert>
#include <string>

namespace jc {

struct A {
  template <typename T = std::string>
  T as() const {
    return s;
  }

  std::string s;
};

template <>
inline bool A::as<bool>() const {
  return s == "true";
}

}  // namespace jc

int main() {
  jc::A a{"hello"};
  assert(a.as() == "hello");
  assert(!a.as<bool>());
  jc::A b{"true"};
  assert(b.as<bool>());
}
```

+ 成员函数模板不能为虚函数，因为虚函数表的大小是固定的，而成员函数模板的实例化个数要编译完成后才能确定

```cpp

namespace jc {

template <typename T>
class Dynamic {
 public:
  virtual ~Dynamic() {}  // OK，每个 Dynamic<T> 对应一个析构函数

  template <typename U>
  virtual void f(const U&) {}  // 错误，编译器不知道一个 Dynamic<T> 中 f() 个数
};

}  // namespace jc

int main() {}
```

## 模板的模板参数（Template Template Parameter）

+ 如果模板参数的类型是类模板，则需要使用模板的模板参数。对于模板的模板参数，C++11 之前只能用 class 关键字修饰，C++11 及其之后可以用别名模板的名称来替代，C++17 可以用 typename 修饰

```cpp
#include <set>
#include <vector>

namespace jc {

template <typename T, template <typename...> class Container>
void f(const Container<T>&) {}

}  // namespace jc

int main() {
  jc::f(std::vector<int>{});
  jc::f(std::vector<double>{});
  jc::f(std::set<int>{});
}
```

+ 实际上容器还有一个模板参数，即内存分配器 allocator

```cpp
#include <cassert>
#include <deque>
#include <string>

namespace jc {

template <typename T, template <typename Elem, typename = std::allocator<Elem>>
                      class Container = std::deque>
class Stack {
 public:
  using reference = T&;
  using const_reference = const T&;

  template <typename, template <typename, typename> class>
  friend class Stack;

  template <typename U,
            template <typename Elem2, typename = std::allocator<Elem2>>
            class Container2>
  Stack<T, Container>& operator=(const Stack<U, Container2>&);

  void push(const T&);

  void pop();

  reference top();

  const_reference top() const;

  std::size_t size() const;

  bool empty() const;

 private:
  Container<T> container_;
};

template <typename T, template <typename, typename> class Container>
template <typename U, template <typename, typename> class Container2>
inline Stack<T, Container>& Stack<T, Container>::operator=(
    const Stack<U, Container2>& rhs) {
  container_.assign(rhs.container_.begin(), rhs.container_.end());
  return *this;
}

template <typename T, template <typename, typename> class Container>
inline void Stack<T, Container>::push(const T& x) {
  container_.emplace_back(x);
}

template <typename T, template <typename, typename> class Container>
inline void Stack<T, Container>::pop() {
  assert(!empty());
  container_.pop_back();
}

template <typename T, template <typename, typename> class Container>
inline typename Stack<T, Container>::reference Stack<T, Container>::top() {
  assert(!empty());
  return container_.back();
}

template <typename T, template <typename, typename> class Container>
inline typename Stack<T, Container>::const_reference Stack<T, Container>::top()
    const {
  assert(!empty());
  return container_.back();
}

template <typename T, template <typename, typename> class Container>
inline std::size_t Stack<T, Container>::size() const {
  return container_.size();
}

template <typename T, template <typename, typename> class Container>
inline bool Stack<T, Container>::empty() const {
  return container_.empty();
}

}  // namespace jc

int main() {
  jc::Stack<std::string> s;
  s.push("hello");
  s.push("world");
  assert(s.size() == 2);
  assert(s.top() == "world");
  s.pop();
  assert(s.size() == 1);
  assert(s.top() == "hello");
  s.pop();
  assert(s.empty());
}
```

## [非类型模板参数（Non-type Template Parameter）](https://en.cppreference.com/w/cpp/language/template_parameters#Non-type_template_parameter)

+ 非类型模板参数表示在编译期或链接期可以确定的常量值

```cpp
#include <bitset>
#include <cassert>

namespace jc {

template <bool IsSet = true, std::size_t N>
std::size_t find_next(const std::bitset<N>& b, std::size_t cur) {
  for (std::size_t i = cur + 1; i < N; ++i) {
    if (!(b.test(i) ^ IsSet)) {
      return i;
    }
  }
  return N;
}

template <bool IsSet = true, std::size_t N>
std::size_t find_prev(const std::bitset<N>& b, std::size_t cur) {
  if (cur > N) {
    cur = N;
  }
  for (int i = static_cast<int>(cur) - 1; i >= 0; --i) {
    if (!(b.test(i) ^ IsSet)) {
      return i;
    }
  }
  return N;
}

template <bool IsSet = true, std::size_t N>
std::size_t circular_find_next(const std::bitset<N>& b, std::size_t cur) {
  if (cur > N) {
    cur = N;
  }
  std::size_t res = find_next<IsSet>(b, cur);
  if (res != N) {
    return res;
  }
  for (std::size_t i = 0; i < cur; ++i) {
    if (!(b.test(i) ^ IsSet)) {
      return i;
    }
  }
  return N;
}

template <bool IsSet = true, std::size_t N>
std::size_t circular_find_prev(const std::bitset<N>& b, std::size_t cur) {
  if constexpr (N == 0) {
    return N;
  }
  std::size_t res = find_prev<IsSet>(b, cur);
  if (res != N) {
    return res;
  }
  for (std::size_t i = N - 1; i > cur; --i) {
    if (!(b.test(i) ^ IsSet)) {
      return i;
    }
  }
  return N;
}

}  // namespace jc

void test_find_next() {
  std::bitset<8> b{"10010111"};
  static constexpr int _next_set[] = {1, 2, 4, 4, 7, 7, 7, 8, 8, 8};
  static constexpr int _next_unset[] = {3, 3, 3, 5, 5, 6, 8, 8, 8, 8};

  for (std::size_t i = 0; i < std::size(_next_set); ++i) {
    assert(jc::find_next<true>(b, i) == _next_set[i]);
    assert(jc::find_next<false>(b, i) == _next_unset[i]);
  }
}

void test_find_prev() {
  std::bitset<8> b{"10010110"};
  static constexpr int _prev_set[] = {8, 8, 1, 2, 2, 4, 4, 4, 7, 7};
  static constexpr int _prev_unset[] = {8, 0, 0, 0, 3, 3, 5, 6, 6, 6};

  for (std::size_t i = 0; i < std::size(_prev_set); ++i) {
    assert(jc::find_prev<true>(b, i) == _prev_set[i]);
    assert(jc::find_prev<false>(b, i) == _prev_unset[i]);
  }
}

void test_circular_find_next() {
  std::bitset<8> b{"01010111"};
  static constexpr int _next_set[] = {1, 2, 4, 4, 6, 6, 0, 0, 0, 0};
  static constexpr int _next_unset[] = {3, 3, 3, 5, 5, 7, 7, 3, 3, 3};

  for (std::size_t i = 0; i < std::size(_next_set); ++i) {
    assert(jc::circular_find_next<true>(b, i) == _next_set[i]);
    assert(jc::circular_find_next<false>(b, i) == _next_unset[i]);
  }
}

void test_circular_find_prev() {
  std::bitset<8> b{"10011001"};
  static constexpr int _prev_set[] = {7, 0, 0, 0, 3, 4, 4, 4, 7, 7};
  static constexpr int _prev_unset[] = {6, 6, 1, 2, 2, 2, 5, 6, 6, 6};

  for (std::size_t i = 0; i < std::size(_prev_set); ++i) {
    assert(jc::circular_find_prev<true>(b, i) == _prev_set[i]);
    assert(jc::circular_find_prev<false>(b, i) == _prev_unset[i]);
  }
}

int main() {
  test_find_next();
  test_find_prev();
  test_circular_find_next();
  test_circular_find_prev();
}
```

+ 模板参数可以由之前的参数推断类型，C++17 允许将非类型模板参数定义为 auto 或 decltype(auto)

```cpp
#include <cassert>

namespace jc {

template <typename>
struct get_class;

template <typename ClassType, typename MemberType>
struct get_class<MemberType ClassType::*> {
  using type = ClassType;
};

template <typename T>
using get_class_t = typename get_class<T>::type;

template <auto ClassMember>
class Wrapper {
 public:
  Wrapper(get_class_t<decltype(ClassMember)>& obj) : obj_(obj) {}

  void increase() { ++(obj_.*ClassMember); }

 private:
  get_class_t<decltype(ClassMember)>& obj_;
};

struct A {
  int i = 0;
};

}  // namespace jc

int main() {
  jc::A a;
  jc::Wrapper<&jc::A::i>{a}.increase();
  assert(a.i == 1);
}
```

+ C++14 允许 auto 作返回类型

```cpp
namespace jc {

template <typename T, typename U>
constexpr auto add(const T& a, const U& b) {
  return a + b;
}

}  // namespace jc

static_assert(jc::add('A', 2) == 'C');

int main() {}
```

## 限制

+ C++20 之前，非类型模板参数不能是浮点数

```cpp
namespace jc {

template <auto N>
struct A {
  static constexpr auto f() { return N; }
};

}  // namespace jc

static_assert(jc::A<42>::f() == 42);
static_assert(jc::A<3.14>::f() == 3.14);  // C++20

int main() {}
```

+ 不能用字符串字面值常量、临时对象、数据成员或其他子对象作模板实参。C++ 标准演进过程中逐渐放宽了对字符数组作为模板实参的限制，C++11 仅允许外链接（external linkage，不定义于单一的文件作用域，链接到全局符号表），C++14 允许外链接或内链接（internal linkage，只能在单个文件内部看到，不能被其他文件访问，不暴露给链接器），C++17 不要求链接

```cpp
namespace jc {

template <const char* s>
struct A {};

}  // namespace jc

constexpr const char* s1 = "hello";  // 内链接对象的指针
extern const char s2[] = "world";    // 外链接
const char s3[] = "down";            // 内链接

int main() {
  static const char s4[] = "demo";  // 无链接
  jc::A<"downdemo">{};              // 错误
  jc::A<s1>{};                      // 错误
  jc::A<s2>{};                      // C++11 允许
  jc::A<s3>{};                      // C++14 允许
  jc::A<s4>{};                      // C++17 允许
}
```

+ 非类型模板参数可以是左值引用，此时实参必须是静态常量

```cpp
#include <cassert>

namespace jc {

template <int& N>
struct A {
  A() { ++N; }
  ~A() { --N; }
};

void test() {
  static int n = 0;
  {
    A<n> a;
    assert(n == 1);
  }
  assert(n == 0);
}

}  // namespace jc

int main() { jc::test(); }
```

+ 函数和数组类型作为非类型模板参数会退化为指针类型

```cpp
namespace jc {

template <int buf[5]>
struct Lexer {};

// template <int* buf>
// struct Lexer {};  // 错误：重定义

template <int fun()>
struct FuncWrap {};

// template <int (*)()>
// struct FuncWrap {};  // 错误：重定义

}  // namespace jc

int main() {}
```

+ 如果模板实参的表达式有大于号，必须用小括号包裹表达式，否则大于号会被编译器视为表示参数列表终止的右尖括号，导致编译错误

```cpp
namespace jc {

template <bool b>
struct A {
  inline static constexpr bool value = b;
};

}  // namespace jc

int main() { static_assert(jc::A<(sizeof(int) > 0)>::value); }
```

## [变量模板（Variable Template）](https://en.cppreference.com/w/cpp/language/variable_template)

+ C++14 提供了变量模板

```cpp
namespace jc {

template <typename T = double>
constexpr T pi{static_cast<T>(3.1415926535897932385)};

static_assert(pi<bool> == true);
static_assert(pi<int> == 3);
static_assert(pi<double> == 3.1415926535897932385);
static_assert(pi<> == 3.1415926535897932385);

}  // namespace jc

int main() {}
```

+ 变量模板可以由非类型参数参数化

```cpp
#include <array>
#include <cassert>

namespace jc {

template <int N>
std::array<int, N> arr{};

template <auto N>
constexpr decltype(N) x = N;

}  // namespace jc

static_assert(jc::x<'c'> == 'c');

int main() {
  jc::arr<10>[0] = 42;
  assert(jc::arr<10>[0] == 42);
}
```

## 变参模板（Variadic Template）

+ 如果函数要接受任意数量任意类型的参数，没有模板时可以通过 [std::va_list](https://en.cppreference.com/w/cpp/utility/variadic/va_list) 实现

```cpp
#include <cassert>
#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <string>

namespace jc {

void test(int n, ...) {
  std::va_list args;
  va_start(args, n);
  assert(va_arg(args, double) == 3.14);
  assert(va_arg(args, int) == 42);
  assert(std::strcmp(va_arg(args, const char*), "hello") == 0);
  assert(std::strcmp(va_arg(args, const char*), "world") == 0);
  va_end(args);
}

void test(const char* fmt, ...) {
  char buf[256];
  std::va_list args;
  va_start(args, fmt);
  std::vsnprintf(buf, 256, fmt, args);
  va_end(args);
  assert(std::strcmp(buf, "3.14 42 hello world") == 0);
}

}  // namespace jc

int main() {
  jc::test(4, 3.14, 42, std::string{"hello"}.c_str(), "world");
  jc::test("%.2f %d %s %s", 3.14, 42, std::string{"hello"}.c_str(), "world");
}
```

+ C++11 引入了变参模板，用省略号表示一个[参数包](https://en.cppreference.com/w/cpp/language/parameter_pack)，类型名后接省略号表示任意数量给定类型的参数。在表达式后跟省略号，如果表达式中有参数包，就会把表达式应用到参数包中的每个参数。如果表达式中出现两次参数包，对整个表达式扩展，而不会做笛卡尔积计算

```cpp
#include <iostream>
#include <string>
#include <tuple>
#include <utility>

namespace jc {

void print() {}  // 参数包展开到零参数时调用

template <typename T, typename... Args>
void print(const T& t, Args&&... args) {
  std::cout << t << ",";
  print(std::forward<Args>(args)...);
}

template <int... Index>
struct A {};

template <typename... List, int... Index>
void test1(const std::tuple<List...>& t, A<Index...>) {
  print(std::get<Index>(t)...);  // print(std::get<2>(t), std::get<3>(t));
}

template <typename... List, int... Index>
void test2(const std::tuple<List...>& t, A<Index...>) {
  print((std::get<Index>(t) + std::get<Index>(t))...);
}

}  // namespace jc

int main() {
  auto t = std::make_tuple(3.14, 42, std::string{"hello"}, "world");
  jc::test1(t, jc::A<2, 3>{});     // hello,world
  jc::test2(t, jc::A<0, 1, 2>{});  // 6.28,84,hellohello,
}
```

+ 注意参数包的省略号不能直接接在数值字面值后

```cpp
template <typename... Args>
void f(const Args&... args) {
  print(args + 1...);    // ERROR：1... 是带多个小数点的字面值，不合法
  print(args + 1 ...);   // OK
  print((args + 1)...);  // OK
}
```

+ 可以直接用逗号运算符做参数包扩展，逗号左侧是对参数包每个元素做的操作，右侧是一个无关紧要的值，这样展开后对每个元素都做了操作，并形成了一个以无关值为元素的数组，这个数组无作用，只是为了满足扩展时省略号不能为表达式最后的 token 而引入

```cpp
#include <iostream>
#include <string>
#include <utility>

namespace jc {

template <typename... Args>
void print(Args&&... args) {
  auto a = {(std::cout << std::forward<Args>(args) << std::endl, 0)...};
}

}  // namespace jc

int main() { jc::print(3.14, 42, std::string{"hello"}, "world"); }
```

+ C++11 引入了 [sizeof...](https://en.cppreference.com/w/cpp/language/sizeof...) 在编译期计算参数包中的元素数，C++17 引入了 if constexpr 判断编译期值，编译期结果为 true 才会实例化代码

```cpp
#include <iostream>
#include <string>
#include <utility>

namespace jc {

template <typename T, typename... Args>
void print(const T& t, Args&&... args) {
  std::cout << t << std::endl;
  if constexpr (sizeof...(args) > 0) {  // 不能用 if，因为零长包也会实例化代码
    print(std::forward<Args>(args)...);  // 当条件满足时才实例化
  }
}

}  // namespace jc

int main() { jc::print(3.14, 42, std::string{"hello"}, "world"); }
```

+ 在 C++11 中可以利用偏特化来达到 if constexpr 的效果

```cpp
#include <iostream>
#include <string>
#include <utility>

namespace jc {

template <bool b>
struct A;

template <typename T, typename... Args>
void print(const T& t, Args&&... args) {
  std::cout << t << std::endl;
  A<(sizeof...(args) > 0)>::f(std::forward<Args>(args)...);
}

template <bool b>
struct A {
  template <typename... Args>
  static void f(Args&&... args) {
    print(std::forward<Args>(args)...);
  }
};

template <>
struct A<false> {
  static void f(...) {}
};

}  // namespace jc

int main() { jc::print(3.14, 42, std::string{"hello"}, "world"); }
```

## [折叠表达式（Fold Expression）](https://en.cppreference.com/w/cpp/language/fold)

+ C++17 引入了折叠表达式，用于获取对所有参数包实参使用二元运算符的计算结果

```cpp
#include <iostream>
#include <tuple>
#include <utility>

namespace jc {

template <typename... Args>
auto sum(Args&&... args) {
  auto a = (... + std::forward<Args>(args));      // (((1 + 2) + 3) + 4)
  auto b = (std::forward<Args>(args) + ...);      // (1 + (2 + (3 + 4)))
  auto c = (5 + ... + std::forward<Args>(args));  // ((((5 + 1) + 2) + 3) + 4)
  auto d = (std::forward<Args>(args) + ... + 5);  // (1 + (2 + (3 + (4 + 5))))
  return std::make_tuple(a, b, c, d);
}

auto print1 = [](auto&&... args) {
  // operator<< 左折叠，std::cout 是初始值
  (std::cout << ... << std::forward<decltype(args)>(args));
};

auto print2 = [](auto&&... args) {
  // operator, 左折叠
  ((std::cout << std::forward<decltype(args)>(args) << ","), ...);
};

}  // namespace jc

int main() {
  auto [a, b, c, d] = jc::sum(1, 2, 3, 4);
  jc::print1(a, b, c, d);  // 10101515
  jc::print2(a, b, c, d);  // 10,10,15,15,
}
```

+ 对于空扩展需要决定类型和值，空的一元折叠表达式通常会产生错误，除了三种例外情况
  + 一个 && 的一元折叠的空扩展产生值 true
  + 一个 || 的一元折叠的空扩展产生值 false
  + 一个 , 的一元折叠空扩展产生一个 void 表达式

|      折叠表达式       |                  计算结果                  |
| :-------------------: | :----------------------------------------: |
|     (... op pack)     | (((pack1 op pack2) op pack3) ... op PackN) |
|     (pack op ...)     |    (pack1 op (... (packN-1 op packN)))     |
| (init op ... op pack) | (((init op pack1) op pack2) ... op PackN)  |
| (pack op ... op init) |      (pack1 op (... (packN op init)))      |

+ 折叠表达式借鉴的是 Haskell 的 fold

```hs
import Data.List (foldl')

foldlList :: [Char]
foldlList = foldl' (\x y -> concat ["(", x, "+", y, ")"]) "0" (map show [1 .. 4])

foldrList :: [Char]
foldrList = foldr ((\x y -> concat ["(", x, "+", y, ")"]) . show) "0" [1 .. 4]

main :: IO ()
main = do
  putStrLn foldlList -- ((((0+1)+2)+3)+4)
  putStrLn foldrList -- (1+(2+(3+(4+0))))
```

+ 实现与 Haskell 类似的左折叠和右折叠

```cpp
#include <iostream>
#include <string>
#include <type_traits>

namespace jc {

template <typename F, typename T, typename... Args>
void foldlList(F&& f, T&& zero, Args&&... x) {
  ((std::invoke(std::forward<F>(f), (std::string(sizeof...(Args), '('))),
    std::invoke(std::forward<F>(f), (std::forward<T>(zero)))),
   ...,
   (std::invoke(std::forward<F>(f), ('+')),
    std::invoke(std::forward<F>(f), (std::forward<Args>(x))),
    std::invoke(std::forward<F>(f), (')'))));
}

template <typename F, typename T, typename... Args>
void foldrList(F&& f, T&& zero, Args&&... x) {
  ((std::invoke(std::forward<F>(f), ('(')),
    std::invoke(std::forward<F>(f), (std::forward<Args>(x))),
    std::invoke(std::forward<F>(f), ('+'))),
   ...,
   (std::invoke(std::forward<F>(f), (std::forward<T>(zero))),
    std::invoke(std::forward<F>(f), (std::string(sizeof...(Args), ')')))));
}

}  // namespace jc

int main() {
  auto print = [](const auto& x) { std::cout << x; };
  jc::foldlList(print, 0, 1, 2, 3, 4);  // ((((0+1)+2)+3)+4)
  jc::foldrList(print, 0, 1, 2, 3, 4);  // (1+(2+(3+(4+0))))
}
```

+ 折叠表达式几乎可以使用所有二元运算符

```cpp
#include <cassert>

namespace jc {

struct Node {
  Node(int i) : val(i) {}

  int val = 0;
  Node* left = nullptr;
  Node* right = nullptr;
};

// 使用 operator->* 的折叠表达式，用于遍历指定的二叉树路径
template <typename T, typename... Args>
Node* traverse(T root, Args... paths) {
  return (root->*...->*paths);  // root ->* paths1 ->* paths2 ...
}

void test() {
  Node* root = new Node{0};
  root->left = new Node{1};
  root->left->right = new Node{2};
  root->left->right->left = new Node{3};

  auto left = &Node::left;
  auto right = &Node::right;
  Node* node1 = traverse(root, left);
  assert(node1->val == 1);
  Node* node2 = traverse(root, left, right);
  assert(node2->val == 2);
  Node* node3 = traverse(node2, left);
  assert(node3->val == 3);
}

}  // namespace jc

int main() { jc::test(); }
```

+ 包扩展可以用于编译期表达式

```cpp
#include <type_traits>

namespace jc {

template <typename T, typename... Args>
constexpr bool is_homogeneous(T, Args...) {
  return (std::is_same_v<T, Args> && ...);  // operator&& 的折叠表达式
}

}  // namespace jc

static_assert(!jc::is_homogeneous(3.14, 42, "hello", "world"));
static_assert(jc::is_homogeneous("hello", "", "world"));

int main() {}
```

## 变参模板的应用

+ 无需指定类型，自动获取 [std::variant](https://en.cppreference.com/w/cpp/utility/variant) 值

```cpp
#include <array>
#include <cassert>
#include <functional>
#include <string>
#include <type_traits>
#include <variant>

namespace jc {

template <typename F, std::size_t... N>
constexpr auto make_array_impl(F f, std::index_sequence<N...>)
    -> std::array<std::invoke_result_t<F, std::size_t>, sizeof...(N)> {
  return {std::invoke(f, std::integral_constant<decltype(N), N>{})...};
}

template <std::size_t N, typename F>
constexpr auto make_array(F f)
    -> std::array<std::invoke_result_t<F, std::size_t>, N> {
  return make_array_impl(f, std::make_index_sequence<N>{});
}

template <typename T, typename Dst, typename... List>
bool get_value_impl(const std::variant<List...>& v, Dst& dst) {
  if (std::holds_alternative<T>(v)) {
    if constexpr (std::is_convertible_v<T, Dst>) {
      dst = static_cast<Dst>(std::get<T>(v));
      return true;
    }
  }
  return false;
}

template <typename Dst, typename... List>
bool get_value(const std::variant<List...>& v, Dst& dst) {
  using Variant = std::variant<List...>;
  using F = std::function<bool(const Variant&, Dst&)>;
  static auto _list = make_array<sizeof...(List)>([](auto i) -> F {
    return &get_value_impl<std::variant_alternative_t<i, Variant>, Dst,
                           List...>;
  });
  return std::invoke(_list[v.index()], v, dst);
}

}  // namespace jc

int main() {
  std::variant<int, std::string> v = std::string{"test"};
  std::string s;
  assert(jc::get_value(v, s));
  assert(s == "test");
  v = 42;
  int i;
  assert(jc::get_value(v, i));
  assert(i == 42);
}
```

+ 字节序转换

```cpp
// https://en.cppreference.com/w/cpp/language/fold

#include <cstdint>
#include <type_traits>
#include <utility>

namespace jc {

template <typename T, size_t... N>
constexpr T bswap_impl(T t, std::index_sequence<N...>) {
  return (((t >> N * 8 & 0xFF) << (sizeof(T) - 1 - N) * 8) | ...);
}

template <typename T, typename U = std::make_unsigned_t<T>>
constexpr U bswap(T t) {
  return bswap_impl<U>(t, std::make_index_sequence<sizeof(T)>{});
}

}  // namespace jc

static_assert(jc::bswap<std::uint32_t>(0x12345678u) == 0x78563412u);
static_assert((0x12345678u >> 0) == 0x12345678u);
static_assert((0x12345678u >> 8) == 0x00123456u);
static_assert((0x12345678u >> 16) == 0x00001234u);
static_assert((0x12345678u >> 24) == 0x00000012u);
static_assert(jc::bswap<std::uint16_t>(0x1234u) == 0x3412u);

int main() {}
```

+ [自定义字面值（User-defined literals）](https://en.cppreference.com/w/cpp/language/user_literal)

```cpp
#include <algorithm>
#include <array>
#include <cassert>
#include <functional>
#include <iostream>
#include <sstream>
#include <string>

namespace jc {

template <char... args>
std::string operator"" _dbg() {
  std::array<char, sizeof...(args)> v{args...};
  std::stringstream os;
  for (const auto& x : v) {
    os << x;
  };
#ifndef NDEBUG
  std::cout << os.str() << std::endl;
#endif
  return os.str();
}

std::string operator"" _encrypt(const char* c, size_t) {
  std::string s{c};
  std::string p{R"(passwd: ")"};
  auto it = std::search(s.begin(), s.end(),
                        std::boyer_moore_horspool_searcher{p.begin(), p.end()});
  if (it != s.end()) {
    it += p.size();
    while (it != s.end() && *it != '\"') {
      *it++ = '*';
    }
  }
#if !defined(NDEBUG)
  std::cout << s << std::endl;
#endif
  return s;
}

}  // namespace jc

int main() {
  using namespace jc;

  assert(12.34_dbg == "12.34");

  std::string s = R"JSON({
  data_dir: "C:\Users\downdemo\.data\*.txt",
  user: "downdemo(accelerate rapidly)",
  passwd: "123456"
})JSON"_encrypt;

  std::string s2 = R"JSON({
  data_dir: "C:\Users\downdemo\.data\*.txt",
  user: "downdemo(accelerate rapidly)",
  passwd: "******"
})JSON";

  assert(s == s2);
}
```

+ 变参基类

```cpp
#include <string>
#include <unordered_set>

namespace jc {

struct A {
  std::string s;
};

struct A_EQ {
  bool operator()(const A& lhs, const A& rhs) const { return lhs.s == rhs.s; }
};

struct A_Hash {
  std::size_t operator()(const A& a) const {
    return std::hash<std::string>{}(a.s);
  }
};

// 定义一个组合所有基类的 operator() 的派生类
template <typename... Bases>
struct Overloader : Bases... {
  using Bases::operator()...;  // C++17
};

using A_OP = Overloader<A_Hash, A_EQ>;

}  // namespace jc

int main() {
  // 将 A_EQ 和 A_Hash 组合到一个类型中

  /* unordered_set 的声明
  template<
  class Key,
      class Hash = std::hash<Key>,
      class KeyEqual = std::equal_to<Key>,
      class Allocator = std::allocator<Key>
  > class unordered_set;
  */

  std::unordered_set<jc::A, jc::A_Hash, jc::A_EQ> s1;
  std::unordered_set<jc::A, jc::A_OP, jc::A_OP> s2;
}
```

+ C++14 使用 [std::integer_sequence](https://en.cppreference.com/w/cpp/utility/integer_sequence) 遍历 [std::tuple](https://en.cppreference.com/w/cpp/utility/tuple)

```cpp
#include <cstddef>
#include <functional>
#include <iostream>
#include <tuple>
#include <type_traits>
#include <utility>

namespace jc {

template <typename F, typename... List, std::size_t... Index>
void apply_impl(F&& f, const std::tuple<List...>& t,
                std::index_sequence<Index...>) {
  std::invoke(std::forward<F>(f), std::get<Index>(t)...);
}

template <typename F, typename... List>
void apply(F&& f, const std::tuple<List...>& t) {
  apply_impl(std::forward<F>(f), t, std::index_sequence_for<List...>{});
}

}  // namespace jc

struct Print {
  template <typename... Args>
  void operator()(const Args&... args) {
    auto no_used = {(std::cout << args << " ", 0)...};
  }
};

int main() {
  auto t = std::make_tuple(3.14, 42, "hello world");
  jc::apply(Print{}, t);  // 3.14 42 hello world
}
```

+ C++11 未提供 [std::integer_sequence](https://en.cppreference.com/w/cpp/utility/integer_sequence)，手动实现一个即可

```cpp
#include <cstddef>
#include <functional>
#include <iostream>
#include <tuple>
#include <type_traits>
#include <utility>

namespace jc {

template <std::size_t... Index>
struct index_sequence {
  using type = index_sequence<Index...>;
};

template <typename List1, typename List2>
struct concat;

template <std::size_t... List1, std::size_t... List2>
struct concat<index_sequence<List1...>, index_sequence<List2...>>
    : index_sequence<List1..., (sizeof...(List1) + List2)...> {};

template <typename List1, typename List2>
using concat_t = typename concat<List1, List2>::type;

template <std::size_t N>
struct make_index_sequence_impl
    : concat_t<typename make_index_sequence_impl<N / 2>::type,
               typename make_index_sequence_impl<N - N / 2>::type> {};

template <>
struct make_index_sequence_impl<0> : index_sequence<> {};

template <>
struct make_index_sequence_impl<1> : index_sequence<0> {};

template <std::size_t N>
using make_index_sequence = typename make_index_sequence_impl<N>::type;

template <typename... Types>
using index_sequence_for = make_index_sequence<sizeof...(Types)>;

static_assert(std::is_same_v<make_index_sequence<3>, index_sequence<0, 1, 2>>);

template <typename F, typename... List, std::size_t... Index>
void apply_impl(F&& f, const std::tuple<List...>& t, index_sequence<Index...>) {
  std::invoke(std::forward<F>(f), std::get<Index>(t)...);
}

template <typename F, typename... List>
void apply(F&& f, const std::tuple<List...>& t) {
  apply_impl(std::forward<F>(f), t, index_sequence_for<List...>{});
}

}  // namespace jc

struct Print {
  template <typename... Args>
  void operator()(const Args&... args) {
    auto no_used = {(std::cout << args << " ", 0)...};
  }
};

int main() {
  auto t = std::make_tuple(3.14, 42, "hello world");
  jc::apply(Print{}, t);  // 3.14 42 hello world
}
```

## 移动语义（Move Semantics）

+ C++11 的[值类别](https://en.cppreference.com/w/cpp/language/value_category)包括左值（lvalue）、纯右值（prvalue）、亡值（xvalue），左值和亡值组成了泛左值（glvalue），纯右值和亡值组成了右值（rvalue）。为了让编译器识别接受右值作为参数的构造函数，则需要引入右值引用符号（&&），以区分移动构造函数和拷贝构造函数

```cpp
#include <cassert>
#include <string>
#include <utility>
#include <vector>

namespace jc {

struct A {
  A() : data(new std::string) {}
  A(const A& rhs) : data(new std::string{*rhs.data}) {}
  A(A&& rhs) noexcept : data(rhs.data) { rhs.data = nullptr; }
  ~A() { delete data; }

  std::string* data = nullptr;
};

}  // namespace jc

int main() {
  std::vector<jc::A> v;
  v.emplace_back(jc::A{});  // 调用默认构造函数、移动构造函数、析构函数
  jc::A a;
  v.emplace_back(a);  // 调用拷贝构造函数
  assert(a.data);
  v.emplace_back(std::move(a));  // 调用移动构造函数
  assert(!a.data);
}
```

+ 右值引用即只能绑定到右值的引用，字面值（纯右值）和临时变量（亡值）就是常见的右值。如果把左值传递给右值引动参数，则需要强制类型转换，[std::move](https://en.cppreference.com/w/cpp/utility/move) 就是不需要显式指定类型的到右值引用的强制类型转换

```cpp
#include <cassert>
#include <string>
#include <type_traits>
#include <utility>

namespace jc {

template <typename T>
constexpr std::remove_reference_t<T>&& move(T&& x) noexcept {
  return static_cast<std::remove_reference_t<T>&&>(x);
}

constexpr int f(const std::string&) { return 1; }
constexpr int f(std::string&&) { return 2; }

}  // namespace jc

int main() {
  std::string s;
  static_assert(jc::f(s) == 1);
  assert(jc::f(std::string{}) == 2);
  static_assert(jc::f(static_cast<std::string&&>(s)) == 2);
  static_assert(jc::f(jc::move(s)) == 2);
  static_assert(jc::f(std::move(s)) == 2);
}
```

## 完美转发（Perfect Forwarding）

+ 右值引用是能接受右值的引用，引用可以取址，是左值，因此右值引用是左值。如果一个函数接受右值引用参数，把参数传递给其他函数时，会按左值传递，这样就丢失了原有的值类别

```cpp
#include <cassert>
#include <string>
#include <utility>

namespace jc {

constexpr int f(const std::string&) { return 1; }
constexpr int f(std::string&&) { return 2; }
constexpr int g(std::string&& s) { return f(s); }

void test() {
  std::string s;
  assert(f(std::string{}) == 2);
  assert(g(std::string{}) == 1);
  static_assert(f(std::move(s)) == 2);
  static_assert(g(std::move(s)) == 1);
}

}  // namespace jc

int main() { jc::test(); }
```

+ 为了转发时保持值类别不丢失，需要手写多个重载版本

```cpp
#include <cassert>
#include <string>
#include <utility>

namespace jc {

constexpr int f(std::string&) { return 1; }
constexpr int f(const std::string&) { return 2; }
constexpr int f(std::string&&) { return 3; }
constexpr int g(std::string& s) { return f(s); }
constexpr int g(const std::string& s) { return f(s); }
constexpr int g(std::string&& s) { return f(std::move(s)); }

void test() {
  std::string s;
  const std::string& s2 = s;
  static_assert(g(s) == 1);
  assert(g(s2) == 2);
  static_assert(g(std::move(s)) == 3);
  assert(g(std::string{}) == 3);
}

}  // namespace jc

int main() { jc::test(); }
```

+ 模板参数中右值引用符号表示的是万能引用（universal reference），因为模板参数本身可以推断为引用，它可以匹配几乎任何类型（少部分特殊类型无法匹配，如位域），传入左值时推断为左值引用类型，传入右值时推断为右值引用类型。对万能引用参数使用 [std::forward](https://en.cppreference.com/w/cpp/utility/forward) 则可以保持值类别不丢失，这种保留值类别的转发手法就叫完美转发，因此万能引用也叫转发引用（forwarding reference）

```cpp
#include <cassert>
#include <string>
#include <type_traits>

namespace jc {

template <typename T>
constexpr T&& forward(std::remove_reference_t<T>& t) noexcept {
  return static_cast<T&&>(t);
}

constexpr int f(std::string&) { return 1; }
constexpr int f(const std::string&) { return 2; }
constexpr int f(std::string&&) { return 3; }

template <typename T>
constexpr int g(T&& s) {
  return f(jc::forward<T>(s));  // 等价于 std::forward
}

void test() {
  std::string s;
  const std::string& s2 = s;
  static_assert(g(s) == 1);             // T = T&& = std::string&
  assert(g(s2) == 2);                   // T = T&& = const std::string&
  static_assert(g(std::move(s)) == 3);  // T = std::string, T&& = std::string&&
  assert(g(std::string{}) == 3);        // T = T&& = std::string&
  assert(g("downdemo") == 3);           // T = T&& = const char (&)[9]
}

}  // namespace jc

int main() { jc::test(); }
```

+ 结合变参模板完美转发转发任意数量的实参

```cpp
#include <iostream>
#include <string>
#include <type_traits>
#include <utility>

namespace jc {

template <typename F, typename... Args>
constexpr void constexpr_for(F&& f, Args&&... args) {
  (std::invoke(std::forward<F>(f), std::forward<Args>(args)), ...);
}

template <typename... Args>
void print(Args&&... args) {
  constexpr_for([](const auto& x) { std::cout << x << std::endl; },
                std::forward<Args>(args)...);
}

}  // namespace jc

int main() { jc::print(3.14, 42, std::string{"hello"}, "world"); }
```

+ [Lambda](https://en.cppreference.com/w/cpp/language/lambda) 中使用完美转发需要借助 decltype 推断类型

```cpp
#include <iostream>
#include <string>
#include <type_traits>
#include <utility>

namespace jc {

constexpr auto constexpr_for = [](auto&& f, auto&&... args) {
  (std::invoke(std::forward<decltype(f)>(f),
               std::forward<decltype(args)>(args)),
   ...);
};

auto print = [](auto&&... args) {
  constexpr_for([](const auto& x) { std::cout << x << std::endl; },
                std::forward<decltype(args)>(args)...);
};

}  // namespace jc

int main() { jc::print(3.14, 42, std::string{"hello"}, "world"); }
```

+ C++20 可以为 lambda 指定模板参数

```cpp
#include <iostream>
#include <string>
#include <type_traits>
#include <utility>

namespace jc {

constexpr auto constexpr_for =
    []<typename F, typename... Args>(F&& f, Args&&... args) {
  (std::invoke(std::forward<F>(f), std::forward<Args>(args)), ...);
};

auto print = []<typename... Args>(Args&& ... args) {
  constexpr_for([](const auto& x) { std::cout << x << std::endl; },
                std::forward<Args>(args)...);
};

}  // namespace jc

int main() { jc::print(3.14, 42, std::string{"hello"}, "world"); }
```

+ C++20 的 lambda 可以捕获参数包

```cpp
#include <iostream>
#include <string>
#include <type_traits>
#include <utility>

namespace jc {

template <typename... Args>
void print(Args&&... args) {
  [... args = std::forward<Args>(args)]<typename F>(F&& f) {
    (std::invoke(std::forward<F>(f), args), ...);
  }([](const auto& x) { std::cout << x << std::endl; });
}

}  // namespace jc

int main() { jc::print(3.14, 42, std::string{"hello"}, "world"); }
```

## 构造函数模板

+ 模板也能用于构造函数，但它不是真正的构造函数，从函数模板实例化而来的函数不和普通函数等价，由成员函数模板实例化的函数不会重写虚函数，由构造函数模板实例化的构造函数不是拷贝或移动构造函数，但对一个 non-const 对象调用构造函数时，万能引用是更优先的匹配

```cpp
#include <string>
#include <utility>

namespace jc {

struct A {
  template <typename T>
  explicit A(T&& t) : s(std::forward<T>(t)) {}

  A(const A& rhs) : s(rhs.s) {}
  A(A&& rhs) noexcept : s(std::move(rhs.s)) {}

  std::string s;
};

}  // namespace jc

int main() {
  const jc::A a{"downdemo"};
  jc::A b{a};  // OK，匹配拷贝构造函数
  //   jc::A c{b};  // 错误，匹配模板构造函数
}
```

+ 为此可以用 [std::enable_if](https://en.cppreference.com/w/cpp/types/enable_if) 约束模板参数，在条件满足的情况下才会匹配模板

```cpp
#include <string>
#include <type_traits>
#include <utility>

namespace jc {

struct A {
  template <typename T,  // 要求 T 能转为 std::string
            typename = std::enable_if_t<std::is_convertible_v<T, std::string>>>
  explicit A(T&& t) : s(std::forward<T>(t)) {}

  A(const A& rhs) : s(rhs.s) {}
  A(A&& rhs) noexcept : s(std::move(rhs.s)) {}

  std::string s;
};

}  // namespace jc

int main() {
  const jc::A a{"downdemo"};
  jc::A b{a};  // OK，匹配拷贝构造函数
  jc::A c{b};  // OK，匹配拷贝构造函数
}
```

+ C++20 可以用 [concepts](https://en.cppreference.com/w/cpp/concepts) 约束模板参数

```cpp
#include <concepts>
#include <string>
#include <utility>

namespace jc {

struct A {
  template <typename T>
    requires std::convertible_to<T, std::string>
  explicit A(T&& t) : s(std::forward<T>(t)) {}

  A(const A& rhs) : s(rhs.s) {}
  A(A&& rhs) noexcept : s(std::move(rhs.s)) {}

  std::string s;
};

}  // namespace jc

int main() {
  const jc::A a{"downdemo"};
  jc::A b{a};  // OK，匹配拷贝构造函数
  jc::A c{b};  // OK，匹配拷贝构造函数
}
```

## [ADL（Argument-Dependent Lookup，Koenig Lookup）](https://en.cppreference.com/w/cpp/language/adl)

+ [Name lookup](https://en.cppreference.com/w/cpp/language/lookup) 是当程序中出现一个名称时，将其与引入它的声明联系起来的过程，它分为 [qualified name lookup](https://en.cppreference.com/w/cpp/language/qualified_lookup) 和 [unqualified name lookup](https://en.cppreference.com/w/cpp/language/lookup)，[unqualified name lookup](https://en.cppreference.com/w/cpp/language/lookup) 对于函数名查找会使用 [ADL](https://en.cppreference.com/w/cpp/language/adl)

```cpp
namespace jc {

struct A {};
struct B {};
void f1(int) {}
void f2(A) {}

}  // namespace jc

namespace jd {

void f1(int i) {
  f1(i);  // 调用 jd::f1()，造成无限递归
}

void f2(jc::A t) {
  f2(t);  // 通过 t 的类型 jc::A 看到 jc，通过 jc 看到 jc::f2()
          // 因为 jd::f2() 也可见，此处产生二义性调用错误
}

void f3(jc::B t) {
  f3(t);  // 通过 t 的类型 jc::B 看到 jc，但 jc 中无 jc::f3()
          // 此处调用 jd::f3()，造成无限递归
}

}  // namespace jd

int main() {}
```

+ [Qualified name lookup](https://en.cppreference.com/w/cpp/language/qualified_lookup) 即对使用了作用域运算符的名称做查找，查找在受限的作用域内进行

```cpp
namespace jc {

int x;

struct Base {
  int i;
};

struct Derived : Base {};

void f(Derived* p) {
  p->i = 0;        // 找到 Base::i
  Derived::x = 0;  // 错误：在受限作用域中找不到 ::x
}

}  // namespace jc

int main() {}
```

+ [Unqualified name lookup](https://en.cppreference.com/w/cpp/language/lookup) 即对不指定作用域的名称做查找，先查找当前作用域，若找不到再继续查找外围作用域

```cpp
namespace jc {

extern int x;  // 1

int f(int x) {  // 2
  if (x < 0) {
    int x = 1;  // 3
    f(x);       // 使用 3
  }
  return x + ::x;  // 分别使用 2、1
}

}  // namespace jc

int main() {}
```

+ [ADL](https://en.cppreference.com/w/cpp/language/adl) 即实参依赖查找，对于一个类，其成员函数与使用了它的非成员函数，都是该类的逻辑组成部分，如果函数接受一个类作为参数，编译器查找函数名时，不仅会查找局部作用域，还会查找类所在的命名空间

```cpp
#include <iostream>
#include <string>

namespace jc {

struct A {};

void f(const A&) {}  // f() 是 A 的逻辑组成部分

}  // namespace jc

jc::A a;

int main() {
  f(a);  // 通过 ADL 找到 jc::f()，如果没有 ADL，就要写成 jc::f(a)
  std::string s;
  std::cout << s;  // std::operator<<() 是 std::string 的逻辑组成部分
  // 如果没有 ADL，就要写成 std::operator<<(std::cout, s)
}
```

+ ADL 是通过实参查找，对于函数模板，查找前无法先得知其为函数，也就无法确定实参，因此不会使用 ADL

```cpp
namespace jc {

class A {};

template <typename>
void f(A*) {}

}  // namespace jc

void g(jc::A* p) {
  f<int>(p);  // 错误，不知道 f<int> 是函数，所以不知道 p 是实参，不会用 ADL
}

int main() {}
```

+ ADL 会忽略 using 声明

```cpp
namespace jc {

template <typename T>
constexpr int f(T) {
  return 1;
}

}  // namespace jc

namespace jd {

using namespace jc;  // 忽略 using 声明，不会调用 jc::f

enum Color { red };
constexpr int f(Color) { return 2; }

}  // namespace jd

constexpr int f(int) { return 3; }

static_assert(::f(jd::red) == 3);  // 受限的函数名称，不使用 ADL
static_assert(f(jd::red) == 2);    // 使用 ADL 找到 jd::f()

int main() {}
```

+ ADL 会查找实参关联的命名空间和类，关联的命名空间和类组成的集合定义如下
  + 内置类型：集合为空
  + 指针和数组类型：所引用类型关联的命名空间和类
  + 枚举类型：关联枚举声明所在的命名空间
  + 类成员：关联成员所在的类
  + 类类型：关联的类包括该类本身、外围类型、直接和间接基类，关联的命名空间为每个关联类所在的命名空间，如果类是一个类模板实例则还包含模板实参本身类型、模板的模板实参所在的类和命名空间
  + 函数类型：所有参数和返回类型关联的命名空间和类
  + 类成员指针类型：成员和类关联的命名空间和类
+ 友元声明在外围作用域不可见，因为如果可见的话，实例化类模板会使普通函数的声明可见，如果没有先实例化类就调用函数，将导致编译错误，但如果友元函数所在类属于 ADL 的关联类集合，则在外围作用域可以找到该友元声明，且调用时，未实例化的类会被实例化

```cpp
namespace jc {

template <typename T>
class A {
  friend void f() {}
  friend void f(A<T>) {}
};

void g(const A<int>& a) {
  f();   // f() 无参数，不能使用 ADL，不可见
  f(a);  // f(A<int>) 关联类 A<int> 所以可见，若类 A<int> 未实例化则调用时实例化
}

}  // namespace jc

int main() {}
```

## [注入类名（Injected Class Name）](https://en.cppreference.com/w/cpp/language/injected-class-name)

+ 为了便于查找，在类作用域中，类名称是自身类型的 public 别名，该名称称为注入类名

```cpp
namespace jc {

int A;

struct A {
  void f() {
    A* p;    // OK：A 是注入类名
    ::A* q;  // 错误：查找到变量名 A，隐藏了 struct A 的名称
  }
};

}  // namespace jc

int main() {}
```

+ 类模板的注入类名可以被用作模板名或类型名

```cpp
namespace jc {

template <template <typename> class>
struct A {};

template <typename T>
struct B {
  B* a;            // B 被当作类型名，等价于 B<T>
  B<void>* b;      // B 被当作模板名
  using c = A<B>;  // B 被当作模板名
  A<jc::B> d;      // jc::B 不是注入类名，总会被当作模板名
};

}  // namespace jc

int main() {}
```

## 非模板中的上下文相关性

+ 解析理论主要面向上下文无关语言，而 C++ 是上下文相关语言，为了解决这个问题，编译器使用一张符号表结合扫描器和解析器
+ 解析某个声明时会把它添加到表中，扫描器找到一个标识符时，会在符号表中查找，如果发现该符号是一个类型就会注释这个标记，如编译器看见 `x*`，扫描器会查找 x，如果发现 x 是一个类型，解析器会看到标记如下，认为表达式是一个声明

```
identifier, type, x
symbol, *
```

+ 如果 x 不是类型，则解析器从扫描器获得标记如下，表达式被视为一个乘积

```cpp
identifier, nontype, x
symbol, *
```

+ 对于 `A<1>(0)`，如果 A 是类模板，则表达式是把 0 转换成 `A<1>` 类型。如果不是类模板，表达式等价于 `(A<1)>0`，计算表达式 A 小于 1 的结果，再将结果与 0 比较大小。因此解析器先查找 `<` 前的名称，如果名称是模板才会把 `<` 看作左尖括号，其他情况则看作小于号

```cpp
namespace jc {

template <bool B>
struct A {
  static const bool value = B;
};

static_assert(A<(1 > 0)>::value);  // 必须使用小括号

}  // namespace jc

int main() {}
```

## [Dependent name](https://en.cppreference.com/w/cpp/language/dependent_name)

### 当前实例化（current instantiation）和未知特化（unknown specialization）

+ Name lookup 对 dependent name 与 non-dependent name 有不同的查找规则，在模板定义中，依赖于模板参数的名称称为 dependent name，dependent name 包含当前实例化和未知特化。类模板的注入类名属于当前实例化，依赖于模板参数但不是当前实例化的为未知特化（unknown specialization）

```cpp
namespace jc {

template <typename T>
struct A {
  using type = T;

  A* a;        // A 是当前实例化
  A<type>* b;  // A<type> 是当前实例化
  A<T*>* c;    // A<T*> 是未知特化

  struct B {
    A* a;        // A 是当前实例化
    A<type>* b;  // A<type> 是当前实例化
    B* c;        // B 是当前实例化
  };

  struct C {
    A* a;        // A 是当前实例化
    A<type>* b;  // A<type> 是当前实例化
    B* c;        // 不在 B 的作用域内，B 是未知特化
    C* d;        // C 是当前实例化
  };
};

template <>
struct A<int>::B {
  int i;
};

}  // namespace jc

int main() {
  jc::A<double>::C{}.c->a;
  jc::A<int>::C{}.c->i;  // 使用特化的 A<int>::B
}
```

### typename 消歧义符

+ 模板名称的问题主要是不能有效确定名称，模板中不能引用其他模板的名称，因为其他模板可能有使原名称失效的特化

```cpp
namespace jc {

template <typename T>
struct A {
  static constexpr int x = 0;  // x 是值
};

template <typename T>
struct B {
  int y;

  void f() {
    A<T>::x* y;  // 默认被看作乘法表达式
  }
};

template <>
struct A<int> {
  using x = int;  // x 是类型
};

}  // namespace jc

int main() {
  jc::B<int>{}.f();   // A<int>::x* y 是声明，int* y
  jc::B<void>{}.f();  // A<void>::x* y 是乘法，0 * y
}
```

+ Dependent name 默认不会被看作类型，如果要表明是类型则需要加上 typename 消歧义符

```cpp
namespace jc {

template <typename T>
struct A {
  static constexpr int x = 0;  // x 是值
};

template <typename T>
struct B {
  int y;

  void f() {
    typename A<T>::x* y;  // 默认被看作声明
  }
};

template <>
struct A<int> {
  using x = int;  // x 是类型
};

}  // namespace jc

int main() {
  jc::B<int>{}.f();   // A<int>::x* y 是声明，int* y
  jc::B<void>{}.f();  // A<void>::x* y 是乘法，0 * y
}
```

+ typename 消歧义符只能用于不在基类列表和初始化列表中的 dependent name，用作用域运算符访问 dependent name 中的成员类型时，必须指定 typename 消歧义符

```cpp
namespace jc {

struct Base {
  int i;
};

template <typename T>
struct A {
  using type = T;
};

template <typename T>
struct Derived : A<T>::type {  // 基类列表中不能加 typename 消歧义符
  Derived()
      : A<T>::type  // 初始化列表中不能加 typename 消歧义符
        (typename A<T>::type{0})  // 必须加 typename 消歧义符
  {}

  A<T> f() {
    typename A<T>::type* p;  // 必须加 typename 消歧义符
    return {};
  }

  A<int>::type* s;  // non-dependent name，typename 消歧义符可有可无
};

}  // namespace jc

int main() { jc::Derived<jc::Base>{}.f(); }
```

### template 消歧义符

+ 访问模板参数的 dependent name 时，要在 dependent name 前加 template 消歧义符，才能让编译器知道引用的是一个模板，否则 `<` 会被视为小于号

```cpp
namespace jc {

template <typename T>
struct A {
  template <typename U>
  struct Impl {
    template <typename Y>
    static void f() {}
  };

  template <typename U>
  static void f() {}
};

}  // namespace jc

template <typename T>
void test() {
  T::template Impl<T>::template f<T>();
  T::template f<T>();
}

int main() { test<jc::A<int>>(); }
```

## Non-dependent base

+ Non-dependent base 是不用知道模板实参就可以推断类型的基类，派生类中查找 non-dependent name 时会先查找 non-dependent base，再查找模板参数列表

```cpp
#include <type_traits>

namespace jc {

template <typename>
struct Base {
  using T = char;
};

template <typename T>
struct Derived1 : Base<void> {  // non-dependent base
  using type = T;               // T 是 Base<void>::T
};

template <typename T>
struct Derived2 : Base<T> {  // dependent base
  using type = T;            // T 是模板参数
};

static_assert(std::is_same_v<Derived1<int>::type, char>);
static_assert(std::is_same_v<Derived2<int>::type, int>);

}  // namespace jc

int main() {}
```

## Dependent base

+ 对于 non-dependent name，不会在 dependent base 中做查找

```cpp
namespace jc {

template <typename>
struct Base {
  static constexpr int value = 1;
};

template <typename T>
struct Derived : Base<T> {  // dependent base
  constexpr int get_value() const {
    return value;  // 错误：不会在 dependent base 中查找 non-dependent name
  }
};

}  // namespace jc

int main() {}
```

+ 如果要在 dependent base 中查找，则可以使用 `this->` 或作用域运算符将 non-dependent name 变为 dependent name

```cpp
namespace jc {

template <typename>
struct Base {
  static constexpr int value = 1;
};

template <typename T>
struct Derived : Base<T> {  // dependent base
  constexpr int get_value() const {
    return this->value;  // dependent name，会在 dependent base 中查找
  }
};

template <>
struct Base<bool> {
  static constexpr int value = 2;
};

}  // namespace jc

int main() {
  static_assert(jc::Derived<int>{}.get_value() == 1);
  static_assert(jc::Derived<bool>{}.get_value() == 2);
}
```

+ 或者使用 using 声明，这样只需要引入一次

```cpp
namespace jc {

template <typename>
struct Base {
  static constexpr int value = 1;
};

template <typename T>
struct Derived : Base<T> {  // dependent base
  using Base<T>::value;

  constexpr int get_value() const {
    return value;  // dependent name，会在 dependent base 中查找
  }
};

template <>
struct Base<bool> {
  static constexpr int value = 2;
};

}  // namespace jc

int main() {
  static_assert(jc::Derived<int>{}.get_value() == 1);
  static_assert(jc::Derived<bool>{}.get_value() == 2);
}
```

+ 使用作用域运算符不会访问虚函数

```cpp
#include <cassert>

namespace jc {

template <typename>
struct Base {
  virtual int f() const { return 1; }
};

template <typename T>
struct Derived : Base<T> {  // dependent base
  virtual int f() const { return 2; }
  int get_value() const { return Base<T>::f(); }
};

template <>
struct Base<bool> {
  virtual int f() const { return 3; }
};

}  // namespace jc

int main() {
  assert(jc::Derived<int>{}.get_value() == 1);
  assert(jc::Derived<bool>{}.get_value() == 3);
}
```

+ 如果需要使用虚函数，则只能使用 `this->` 或 using 声明

```cpp
#include <cassert>

namespace jc {

template <typename>
struct Base {
  virtual int f() const { return 1; }
};

template <typename T>
struct Derived1 : Base<T> {  // dependent base
  virtual int f() const { return 2; }
  int get_value() const { return this->f(); }
};

template <typename T>
struct Derived2 : Base<T> {  // dependent base
  using Base<T>::f;
  virtual int f() const { return 2; }
  int get_value() const { return f(); }
};

template <>
struct Base<bool> {
  virtual int f() const { return 3; }
};

}  // namespace jc

int main() {
  assert(jc::Derived1<int>{}.get_value() == 2);
  assert(jc::Derived1<bool>{}.get_value() == 2);
  assert(jc::Derived2<int>{}.get_value() == 2);
  assert(jc::Derived2<bool>{}.get_value() == 2);
}
```

## 隐式实例化

+ 编译器遇到模板特化时会用所给的模板实参替换对应的模板参数，从而产生特化。如果声明类模板的指针或引用，不需要看到类模板定义，但如果要访问特化的成员或想知道模板特化的大小，就要先看到定义

```cpp
namespace jc {

template <typename T>
struct A;

A<int>* p = 0;  // OK：不需要类模板定义

template <typename T>
struct A {
  void f();
};

void g(A<int>& a) {  // 只使用类模板声明
  a.f();             // 使用了类模板定义，需要 A::f() 的定义
}

template <typename T>
void A<T>::f() {}

}  // namespace jc

int main() {}
```

+ 函数重载时，如果候选函数的参数是类类型，则该类必须可见。如果重载函数的参数是类模板，为了检查重载匹配，就需要实例化类模板。通过 [C++ Insights](https://cppinsights.io/) 或在 Visual Studio 中使用 `/d1templateStats` 命令行参数查看模板的实例化结果

```cpp
namespace jc {

template <typename T>
struct A {
 A(int);
};

void f(A<double>) {}
void f(int) {}

}  // namespace jc

int main() {
  jc::f(42);  // 两个函数声明都匹配，调用第二个，但仍会实例化第一个
}
```

## 延迟实例化（Lazy Instantiation）

+ 隐式实例化类模板时，也会实例化每个成员声明，但不会实例化定义。例外的是匿名 union 和虚函数，union 成员会被实例化，虚函数是否实例化依赖于具体实现

```cpp
namespace jc {

template <int N>
struct A {
  int a[N];  // 编译器会假设 N 是正整数，实例化时 N <= 0 则失败
};

template <typename T, int N>
struct B {
  void f() {
    A<N> a;  // 如果 N <= 0，调用时出错
  }

  //   void error() {  // 即使不被调用也会引发错误
  //     A<-1> a;  // 要求给出 A<-1> 的完整定义，定义 -1 大小的数组出错
  //   }

  //   virtual void g();  // 虚函数只有声明没有定义会导致链接错误

  struct Nested {  // N <= 0 时使用该定义出错
    A<N> a;
  };

  //   union {    // union 的所有成员声明都会被生成
  //     A<N> a;  // N <= 0 时出错
  //   };
};

}  // namespace jc

int main() {
  jc::B<int, -1> b;
  //   b.f();                     // 调用则出错
  //   jc::B<int, -1>::Nested{};  // 错误
}
```

## 两阶段查找（Two-Phase Lookup）

+ 编译器解析模板时，不能解析 [dependent name](https://en.cppreference.com/w/cpp/language/dependent_name)，于是会在 POI（point of instantiation）再次查找 dependent name，而 non-dependent name 在首次看到模板时就会进行查找，因此就有了两阶段查找，第一阶段发生在模板解析阶段，第二阶段在模板实例化阶段
  + 第一阶段使用 [unqualified name lookup](https://en.cppreference.com/w/cpp/language/lookup)（对于函数名查找会使用 [ADL](https://en.cppreference.com/w/cpp/language/adl)）查找 non-dependent name 和非受限的 dependent name（如具有 dependent name 实参的函数名称），但后者的查找不完整，在实例化时还会再次查找
  + 第二阶段发生的地点称为 POI，该阶段查找受限的 dependent name，并对非受限的 dependent name 再次进行 ADL

## POI（Points of Instantiation）

+ 编译器会在模板中的某个位置访问模板实例的声明或定义，实例化相应的模板定义时就会产生 POI，POI 是代码中的一个点，在该点会插入替换后的模板实例

```cpp
namespace jc {

struct A {
  A(int i) : i(i) {}
  int i;
};

A operator-(const A& a) { return A{-a.i}; }

bool operator<(const A& lhs, const A& rhs) { return lhs.i < rhs.i; }

using Int = A;  // 若使用 int 而不使用 A 则无法使用 ADL 找到 g

template <typename T>
void f(T i) {
  if (i < 0) {
    g(-i);  // POI 二阶段查找，T 为 A 可以使用 ADL，T 为 int 则找不到 g
  }
}

// 此处不能为 POI，因为 g() 不可见，无法解析 g(-i)
void g(Int) {
  // 此处不能为 POI，不允许在此处插入 f<Int>(Int) 的定义
  f<Int>(42);  // 调用点
  // 此处不能为 POI，不允许在此处插入 f<Int>(Int) 的定义
}
// 是 POI，此时 g() 可见，实例化 f<Int>(Int)

}  // namespace jc

int main() {}
```

+ 类模板实例的 POI 位置只能定义在包含该实例的声明或定义前的最近作用域

```cpp
namespace jc {

template <typename T>
struct A {
  T x;
};

// POI
int f() {
  // 不能是 POI，A<int> 的定义不能出现在函数作用域内
  return sizeof(A<int>);
  // 不能是 POI，A<int> 的定义不能出现在函数作用域内
}
// 不能是 POI，如果是 POI 则 sizeof(A<int>) 无效，因为编译后才知道大小

}  // namespace jc

int main() {}
```

+ 实例化一个模板时，可能附带实例化其他模板

```cpp
namespace jc {

template <typename T>
struct A {
  using type = int;
};

// A<char> 的 POI
template <typename T>
void f() {
  A<char>::type a = 0;
  typename A<T>::type b = 0;
}

}  // namespace jc

int main() {
  jc::f<double>();
  // A<double> 的 POI
  // f<double> 的 POI
  // f 使用了 dependent name A<T>，需要一个二次 POI
  // 此处有两个 POI，对于类实例，二次 POI 位于主 POI 之前（函数实例则位置相同）
}
```

+ 一个编译单元通常会包含一个实例的多个 POI，对类模板实例，每个编译单元只保留首个 POI，忽略其他 POI（它们不会被真正认为是 POI），对函数模板和变量模板的实例，所有 POI 都会被保留

## 模板的链接（Linkage of Template）

+ 类模板不能和其他实例共用一个名称

```cpp
namespace jc {

int A;

class A;  // OK：两者名称在不同的空间

int B;

template <typename T>
struct B;  // 错误：名称冲突

struct C;

template <typename T>
struct C;  // 错误：名称冲突

}  // namespace jc

int main() {}
```

+ 模板不能有 C linkage

```cpp
namespace jc {

extern "C++" template <typename T>
void normal();  // 默认方式，链接规范可以省略不写

extern "C" template <typename T>
void invalid();  // 错误：不能使用 C 链接

extern "Java" template <typename T>
void java_link();  // 非标准链接，某些编译器可能支持

}  // namespace jc

int main() {}
```

+ 模板通常具有外链接（external linkage），唯一例外的是 static 修饰的函数模板

```cpp
template <typename T>  // 与其他文件中同名的声明指向相同的实例
void external();

template <typename T>  // 与其他文件中同名的模板无关
static void internal();

template <typename T>  // 重复声明
static void internal();

namespace {
template <typename>  // 与其他文件中同名的模板无关
void other_internal();
}

namespace {
template <typename>  // 重复声明
void other_internal();
}

struct {
  template <typename T>
  void f(T) {}  // 无链接：不能被重复声明
} x;

int main() {}
```

## 链接错误

+ 和普通的函数不同，如果将模板的声明和实现分离，将出现链接错误，原因是编译器在函数调用处未看到实例化的函数定义，只是假设在别处提供了定义，并产生一个指向该定义的引用，并让链接器利用该引用解决此问题

```cpp
// a.hpp
#pragma once

namespace jc {

template <typename T>
class A {
 public:
  void f();
};

}  // namespace jc

// a.cpp
#include "a.hpp"

namespace jc {

template <typename T>
void A<T>::f() {}

}  // namespace jc

// main.cpp
#include "a.hpp"

int main() {
  jc::A<int>{}.f();  // 链接错误
}
```

+ 推荐的做法是直接在头文件中实现模板

```cpp
// a.hpp
#pragma once

namespace jc {

template <typename T>
class A {
 public:
  void f();
};

template <typename T>
inline void A<T>::f() {}

}  // namespace jc

// main.cpp
#include "a.hpp"

int main() { jc::A<int>{}.f(); }
```

## [显式实例化（Explicit Instantiation）](https://en.cppreference.com/w/cpp/language/class_template#Explicit_instantiation)

+ 如果希望在头文件中不暴露模板实现，则可以使用显式实例化，显式实例化相当于为模板手动实例化指定的类型，但用户仅能使用已指定类型的模板，可以在头文件中使用 extern 声明显式实例化，告知用户支持的实例化类型

```cpp
// a.hpp
#pragma once

namespace jc {

template <typename T>
class A {
 public:
  void f();
};

extern template class A<int>;         // 声明
extern template void A<double>::f();  // 声明

}  // namespace jc

// a.cpp
#include "a.hpp"

namespace jc {

template <typename T>
void A<T>::f() {}

template class A<int>;  // 实例化 A<int>，同时会实例化其所有成员
template void A<double>::f();  // 仅实例化该成员

}  // namespace jc

// main.cpp
#include "a.hpp"

int main() {
  jc::A<int>{}.f();
  jc::A<double>{}.f();
}
```

+ 可以把显式实例化可提取到一个单独的文件中，注意这个文件要包含定义模板的文件

```cpp
// a.hpp
#pragma once

namespace jc {

template <typename T>
class A {
 public:
  void f();
};

extern template class A<int>;
extern template void A<double>::f();

}  // namespace jc

// a.cpp
#include "a.hpp"

namespace jc {

template <typename T>
void A<T>::f() {}

template class A<int>;
template void A<double>::f();

}  // namespace jc

// a_init.cpp
#include "a.cpp"

namespace jc {

template class A<int>;
template void A<double>::f();

}  // namespace jc

// main.cpp
#include "a.hpp"

int main() {
  jc::A<int>{}.f();
  jc::A<double>{}.f();
}
```

+ 显式实例化不会影响类型推断规则，它只是实例化了一个实例，并不是一个可以优先匹配的非模板函数。从函数模板实例化而来的函数永远不和普通函数等价

```cpp
namespace jc {

template <typename T>
void f(T, T) {}

template void f<double>(double, double);

}  // namespace jc

int main() {
  jc::f<double>(1, 3.14);  // OK
  jc::f(1, 3.14);  // 错误：推断类型不一致，不存在普通函数 f(double, double)
}
```

+ 显式实例化的本质是创建一个特化的实例，因此显式实例化之后，不能定义同类型的特化

```cpp
namespace jc {

template <typename T>
struct A {
  void f();
};

template <typename T>
void A<T>::f() {}

// template<> struct A<int> { void f() {} };
template struct A<int>;  // 相当于创建如上实例

// template <>
// struct A<int> {};  // 不允许重定义

}  // namespace jc

int main() {}
```

## Deduced Context

+ 复杂的类型声明的匹配过程从最顶层构造开始，然后不断递归子构造，即各种组成元素，这些构造被称为 deduced context，non-deduced context 不会参与推断，而是使用其他处推断的结果，受限类型名称如 `A<T>::type` 不能用来推断 T，非类型表达式如 `A<N + 1>` 不能用来推断 N

```cpp
namespace jc {

template <int N>
struct A {
  using T = int;

  void f(int) {}
};

template <int N>  // A<N>::T 是 non-deduced context，X<N>::*p 是 deduced context
void f(void (A<N>::*p)(typename A<N>::T)) {}

}  // namespace jc

int main() {
  using namespace jc;
  f(&A<0>::f);  // 由 A<N>::*p 推断 N 为 0，A<N>::T 则使用 N 变为 A<0>::T
}
```

+ 默认实参不能用于推断

```cpp
namespace jc {

template <typename T>
void f(T x = 42) {}

}  // namespace jc

int main() {
  jc::f<int>();  // T = int
  jc::f();       // 错误：无法推断 T
}
```

## 特殊的推断情况

+ 成员函数的推断

```cpp
namespace jc {

struct A {
  void f(int*) const noexcept {}
};

template <typename RT, typename T, typename... Args>
void f(RT (T::*)(Args...) const) {}

}  // namespace jc

int main() {
  jc::f(&jc::A::f);  // RT = void，T = A，Args = int*
}
```

+ 取函数模板地址和调用转型运算符模板的推断

```cpp
namespace jc {

template <typename T>
void f(T) {}

struct A {
  template <typename T>
  operator T&() {
    static T x;
    return x;
  }
};

void g(int (&)[3]) {}

}  // namespace jc

int main() {
  void (*pf)(int) = &jc::f;  // 推断为 f<int>(int)

  jc::A a;
  jc::g(a);  // a 要转为 int(&)[3]，T 推断为 int[3]
}
```

+ 初始化列表作为实参没有具体类型，不能直接推断为初始化列表

```cpp
#include <initializer_list>

namespace jc {

template <typename T>
void f(T) {}

template <typename T>
void g(std::initializer_list<T>) {}

}  // namespace jc

int main() {
  // jc::f({1, 2, 3});  // 错误：不能推断出 T 为 initializer_list
  jc::g({1, 2, 3});  // OK：T 为 int
}
```

+ 参数包的推断

```cpp
namespace jc {

template <typename T, typename U>
struct A {};

template <typename T, typename... Args>
void f(const A<T, Args>&...);

template <typename... T, typename... U>
void g(const A<T, U>&...);

}  // namespace jc

int main() {
  using namespace jc;
  f(A<int, bool>{}, A<int, char>{});   // T = int, Args = [bool,char]
  g(A<int, bool>{}, A<int, char>{});   // T = [int, int], U = [bool, char]
  g(A<int, bool>{}, A<char, char>{});  // T = [int, char], U = [bool, char]
  // f(A<int, bool>{}, A<char, char>{});  // 错误，T 分别推断为 int 和 char
}
```

+ 完美转发处理空指针常量时，整型值会被当作常量值 0

```cpp
#include <utility>

namespace jc {

constexpr int g(...) { return 1; }
constexpr int g(int*) { return 2; }

template <typename T>
constexpr int f(T&& t) {
  return g(std::forward<T>(t));
}

}  // namespace jc

static_assert(jc::f(0) == 1);
static_assert(jc::g(0) == 2);
static_assert(jc::f(nullptr) == 2);
static_assert(jc::g(nullptr) == 2);

int main() {}
```

## [SFINAE（Substitution Failure Is Not An Error）](https://en.cppreference.com/w/cpp/language/sfinae)

+ SFINAE 用于禁止不相关函数模板在重载解析时造成错误，当替换返回类型无意义时，会忽略（SFINAE out）匹配而选择另一个更差的匹配

```cpp
#include <vector>

namespace jc {

template <typename T, std::size_t N>
T* begin(T (&a)[N]) {
  return a;
}

template <typename Container>
typename Container::iterator begin(Container& c) {
  return c.begin();
}

}  // namespace jc

int main() {
  std::vector<int> v;
  int a[10] = {};

  jc::begin(v);  // OK：只匹配第二个，SFINAE out 第一个
  jc::begin(a);  // OK：只匹配第一个，SFINAE out 第二个
}
```

+ SFINAE 只发生于函数模板替换的即时上下文中，对于模板定义中不合法的表达式，不会使用 SFINAE 机制

```cpp
namespace jc {

template <typename T, typename U>
auto f(T t, U u) -> decltype(t + u) {
  return t + u;
}

void f(...) {}

template <typename T, typename U>
auto g(T t, U u) -> decltype(auto) {  // 必须实例化 t 和 u 来确定返回类型
  return t + u;  // 不是即时上下文，不会使用 SFINAE
}

void g(...) {}

struct X {};

using A = decltype(f(X{}, X{}));  // OK：A 为 void
using B = decltype(g(X{}, X{}));  // 错误：g<X, X> 的实例化非法

}  // namespace jc

int main() {}
```

+ 一个简单的 SFINAE 技巧是使用尾置返回类型，用 devltype 和逗号运算符定义返回类型，在 decltype 中定义必须有效的表达式

```cpp
#include <cassert>
#include <string>

namespace jc {

template <typename T>
auto size(const T& t) -> decltype(t.size(), T::size_type()) {
  return t.size();
}

}  // namespace jc

int main() {
  std::string s;
  assert(jc::size(s) == 0);
}
```

+ 如果替换时使用了类成员，则会实例化类模板，此期间发生的错误不在即时上下文中，即使另一个函数模板匹配无误也不会使用 SFINAE

```cpp
namespace jc {

template <typename T>
class Array {
 public:
  using iterator = T*;
};

template <typename T>
void f(typename Array<T>::iterator) {}

template <typename T>
void f(T*) {}

}  // namespace jc

int main() {
  jc::f<int&>(0);  // 错误：第一个模板实例化 Array<int&>，创建引用的指针是非法的
}
```

+ SFINAE 最出名的应用是 [std::enable_if](https://en.cppreference.com/w/cpp/types/enable_if)

```cpp
#include <cassert>
#include <iostream>
#include <sstream>
#include <string>
#include <type_traits>

namespace jc {

template <
    typename K, typename V,
    std::enable_if_t<std::is_same_v<std::decay_t<V>, bool>, void*> = nullptr>
void append(std::ostringstream& os, const K& k, const V& v) {
  os << R"(")" << k << R"(":)" << std::boolalpha << v;
}

template <typename K, typename V,
          std::enable_if_t<!std::is_same_v<std::decay_t<V>, bool> &&
                               std::is_arithmetic_v<std::decay_t<V>>,
                           void*> = nullptr>
void append(std::ostringstream& os, const K& k, const V& v) {
  os << R"(")" << k << R"(":)" << v;
}

template <
    typename K, typename V,
    std::enable_if_t<std::is_constructible_v<std::string, std::decay_t<V>>,
                     void*> = nullptr>
void append(std::ostringstream& os, const K& k, const V& v) {
  os << R"(")" << k << R"(":")" << v << R"(")";
}

void kv_string_impl(std::ostringstream& os) {}

template <typename V, typename... Args>
std::void_t<decltype(std::cout << std::declval<std::decay_t<V>>())>
kv_string_impl(std::ostringstream& os, const std::string& k, const V& v,
               const Args&... args) {
  append(os, k, v);
  if constexpr (sizeof...(args) >= 2) {
    os << ",";
  }
  kv_string_impl(os, args...);
}

template <typename... Args>
std::string kv_string(const std::string& field, const Args&... args) {
  std::ostringstream os;
  os << field << ":{";
  kv_string_impl(os, args...);
  os << "}";
  return os.str();
}

}  // namespace jc

int main() {
  std::string a{R"(data:{})"};
  std::string b{R"(data:{"name":"jc","ID":1})"};
  std::string c{R"(data:{"name":"jc","ID":1,"active":true})"};
  assert(a == jc::kv_string("data"));
  assert(b == jc::kv_string("data", "name", "jc", "ID", 1));
  assert(c == jc::kv_string("data", "name", "jc", "ID", 1, "active", true));
}
```

## Deduction Guides

+ 字符串字面值传引用时推断为字符数组

```cpp
#include <vector>

namespace jc {

template <typename T>
class A {
 public:
  A(const T& val) : container_({val}) {}

 private:
  std::vector<T> container_;
};

}  // namespace jc

int main() {
  jc::A a = "downdemo";  // 错误：T 为 char[9]，构造 std::vector<char[9]> 出错
}
```

+ 改为传值，字符串字面值会推断为 `const char*`

```cpp
#include <type_traits>
#include <vector>

namespace jc {

template <typename T>
class A {
 public:
  A(T val) : container_({std::move(val)}) {}

 private:
  std::vector<T> container_;
};

}  // namespace jc

int main() {
  jc::A a = "downdemo";
  static_assert(std::is_same_v<decltype(a), jc::A<const char*>>);
}
```

+ C++17 可以定义 deduction guides 对特定类型的实参指定其推断类型

```cpp
#include <string>
#include <type_traits>
#include <vector>

namespace jc {

template <typename T>
class A {
 public:
  A(const T& val) : container_({val}) {}

 private:
  std::vector<T> container_;
};

A(const char*)->A<std::string>;

}  // namespace jc

int main() {
  jc::A a{"downdemo"};  // 等号初始化会出错，const char[9] 不能转为 std::string
  static_assert(std::is_same_v<decltype(a), jc::A<std::string>>);
}
```

+ 为聚合类模板定义 deduction guides

```cpp
#include <cassert>
#include <string>
#include <type_traits>

namespace jc {

template <typename T>
struct A {
  T x;
  std::string s;
};

A(const char*, const char*)->A<std::string>;

}  // namespace jc

int main() {
  jc::A a = {"down", "demo"};
  assert(a.x == "down");
  static_assert(std::is_same_v<decltype(a.x), std::string>);
}
```

+ 使用花括号赋值可以解决没有初始化列表的问题，圆括号则不行

```cpp
namespace jc {

template <typename T>
struct A {
  T x;
};

template <typename T>
A(T) -> A<T>;

}  // namespace jc

int main() {
  jc::A a1{0};     // OK
  jc::A a2 = {0};  // OK
  jc::A a3(0);   // 错误：没有初始化列表，int 不能转为 jc::A<int>
  jc::A a4 = 0;  // 错误：没有初始化列表，int 不能转为 jc::A<int>
}
```

+ explicit 声明的 deduction guides 只用于直接初始化

```cpp
namespace jc {

template <typename T, typename U>
struct A {
  A(const T&) {}
  A(T&&) {}
};

template <typename T>
A(const T&) -> A<T, T&>;

template <typename T>
explicit A(T&&) -> A<T, T>;  // 只能用于直接初始化

}  // namespace jc

int main() {
  jc::A a = 1;  // A<int, int&> a = 1;
  jc::A b{2};   // A<int, int> b{2};
}
```

+ [std::array](https://en.cppreference.com/w/cpp/container/array) 是一个聚合类模板，[C++17 为其定义了一个 deduction guides](https://en.cppreference.com/w/cpp/container/array/deduction_guides) 来推断模板参数

```cpp
#include <array>
#include <type_traits>

// template <typename T, typename... U>
// array(T, U...)
//     -> array<
//   enable_if_t<(is_same_v<T, U> && ...), T>,
//   1 + sizeof...(U)
//  >;

int main() {
  std::array a{1, 2, 3, 4};
  static_assert(std::is_same_v<decltype(a), std::array<int, 4>>);
}
```

+ C++17 允许[类模板实参推断](https://en.cppreference.com/w/cpp/language/class_template_argument_deduction)，但类模板的所有参数要么通过显式指定指出，要么通过实参推断推出，不能一部分使用显式指定一部分使用推断

```cpp
#include <string>

namespace jc {

template <typename T, typename U, typename Y = U>
struct A {
  A(T x = T{}, U y = U{}, Y z = Y{}) {}
};

}  // namespace jc

int main() {
  jc::A{1, 3.14, "hello"};  // T = int，U = double，T3 = const char*
  jc::A{1, 3.14};           // T = int，U = Y = double
  jc::A{"hi", "downdemo"};  // T = U = Y = const char*
  jc::A<std::string>{"hi", "downdemo", 42};  // 错误：只指定了 T，U 未推断
  jc::A<>{1, 3.14, 42};                      // 错误：T 和 U 都未指定
}
```

+ 类模板实参推断的本质是为每个构造函数和构造函数模板隐式添加一个 deduction guides

```cpp
#include <type_traits>
#include <vector>

namespace jc {

template <typename T>
class A {
 public:
  A(const T& val) : container_({val}) {}

 private:
  std::vector<T> container_;
};

// template <typename T>
// A(const T&) -> A<T>;  // 隐式 deduction guides

}  // namespace jc

int main() {
  jc::A a1 = 0;
  jc::A a2{0};
  jc::A a3(0);
  auto a4 = jc::A{0};
  static_assert(std::is_same_v<decltype(a1), jc::A<int>>);
  static_assert(std::is_same_v<decltype(a2), jc::A<int>>);
  static_assert(std::is_same_v<decltype(a3), jc::A<int>>);
  static_assert(std::is_same_v<decltype(a4), jc::A<int>>);
}
```

## Deduction Guides 的问题

+ 用类模板实例作为实参时，Deduction guides 对实参推断的类型有歧义，标准委员会对于该情况有争议地规定，推断时不会将实参推断为类模板的实例

```cpp
#include <type_traits>

namespace jc {

template <typename T>
struct A {
  A(T x) {}
};

template <typename T>
A(T) -> A<T>;

}  // namespace jc

int main() {
  jc::A a1{0};
  jc::A a2{a1};  // A<int> 还是 A<A<int>>？标准委员会规定为 A<int>
  jc::A a3(a1);  // A<int> 还是 A<A<int>>？标准委员会规定为 A<int>
  static_assert(std::is_same_v<decltype(a1), jc::A<int>>);
  static_assert(std::is_same_v<decltype(a2), jc::A<int>>);
  static_assert(std::is_same_v<decltype(a3), jc::A<int>>);
}
```

+ 这个争议造成的问题如下

```cpp
#include <type_traits>
#include <vector>

namespace jc {

template <typename T, typename... Args>
auto f(const T& x, const Args&... args) {  // 如果 T 为 std::vector
  return std::vector{x, args...};  // 参数包是否为空将决定不同的返回类型
}

}  // namespace jc

int main() {
  using std::vector;
  vector v1{1, 2, 3};
  vector v2{v1};
  vector v3{v1, v1};
  static_assert(std::is_same_v<decltype(v1), vector<int>>);
  static_assert(std::is_same_v<decltype(v2), vector<int>>);
  static_assert(std::is_same_v<decltype(v3), vector<vector<int>>>);
  static_assert(std::is_same_v<decltype(jc::f(v1)), vector<int>>);
  static_assert(std::is_same_v<decltype(jc::f(v1, v1)), vector<vector<int>>>);
}
```

+ 添加隐式 deduction guides 是有争议的，主要反对观点是这个特性自动将接口添加到已存在的库中，并且对于有限定名称的情况，deduction guides 会失效

```cpp
namespace jc {

template <typename T>
struct type_identity {
  using type = T;
};

template <typename T>
class A {
 public:
  using ArgType = typename type_identity<T>::type;
  A(ArgType) {}
};

template <typename T>
A(typename type_identity<T>::type) -> A<T>;
// 该 deduction guides 无效，因为有限定名称符 type_identity<T>::

}  // namespace jc

int main() {
  jc::A a{0};  // 错误
}
```

+ 为了保持向后兼容性，如果模板名称是[注入类名](https://en.cppreference.com/w/cpp/language/injected-class-name)，则禁用类模板实参推断

```cpp
#include <type_traits>

namespace jc {

template <typename T>
struct A {
  template <typename U>
  A(U x) {}

  template <typename U>
  auto f(U x) {
    return A(x);  // 根据注入类名规则 A 是 A<T>，根据类模板实参推断 A 是 A<U>
  }
};

}  // namespace jc

int main() {
  jc::A<int> a{0};
  auto res = a.f<double>(3.14);
  static_assert(std::is_same_v<decltype(res), jc::A<int>>);
}
```

+ 使用转发引用的 deduction guides 可能推断出引用类型，导致实例化错误或产生空悬引用，因此标准委员会决定使用隐式 deduction guides 的推断时，禁用 T&& 这个特殊的推断规则

```cpp
#include <string>
#include <type_traits>

namespace jc {

template <typename T>
struct A {
  A(const T&) {}
  A(T&&) {}
};

// template <typename T>
// A(const T&) -> A<T>;  // 隐式生成

// template <typename T>
// A(T&&) -> A<T>;  // 不会隐式生成该 deduction guides

}  // namespace jc

int main() {
  std::string s;
  jc::A a = s;  // T 推断为 std::string
  static_assert(std::is_same_v<decltype(a), jc::A<std::string>>);
  // 若指定 T&& 的 deduction guides，则 T 推断为 std::string&
}
```

+ Deduction guides 只用于推断而非调用，实参的传递方式不必完全对应构造函数

```cpp
#include <iostream>
#include <type_traits>
#include <utility>

namespace jc {

template <typename T>
struct A {};

template <typename T>
struct B {
  B(const A<T>&) { std::cout << 1 << std::endl; }
  B(A<T>&&) { std::cout << 2 << std::endl; }
};

template <typename T>
B(A<T>) -> B<T>;  // 不需要完全对应构造函数

}  // namespace jc

int main() {
  jc::A<int> a;
  jc::B{a};             // 1
  jc::B{std::move(a)};  // 2
}
```

## [函数模板重载](https://en.cppreference.com/w/cpp/language/function_template#Function_template_overloading)

+ 对于实参推断能匹配多个模板的情况，标准规定了偏序（partial ordering）规则，最终将调用最特殊（能接受更少类型）的模板

```cpp
#include <cassert>

namespace jc {

template <typename T>
int f(T) {
  return 1;
}

template <typename T>
int f(T*) {
  return 2;
}

}  // namespace jc

int main() {
  int* p = nullptr;
  assert(jc::f<int*>(p) == 1);
  assert(jc::f<int>(p) == 2);
  assert(jc::f(p) == 2);  // 两个模板均匹配，第二个模板更特殊
  assert(jc::f(0) == 1);  // 0 推断为 int，匹配第一个模板
  assert(jc::f(nullptr) == 1);  // nullptr 推断为 std::nullptr_t，匹配第一个模板
}
```

+ 对于两个模板，用实参替代第一个模板的参数，替代后的结果作为实参去推断第二个模板，如果推断成功，反过来用第二个模板推断第一个模板，若推断失败，则第一个模板更特殊，如果均推断失败或推断成功，则两个模板没有偏序关系

```cpp
#include <cassert>

namespace jc {

template <typename T>
int f(T) {  // 1
  return 1;
}

template <typename T>
int f(T*) {  // 2
  return 2;
}

template <typename T>
int f(const T*) {  // 3
  return 3;
}

}  // namespace jc

int main() {
  const int* p = nullptr;
  assert(jc::f(p) == 3);
  // 推断结果：
  // 1: f(T)        [T = const int*]
  // 2: f(T*)       [T = const int]
  // 3: f(const T*) [T = int]
  // 偏序处理：
  // 用 2 推断 1，T = U*，推断成功
  // 用 1 推断 2，T* = U，无法推断 T
  // 2 比 1 特殊
  // 用 3 推断 1，T = const U*，推断成功
  // 用 1 推断 3，const T* = U，无法推断 T
  // 3 比 1 特殊
  // 用 3 推断 2，T = const U，推断成功
  // 用 2 推断 3，const T = U，无法推断 T
  // 3 比 2 特殊
  // 3 最特殊，因此调用 3
}
```

+ 函数模板可以和非模板函数重载

```cpp
#include <iostream>

namespace jc {

struct A {
  A() = default;
  A(const A&) { std::cout << 1; }
  A(A&&) { std::cout << 2; }

  template <typename T>
  A(T&&) {
    std::cout << 3;
  }
};

}  // namespace jc

int main() {
  jc::A a1;
  jc::A a2{a1};  // 3，对 non-const 对象，成员模板优于拷贝构造函数
  jc::A a3{std::move(a1)};  // 2，移动构造函数
  const jc::A b1;
  jc::A b2{b1};             // 1，拷贝构造函数
  jc::A b3{std::move(b1)};  // 3，const A&&，更匹配成员模板
}
```

+ 变参模板的重载

```cpp
namespace jc {

template <typename... Ts>
struct A {};

template <typename T>
constexpr int f(A<T*>) {
  return 1;
}

template <typename... Ts>
constexpr int f(A<Ts...>) {
  return 2;
}

template <typename... Ts>
constexpr int f(A<Ts*...>) {
  return 3;
}

static_assert(f(A<int*>{}) == 1);
static_assert(f(A<int, double>{}) == 2);
static_assert(f(A<int*, double*>{}) == 3);

}  // namespace jc

int main() {}
```

## [特化（Specialization）](https://en.cppreference.com/w/cpp/language/template_specialization)

+ 函数模板特化引入了重载和实参推断，如果能推断特化版本，就可以不显式声明模板实参

```cpp
#include <cassert>

namespace jc {

template <typename T>
int f(T) {  // 1
  return 1;
}

template <typename T>
int f(T*) {  // 2
  return 2;
}

template <>
int f(int) {  // OK：1 的特化
  return 3;
}

template <>
int f(int*) {  // OK：2 的特化
  return 4;
}

}  // namespace jc

int main() {
  int* p = nullptr;
  assert(jc::f(p) == 4);
  assert(jc::f(0) == 3);
  assert(jc::f(nullptr) == 1);
}
```

+ 函数模板的特化不能有默认实参，但会使用要被特化的模板的默认实参

```cpp
namespace jc {

template <typename T>
constexpr int f(T x = 1) {  // T 不会由默认实参推断
  return x;
}

template <>
constexpr int f(int x) {  // 不能指定默认实参
  return x + 1;
}

static_assert(f<int>() == 2);

}  // namespace jc

int main() {}
```

+ 类模板特化的实参列表必须对应模板参数，如果有默认实参可以不指定对应参数。可以特化整个类模板，也可以特化部分成员。如果对某种类型特化类模板成员，就不能再特化整个类模板，其他未特化的成员会被保留

```cpp
#include <cassert>

namespace jc {

template <typename T, typename U = int>
struct A;

template <>
struct A<void> {
  constexpr int f();
};

constexpr int A<void>::f() { return 1; }

template <>
struct A<int, int> {
  int i = 0;
};

template <>
struct A<char, char> {
  template <typename T>
  struct B {
    int f() { return i; }
    static int i;
  };
};

template <typename T>
int A<char, char>::B<T>::i = 1;

template <>
int A<char, char>::B<double>::i = 2;

template <>
int A<char, char>::B<char>::f() {
  return 0;
};

// template <>
// struct A<char, char> {};  // 错误，不能对已经特化过成员的类型做特化

template <>
struct A<char, char>::B<bool> {
  int j = 3;
};

}  // namespace jc

int main() {
  static_assert(jc::A<void>{}.f() == 1);
  static_assert(jc::A<void, int>{}.f() == 1);
  // jc::A<void, double>{};  // 错误：未定义类型
  assert((jc::A<int, int>{}.i == 0));
  assert((jc::A<char, char>::B<int>{}.i == 1));
  assert((jc::A<char, char>::B<int>{}.f() == 1));
  assert((jc::A<char, char>::B<double>{}.i == 2));
  assert((jc::A<char, char>::B<double>{}.f() == 2));
  assert((jc::A<char, char>::B<char>{}.i == 1));
  assert((jc::A<char, char>::B<char>{}.f() == 0));
  assert((jc::A<char, char>::B<bool>{}.j == 3));
}
```

+ 类模板特化必须在实例化之前，对已实例化的类型不能再进行特化

```cpp
namespace jc {

template <typename T>
struct A {};

A<int> a;

template <>
struct A<double> {};  // OK

template <>
struct A<int> {};  // 错误：不能特化已实例化的 A<int>

}  // namespace jc

int main() {}
```

## [偏特化（Partial Specialization）](https://en.cppreference.com/w/cpp/language/partial_specialization)

+ 类模板偏特化限定一些类型，而非某个具体类型

```cpp
namespace jc {

template <typename T>
struct A;  // primary template

template <typename T>
struct A<const T> {};

template <typename T>
struct A<T*> {
  static constexpr int size = 0;
};

template <typename T, int N>
struct A<T[N]> {
  static constexpr int size = N;
};

template <typename Class>
struct A<int * Class::*> {
  static constexpr int i = 1;
};

template <typename T, typename Class>
struct A<T * Class::*> {
  static constexpr int i = 2;
};

template <typename Class>
struct A<void (Class::*)()> {
  static constexpr int i = 3;
};

template <typename RT, typename Class>
struct A<RT (Class::*)() const> {
  static constexpr int i = 4;
};

template <typename RT, typename Class, typename... Args>
struct A<RT (Class::*)(Args...)> {
  static constexpr int i = 5;
};

template <typename RT, typename Class, typename... Args>
struct A<RT (Class::*)(Args...) const noexcept> {
  static constexpr int i = 6;
};

struct B {
  int* i = nullptr;
  double* j = nullptr;
  void f1() {}
  constexpr int f2() const { return 0; }
  void f3(int&, double) {}
  void f4(int&, double) const noexcept {}
};

static_assert(A<decltype(&B::i)>::i == 1);
static_assert(A<decltype(&B::j)>::i == 2);
static_assert(A<decltype(&B::f1)>::i == 3);
static_assert(A<decltype(&B::f2)>::i == 4);
static_assert(A<decltype(&B::f3)>::i == 5);
static_assert(A<decltype(&B::f4)>::i == 6);

}  // namespace jc

int main() {
  int a[] = {1, 2, 3};
  static_assert(jc::A<decltype(&a)>::size == 0);
  static_assert(jc::A<decltype(a)>::size == 3);
  // jc::A<const int[3]>{};  // 错误：匹配多个版本
}
```

+ [变量模板（variable template）](https://en.cppreference.com/w/cpp/language/variable_template)的特化和偏特化

```cpp
#include <cassert>
#include <list>
#include <type_traits>
#include <vector>

namespace jc {

template <typename T>
constexpr int i = sizeof(T);

template <>
constexpr int i<void> = 0;

template <typename T>
constexpr int i<T&> = sizeof(void*);

static_assert(i<int> == sizeof(int));
static_assert(i<double> == sizeof(double));
static_assert(i<void> == 0);
static_assert(i<int&> == sizeof(void*));

// 变量模板特化的类型可以不匹配 primary template
template <typename T>
typename T::iterator null_iterator;

template <>
int* null_iterator<std::vector<int>> = nullptr;

template <typename T, std::size_t N>
T* null_iterator<T[N]> = nullptr;

}  // namespace jc

int main() {
  auto it1 = jc::null_iterator<std::vector<int>>;
  auto it2 = jc::null_iterator<std::list<int>>;
  auto it3 = jc::null_iterator<double[3]>;
  static_assert(std::is_same_v<decltype(it1), int*>);
  assert(!it1);
  static_assert(std::is_same_v<decltype(it2), std::list<int>::iterator>);
  static_assert(std::is_same_v<decltype(it3), double*>);
  assert(!it3);
}
```

## Traits 的偏特化实现

+ [std::is_same](https://en.cppreference.com/w/cpp/types/is_same)

```cpp
#include <type_traits>

namespace jc {

template <typename T, typename U>
struct is_same {
  static constexpr bool value = false;
};

template <typename T>
struct is_same<T, T> {
  static constexpr bool value = true;
};

template <typename T, typename U>
constexpr bool is_same_v = is_same<T, U>::value;

}  // namespace jc

static_assert(jc::is_same_v<int, int>);
static_assert(!jc::is_same_v<int, double>);
static_assert(!jc::is_same_v<int, int&>);

int main() {}
```

+ 获取元素类型

```cpp
#include <type_traits>

namespace jc {

template <typename T>
struct get_element {
  using type = T;
};

template <typename T>
struct get_element<T[]> {
  using type = typename get_element<T>::type;
};

template <typename T, std::size_t N>
struct get_element<T[N]> {
  using type = typename get_element<T>::type;
};

template <typename T>
using get_element_t = typename get_element<T>::type;

}  // namespace jc

static_assert(std::is_same_v<jc::get_element_t<int>, int>);
static_assert(std::is_same_v<jc::get_element_t<int[]>, int>);
static_assert(std::is_same_v<jc::get_element_t<int[3][4][5]>, int>);

int main() {}
```

+ [std::remove_reference](https://en.cppreference.com/w/cpp/types/remove_reference)

```cpp
#include <type_traits>

namespace jc {

template <typename T>
struct remove_reference {
  using type = T;
};

template <typename T>
struct remove_reference<T&> {
  using type = T;
};

template <typename T>
struct remove_reference<T&&> {
  using type = T;
};

template <typename T>
using remove_reference_t = typename remove_reference<T>::type;

}  // namespace jc

static_assert(std::is_same_v<jc::remove_reference_t<int>, int>);
static_assert(std::is_same_v<jc::remove_reference_t<int&>, int>);
static_assert(std::is_same_v<jc::remove_reference_t<int&&>, int>);

int main() {}
```

+ [std::enable_if](https://en.cppreference.com/w/cpp/types/enable_if)

```cpp
#include <list>
#include <type_traits>
#include <utility>
#include <vector>

namespace jc {

template <bool, typename T = void>
struct enable_if {};

template <typename T>
struct enable_if<true, T> {
  using type = T;
};

template <bool B, typename T = void>
using enable_if_t = typename enable_if<B, T>::type;

}  // namespace jc

struct Base {};
struct Derived1 : Base {};
struct Derived2 : Base {};

template <typename T, template <typename...> class V>
void impl(const V<T>&) {
  static_assert(std::is_constructible_v<Base*, T*>);
}

template <typename T, template <typename...> class V, typename... Args,
          jc::enable_if_t<std::is_constructible_v<Base*, T*>, void*> = nullptr>
void f(const V<T>& t, Args&&... args) {
  impl(t);
  if constexpr (sizeof...(args) > 0) {
    f(std::forward<Args>(args)...);
  }
}

int main() { f(std::vector<Derived1>{}, std::list<Derived2>{}); }
```

## 元函数转发（Metafunction Forwarding）

+ Traits 可以视为对类型做操作的函数，称为元函数，元函数一般包含一些相同的成员，将相同成员封装成一个基类作为基本元函数，继承这个基类即可使用成员，这种实现方式称为元函数转发，标准库中实现了 [std::integral_constant](https://en.cppreference.com/w/cpp/types/integral_constant) 作为基本元函数

```cpp
#include <cassert>
#include <type_traits>

namespace jc {

template <class T, T v>
struct integral_constant {
  static constexpr T value = v;
  using value_type = T;
  using type = integral_constant<T, v>;
  constexpr operator value_type() const noexcept { return value; }
  constexpr value_type operator()() const noexcept { return value; }
};

constexpr int to_int(char c) {
  // hexadecimal letters:
  if (c >= 'A' && c <= 'F') {
    return static_cast<int>(c) - static_cast<int>('A') + 10;
  }
  if (c >= 'a' && c <= 'f') {
    return static_cast<int>(c) - static_cast<int>('a') + 10;
  }
  assert(c >= '0' && c <= '9');
  return static_cast<int>(c) - static_cast<int>('0');
}

template <std::size_t N>
constexpr int parse_int(const char (&arr)[N]) {
  int base = 10;   // to handle base (default: decimal)
  int offset = 0;  // to skip prefixes like 0x
  if (N > 2 && arr[0] == '0') {
    switch (arr[1]) {
      case 'x':  // prefix 0x or 0X, so hexadecimal
      case 'X':
        base = 16;
        offset = 2;
        break;
      case 'b':  // prefix 0b or 0B (since C++14), so binary
      case 'B':
        base = 2;
        offset = 2;
        break;
      default:  // prefix 0, so octal
        base = 8;
        offset = 1;
        break;
    }
  }
  int res = 0;
  int multiplier = 1;
  for (std::size_t i = 0; i < N - offset; ++i) {
    if (arr[N - 1 - i] != '\'') {
      res += to_int(arr[N - 1 - i]) * multiplier;
      multiplier *= base;
    }
  }
  return res;
}

template <char... cs>
constexpr auto operator"" _c() {
  return integral_constant<int, parse_int<sizeof...(cs)>({cs...})>{};
}

static_assert(std::is_same_v<decltype(2_c), integral_constant<int, 2>>);
static_assert(std::is_same_v<decltype(0xFF_c), integral_constant<int, 255>>);
static_assert(
    std::is_same_v<decltype(0b1111'1111_c), integral_constant<int, 255>>);

}  // namespace jc

static_assert(jc::integral_constant<int, 42>::value == 42);
static_assert(std::is_same_v<int, jc::integral_constant<int, 0>::value_type>);
static_assert(jc::integral_constant<int, 42>{} == 42);

int main() {
  jc::integral_constant<int, 42> f;
  static_assert(f() == 42);
}
```

+ 利用元函数转发实现 [std::is_same](https://en.cppreference.com/w/cpp/types/is_same)

```cpp
namespace jc {

template <class T, T v>
struct integral_constant {
  static constexpr T value = v;
  using value_type = T;
  using type = integral_constant<T, v>;
  constexpr operator value_type() const noexcept { return value; }
  constexpr value_type operator()() const noexcept { return value; }
};

template <bool B>
using bool_constant = integral_constant<bool, B>;

using true_type = bool_constant<true>;
using false_type = bool_constant<false>;

template <typename T, typename U>
struct is_same : false_type {};

template <typename T>
struct is_same<T, T> : true_type {};

template <typename T, typename U>
constexpr bool is_same_v = is_same<T, U>::value;

}  // namespace jc

static_assert(jc::is_same_v<int, int>);
static_assert(!jc::is_same_v<int, double>);
static_assert(!jc::is_same_v<int, int&>);

int main() {}
```

## SFINAE-based traits

+ [std::is_default_constructible](https://en.cppreference.com/w/cpp/types/is_default_constructible)

```cpp
#include <type_traits>

namespace jc {

template <typename T>
struct is_default_constructible {
 private:
  template <typename U, typename = decltype(U())>
  static std::true_type test(void*);

  template <typename>
  static std::false_type test(...);

 public:
  static constexpr bool value = decltype(test<T>(nullptr))::value;
};

template <typename T>
constexpr bool is_default_constructible_v = is_default_constructible<T>::value;

}  // namespace jc

struct A {
  A() = delete;
};

static_assert(!jc::is_default_constructible_v<A>);

int main() {}
```

+ [std::void_t](https://en.cppreference.com/w/cpp/types/void_t)

```cpp
#include <type_traits>

namespace jc {

template <typename...>
using void_t = void;

template <typename, typename = void_t<>>
struct is_default_constructible : std::false_type {};

template <typename T>
struct is_default_constructible<T, void_t<decltype(T())>> : std::true_type {};

template <typename T>
constexpr bool is_default_constructible_v = is_default_constructible<T>::value;

}  // namespace jc

struct A {
  A() = delete;
};

static_assert(!jc::is_default_constructible_v<A>);

int main() {}
```

+ [std::declval](https://en.cppreference.com/w/cpp/utility/declval)

```cpp
#include <type_traits>

namespace jc {

template <typename>
constexpr bool always_false = false;

template <typename T>
std::add_rvalue_reference_t<T> declval() noexcept {
  static_assert(always_false<T>, "declval not allowed in an evaluated context");
}

template <typename, typename = std::void_t<>>
struct has_less : std::false_type {};

template <typename T>
struct has_less<T, std::void_t<decltype(jc::declval<T>() < jc::declval<T>())>>
    : std::true_type {};

template <typename T>
constexpr bool has_less_v = has_less<T>::value;

}  // namespace jc

struct A {
  A() = delete;
  bool operator<(const A& rhs) const { return i < rhs.i; }
  int i;
};

static_assert(jc::has_less_v<A>);

int main() {}
```

+ [std::is_nothrow_move_constructible](https://en.cppreference.com/w/cpp/types/is_move_constructible)

```cpp
#include <type_traits>

namespace jc {

template <typename T, typename = std::void_t<>>
struct is_nothrow_move_constructible : std::false_type {};

template <typename T>
struct is_nothrow_move_constructible<
    T, std::void_t<decltype(T(std::declval<T>()))>>
    : std::bool_constant<noexcept(T(std::declval<T>()))> {};

template <typename T>
constexpr bool is_nothrow_move_constructible_v =
    is_nothrow_move_constructible<T>::value;

}  // namespace jc

struct A {
  A(A&&) noexcept {}
};

struct B {
 private:
  B(B&&) noexcept {};
};

static_assert(jc::is_nothrow_move_constructible_v<A>);
static_assert(!jc::is_nothrow_move_constructible_v<B>);

int main() {}
```

+ [std::is_convertible](https://en.cppreference.com/w/cpp/types/is_convertible)

```cpp
#include <type_traits>

namespace jc {

// 转为 void 类型需要单独处理，转为数组和函数类型总是 false
template <typename From, typename To,
          bool = std::is_void_v<To> || std::is_array_v<To> ||
                 std::is_function_v<To>>
struct is_convertible_impl {
  using type = std::bool_constant<std::is_void_v<To> && std::is_void_v<From>>;
};

template <typename From, typename To>
struct is_convertible_impl<From, To, false> {
 private:
  static void f(To);

  template <typename T, typename U,
            typename = decltype(f(std::declval<T>()))>  // 将 T 转为 To
  static std::true_type test(void*);

  template <typename, typename>
  static std::false_type test(...);

 public:
  using type = decltype(test<From, To>(nullptr));
};

template <typename From, typename To>
struct is_convertible : is_convertible_impl<From, To>::type {};

template <typename From, typename To>
constexpr bool is_convertible_v = is_convertible<From, To>::value;

}  // namespace jc

struct A {};
struct B : A {};

static_assert(jc::is_convertible_v<B, A>);
static_assert(jc::is_convertible_v<B*, A*>);
static_assert(!jc::is_convertible_v<A*, B*>);
static_assert(jc::is_convertible_v<void, void>);
static_assert(!jc::is_convertible_v<int*, int[]>);

int main() {}
```

+ [std::is_class](https://en.cppreference.com/w/cpp/types/is_class)

```cpp
#include <string>
#include <type_traits>
#include <vector>

namespace jc {

template <typename T, typename = std::void_t<>>
struct is_class : std::false_type {};

template <typename T>
struct is_class<T, std::void_t<int T::*>> : std::true_type {};

template <class T>
constexpr bool is_class_v = is_class<T>::value;

}  // namespace jc

union A {};

static_assert(jc::is_class_v<std::string>);
static_assert(jc::is_class_v<std::vector<int>>);
static_assert(jc::is_class_v<A>);
static_assert(std::is_union_v<A>);   // 仅能由编译器开洞实现
static_assert(!std::is_class_v<A>);  // 排除了 union 类型

int main() {}
```

+ [std::is_member_pointer](https://en.cppreference.com/w/cpp/types/is_member_pointer)

```cpp
#include <cassert>
#include <type_traits>

namespace jc {

template <class T>
struct is_member_pointer_helper : std::false_type {};

template <class T, class U>
struct is_member_pointer_helper<T U::*> : std::true_type {};

template <class T>
struct is_member_pointer : is_member_pointer_helper<std::remove_cv_t<T>> {};

template <class T>
constexpr bool is_member_pointer_v = is_member_pointer<T>::value;

}  // namespace jc

struct A {
  int f() const { return 1; }
  int i = 0;
};

static_assert(jc::is_member_pointer_v<decltype(&A::f)>);
static_assert(jc::is_member_pointer_v<int (A::*)() const>);
static_assert(jc::is_member_pointer_v<void (A::*)()>);
static_assert(jc::is_member_pointer_v<decltype(&A::i)>);
static_assert(jc::is_member_pointer_v<int A::*>);
static_assert(jc::is_member_pointer_v<double A::*>);

int main() {
  int (A::*pf)() const = &A::f;
  int A::*pi = &A::i;

  assert((A{}.*pf)() == 1);
  static_assert(jc::is_member_pointer_v<decltype(pf)>);

  assert(A{}.*pi == 0);
  static_assert(jc::is_member_pointer_v<decltype(pi)>);
}
```

+ [std::is_member_function_pointer](https://en.cppreference.com/w/cpp/types/is_member_function_pointer)

```cpp
#include <type_traits>

namespace jc {

template <class T>
struct is_member_function_pointer_helper : std::false_type {};

template <class T, class U>
struct is_member_function_pointer_helper<T U::*> : std::is_function<T> {};

template <class T>
struct is_member_function_pointer
    : is_member_function_pointer_helper<std::remove_cv_t<T>> {};

template <class T>
constexpr bool is_member_function_pointer_v =
    is_member_function_pointer<T>::value;

}  // namespace jc

struct A {
  void f() {}
  static void g() {}
  int i = 0;
};

void f() {}

static_assert(jc::is_member_function_pointer_v<decltype(&A::f)>);
static_assert(!jc::is_member_function_pointer_v<decltype(&A::g)>);
static_assert(!jc::is_member_function_pointer_v<decltype(&f)>);
static_assert(!jc::is_member_function_pointer_v<decltype(&A::i)>);
static_assert(!jc::is_member_function_pointer_v<int A::*>);
static_assert(!jc::is_member_function_pointer_v<double A::*>);

int main() {}
```

+ [std::is_member_object_pointer](https://en.cppreference.com/w/cpp/types/is_member_object_pointer)

```cpp
#include <type_traits>

namespace jc {

template <class T>
struct is_member_object_pointer
    : std::bool_constant<std::is_member_pointer_v<T> &&
                         !std::is_member_function_pointer_v<T>> {};

template <class T>
constexpr bool is_member_object_pointer_v = is_member_object_pointer<T>::value;

}  // namespace jc

struct A {
  int f() const { return 1; }
  int i = 0;
};

static_assert(!jc::is_member_object_pointer_v<decltype(&A::f)>);
static_assert(!jc::is_member_object_pointer_v<int (A::*)() const>);
static_assert(!jc::is_member_object_pointer_v<void (A::*)()>);
static_assert(jc::is_member_object_pointer_v<decltype(&A::i)>);
static_assert(jc::is_member_object_pointer_v<int A::*>);
static_assert(jc::is_member_object_pointer_v<double A::*>);

int main() {}
```

+ 检查可访问的 non-static 成员

```cpp
#include <type_traits>
#include <utility>
#include <vector>

#define DEFINE_HAS_VAR(V)                                                  \
  template <typename, typename = std::void_t<>>                            \
  struct has_var_##V : std::false_type {};                                 \
  template <typename T>                                                    \
  struct has_var_##V<T, std::void_t<decltype(&T::V)>> : std::true_type {}; \
  template <typename T>                                                    \
  constexpr bool has_var_##V##_v = has_var_##V<T>::value;

#define DEFINE_HAS_METHOD(F)                                           \
  template <typename, typename = std::void_t<>>                        \
  struct has_func_##F : std::false_type {};                            \
  template <typename T>                                                \
  struct has_func_##F<T, std::void_t<decltype(std::declval<T>().F())>> \
      : std::true_type {};                                             \
  template <typename T>                                                \
  constexpr bool has_func_##F##_v = has_func_##F<T>::value;

namespace jc {

DEFINE_HAS_VAR(first);
DEFINE_HAS_METHOD(begin);

}  // namespace jc

static_assert(jc::has_var_first_v<std::pair<int, int>>);
static_assert(jc::has_func_begin_v<std::vector<int>>);

int main() {}
```

+ [Detection idiom](https://en.cppreference.com/w/cpp/experimental/is_detected)

```cpp
#include <type_traits>
#include <utility>
#include <vector>

namespace jc {

template <typename, template <typename...> class Op, typename... Args>
struct detector : std::false_type {};

template <template <typename...> class Op, typename... Args>
struct detector<std::void_t<Op<Args...>>, Op, Args...> : std::true_type {};

template <template <typename...> class Op, typename... Args>
using is_detected = detector<void, Op, Args...>;

template <typename T>
using has_emplace_back = decltype(std::declval<T>().emplace_back(
    std::declval<typename T::value_type>()));

template <typename T>
constexpr bool has_emplace_back_v =
    is_detected<has_emplace_back, std::remove_reference_t<T>>::value;

}  // namespace jc

static_assert(jc::has_emplace_back_v<std::vector<int>>);
static_assert(jc::has_emplace_back_v<std::vector<int>&>);
static_assert(jc::has_emplace_back_v<std::vector<int>&&>);

int main() {}
```

## 空基类优化（EBCO，Empty Base Class Optimization）

+ 为了保证给类动态分配内存时有不同的地址，C++ 规定空类大小必须大于 0

```cpp
namespace jc {

struct A {};
struct B {};

static_assert(sizeof(A) > 0);
static_assert(sizeof(B) > 0);

}  // namespace jc

int main() {
  jc::A a;
  jc::B b;
  static_assert((void*)&a != (void*)&b);
}
```

+ 一般编译器将空类大小设为 1 字节，对于空类存在继承关系的情况，如果支持 EBCO，可以优化派生类的空间占用大小

```cpp
/* 不支持 EBCO 的内存布局：
 * [    ] } A } B } C
 * [    ]     }   }
 * [    ]         }
 *
 * 支持 EBCO 的内存布局：
 * [    ] } A } B } C
 */

namespace jc {

struct A {
  using Int = int;
};

struct B : A {};
struct C : B {};

static_assert(sizeof(A) == 1);
static_assert(sizeof(A) == sizeof(B));
static_assert(sizeof(A) == sizeof(C));

}  // namespace jc

int main() {}
```

+ 模板参数可能是空类

```cpp
namespace jc {

struct A {};
struct B {};

template <typename T, typename U>
struct C {
  T a;
  U b;
};

static_assert(sizeof(C<A, B>) == 2);

}  // namespace jc

int main() {}
```

+ 为了利用 EBCO 压缩内存空间，可以将模板参数设为基类

```cpp
namespace jc {

struct A {};
struct B {};

template <typename T, typename U>
struct C : T, U {};

static_assert(sizeof(C<A, B>) == 1);

}  // namespace jc

int main() {}
```

+ 但模板参数可能是相同类型，或者不一定是类，此时将其设为基类在实例化时会报错。如果已知一个模板参数类型为空类，把可能为空的类型参数与一个不为空的成员利用 EBCO 合并起来，即可把空类占用的空间优化掉

```cpp
namespace jc {

template <typename Base, typename Member>
class Pair : private Base {
 public:
  Pair(const Base& b, const Member& m) : Base(b), member_(m) {}

  const Base& first() const { return (const Base&)*this; }

  Base& first() { return (Base&)*this; }

  const Member& second() const { return this->member_; }

  Member& second() { return this->member_; }

 private:
  Member member_;
};

template <typename T>
struct Unoptimizable {
  T info;
  void* storage;
};

template <typename T>
struct Optimizable {
  Pair<T, void*> info_and_storage;
};

}  // namespace jc

struct A {};

static_assert(sizeof(jc::Unoptimizable<A>) == 2 * sizeof(void*));
static_assert(sizeof(jc::Optimizable<A>) == sizeof(void*));

int main() {}
```

## 奇异递归模板模式（CRTP，The Curiously Recurring Template Pattern）

+ CRTP 的实现手法是将派生类作为基类的模板参数

```cpp
#include <cassert>

namespace jc {

template <typename T>
class Base {
 public:
  static int count() { return i; }

 protected:
  Base() { ++i; }
  Base(const Base<T> &) { ++i; }
  Base(Base<T> &&) noexcept { ++i; }
  ~Base() { --i; }

 private:
  inline static int i = 0;
};

template <typename T>
class Derived : public Base<Derived<T>> {};

}  // namespace jc

int main() {
  jc::Derived<int> a, b;
  jc::Derived<char> c;
  assert(jc::Derived<int>::count() == 2);
  assert(jc::Derived<char>::count() == 1);
}
```

+ 通常大量运算符重载会一起出现，但通常这些运算符只需要一个定义，其他运算符可以提取到基类中基于这一个来实现

```cpp
#include <cassert>

namespace jc {

template <typename T>
class Base {
  friend bool operator!=(const T& lhs, const T& rhs) { return !(lhs == rhs); }
};

class Derived : public Base<Derived> {
  friend bool operator==(const Derived& lhs, const Derived& rhs) {
    return lhs.i_ == rhs.i_;
  }

 public:
  Derived(int i) : i_(i) {}

 private:
  int i_ = 0;
};

}  // namespace jc

int main() {
  jc::Derived a{1};
  jc::Derived b{2};
  assert(a != b);
}
```

+ CRTP 基类可以基于 CRTP 派生类暴露的小得多的接口定义大部分接口，这个模式称为 facade 模式

```cpp
#include <cassert>
#include <iterator>
#include <type_traits>
#include <vector>

namespace jc {

template <typename Derived, typename Value, typename Category>
class IteratorFacade {
 public:
  using value_type = std::remove_const_t<Value>;
  using reference = Value&;
  using pointer = Value*;
  using difference_type = std::ptrdiff_t;
  using iterator_category = Category;

 public:
  reference operator*() const { return as_derived().dereference(); }

  Derived& operator++() {
    as_derived().increment();
    return as_derived();
  }

  Derived operator++(int) {
    Derived tmp(as_derived());
    as_derived().increment();
    return tmp;
  }

  friend bool operator==(const IteratorFacade& lhs, const IteratorFacade& rhs) {
    return lhs.as_derived().equals(rhs.as_derived());
  }

  friend bool operator!=(const IteratorFacade& lhs, const IteratorFacade& rhs) {
    return !operator==(lhs, rhs);
  }

 private:
  Derived& as_derived() { return *static_cast<Derived*>(this); }

  const Derived& as_derived() const {
    return *static_cast<const Derived*>(this);
  }
};

template <typename T>
struct ListNode {
  ListNode(T x) : value(x) {}

  T value;
  ListNode<T>* next = nullptr;
};

template <typename T>
class ListNodeIterator
    : public IteratorFacade<ListNodeIterator<T>, T, std::forward_iterator_tag> {
 public:
  ListNodeIterator(ListNode<T>* t = nullptr) : t_(t) {}
  T& dereference() const { return t_->value; }
  void increment() { t_ = t_->next; }
  bool equals(const ListNodeIterator& rhs) const { return t_ == rhs.t_; }

 private:
  ListNode<T>* t_ = nullptr;
};

}  // namespace jc

int main() {
  auto a = new jc::ListNode<int>{1};
  auto b = new jc::ListNode<int>{2};
  auto c = new jc::ListNode<int>{3};
  a->next = b;
  b->next = c;

  auto first = jc::ListNodeIterator{a};
  auto last = ++jc::ListNodeIterator{c};

  std::vector<int> v;
  for (auto it = first; it != last; ++it) {
    v.emplace_back(*it);
  }
  assert((v == std::vector<int>{1, 2, 3}));

  delete c;
  delete b;
  delete a;
}
```

## Mixins

+ 使用 Mixins 手法可以更方便地引入额外信息

```cpp
#include <cassert>
#include <string>

namespace jc {

template <typename... Mixins>
struct Point : Mixins... {
  Point() : Mixins()..., x(0.0), y(0.0) {}
  Point(double x, double y) : Mixins()..., x(x), y(y) {}
  double x;
  double y;
};

struct Label {
  std::string label = "point";
};

struct Color {
  enum { red, green, blue };
};

using CustomPoint = Point<Label, Color>;

}  // namespace jc

int main() {
  jc::CustomPoint p;
  assert(p.label == "point");
  assert(p.red == jc::Color::red);
  assert(p.green == jc::Color::green);
  assert(p.blue == jc::Color::blue);
}
```

+ CRTP-mixin

```cpp
#include <cassert>
#include <string>

namespace jc {

template <typename T>
class Base {
 public:
  static int count() { return i; }

 protected:
  Base() { ++i; }
  Base(const Base<T> &) { ++i; }
  Base(Base<T> &&) noexcept { ++i; }
  ~Base() { --i; }

 private:
  inline static int i = 0;
};

template <template <typename> class... Mixins>
struct Point : Mixins<Point<>>... {
  Point() : Mixins<Point<>>()..., x(0.0), y(0.0) {}
  Point(double x, double y) : Mixins<Point<>>()..., x(x), y(y) {}
  double x;
  double y;
};

template <typename T>
struct Label {
  std::string label = "point";
};

template <typename T>
struct Color {
  enum { red, green, blue };
};

using PointCount = Point<Base, Label, Color>;

}  // namespace jc

int main() {
  jc::PointCount a, b, c;
  assert(jc::PointCount::count() == 3);
  assert(a.label == "point");
  assert(a.red == jc::Color<void>::red);
  assert(a.green == jc::Color<void>::green);
  assert(a.blue == jc::Color<void>::blue);
}
```

+ Mixins 参数化成员函数的虚拟性

```cpp
#include <cassert>

namespace jc {

template <typename... Mixins>
class Base : private Mixins... {
 public:
  int f() { return 1; }  // 是否为虚函数由 Mixins 中的声明决定
};

template <typename... Mixins>
class Derived : public Base<Mixins...> {
 public:
  int f() { return 2; }
};

}  // namespace jc

struct A {};

struct B {
  virtual int f() = 0;
};

int main() {
  jc::Base<A>* p = new jc::Derived<A>;
  assert(p->f() == 1);

  jc::Base<B>* q = new jc::Derived<B>;
  assert(q->f() == 2);
}
```

## 指定模板参数

+ 模板常常带有一长串类型参数，不过通常都设有默认值

```cpp
struct A {};
struct B {};
struct C {};

template <typename T1 = A, typename T2 = B, typename T3 = C>
struct MyClass {};
```

+ 现在想指定某个实参，而其他参数依然使用默认实参

```cpp
namespace jc {

struct A {};
struct B {};
struct C {
  static constexpr int f() { return 1; }
};

struct Alias {
  using P1 = A;
  using P2 = B;
  using P3 = C;
};

template <typename T>
struct SetT1 : virtual Alias {
  using P1 = T;
};

template <typename T>
struct SetT2 : virtual Alias {
  using P2 = T;
};

template <typename T>
struct SetT3 : virtual Alias {
  using P3 = T;
};

// 由于不能从多个相同类直接继承，需要一个中间层用于区分
template <typename T, int N>
struct Mid : T {};

template <typename T1, typename T2, typename T3>
struct SetBase : Mid<T1, 1>, Mid<T2, 2>, Mid<T3, 3> {};

/* Alias 要被用作默认实参
 * 但 SetBase 会将其多次指定为 Mid 的基类
 * 为了防止多次继承产生二义性
 * 虚派生一个新类替代 Alias 作为默认实参
 */
struct Args : virtual Alias {};  // Args 即包含了别名 P1、P2、P3

template <typename T1 = Args, typename T2 = Args, typename T3 = Args>
struct MyClass {
  using Policies = SetBase<T1, T2, T3>;

  constexpr int f() { return Policies::P3::f(); }
};

struct D {
  static constexpr int f() { return 2; }
};

static_assert(MyClass{}.f() == 1);
static_assert(MyClass<SetT3<D>>{}.f() == 2);

}  // namespace jc

int main() {}
```

## 类型擦除（Type Erasure）

+ 类型擦除即将不同类型抽象为相同的表示，但使用时仍可以还原出原有类型的行为，[std::any](https://en.cppreference.com/w/cpp/utility/any) 和 [std::function](https://en.cppreference.com/w/cpp/utility/functional/function) 就使用了类型擦除的手法。比起函数指针，[std::function](https://en.cppreference.com/w/cpp/utility/functional/function) 在编译期擦除掉了不需要关心的原有类型，用同一种表示抽象了所有函数类型，并且能存储 lambda 或函数对象，使用时又能像调用函数一样使用原有类型

```cpp
#include <any>
#include <cassert>
#include <exception>
#include <type_traits>

namespace jc {

template <typename T>
class is_equality_comparable {
 private:
  static void* conv(bool);

  template <typename U>
  static std::true_type test(
      decltype(conv(std::declval<const U&>() == std::declval<const U&>())),
      decltype(conv(!(std::declval<const U&>() == std::declval<const U&>()))));

  template <typename U>
  static std::false_type test(...);

 public:
  static constexpr bool value = decltype(test<T>(nullptr, nullptr))::value;
};

template <typename T, bool = is_equality_comparable<T>::value>
struct try_equals {
  static bool equals(const T& lhs, const T& rhs) { return lhs == rhs; }
};

struct not_equality_comparable : std::exception {};

template <typename T>
struct try_equals<T, false> {
  static bool equals(const T& lhs, const T& rhs) {
    throw not_equality_comparable();
  }
};

template <typename R, typename... Args>
class functor_bridge {
 public:
  virtual ~functor_bridge() {}
  virtual functor_bridge* clone() const = 0;
  virtual R invoke(Args... args) const = 0;
  virtual bool equals(const functor_bridge*) const = 0;
};

template <typename F, typename R, typename... Args>
class functor_bridge_impl : public functor_bridge<R, Args...> {
 public:
  template <typename T>
  functor_bridge_impl(T&& f) : f_(std::forward<T>(f)) {}

  virtual functor_bridge_impl* clone() const override {
    return new functor_bridge_impl(f_);
  }

  virtual R invoke(Args... args) const override {
    return f_(std::forward<Args>(args)...);
  }

  virtual bool equals(const functor_bridge<R, Args...>* rhs) const override {
    if (auto p = dynamic_cast<const functor_bridge_impl*>(rhs)) {
      return try_equals<F>::equals(f_, p->f_);
    }
    return false;
  }

 private:
  F f_;
};

template <typename>
class function;

template <typename R, typename... Args>
class function<R(Args...)> {
  friend bool operator==(const function& lhs, const function& rhs) {
    if (!lhs || !rhs) {
      return !lhs && !rhs;
    }
    return lhs.bridge_->equals(rhs.bridge_);
  }

  friend bool operator!=(const function& lhs, const function& rhs) {
    return !(lhs == rhs);
  }

  friend void swap(function& lhs, function& rhs) noexcept {
    std::swap(lhs.bridge_, rhs.bridge_);
  }

 public:
  function() = default;

  function(const function& rhs) {
    if (rhs.bridge_) {
      bridge_ = rhs.bridge_->clone();
    }
  }

  function(function& rhs) : function(static_cast<const function&>(rhs)) {}

  function(function&& rhs) noexcept : bridge_(rhs.bridge_) {
    rhs.bridge_ = nullptr;
  }

  template <typename F>
  function(F&& f) {
    using Bridge = functor_bridge_impl<std::decay_t<F>, R, Args...>;
    bridge_ = new Bridge(std::forward<F>(f));  // type erasure
  }

  ~function() { delete bridge_; }

  function& operator=(const function& rhs) {
    function tmp(rhs);
    swap(*this, tmp);
    return *this;
  }

  function& operator=(function&& rhs) noexcept {
    delete bridge_;
    bridge_ = rhs.bridge_;
    rhs.bridge_ = nullptr;
    return *this;
  }

  template <typename F>
  function& operator=(F&& rhs) {
    function tmp(std::forward<F>(rhs));
    swap(*this, tmp);
    return *this;
  }

  explicit operator bool() const { return bridge_ == nullptr; }

  R operator()(Args... args) const {
    return bridge_->invoke(std::forward<Args>(args)...);
  }

 private:
  functor_bridge<R, Args...>* bridge_ = nullptr;
};

}  // namespace jc

int main() {
  jc::function<bool(int)> f = [](const std::any& a) -> int {
    return std::any_cast<int>(a);
  };
  assert(f(3.14) == 1);
}
```

## 元编程（Metaprogramming）

+ 元编程将计算在编译期完成，避免了运行期计算的开销

```cpp
#include <type_traits>

namespace jc {

template <int N, int... Ns>
struct max;

template <int N>
struct max<N> : std::integral_constant<int, N> {};

template <int N1, int N2, int... Ns>
struct max<N1, N2, Ns...>
    : std::integral_constant<int, (N1 < N2) ? max<N2, Ns...>::value
                                            : max<N1, Ns...>::value> {};

template <int... Ns>
inline constexpr auto max_v = max<Ns...>::value;

}  // namespace jc

static_assert(jc::max_v<3, 2, 1, 5, 4> == 5);

int main() {}
```

+ 模板元编程通常使用偏特化和递归实现，由于编译期需要实例化代码，如果递归层次过深，会带来代码体积膨胀的问题

```cpp
#include <type_traits>

namespace jc {

template <int N, int L = 1, int R = N>
struct sqrt {
  static constexpr auto M = L + (R - L) / 2;
  static constexpr auto T = N / M;
  static constexpr auto value =  // 避免递归实例化所有分支
      std::conditional_t<(T < M), sqrt<N, L, M>, sqrt<N, M + 1, R>>::value;
};

template <int N, int M>
struct sqrt<N, M, M> {
  static constexpr auto value = M - 1;
};

template <int N>
inline constexpr auto sqrt_v = sqrt<N, 1, N>::value;

}  // namespace jc

static_assert(jc::sqrt_v<10000> == 100);

int main() {}
```

+ C++14 支持 constexpr 函数，简化了实现并且没有递归实例化的代码膨胀问题

```cpp
namespace jc {

template <int N>
constexpr int sqrt() {
  if constexpr (N <= 1) {
    return N;
  }
  int l = 1;
  int r = N;
  while (l < r) {
    int m = l + (r - l) / 2;
    int t = N / m;
    if (m == t) {
      return m;
    } else if (m > t) {
      r = m;
    } else {
      l = m + 1;
    }
  }
  return l - 1;
}

}  // namespace jc

static_assert(jc::sqrt<10000>() == 100);

int main() {}
```

## 循环展开（Loop Unrolling）

+ 在一些机器上，for 循环的汇编将产生分支指令

```cpp
#include <array>
#include <cassert>

namespace jc {

template <typename T, std::size_t N>
auto dot_product(const std::array<T, N>& lhs, const std::array<T, N>& rhs) {
  T res{};
  for (std::size_t i = 0; i < N; ++i) {
    res += lhs[i] * rhs[i];
  }
  return res;
}

}  // namespace jc

int main() {
  std::array<int, 3> a{1, 2, 3};
  std::array<int, 3> b{4, 5, 6};
  assert(jc::dot_product(a, b) == 32);
}
```

+ 循环展开是一种牺牲体积加快程序执行速度的方法，现代编译器会优化循环为目标平台最高效形式。使用元编程可以展开循环，虽然已经没有必要，但还是给出实现

```cpp
#include <array>
#include <cassert>

namespace jc {

template <typename T, std::size_t N>
struct dot_product_impl {
  static T value(const T* lhs, const T* rhs) {
    return *lhs * *rhs + dot_product_impl<T, N - 1>::value(lhs + 1, rhs + 1);
  }
};

template <typename T>
struct dot_product_impl<T, 0> {
  static T value(const T*, const T*) { return T{}; }
};

template <typename T, std::size_t N>
auto dot_product(const std::array<T, N>& lhs, const std::array<T, N>& rhs) {
  return dot_product_impl<T, N>::value(&*std::begin(lhs), &*std::begin(rhs));
}

}  // namespace jc

int main() {
  std::array<int, 3> a{1, 2, 3};
  std::array<int, 3> b{4, 5, 6};
  assert(jc::dot_product(a, b) == 32);
}
```

## [Unit Type](https://en.wikipedia.org/wiki/Unit_type)

+ [std::ratio](https://en.cppreference.com/w/cpp/numeric/ratio/ratio)

```cpp
#include <cassert>
#include <cmath>
#include <type_traits>

namespace jc {

template <int N, int D = 1>
struct ratio {
  static constexpr int num = N;
  static constexpr int den = D;
  using type = ratio<num, den>;
};

template <typename R1, typename R2>
struct ratio_add_impl {
 private:
  static constexpr int den = R1::den * R2::den;
  static constexpr int num = R1::num * R2::den + R2::num * R1::den;

 public:
  using type = ratio<num, den>;
};

template <typename R1, typename R2>
using ratio_add = typename ratio_add_impl<R1, R2>::type;

template <typename T, typename U = ratio<1>>
class duration {
 public:
  using rep = T;
  using period = typename U::type;

 public:
  constexpr duration(rep r = 0) : r_(r) {}
  constexpr rep count() const { return r_; }

 private:
  rep r_;
};

template <typename T1, typename U1, typename T2, typename U2>
constexpr auto operator+(const duration<T1, U1>& lhs,
                         const duration<T2, U2>& rhs) {
  using CommonType = ratio<1, ratio_add<U1, U2>::den>;
  auto res =
      (lhs.count() * U1::num / U1::den + rhs.count() * U2::num / U2::den) *
      CommonType::den;
  return duration<decltype(res), CommonType>{res};
}

}  // namespace jc

int main() {
  constexpr auto a = jc::duration<double, jc::ratio<1, 1000>>(10);  // 10 ms
  constexpr auto b = jc::duration<double, jc::ratio<1, 3>>(7.5);    // 2.5 s
  constexpr auto c = a + b;  // 10 * 3 + 7.5 * 1000 = 7530 * 1/3000 s
  assert(std::abs(c.count() - 7530) < 1e-6);
  static_assert(std::is_same_v<std::decay_t<decltype(c)>,
                               jc::duration<double, jc::ratio<1, 3000>>>);
  static_assert(decltype(c)::period::num == 1);
  static_assert(decltype(c)::period::den == 3000);
}
```

## Typelist

```cpp
// typelist.hpp

#pragma once

#include <type_traits>

namespace jc {

template <typename...>
struct typelist {};

template <typename List>
struct front;

template <typename Head, typename... Tail>
struct front<typelist<Head, Tail...>> {
  using type = Head;
};

template <typename List>
using front_t = typename front<List>::type;

// pop_front_t
template <typename List>
struct pop_front;

template <typename Head, typename... Tail>
struct pop_front<typelist<Head, Tail...>> {
  using type = typelist<Tail...>;
};

template <typename List>
using pop_front_t = typename pop_front<List>::type;

// push_front_t
template <typename List, typename NewElement>
struct push_front;

template <typename... Elements, typename NewElement>
struct push_front<typelist<Elements...>, NewElement> {
  using type = typelist<NewElement, Elements...>;
};

template <typename List, typename NewElement>
using push_front_t = typename push_front<List, NewElement>::type;

// nth_element_t
template <typename List, std::size_t N>
struct nth_element : nth_element<pop_front_t<List>, N - 1> {};

template <typename List>
struct nth_element<List, 0> : front<List> {};

template <typename List, std::size_t N>
using nth_element_t = typename nth_element<List, N>::type;

// is_empty
template <typename T>
struct is_empty {
  static constexpr bool value = false;
};

template <>
struct is_empty<typelist<>> {
  static constexpr bool value = true;
};

template <typename T>
inline constexpr bool is_empty_v = is_empty<T>::value;

// find_index_of_t
template <typename List, typename T, std::size_t N = 0,
          bool Empty = is_empty_v<List>>
struct find_index_of;

template <typename List, typename T, std::size_t N>
struct find_index_of<List, T, N, false>
    : std::conditional_t<std::is_same_v<front_t<List>, T>,
                         std::integral_constant<std::size_t, N>,
                         find_index_of<pop_front_t<List>, T, N + 1>> {};

template <typename List, typename T, std::size_t N>
struct find_index_of<List, T, N, true> {};

template <typename List, typename T>
using find_index_of_t = typename find_index_of<List, T>::type;

// push_back_t
template <typename List, typename NewElement, bool = is_empty_v<List>>
struct push_back_impl;

template <typename List, typename NewElement>
struct push_back_impl<List, NewElement, false> {
 private:
  using head = front_t<List>;
  using tail = pop_front_t<List>;
  using new_tail = typename push_back_impl<tail, NewElement>::type;

 public:
  using type = push_front_t<new_tail, head>;
};

template <typename List, typename NewElement>
struct push_back_impl<List, NewElement, true> {
  using type = push_front_t<List, NewElement>;
};

template <typename List, typename NewElement>
struct push_back : push_back_impl<List, NewElement> {};

/*
 * template <typename List, typename NewElement>
 * struct push_back;
 *
 * template <typename... Elements, typename NewElement>
 * struct push_back<typelist<Elements...>, NewElement> {
 * using type = typelist<Elements..., NewElement>;
 * };
 */

template <typename List, typename NewElement>
using push_back_t = typename push_back<List, NewElement>::type;

// reverse_t
template <typename List, bool Empty = is_empty_v<List>>
struct reverse;

template <typename List>
using reverse_t = typename reverse<List>::type;

template <typename List>
struct reverse<List, false>
    : push_back<reverse_t<pop_front_t<List>>, front_t<List>> {};

template <typename List>
struct reverse<List, true> {
  using type = List;
};

// pop_back_t
template <typename List>
struct pop_back {
  using type = reverse_t<pop_front_t<reverse_t<List>>>;
};

template <typename List>
using pop_back_t = typename pop_back<List>::type;

// largest_type_t
template <typename List, bool = is_empty_v<List>>
struct largest_type;

template <typename List>
struct largest_type<List, false> {
 private:
  using contender = front_t<List>;
  using best = typename largest_type<pop_front_t<List>>::type;

 public:
  using type =
      std::conditional_t<(sizeof(contender) >= sizeof(best)), contender, best>;
};

template <typename List>
struct largest_type<List, true> {
  using type = char;
};

template <typename List>
using largest_type_t = typename largest_type<List>::type;

// transform_t
template <typename List, template <typename T> class MetaFun,
          bool = is_empty_v<List>>
struct transform;

/*
 * template <typename List, template <typename T> class MetaFun>
 * struct transform<List, MetaFun, false>
 *     : push_front<typename transform<pop_front_t<List>, MetaFun>::type,
 *                  typename MetaFun<front_t<List>>::type> {};
 */

template <typename... Elements, template <typename T> class MetaFun>
struct transform<typelist<Elements...>, MetaFun, false> {
  using type = typelist<typename MetaFun<Elements>::type...>;
};

template <typename List, template <typename T> class MetaFun>
struct transform<List, MetaFun, true> {
  using type = List;
};

template <typename List, template <typename T> class MetaFun>
using transform_t = typename transform<List, MetaFun>::type;

// accumulate_t
template <typename List, template <typename T, typename U> class F,
          typename Init, bool = is_empty_v<List>>
struct accumulate;

template <typename List, template <typename T, typename U> class MetaFun,
          typename Init>
struct accumulate<List, MetaFun, Init, false>
    : accumulate<pop_front_t<List>, MetaFun,
                 typename MetaFun<Init, front_t<List>>::type> {};

template <typename List, template <typename T, typename U> class MetaFun,
          typename Init>
struct accumulate<List, MetaFun, Init, true> {
  using type = Init;
};

template <typename List, template <typename T, typename U> class MetaFun,
          typename Init>
using accumulate_t = typename accumulate<List, MetaFun, Init>::type;

// insert_sorted_t
template <typename T>
struct type_identity {
  using type = T;
};

template <typename List, typename Element,
          template <typename T, typename U> class Compare,
          bool = is_empty_v<List>>
struct insert_sorted;

template <typename List, typename Element,
          template <typename T, typename U> class Compare>
struct insert_sorted<List, Element, Compare, false> {
 private:
  // compute the tail of the resulting list:
  using new_tail = typename std::conditional_t<
      Compare<Element, front_t<List>>::value, type_identity<List>,
      insert_sorted<pop_front_t<List>, Element, Compare>>::type;

  // compute the head of the resulting list:
  using new_head = std::conditional_t<Compare<Element, front_t<List>>::value,
                                      Element, front_t<List>>;

 public:
  using type = push_front_t<new_tail, new_head>;
};

template <typename List, typename Element,
          template <typename T, typename U> class Compare>
struct insert_sorted<List, Element, Compare, true> : push_front<List, Element> {
};

template <typename List, typename Element,
          template <typename T, typename U> class Compare>
using insert_sorted_t = typename insert_sorted<List, Element, Compare>::type;

// insertion_sort_t
template <typename List, template <typename T, typename U> class Compare,
          bool = is_empty_v<List>>
struct insertion_sort;

template <typename List, template <typename T, typename U> class Compare>
using insertion_sort_t = typename insertion_sort<List, Compare>::type;

template <typename List, template <typename T, typename U> class Compare>
struct insertion_sort<List, Compare, false>
    : insert_sorted<insertion_sort_t<pop_front_t<List>, Compare>, front_t<List>,
                    Compare> {};

template <typename List, template <typename T, typename U> class Compare>
struct insertion_sort<List, Compare, true> {
  using type = List;
};

// multiply_t
template <typename T, typename U>
struct multiply;

template <typename T, T Value1, T Value2>
struct multiply<std::integral_constant<T, Value1>,
                std::integral_constant<T, Value2>> {
  using type = std::integral_constant<T, Value1 * Value2>;
};

template <typename T, typename U>
using multiply_t = typename multiply<T, U>::type;

// for std::index_sequence
template <std::size_t... Values>
struct is_empty<std::index_sequence<Values...>> {
  static constexpr bool value = sizeof...(Values) == 0;
};

template <std::size_t Head, std::size_t... Tail>
struct front<std::index_sequence<Head, Tail...>> {
  using type = std::integral_constant<std::size_t, Head>;
  static constexpr std::size_t value = Head;
};

template <std::size_t Head, std::size_t... Tail>
struct pop_front<std::index_sequence<Head, Tail...>> {
  using type = std::index_sequence<Tail...>;
};

template <std::size_t... Values, std::size_t New>
struct push_front<std::index_sequence<Values...>,
                  std::integral_constant<std::size_t, New>> {
  using type = std::index_sequence<New, Values...>;
};

template <std::size_t... Values, std::size_t New>
struct push_back<std::index_sequence<Values...>,
                 std::integral_constant<std::size_t, New>> {
  using type = std::index_sequence<Values..., New>;
};

// select_t
template <typename Types, typename Indices>
struct select;

template <typename Types, std::size_t... Indices>
struct select<Types, std::index_sequence<Indices...>> {
  using type = typelist<nth_element_t<Types, Indices>...>;
};

template <typename Types, typename Indices>
using select_t = typename select<Types, Indices>::type;

// Cons
struct Nil {};

template <typename Head, typename Tail = Nil>
struct Cons {
  using head = Head;
  using tail = Tail;
};

template <typename List>
struct front {
  using type = typename List::head;
};

template <typename List, typename Element>
struct push_front {
  using type = Cons<Element, List>;
};

template <typename List>
struct pop_front {
  using type = typename List::tail;
};

template <>
struct is_empty<Nil> {
  static constexpr bool value = true;
};

}  // namespace jc

namespace jc::test {

template <typename T, typename U>
struct smaller {
  static constexpr bool value = sizeof(T) < sizeof(U);
};

template <typename T, typename U>
struct less;

template <typename T, T M, T N>
struct less<std::integral_constant<T, M>, std::integral_constant<T, N>> {
  static constexpr bool value = M < N;
};

template <typename T, T... Values>
using integral_constant_typelist =
    typelist<std::integral_constant<T, Values>...>;

static_assert(std::is_same_v<integral_constant_typelist<std::size_t, 2, 3, 5>,
                             typelist<std::integral_constant<std::size_t, 2>,
                                      std::integral_constant<std::size_t, 3>,
                                      std::integral_constant<std::size_t, 5>>>);
static_assert(is_empty_v<typelist<>>);
using T1 = push_front_t<typelist<>, char>;
static_assert(std::is_same_v<T1, typelist<char>>);
static_assert(std::is_same_v<front_t<T1>, char>);
using T2 = push_front_t<T1, double>;
static_assert(std::is_same_v<T2, typelist<double, char>>);
static_assert(std::is_same_v<front_t<T2>, double>);
static_assert(std::is_same_v<pop_front_t<T2>, typelist<char>>);
using T3 = push_back_t<T2, int*>;
static_assert(std::is_same_v<T3, typelist<double, char, int*>>);
static_assert(std::is_same_v<nth_element_t<T3, 0>, double>);
static_assert(std::is_same_v<nth_element_t<T3, 1>, char>);
static_assert(std::is_same_v<nth_element_t<T3, 2>, int*>);
static_assert(std::is_same_v<find_index_of_t<T3, double>,
                             std::integral_constant<std::size_t, 0>>);
static_assert(std::is_same_v<find_index_of_t<T3, char>,
                             std::integral_constant<std::size_t, 1>>);
static_assert(std::is_same_v<find_index_of_t<T3, int*>,
                             std::integral_constant<std::size_t, 2>>);
static_assert(std::is_same_v<reverse_t<T3>, typelist<int*, char, double>>);
static_assert(std::is_same_v<pop_back_t<T3>, typelist<double, char>>);
static_assert(std::is_same_v<largest_type_t<T3>, double>);
static_assert(std::is_same_v<transform_t<T3, std::add_const>,
                             typelist<const double, const char, int* const>>);
static_assert(std::is_same_v<accumulate_t<T3, push_front, typelist<>>,
                             typelist<int*, char, double>>);
static_assert(std::is_same_v<insertion_sort_t<T3, smaller>,
                             typelist<char, int*, double>>);
static_assert(accumulate_t<integral_constant_typelist<int, 2, 3, 5>, multiply,
                           std::integral_constant<int, 1>>::value == 30);

static_assert(
    std::is_same_v<insertion_sort_t<std::index_sequence<2, 3, 0, 1>, less>,
                   std::index_sequence<0, 1, 2, 3>>);
static_assert(is_empty_v<std::index_sequence<>>);
static_assert(std::is_same_v<std::make_index_sequence<4>,
                             std::index_sequence<0, 1, 2, 3>>);
static_assert(front<std::make_index_sequence<4>>::value == 0);
static_assert(std::is_same_v<front_t<std::make_index_sequence<4>>,
                             std::integral_constant<std::size_t, 0>>);
static_assert(std::is_same_v<pop_front_t<std::make_index_sequence<4>>,
                             std::index_sequence<1, 2, 3>>);
static_assert(
    std::is_same_v<push_front_t<std::make_index_sequence<4>,
                                std::integral_constant<std::size_t, 4>>,
                   std::index_sequence<4, 0, 1, 2, 3>>);
static_assert(
    std::is_same_v<push_back_t<std::make_index_sequence<4>,
                               std::integral_constant<std::size_t, 4>>,
                   std::index_sequence<0, 1, 2, 3, 4>>);
static_assert(std::is_same_v<select_t<typelist<bool, char, int, double>,
                                      std::index_sequence<2, 3, 0, 1>>,
                             typelist<int, double, bool, char>>);

using ConsList = Cons<int, Cons<char, Cons<short, Cons<double>>>>;
static_assert(is_empty_v<Nil>);
static_assert(std::is_same_v<
              push_front_t<ConsList, bool>,
              Cons<bool, Cons<int, Cons<char, Cons<short, Cons<double>>>>>>);
static_assert(std::is_same_v<pop_front_t<ConsList>,
                             Cons<char, Cons<short, Cons<double>>>>);
static_assert(std::is_same_v<front_t<ConsList>, int>);
static_assert(std::is_same_v<insertion_sort_t<ConsList, smaller>,
                             Cons<char, Cons<short, Cons<int, Cons<double>>>>>);

}  // namespace jc::test
```

## [std::tuple](https://en.cppreference.com/w/cpp/utility/tuple)

```cpp
#include <cassert>
#include <complex>
#include <cstring>
#include <functional>
#include <ostream>
#include <sstream>
#include <string>
#include <type_traits>
#include <utility>

#include "typelist.hpp"

namespace jc {

template <typename... Types>
class tuple;

template <typename Head, typename... Tail>
class tuple<Head, Tail...> {
 public:
  tuple() = default;

  tuple(const Head& head, const tuple<Tail...>& tail)
      : head_(head), tail_(tail) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(VHead&& head, VTail&&... tail)
      : head_(std::forward<VHead>(head)), tail_(std::forward<VTail>(tail)...) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(const tuple<VHead, VTail...>& rhs)
      : head_(rhs.get_head()), tail_(rhs.get_tail()) {}

  // for push_back_tuple
  template <typename V, typename VHead, typename... VTail>
  tuple(const V& v, const tuple<VHead, VTail...>& rhs) : head_(v), tail_(rhs) {}

  Head& get_head() { return head_; }

  const Head& get_head() const { return head_; }

  tuple<Tail...>& get_tail() { return tail_; }

  const tuple<Tail...>& get_tail() const { return tail_; }

  template <typename T, T Index>
  auto& operator[](std::integral_constant<T, Index>);

 private:
  Head head_;
  tuple<Tail...> tail_;
};

template <>
class tuple<> {};

template <std::size_t N>
struct tuple_get {
  template <typename Head, typename... Tail>
  static auto& apply(const tuple<Head, Tail...>& t) {
    return tuple_get<N - 1>::apply(t.get_tail());
  }
};

template <>
struct tuple_get<0> {
  template <typename Head, typename... Tail>
  static const Head& apply(const tuple<Head, Tail...>& t) {
    return t.get_head();
  }
};

template <std::size_t N, typename... Types>
auto& get(const tuple<Types...>& t) {
  return tuple_get<N>::apply(t);
}

template <typename Head, typename... Tail>
template <typename T, T Index>
inline auto& tuple<Head, Tail...>::operator[](
    std::integral_constant<T, Index>) {
  return get<Index>(*this);
}

template <typename... Types>
auto make_tuple(Types&&... args) {
  return tuple<std::decay_t<Types>...>(std::forward<Types>(args)...);
}

bool operator==(const tuple<>&, const tuple<>&) { return true; }

template <
    typename Head1, typename... Tail1, typename Head2, typename... Tail2,
    std::enable_if_t<sizeof...(Tail1) == sizeof...(Tail2), void*> = nullptr>
bool operator==(const tuple<Head1, Tail1...>& lhs,
                const tuple<Head2, Tail2...>& rhs) {
  return lhs.get_head() == rhs.get_head() && lhs.get_tail() == rhs.get_tail();
}

void print_tuple(std::ostream& os, const tuple<>&, bool is_first = true) {
  os << (is_first ? '(' : ')');
}

template <typename Head, typename... Tail>
void print_tuple(std::ostream& os, const tuple<Head, Tail...>& t,
                 bool is_first = true) {
  os << (is_first ? "(" : ", ") << t.get_head();
  print_tuple(os, t.get_tail(), false);
}

template <typename... Types>
std::ostream& operator<<(std::ostream& os, const tuple<Types...>& t) {
  print_tuple(os, t);
  return os;
}

}  // namespace jc

namespace jc {  // typelist

template <>
struct is_empty<tuple<>> {
  static constexpr bool value = true;
};

template <typename Head, typename... Tail>
class front<tuple<Head, Tail...>> {
 public:
  using type = Head;
};

template <typename Head, typename... Tail>
class pop_front<tuple<Head, Tail...>> {
 public:
  using type = tuple<Tail...>;
};

template <typename... Types, typename Element>
class push_front<tuple<Types...>, Element> {
 public:
  using type = tuple<Element, Types...>;
};

template <typename... Types, typename Element>
class push_back<tuple<Types...>, Element> {
 public:
  using type = tuple<Types..., Element>;
};

template <typename... Types>
pop_front_t<tuple<Types...>> pop_front_tuple(const tuple<Types...>& t) {
  return t.get_tail();
}

template <typename... Types, typename V>
push_front_t<tuple<Types...>, V> push_front_tuple(const tuple<Types...>& t,
                                                  const V& v) {
  return push_front_t<tuple<Types...>, V>{v, t};
}

template <typename V>
tuple<V> push_back_tuple(const tuple<>&, const V& v) {
  return tuple<V>{v};
}

template <typename Head, typename... Tail, typename V>
tuple<Head, Tail..., V> push_back_tuple(const tuple<Head, Tail...>& t,
                                        const V& v) {
  return tuple<Head, Tail..., V>{t.get_head(),
                                 push_back_tuple(t.get_tail(), v)};
}

template <typename... Types, std::size_t... Indices>
auto select_tuple(const tuple<Types...>& t, std::index_sequence<Indices...>) {
  // find std::make_tuple using ADL so explicitly specify the namespace
  return jc::make_tuple(get<Indices>(t)...);
}

template <typename... Types>
auto reverse_tuple(const tuple<Types...>& t) {
  return select_tuple(t, reverse_t<std::index_sequence_for<Types...>>{});
}

// The following implementation copies elements repeatedly.
// tuple<> reverse_tuple(const tuple<>& t) { return t; }

// template <typename Head, typename... Tail>
// reverse_t<tuple<Head, Tail...>> reverse_tuple(const tuple<Head, Tail...>& t)
// {
//   return push_back_tuple(reverse_tuple(t.get_tail()), t.get_head());
// }

template <typename... Types>
pop_back_t<tuple<Types...>> pop_back_tuple(const tuple<Types...>& t) {
  return reverse_tuple(pop_front_tuple(reverse_tuple(t)));
}

template <std::size_t I, std::size_t N,
          typename IndexList = std::index_sequence<>>
struct replicated_index_list;

template <std::size_t I, std::size_t... Indices>
struct replicated_index_list<I, 0, std::index_sequence<Indices...>> {
  using type = std::index_sequence<Indices...>;
};

template <std::size_t I, std::size_t N, std::size_t... Indices>
struct replicated_index_list<I, N, std::index_sequence<Indices...>>
    : replicated_index_list<I, N - 1, std::index_sequence<Indices..., I>> {};

template <std::size_t I, std::size_t N>
using replicated_index_list_t = typename replicated_index_list<I, N>::type;

template <std::size_t I, std::size_t N, typename... Types>
auto splat_tuple(const tuple<Types...>& t) {
  return select_tuple(t, replicated_index_list_t<I, N>{});
}

template <typename List, template <typename T, typename U> class F>
struct metafun_of_nth_element {
  template <typename T, typename U>
  struct Apply;

  template <std::size_t N, std::size_t M>
  struct Apply<std::integral_constant<std::size_t, M>,
               std::integral_constant<std::size_t, N>>
      : F<nth_element_t<List, M>, nth_element_t<List, N>> {};
};

template <template <typename T, typename U> class Compare, typename... Types>
auto sort_tuple(const tuple<Types...>& t) {
  return select_tuple(
      t,
      insertion_sort_t<
          std::index_sequence_for<Types...>,
          metafun_of_nth_element<tuple<Types...>, Compare>::template Apply>());
}

template <typename F, typename... Types, std::size_t... Indices>
auto apply_impl(F&& f, const tuple<Types...>& t,
                std::index_sequence<Indices...>) {
  return std::invoke(std::forward<F>(f), get<Indices>(t)...);
}

template <typename F, typename... Types>
auto apply(F&& f, const tuple<Types...>& t) {
  return apply_impl(std::forward<F>(f), t, std::index_sequence_for<Types...>{});
}

}  // namespace jc

namespace jc {  // integral_constant literal

constexpr int to_int(char c) {
  // hexadecimal letters:
  if (c >= 'A' && c <= 'F') {
    return static_cast<int>(c) - static_cast<int>('A') + 10;
  }
  if (c >= 'a' && c <= 'f') {
    return static_cast<int>(c) - static_cast<int>('a') + 10;
  }
  assert(c >= '0' && c <= '9');
  return static_cast<int>(c) - static_cast<int>('0');
}

template <std::size_t N>
constexpr int parse_int(const char (&arr)[N]) {
  int base = 10;   // to handle base (default: decimal)
  int offset = 0;  // to skip prefixes like 0x
  if (N > 2 && arr[0] == '0') {
    switch (arr[1]) {
      case 'x':  // prefix 0x or 0X, so hexadecimal
      case 'X':
        base = 16;
        offset = 2;
        break;
      case 'b':  // prefix 0b or 0B (since C++14), so binary
      case 'B':
        base = 2;
        offset = 2;
        break;
      default:  // prefix 0, so octal
        base = 8;
        offset = 1;
        break;
    }
  }
  int res = 0;
  int multiplier = 1;
  for (std::size_t i = 0; i < N - offset; ++i) {
    if (arr[N - 1 - i] != '\'') {
      res += to_int(arr[N - 1 - i]) * multiplier;
      multiplier *= base;
    }
  }
  return res;
}

template <char... cs>
constexpr auto operator"" _c() {
  return std::integral_constant<int, parse_int<sizeof...(cs)>({cs...})>{};
}

}  // namespace jc

void test_make_tuple() {
  auto t = jc::make_tuple(42, 3.14, "downdemo");
  static_assert(std::is_same_v<decltype(jc::get<0>(t)), const int&>);
  static_assert(std::is_same_v<decltype(jc::get<1>(t)), const double&>);
  static_assert(std::is_same_v<decltype(jc::get<2>(t)), const char* const&>);
  assert(jc::get<0>(t) == 42);
  assert(jc::get<1>(t) == 3.14);
  assert(std::strcmp(jc::get<2>(t), "downdemo") == 0);

  using jc::operator"" _c;
  assert((t[0_c] == 42));
  assert((t[1_c] == 3.14));
  assert((std::strcmp(t[2_c], "downdemo") == 0));

  std::ostringstream os;
  os << t;
  assert(os.str() == "(42, 3.14, downdemo)");
}

void test_typelist() {
  jc::tuple<int, double, std::string> t{42, 3.14, "downdemo"};
  static_assert(std::is_same_v<jc::front_t<decltype(t)>, int>);
  static_assert(std::is_same_v<jc::pop_front_t<decltype(t)>,
                               jc::tuple<double, std::string>>);
  static_assert(std::is_same_v<jc::push_front_t<decltype(t), bool>,
                               jc::tuple<bool, int, double, std::string>>);
  static_assert(std::is_same_v<jc::push_back_t<decltype(t), bool>,
                               jc::tuple<int, double, std::string, bool>>);
  static_assert(std::is_same_v<jc::reverse_t<decltype(t)>,
                               jc::tuple<std::string, double, int>>);

  auto t2 = jc::pop_front_tuple(t);
  static_assert(std::is_same_v<decltype(t2), jc::tuple<double, std::string>>);
  assert(jc::get<0>(t2) == 3.14);
  assert(jc::get<1>(t2) == "downdemo");

  auto t3 = jc::push_front_tuple(t, true);
  static_assert(
      std::is_same_v<decltype(t3), jc::tuple<bool, int, double, std::string>>);
  assert(jc::get<0>(t3) == true);
  assert(jc::get<1>(t3) == 42);
  assert(jc::get<2>(t3) == 3.14);
  assert(jc::get<3>(t3) == "downdemo");

  auto t4 = jc::push_back_tuple(t, true);
  static_assert(
      std::is_same_v<decltype(t4), jc::tuple<int, double, std::string, bool>>);
  assert(jc::get<0>(t4) == 42);
  assert(jc::get<1>(t4) == 3.14);
  assert(jc::get<2>(t4) == "downdemo");
  assert(jc::get<3>(t4) == true);

  auto t5 = jc::reverse_tuple(t);
  static_assert(
      std::is_same_v<decltype(t5), jc::tuple<std::string, double, int>>);
  assert(jc::get<0>(t5) == "downdemo");
  assert(jc::get<1>(t5) == 3.14);
  assert(jc::get<2>(t5) == 42);

  auto t6 = jc::pop_back_tuple(t);
  static_assert(std::is_same_v<decltype(t6), jc::tuple<int, double>>);
  assert(jc::get<0>(t6) == 42);
  assert(jc::get<1>(t6) == 3.14);

  auto t7 = jc::splat_tuple<0, 3>(t);
  static_assert(std::is_same_v<decltype(t7), jc::tuple<int, int, int>>);
  assert(jc::get<0>(t7) == 42);
  assert(jc::get<1>(t7) == 42);
  assert(jc::get<2>(t7) == 42);
}

void test_sort_tuple() {
  auto t = jc::make_tuple(17LL, std::complex<double>(1, 2), 42, 3.14);
  auto t2 = jc::sort_tuple<jc::test::smaller>(t);
  static_assert(
      std::is_same_v<decltype(t),
                     jc::tuple<long long, std::complex<double>, int, double>>);
  static_assert(
      std::is_same_v<decltype(t2),
                     jc::tuple<int, double, long long, std::complex<double>>>);
  std::ostringstream os;
  os << t;
  assert(os.str() == "(17, (1,2), 42, 3.14)");
  os.str("");
  os << t2;
  assert(os.str() == "(42, 3.14, 17, (1,2))");
}

void test_apply() {
  std::ostringstream os;
  auto f = [&os](auto&&... args) { ((os << args << " "), ...); };
  auto t = jc::make_tuple(42, 3.14, "downdemo");
  jc::apply(f, t);
  assert(os.str() == "42 3.14 downdemo ");
}

int main() {
  test_make_tuple();
  test_typelist();
  test_sort_tuple();
  test_apply();
}
```

+ 上述 tuple 的存储实现比实际需要使用了更多的空间，一个问题在于 tail 成员最终将是一个空 tuple，为此可使用 EBCO，派生自尾 tuple，而不是让它作为一个成员，这样可以消除一个字节的大小，[std::tuple](https://en.cppreference.com/w/cpp/utility/tuple) 采用了这种做法

```cpp
namespace jc {

template <typename... Types>
class tuple;

template <typename Head, typename... Tail>
class tuple<Head, Tail...> : private tuple<Tail...> {
 public:
  tuple() = default;

  tuple(const Head& head, const tuple<Tail...>& tail)
      : head_(head), TailElt(tail) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(VHead&& head, VTail&&... tail)
      : head_(std::forward<VHead>(head)),
        TailElt(std::forward<VTail>(tail)...) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(const tuple<VHead, VTail...>& rhs)
      : head_(rhs.get_head()), TailElt(rhs.get_tail()) {}

  // for push_back_tuple
  template <typename V, typename VHead, typename... VTail>
  tuple(const V& v, const tuple<VHead, VTail...>& rhs)
      : head_(v), TailElt(rhs) {}

  Head& get_head() { return head_; }

  const Head& get_head() const { return head_; }

  tuple<Tail...>& get_tail() { return *this; }

  const tuple<Tail...>& get_tail() const { return *this; }

  template <typename T, T Index>
  auto& operator[](std::integral_constant<T, Index>);

 private:
  Head head_;
  using TailElt = tuple<Tail...>;
};

template <>
class tuple<> {};

}  // namespace jc
```

+ 由于 tail 在基类中，会先于 head 成员初始化，为此引入一个包裹 head 成员的类模板 tuple_elt，将其作为基类并置于 tail 之前。由于不能继承相同的类型，为此需要给 tuple_elt 一个额外的模板参数用于区分类型，以允许 tuple 有相同类型的元素

```cpp
namespace jc {

template <std::size_t N, typename T>
class tuple_elt {
 public:
  tuple_elt() = default;

  template <typename U>
  tuple_elt(U&& rhs) : value_(std::forward<U>(rhs)) {}

  T& get() { return value_; }
  const T& get() const { return value_; }

 private:
  T value_;
};

template <typename... Types>
class tuple;

template <typename Head, typename... Tail>
class tuple<Head, Tail...> : private tuple_elt<sizeof...(Tail), Head>,
                             private tuple<Tail...> {
 public:
  tuple() = default;

  tuple(const Head& head, const tuple<Tail...>& tail)
      : HeadElt(head), TailElt(tail) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(VHead&& head, VTail&&... tail)
      : HeadElt(std::forward<VHead>(head)),
        TailElt(std::forward<VTail>(tail)...) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(const tuple<VHead, VTail...>& rhs)
      : HeadElt(rhs.get_head()), TailElt(rhs.get_tail()) {}

  // for push_back_tuple
  template <typename V, typename VHead, typename... VTail>
  tuple(const V& v, const tuple<VHead, VTail...>& rhs)
      : HeadElt(v), TailElt(rhs) {}

  Head& get_head() { return static_cast<HeadElt*>(this)->get(); }

  const Head& get_head() const {
    return static_cast<const HeadElt*>(this)->get();
  }

  tuple<Tail...>& get_tail() { return *this; }

  const tuple<Tail...>& get_tail() const { return *this; }

  template <typename T, T Index>
  auto& operator[](std::integral_constant<T, Index>);

 private:
  using HeadElt = tuple_elt<sizeof...(Tail), Head>;
  using TailElt = tuple<Tail...>;
};

template <>
class tuple<> {};

}  // namespace jc
```

+ 让 tuple_elt 继承元素类型以进一步使用 EBCO

```cpp
namespace jc {

template <std::size_t N, typename T,
          bool = std::is_class_v<T> && !std::is_final_v<T>>
class tuple_elt;

template <std::size_t N, typename T>
class tuple_elt<N, T, false> {
 public:
  tuple_elt() = default;

  template <typename U>
  tuple_elt(U&& rhs) : value_(std::forward<U>(rhs)) {}

  T& get() { return value_; }

  const T& get() const { return value_; }

 private:
  T value_;
};

template <std::size_t N, typename T>
class tuple_elt<N, T, true> : private T {
 public:
  tuple_elt() = default;

  template <typename U>
  tuple_elt(U&& rhs) : T(std::forward<U>(rhs)) {}

  T& get() { return *this; }

  const T& get() const { return *this; }
};

template <typename... Types>
class tuple;

template <typename Head, typename... Tail>
class tuple<Head, Tail...> : private tuple_elt<sizeof...(Tail), Head>,
                             private tuple<Tail...> {
 public:
  tuple() = default;

  tuple(const Head& head, const tuple<Tail...>& tail)
      : HeadElt(head), TailElt(tail) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(VHead&& head, VTail&&... tail)
      : HeadElt(std::forward<VHead>(head)),
        TailElt(std::forward<VTail>(tail)...) {}

  template <
      typename VHead, typename... VTail,
      std::enable_if_t<sizeof...(VTail) == sizeof...(Tail), void*> = nullptr>
  tuple(const tuple<VHead, VTail...>& rhs)
      : HeadElt(rhs.get_head()), TailElt(rhs.get_tail()) {}

  // for push_back_tuple
  template <typename V, typename VHead, typename... VTail>
  tuple(const V& v, const tuple<VHead, VTail...>& rhs)
      : HeadElt(v), TailElt(rhs) {}

  Head& get_head() { return static_cast<HeadElt*>(this)->get(); }

  const Head& get_head() const {
    return static_cast<const HeadElt*>(this)->get();
  }

  tuple<Tail...>& get_tail() { return *this; }

  const tuple<Tail...>& get_tail() const { return *this; }

  template <typename T, T Index>
  auto& operator[](std::integral_constant<T, Index>);

 private:
  using HeadElt = tuple_elt<sizeof...(Tail), Head>;
  using TailElt = tuple<Tail...>;
};

template <>
class tuple<> {};

}  // namespace jc
```

## [std::variant](https://en.cppreference.com/w/cpp/utility/variant)

```cpp
#include <cassert>
#include <exception>
#include <new>  // for std::launder()
#include <sstream>
#include <string>
#include <type_traits>
#include <utility>

#include "typelist.hpp"

namespace jc {

class computed_result_type;

template <typename Visitor, typename T>
using visit_element_result =
    decltype(std::declval<Visitor>()(std::declval<T>()));

template <typename R, typename Visitor, typename... Types>
struct visit_result {
  using type = R;
};

template <typename Visitor, typename... Types>
struct visit_result<computed_result_type, Visitor, Types...> {
  using type = std::common_type_t<visit_element_result<Visitor, Types>...>;
};

template <typename R, typename Visitor, typename... Types>
using visit_result_t = typename visit_result<R, Visitor, Types...>::type;

struct empty_variant : std::exception {};

template <typename R, typename V, typename Visitor, typename Head,
          typename... Tail>
R variant_visit_impl(V&& variant, Visitor&& vis, typelist<Head, Tail...>) {
  if (variant.template is<Head>()) {
    return static_cast<R>(std::forward<Visitor>(vis)(
        std::forward<V>(variant).template get<Head>()));
  } else if constexpr (sizeof...(Tail) > 0) {
    return variant_visit_impl<R>(std::forward<V>(variant),
                                 std::forward<Visitor>(vis),
                                 typelist<Tail...>{});
  } else {
    throw empty_variant();
  }
}

template <typename... Types>
class variant_storage {
 public:
  unsigned char get_discriminator() const { return discriminator_; }

  void set_discriminator(unsigned char d) { discriminator_ = d; }

  void* get_raw_buffer() { return buffer_; }

  const void* get_raw_buffer() const { return buffer_; }

  template <typename T>
  T* get_buffer_as() {
    return std::launder(reinterpret_cast<T*>(buffer_));
  }

  template <typename T>
  const T* get_buffer_as() const {
    return std::launder(reinterpret_cast<const T*>(buffer_));
  }

 private:
  using largest_t = largest_type_t<typelist<Types...>>;
  alignas(Types...) unsigned char buffer_[sizeof(largest_t)];
  unsigned char discriminator_ = 0;
};

template <typename... Types>
class variant;

template <typename T, typename... Types>
class variant_choice {
  using Derived = variant<Types...>;

  Derived& get_derived() { return *static_cast<Derived*>(this); }

  const Derived& get_derived() const {
    return *static_cast<const Derived*>(this);
  }

 protected:
  static constexpr unsigned Discriminator =
      find_index_of_t<typelist<Types...>, T>::value + 1;

 public:
  variant_choice() = default;

  variant_choice(const T& value) {
    new (get_derived().get_raw_buffer()) T(value);  // CRTP
    get_derived().set_discriminator(Discriminator);
  }

  variant_choice(T&& value) {
    new (get_derived().get_raw_buffer()) T(std::move(value));
    get_derived().set_discriminator(Discriminator);
  }

  bool destroy() {
    if (get_derived().get_discriminator() == Discriminator) {
      get_derived().template get_buffer_as<T>()->~T();
      return true;
    }
    return false;
  }

  Derived& operator=(const T& value) {
    if (get_derived().get_discriminator() == Discriminator) {
      *get_derived().template get_buffer_as<T>() = value;
    } else {
      get_derived().destroy();
      new (get_derived().get_raw_buffer()) T(value);
      get_derived().set_discriminator(Discriminator);
    }
    return get_derived();
  }

  Derived& operator=(T&& value) {
    if (get_derived().get_discriminator() == Discriminator) {
      *get_derived().template get_buffer_as<T>() = std::move(value);
    } else {
      get_derived().destroy();
      new (get_derived().get_raw_buffer()) T(std::move(value));
      get_derived().set_discriminator(Discriminator);
    }
    return get_derived();
  }
};

/*
 * class variant<int, double, std::string>
 *     : private variant_storage<int, double, std::string>,
 *       private variant_choice<int, int, double, std::string>,
 *       private variant_choice<double, int, double, std::string>,
 *       private variant_choice<std::string, int, double, std::string> {};
 *
 * variant_choice<int, int, double, std::string>::discriminator_ == 1;
 * variant_choice<double, int, double, std::string>::discriminator_ == 2;
 * variant_choice<std::string, int, double, std::string>::discriminator_ == 3;
 */
template <typename... Types>
class variant : private variant_storage<Types...>,
                private variant_choice<Types, Types...>... {
  template <typename T, typename... OtherTypes>
  friend class variant_choice;  // enable CRTP

 public:
  /*
   * ctor of variant<int, double, string>:
   * variant(const int&);
   * variant(int&&);
   * variant(const double&);
   * variant(double&&);
   * variant(const string&);
   * variant(string&&);
   */
  using variant_choice<Types, Types...>::variant_choice...;
  using variant_choice<Types, Types...>::operator=...;

  variant() { *this = front_t<typelist<Types...>>(); }

  variant(const variant& rhs) {
    if (!rhs.empty()) {
      rhs.visit([&](const auto& value) { *this = value; });
    }
  }

  variant(variant&& rhs) {
    if (!rhs.empty()) {
      std::move(rhs).visit([&](auto&& value) { *this = std::move(value); });
    }
  }

  template <typename... SourceTypes>
  variant(const variant<SourceTypes...>& rhs) {
    if (!rhs.empty()) {
      rhs.visit([&](const auto& value) { *this = value; });
    }
  }

  template <typename... SourceTypes>
  variant(variant<SourceTypes...>&& rhs) {
    if (!rhs.empty()) {
      std::move(rhs).visit([&](auto&& value) { *this = std::move(value); });
    }
  }

  variant& operator=(const variant& rhs) {
    if (!rhs.empty()) {
      rhs.visit([&](const auto& value) { *this = value; });
    } else {
      destroy();
    }
    return *this;
  }

  variant& operator=(variant&& rhs) {
    if (!rhs.empty()) {
      std::move(rhs).visit([&](auto&& value) { *this = std::move(value); });
    } else {
      destroy();
    }
    return *this;
  }

  template <typename... SourceTypes>
  variant& operator=(const variant<SourceTypes...>& rhs) {
    if (!rhs.empty()) {
      rhs.visit([&](const auto& value) { *this = value; });
    } else {
      destroy();
    }
    return *this;
  }

  template <typename... SourceTypes>
  variant& operator=(variant<SourceTypes...>&& rhs) {
    if (!rhs.empty()) {
      std::move(rhs).visit([&](auto&& value) { *this = std::move(value); });
    } else {
      destroy();
    }
    return *this;
  }

  bool empty() const { return this->get_discriminator() == 0; }

  ~variant() { destroy(); }

  void destroy() {
    (variant_choice<Types, Types...>::destroy(), ...);
    this->set_discriminator(0);
  }

  template <typename T>
  bool is() const {
    return this->get_discriminator() ==
           variant_choice<T, Types...>::Discriminator;
  }

  template <typename T>
  T& get() & {
    if (empty()) {
      throw empty_variant();
    }
    assert(is<T>());
    return *this->template get_buffer_as<T>();
  }

  template <typename T>
  const T& get() const& {
    if (empty()) {
      throw empty_variant();
    }
    assert(is<T>());
    return *this->template get_buffer_as<T>();
  }

  template <typename T>
  T&& get() && {
    if (empty()) {
      throw empty_variant();
    }
    assert(is<T>());
    return std::move(*this->template get_buffer_as<T>());
  }

  template <typename R = computed_result_type, typename Visitor>
  visit_result_t<R, Visitor, Types&...> visit(Visitor&& vis) & {
    using Result = visit_result_t<R, Visitor, Types&...>;
    return variant_visit_impl<Result>(*this, std::forward<Visitor>(vis),
                                      typelist<Types...>{});
  }

  template <typename R = computed_result_type, typename Visitor>
  visit_result_t<R, Visitor, const Types&...> visit(Visitor&& vis) const& {
    using Result = visit_result_t<R, Visitor, const Types&...>;
    return variant_visit_impl<Result>(*this, std::forward<Visitor>(vis),
                                      typelist<Types...>{});
  }

  template <typename R = computed_result_type, typename Visitor>
  visit_result_t<R, Visitor, Types&&...> visit(Visitor&& vis) && {
    using Result = visit_result_t<R, Visitor, Types&&...>;
    return variant_visit_impl<Result>(
        std::move(*this), std::forward<Visitor>(vis), typelist<Types...>{});
  }
};

}  // namespace jc

namespace jc::test {

struct copied_noncopyable : std::exception {};

struct noncopyable {
  noncopyable() = default;

  noncopyable(const noncopyable&) { throw copied_noncopyable(); }

  noncopyable(noncopyable&&) = default;

  noncopyable& operator=(const noncopyable&) { throw copied_noncopyable(); }

  noncopyable& operator=(noncopyable&&) = default;
};

template <typename V, typename Head, typename... Tail>
void print_impl(std::ostream& os, const V& v) {
  if (v.template is<Head>()) {
    os << v.template get<Head>();
  } else if constexpr (sizeof...(Tail) > 0) {
    print_impl<V, Tail...>(os, v);
  }
}

template <typename... Types>
void print(std::ostream& os, const variant<Types...>& v) {
  print_impl<variant<Types...>, Types...>(os, v);
}

}  // namespace jc::test

void test_variant() {
  jc::variant<int, double, std::string> v{42};
  assert(!v.empty());
  assert(v.is<int>());
  assert(v.get<int>() == 42);
  v = 3.14;
  assert(v.is<double>());
  assert(v.get<double>() == 3.14);
  v = "hello";
  assert(v.is<std::string>());
  assert(v.get<std::string>() == "hello");

  std::stringstream os;
  v.visit([&os](const auto& value) { os << value; });
  assert(os.str() == "hello");

  os.str("");
  jc::test::print(os, v);
  assert(os.str() == "hello");

  jc::variant<int, double, std::string> v2;
  assert(!v2.empty());
  assert(v2.is<int>());
  v2 = std::move(v);
  assert(v.is<std::string>());
  assert(v.get<std::string>().empty());
  assert(v2.is<std::string>());
  assert(v2.get<std::string>() == "hello");
  v2.destroy();
  assert(v2.empty());
}

void test_noncopyable() {
  jc::variant<int, jc::test::noncopyable> v(42);
  try {
    jc::test::noncopyable nc;
    v = nc;
  } catch (jc::test::copied_noncopyable) {
    assert(!v.is<int>() && !v.is<jc::test::noncopyable>());
  }
}

int main() {
  test_variant();
  test_noncopyable();
}
```

## 表达式模板（Expression Template）

+ 表达式模板支持对数组像内置类型一样进行数值运算，并且不会产生临时对象

```cpp
#include <cassert>
#include <cstddef>
#include <type_traits>

namespace jc {

template <typename T>
class SArray {
 public:
  explicit SArray(std::size_t sz) : data_(new T[sz]), sz_(sz) { init(); }

  SArray(const SArray<T>& rhs) : data_(new T[rhs.sz_]), sz_(rhs.sz_) {
    copy(rhs);
  }

  SArray<T>& operator=(const SArray<T>& rhs) {
    if (&rhs != this) {
      copy(rhs);
    }
    return *this;
  }

  ~SArray() { delete[] data_; }

  std::size_t size() const { return sz_; }

  T& operator[](std::size_t i) { return data_[i]; }

  const T& operator[](std::size_t i) const { return data_[i]; }

  SArray<T>& operator+=(const SArray<T>& rhs) {
    assert(sz_ == rhs.sz_);
    for (std::size_t i = 0; i < sz_; ++i) {
      (*this)[i] += rhs[i];
    }
    return *this;
  }

  SArray<T>& operator*=(const SArray<T>& rhs) {
    assert(sz_ == rhs.sz_);
    for (std::size_t i = 0; i < sz_; ++i) {
      (*this)[i] *= rhs[i];
    }
    return *this;
  }

  SArray<T>& operator*=(const T& rhs) {
    for (std::size_t i = 0; i < sz_; ++i) {
      (*this)[i] *= rhs;
    }
    return *this;
  }

 protected:
  void init() {
    for (std::size_t i = 0; i < sz_; ++i) {
      data_[i] = T{};
    }
  }

  void copy(const SArray<T>& rhs) {
    assert(sz_ == rhs.sz_);
    for (std::size_t i = 0; i < sz_; ++i) {
      data_[i] = rhs.data_[i];
    }
  }

 private:
  T* data_;
  std::size_t sz_;
};

template <typename T>
SArray<T> operator+(const SArray<T>& lhs, const SArray<T>& rhs) {
  assert(lhs.size() == rhs.size());
  SArray<T> res{lhs.size()};
  for (std::size_t i = 0; i < lhs.size(); ++i) {
    res[i] = lhs[i] + rhs[i];
  }
  return res;
}

template <typename T>
SArray<T> operator*(const SArray<T>& lhs, const SArray<T>& rhs) {
  assert(lhs.size() == rhs.size());
  SArray<T> res{lhs.size()};
  for (std::size_t i = 0; i < lhs.size(); ++i) {
    res[i] = lhs[i] * rhs[i];
  }
  return res;
}

template <typename T>
SArray<T> operator*(const T& lhs, const SArray<T>& rhs) {
  SArray<T> res{rhs.size()};
  for (std::size_t i = 0; i < rhs.size(); ++i) {
    res[i] = lhs * rhs[i];
  }
  return res;
}

template <typename T>
class A_Scalar {
 public:
  constexpr A_Scalar(const T& v) : value_(v) {}

  constexpr const T& operator[](std::size_t) const { return value_; }

  constexpr std::size_t size() const { return 0; };

 private:
  const T& value_;
};

template <typename T>
struct A_Traits {
  using type = const T&;
};

template <typename T>
struct A_Traits<A_Scalar<T>> {
  using type = A_Scalar<T>;
};

template <typename T, typename OP1, typename OP2>
class A_Add {
 public:
  A_Add(const OP1& op1, const OP2& op2) : op1_(op1), op2_(op2) {}

  T operator[](std::size_t i) const { return op1_[i] + op2_[i]; }

  std::size_t size() const {
    assert(op1_.size() == 0 || op2_.size() == 0 || op1_.size() == op2_.size());
    return op1_.size() != 0 ? op1_.size() : op2_.size();
  }

 private:
  typename A_Traits<OP1>::type op1_;
  typename A_Traits<OP2>::type op2_;
};

template <typename T, typename OP1, typename OP2>
class A_Mult {
 public:
  A_Mult(const OP1& op1, const OP2& op2) : op1_(op1), op2_(op2) {}

  T operator[](std::size_t i) const { return op1_[i] * op2_[i]; }

  std::size_t size() const {
    assert(op1_.size() == 0 || op2_.size() == 0 || op1_.size() == op2_.size());
    return op1_.size() != 0 ? op1_.size() : op2_.size();
  }

 private:
  typename A_Traits<OP1>::type op1_;
  typename A_Traits<OP2>::type op2_;
};

template <typename T, typename A1, typename A2>
class A_Subscript {
 public:
  A_Subscript(const A1& a1, const A2& a2) : a1_(a1), a2_(a2) {}

  T& operator[](std::size_t i) {
    return const_cast<T&>(a1_[static_cast<std::size_t>(a2_[i])]);
  }

  decltype(auto) operator[](std::size_t i) const {
    return a1_[static_cast<std::size_t>(a2_[i])];
  }

  std::size_t size() const { return a2_.size(); }

 private:
  const A1& a1_;
  const A2& a2_;
};

}  // namespace jc

namespace jc::test {

template <typename T, typename Rep = SArray<T>>
class Array {
 public:
  explicit Array(std::size_t i) : r_(i) {}

  Array(const Rep& rhs) : r_(rhs) {}

  Array& operator=(const Array& rhs) {
    assert(size() == rhs.size());
    for (std::size_t i = 0; i < rhs.size(); ++i) {
      r_[i] = rhs[i];
    }
    return *this;
  }

  template <typename T2, typename Rep2>
  Array& operator=(const Array<T2, Rep2>& rhs) {
    assert(size() == rhs.size());
    for (std::size_t i = 0; i < rhs.size(); ++i) {
      r_[i] = rhs[i];
    }
    return *this;
  }

  std::size_t size() const { return r_.size(); }

  T& operator[](std::size_t i) {
    assert(i < size());
    return r_[i];
  }

  decltype(auto) operator[](std::size_t i) const {
    assert(i < size());
    return r_[i];
  }

  template <typename T2, typename Rep2>
  Array<T, A_Subscript<T, Rep, Rep2>> operator[](const Array<T2, Rep2>& rhs) {
    return Array<T, A_Subscript<T, Rep, Rep2>>{
        A_Subscript<T, Rep, Rep2>{this->rep(), rhs.rep()}};
  }

  template <typename T2, typename Rep2>
  decltype(auto) operator[](const Array<T2, Rep2>& rhs) const {
    return Array<T, A_Subscript<T, Rep, Rep2>>{
        A_Subscript<T, Rep, Rep2>{this->rep(), rhs.rep()}};
  }

  Rep& rep() { return r_; }

  const Rep& rep() const { return r_; }

 private:
  Rep r_;
};

template <typename T, typename R1, typename R2>
Array<T, A_Add<T, R1, R2>> operator+(const Array<T, R1>& lhs,
                                     const Array<T, R2>& rhs) {
  return Array<T, A_Add<T, R1, R2>>{A_Add<T, R1, R2>{lhs.rep(), rhs.rep()}};
}

template <typename T, typename R1, typename R2>
Array<T, A_Mult<T, R1, R2>> operator*(const Array<T, R1>& lhs,
                                      const Array<T, R2>& rhs) {
  return Array<T, A_Mult<T, R1, R2>>{A_Mult<T, R1, R2>{lhs.rep(), rhs.rep()}};
}

template <typename T, typename R2>
Array<T, A_Mult<T, A_Scalar<T>, R2>> operator*(const T& lhs,
                                               const Array<T, R2>& rhs) {
  return Array<T, A_Mult<T, A_Scalar<T>, R2>>{
      A_Mult<T, A_Scalar<T>, R2>{A_Scalar<T>(lhs), rhs.rep()}};
}

}  // namespace jc::test

int main() {
  constexpr std::size_t sz = 1000;
  constexpr double a = 10;
  constexpr double b = 2;
  jc::test::Array<double> x{sz};
  jc::test::Array<double> y{sz};
  assert(x.size() == sz);
  assert(y.size() == sz);
  for (std::size_t i = 0; i < sz; ++i) {
    x[i] = a;
    y[i] = b;
  }
  x = 1.2 * x + x * y;
  static_assert(std::is_same_v<
                decltype(1.2 * x),
                jc::test::Array<double, jc::A_Mult<double, jc::A_Scalar<double>,
                                                   jc::SArray<double>>>>);
  static_assert(std::is_same_v<
                decltype(x * y),
                jc::test::Array<double, jc::A_Mult<double, jc::SArray<double>,
                                                   jc::SArray<double>>>>);

  static_assert(
      std::is_same_v<
          decltype(1.2 * x + x * y),
          jc::test::Array<double,
                          jc::A_Add<double,
                                    jc::A_Mult<double, jc::A_Scalar<double>,
                                               jc::SArray<double>>,
                                    jc::A_Mult<double, jc::SArray<double>,
                                               jc::SArray<double>>>>>);

  for (std::size_t i = 0; i < sz; ++i) {
    assert(x[i] == 1.2 * a + a * b);
    y[i] = static_cast<double>(i);
  }

  /*
   * x[y] = 2.0 * x[y] equals to:
   * for (std::size_t i = 0; i < y.size(); ++i) {
   *   x[y[i]] = 2 * x[y[i]];
   * }
   */
  x[y] = 2.0 * x[y];
  for (std::size_t i = 0; i < sz; ++i) {
    assert(x[i] == 2.0 * (1.2 * a + a * b));
  }
}
```

## 性能与约束

+ 表达式模板可以提高数组操作性能，跟踪其行为可以发现很多小的内联函数互相调用，调用堆栈分配了很多小的表达式模板对象，因此编译器必须执行完整的内联和去除小对象操作，以产生性能上和手写循环媲美的代码
+ 表达式模板没有解决所有数组数值运算的问题，如对 `x = A * x` 的运算，A 是 `n * n` 矩阵，x 是 n 个元素的 vector，临时变量的使用不可避免，因为最终结果的每个元素都依赖于 x 每个元素的初始值，而表达式模板会在一次计算后更新 x 的元素，计算下一个元素时用到已更新的元素就改变了原数组，但针对 `x = A * y`，如果 x 和 y 不互为别名，就不需要临时对象，因此必须在运行期知道操作数是否为别名关系，即必须生成运行期结构来表示表达式树，而不是在表达式模板的类型中编码这棵树

## 浅实例化（Shallow Instantiation）

+ 模板的报错会跟踪导致问题的所有层次，冗长的报错信息使调试变得更为繁琐，真正的问题一般出现在一长串实例化之后

```cpp
template <typename T>
void f1(T& i) {
  *i = 0;  // 假设 T 为指针类型
}

template <typename T>
void f2(T& i) {
  f1(i);
}

template <typename T>
void f3(typename T::Type i) {
  f2(i);
}

template <typename T>
void f4(const T&) {
  typename T::Type i = 42;
  f3<T>(i);
}

struct A {
  using Type = int;
};

int main() {
  f4(A{});  // 错误，只能在实例化时被检测到
            // 实例化 f4<A>(const A&)
            // 实例化 f3<A>(int)
            // 实例化 f2<int>(int&)
            // 实例化 f1<int>(int&)，解引用 int 出错
}

/*
 * error C2100: 非法的间接寻址
 * message : 查看对正在编译的函数 模板 实例化“void f1<T>(T &)”的引用
 *         with
 *         [
 *             T=A::Type
 *         ]
 * message : 查看对正在编译的函数 模板 实例化“void f2<A::Type>(T &)”的引用
 *         with
 *         [
 *             T=A::Type
 *         ]
 * message : 查看对正在编译的函数 模板 实例化“void f3<T>(A::Type)”的引用
 *         with
 *         [
 *             T=A
 *         ]
 * message : 查看对正在编译的函数 模板 实例化“void f4<A>(const T &)”的引用
 *         with
 *         [
 *             T=A
 *         ]
 */
```

+ 一种简单的减少报错信息长度的方式是提前使用参数

```cpp
template <typename T>
void f1(T& i) {
  *i = 0;  // 假设 T 为指针类型
}

template <typename T>
void f2(T& i) {
  f1(i);
}

template <typename T>
void f3(typename T::Type i) {
  f2(i);
}

template <typename T>
void f4(const T&) {
  class ShallowChecks {  // 未调用，不影响运行期
    static void deref(typename T::Type p) { *p; }
  };
  typename T::Type i = 42;
  f3<T>(i);
}

struct A {
  using Type = int;
};

int main() {
  f4(A{});  // 实例化 f4<A>(const A&) 时检测到错误
}

/*
 * error C2100: 非法的间接寻址
 * message : 查看对正在编译的函数 模板 实例化“void f4<A>(const T &)”的引用
 *         with
 *         [
 *             T=A
 *         ]
 */
```

## 静态断言（Static Assertion）

+ C++11 引入了[static_assert](https://en.cppreference.com/w/cpp/language/static_assert)，在编译期进行断言，比如下列静态断言确保编译平台带 64 位指针

```cpp
static_assert(sizeof(void*) * CHAR_BIT == 64, "Not a 64-bit platform");
```

+ 创建一个检查解引用的 traits，用 [static_assert](https://en.cppreference.com/w/cpp/language/static_assert) 提供更明确的诊断信息

```cpp
#include <type_traits>

template <typename T>
class has_dereference {
 private:
  template <typename U>
  struct Identity;

  template <typename U>
  static std::true_type test(Identity<decltype(*std::declval<U>())>*);

  template <typename U>
  static std::false_type test(...);

 public:
  static constexpr bool value = decltype(test<T>(nullptr))::value;
};

template <typename T>
inline constexpr bool has_dereference_v = has_dereference<T>::value;

template <typename T>
void f(T& i) {
  static_assert(has_dereference_v<T>, "T is not dereferenceable");
  *i = 0;
}

int main() {
  int i = 42;
  f(i);  // static_assert 报错：T is not dereferenceable
}
```

+ C++17 可以用 [std::void_t](https://en.cppreference.com/w/cpp/types/void_t) 简化 traits 的实现

```cpp
#include <type_traits>

template <typename, typename = std::void_t<>>
struct has_dereference : std::false_type {};

template <typename T>
struct has_dereference<T, std::void_t<decltype(*std::declval<T>())>>
    : std::true_type {};

template <typename T>
inline constexpr bool has_dereference_v = has_dereference<T>::value;
```

## [Concepts](https://en.cppreference.com/w/cpp/concepts)

+ C++20 可以用 concepts 约束类型，代码更简洁

```cpp
template <typename T>
concept Dereferenceable = requires(T x) {
  *x;
};

template <typename T>
  requires Dereferenceable<T>
void f(T& i) {
  *i = 0;
}

/* 等价写法
 * template <typename T>
 *   requires requires(T x) { *x; }
 * void f(T& i) {
 *   *i = 0;
 * }
 */

int main() {
  int i = 42;
  f(i);  // 未满足关联约束
}
```

## 原型（Archetype）

+ 模板的一个挑战是确保满足特定约束的实参都能通过编译，为了测试满足要求的模板参数，引入原型的概念。原型是用户定义的类，以尽可能小的方式来满足模板大多数要求，而不提供任何外来的操作

```cpp
// 要求 T 是可比较类型
template <typename T>
int find(const T* a, int n, const T& v) {
  int i = 0;
  while (i != n && a[i] != v) {
    ++i;
  }
  return i;
}

struct EqualityComparable {};

struct ConvertibleToBool {
  operator bool() const { return true; }  // 提供本类型到 bool 的隐式转换
};

ConvertibleToBool  // 返回类型要求能转换为 bool
operator==(const EqualityComparable&, const EqualityComparable&) {
  return ConvertibleToBool{};
}

// 实例化 find<EqualityComparable>
template int find(const EqualityComparable*, int, const EqualityComparable&);

int main() {}
```

+ 实例化将失败，改用 `operator==` 比较即可解决此问题

```cpp
template <typename T>
int find(const T* a, int n, const T& v) {
  int i = 0;
  while (i != n && !(a[i] == v)) {
    ++i;
  }
  return i;
}
```

+ 但这又在无意中对结果使用了 `operator!`，如果要发现这点，在 ConvertibleToBool 中禁用 `operator!` 即可，当其被使用时将报错

```cpp
struct ConvertibleToBool {
  operator bool() const { return true; }
  bool operator!() = delete;
};
```

+ 可以再对原型做其他扩展，比如禁用 `operator&&` 和 `operator||` 来找出其他的一些模板定义中的问题

## 跟踪程序（Tracer）

+ 以上都是编译或链接时的 bug，更大的挑战是确保程序在运行期表现正确
+ Tracer 是一个用户定义的类，它能用作要测试的模板的实参。通常 tracer 也是一个原型，但包含一些额外的信息。下面是一个用于测试 [std::sort](https://en.cppreference.com/w/cpp/algorithm/sort) 的 tracer，它提供 [std::sort](https://en.cppreference.com/w/cpp/algorithm/sort) 需要的功能（比如 `operator==` 和 `operator>`），并给出算法开销的直观结果，但不揭示排序模板的正确性

```cpp
#include <algorithm>
#include <iostream>

class SortTracer {
 public:
  static long creations() { return n_created; }
  static long destructions() { return n_destroyed; }
  static long assignments() { return n_assigned; }
  static long comparisons() { return n_compared; }
  static long max_live() { return n_max_live; }

 public:
  SortTracer(int v = 0) : value(v), generation(1) {
    ++n_created;
    update_max_live();
    std::cerr << "SortTracer #" << n_created << ", created generation "
              << generation << " (total: " << n_created - n_destroyed << ")\n";
  }

  SortTracer(const SortTracer& rhs)
      : value(rhs.value), generation(rhs.generation + 1) {
    ++n_created;
    update_max_live();
    std::cerr << "SortTracer #" << n_created << ", copied as generation "
              << generation << " (total: " << n_created - n_destroyed << ")\n";
  }

  ~SortTracer() {
    ++n_destroyed;
    update_max_live();
    std::cerr << "SortTracer generation " << generation
              << " destroyed (total: " << n_created - n_destroyed << ")\n";
  }

  SortTracer& operator=(const SortTracer& rhs) {
    ++n_assigned;
    std::cerr << "SortTracer assignment #" << n_assigned << " (generation "
              << generation << " = " << rhs.generation << ")\n";
    value = rhs.value;
    return *this;
  }

  friend bool operator<(const SortTracer& lhs, const SortTracer& rhs) {
    ++n_compared;
    std::cerr << "SortTracer comparison #" << n_compared << " (generation "
              << lhs.generation << " < " << rhs.generation << ")\n";
    return lhs.value < rhs.value;
  }

  int val() const { return value; }

 private:
  int value;                           // integer value to be sorted
  int generation;                      // generation of this tracer
  inline static long n_created = 0;    // number of constructor calls
  inline static long n_destroyed = 0;  // number of destructor calls
  inline static long n_assigned = 0;   // number of assignments
  inline static long n_compared = 0;   // number of comparisons
  inline static long n_max_live = 0;   // maximum of existing objects

  // recompute maximum of existing objects
  static void update_max_live() {
    if (n_created - n_destroyed > n_max_live) {
      n_max_live = n_created - n_destroyed;
    }
  }
};

int main() {
  SortTracer input[] = {7, 3, 5, 6, 4, 2, 0, 1, 9, 8};

  // 打印初始值
  for (int i = 0; i < 10; ++i) {
    std::cerr << input[i].val() << ' ';
  }
  std::cerr << '\n';

  // 记录初始条件
  long created_at_start = SortTracer::creations();
  long max_live_at_start = SortTracer::max_live();
  long assigned_at_start = SortTracer::assignments();
  long compared_at_start = SortTracer::comparisons();

  // 执行
  std::cerr << "---[ Start std::sort() ]--------------------\n";
  std::sort<>(&input[0], &input[9] + 1);
  std::cerr << "---[ End std::sort() ]----------------------\n";

  // 检查结果
  for (int i = 0; i < 10; ++i) {
    std::cerr << input[i].val() << ' ';
  }
  std::cerr << "\n\n";

  // final report
  std::cerr << "std::sort() of 10 SortTracer's was performed by:\n"
            << SortTracer::creations() - created_at_start
            << " temporary tracers\n"
            << "up to " << SortTracer::max_live()
            << " tracers at the same time (" << max_live_at_start
            << " before)\n"
            << SortTracer::assignments() - assigned_at_start << " assignments\n"
            << SortTracer::comparisons() - compared_at_start
            << " comparisons\n\n";
}

/*
 * SortTracer #1, created generation 1 (total: 1)
 * SortTracer #2, created generation 1 (total: 2)
 * SortTracer #3, created generation 1 (total: 3)
 * SortTracer #4, created generation 1 (total: 4)
 * SortTracer #5, created generation 1 (total: 5)
 * SortTracer #6, created generation 1 (total: 6)
 * SortTracer #7, created generation 1 (total: 7)
 * SortTracer #8, created generation 1 (total: 8)
 * SortTracer #9, created generation 1 (total: 9)
 * SortTracer #10, created generation 1 (total: 10)
 * 7 3 5 6 4 2 0 1 9 8
 * ---[ Start std::sort() ]--------------------
 * SortTracer #11, copied as generation 2 (total: 11)
 * SortTracer comparison #1 (generation 2 < 1)
 * SortTracer comparison #2 (generation 1 < 2)
 * SortTracer assignment #1 (generation 1 = 1)
 * SortTracer assignment #2 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #12, copied as generation 2 (total: 11)
 * SortTracer comparison #3 (generation 2 < 1)
 * SortTracer comparison #4 (generation 2 < 1)
 * SortTracer comparison #5 (generation 1 < 2)
 * SortTracer assignment #3 (generation 1 = 1)
 * SortTracer comparison #6 (generation 2 < 1)
 * SortTracer assignment #4 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #13, copied as generation 2 (total: 11)
 * SortTracer comparison #7 (generation 2 < 1)
 * SortTracer comparison #8 (generation 2 < 1)
 * SortTracer comparison #9 (generation 1 < 2)
 * SortTracer assignment #5 (generation 1 = 1)
 * SortTracer comparison #10 (generation 2 < 1)
 * SortTracer assignment #6 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #14, copied as generation 2 (total: 11)
 * SortTracer comparison #11 (generation 2 < 1)
 * SortTracer comparison #12 (generation 2 < 1)
 * SortTracer comparison #13 (generation 1 < 2)
 * SortTracer assignment #7 (generation 1 = 1)
 * SortTracer comparison #14 (generation 2 < 1)
 * SortTracer comparison #15 (generation 1 < 2)
 * SortTracer assignment #8 (generation 1 = 1)
 * SortTracer comparison #16 (generation 2 < 1)
 * SortTracer comparison #17 (generation 1 < 2)
 * SortTracer assignment #9 (generation 1 = 1)
 * SortTracer comparison #18 (generation 2 < 1)
 * SortTracer assignment #10 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #15, copied as generation 2 (total: 11)
 * SortTracer comparison #19 (generation 2 < 1)
 * SortTracer comparison #20 (generation 1 < 2)
 * SortTracer assignment #11 (generation 1 = 1)
 * SortTracer assignment #12 (generation 1 = 1)
 * SortTracer assignment #13 (generation 1 = 1)
 * SortTracer assignment #14 (generation 1 = 1)
 * SortTracer assignment #15 (generation 1 = 1)
 * SortTracer assignment #16 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #16, copied as generation 2 (total: 11)
 * SortTracer comparison #21 (generation 2 < 1)
 * SortTracer comparison #22 (generation 1 < 2)
 * SortTracer assignment #17 (generation 1 = 1)
 * SortTracer assignment #18 (generation 1 = 1)
 * SortTracer assignment #19 (generation 1 = 1)
 * SortTracer assignment #20 (generation 1 = 1)
 * SortTracer assignment #21 (generation 1 = 1)
 * SortTracer assignment #22 (generation 1 = 1)
 * SortTracer assignment #23 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #17, copied as generation 2 (total: 11)
 * SortTracer comparison #23 (generation 2 < 1)
 * SortTracer comparison #24 (generation 2 < 1)
 * SortTracer comparison #25 (generation 1 < 2)
 * SortTracer assignment #24 (generation 1 = 1)
 * SortTracer comparison #26 (generation 2 < 1)
 * SortTracer comparison #27 (generation 1 < 2)
 * SortTracer assignment #25 (generation 1 = 1)
 * SortTracer comparison #28 (generation 2 < 1)
 * SortTracer comparison #29 (generation 1 < 2)
 * SortTracer assignment #26 (generation 1 = 1)
 * SortTracer comparison #30 (generation 2 < 1)
 * SortTracer comparison #31 (generation 1 < 2)
 * SortTracer assignment #27 (generation 1 = 1)
 * SortTracer comparison #32 (generation 2 < 1)
 * SortTracer comparison #33 (generation 1 < 2)
 * SortTracer assignment #28 (generation 1 = 1)
 * SortTracer comparison #34 (generation 2 < 1)
 * SortTracer comparison #35 (generation 1 < 2)
 * SortTracer assignment #29 (generation 1 = 1)
 * SortTracer comparison #36 (generation 2 < 1)
 * SortTracer assignment #30 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #18, copied as generation 2 (total: 11)
 * SortTracer comparison #37 (generation 2 < 1)
 * SortTracer comparison #38 (generation 2 < 1)
 * SortTracer assignment #31 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * SortTracer #19, copied as generation 2 (total: 11)
 * SortTracer comparison #39 (generation 2 < 1)
 * SortTracer comparison #40 (generation 2 < 1)
 * SortTracer comparison #41 (generation 1 < 2)
 * SortTracer assignment #32 (generation 1 = 1)
 * SortTracer comparison #42 (generation 2 < 1)
 * SortTracer assignment #33 (generation 1 = 2)
 * SortTracer generation 2 destroyed (total: 10)
 * ---[ End std::sort() ]----------------------
 * 0 1 2 3 4 5 6 7 8 9
 *
 * std::sort() of 10 SortTracer's was performed by:
 * 9 temporary tracers
 * up to 11 tracers at the same time (10 before)
 * 33 assignments
 * 42 comparisons
 *
 * SortTracer generation 1 destroyed (total: 9)
 * SortTracer generation 1 destroyed (total: 8)
 * SortTracer generation 1 destroyed (total: 7)
 * SortTracer generation 1 destroyed (total: 6)
 * SortTracer generation 1 destroyed (total: 5)
 * SortTracer generation 1 destroyed (total: 4)
 * SortTracer generation 1 destroyed (total: 3)
 * SortTracer generation 1 destroyed (total: 2)
 * SortTracer generation 1 destroyed (total: 1)
 * SortTracer generation 1 destroyed (total: 0)
 */
```

