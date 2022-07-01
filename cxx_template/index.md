# C++模板


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

### 变参模板

### 两阶段编译检查(Two-Phase Translation) 
1. 在模板定义阶段，模板的检查并不包含类型参数的检查。只包含下面几个方面：
  + 语法检查。比如少了分号。
  + 使用了未定义的不依赖于模板参数的名称（类型名，函数名，......）
  + 未使用模板参数的static assertions。

2. 在模板实例化阶段，为确保所有代码都是有效的，模板会再次被检查，尤其是那些依赖于类型参数的部分。

总结来说

+ 模板实例化(不同于预编译)
+ 程序编译

### 类型推断中的类型转换

在类型推断的时候自动的类型转换是受限制的：

+ 如果调用参数是按引用传递的，任何类型转换都不被允许。通过模板类型参数T 定义的
两个参数，它们实参的类型必须完全一样。
+ 如果调用参数是按值传递的，那么只有退化（decay）这一类简单转换是被允许的：`const`
和`volatile `限制符会被忽略，引用被转换成被引用的类型，raw array 和函数被转换为相
应的指针类型。通过模板类型参数T 定义的两个参数，它们实参的类型在退化（decay）
后必须一样。

#### 默认类型推断

```c
template<typename T = std::string>
void HelloWorld(T f = "HelloWorld")
{
	std::cout << f << std::endl;
}

HelloWorld(); 		// HelloWorld
HelloWorld(123);	// 123
```

### 多模板参数调用

主要是为了完成`max(1, 21)`,但是

```c
template<typename T>
T max (T a, T b)
{
	return b < a ? a : b;
}
```
如果直接调用`error: no matching function for call to 'max(int, double)',会报错,因此可以使用如下方法:

#### 显式指定
将模板显式指定,从而可以保证编译进行
```c
std::cout << max<double>(1, 2.1) << std::endl;
```
但是总感觉不太优雅,因此一定存在更加优秀的方案,但是此时也可以称之为一个方案

#### 返回值指定
```c
template<typename T1, typename T2, typename T3>
T1 max (T2 a, T3 b)
{
	return b < a ? a : b;
}
std::cout << max<int, int, double>(1, 2.1) << std::endl;

```

#### 自动推导

> 不指定类型,而是由编译器自行判断

```c
template<typename T1, typename T2>
auto min(T1 a, T2 b)
{
	return b < a ? a : b;
}

std::cout << max(1, 2.1) << std::endl;
```

#### 公共类型推导

```c++
template<typename T1, typename T2>
std::common_type_t<T1,T2> max(T1 a, T2 b)
{
	return b < a ? a : b;
}
```

## 模板提高

### 移动语义和enable_if<>

### 模板参数传递

### 编译期编程

## 模板进阶

