# C++笔记


## 前言

这个是我在学习`C++`语言中所记录的笔记,有可能会存在错误和遗漏,并且我有一点点C语言基础,
会大量的提及C语言与C++的不同,从而造成笔记晦涩;

另外C++的学习是一个长期且艰难的过程,因此本文进行了切分;

## C++基础语法

### 第一个程序
```c++
#include <iostream>

using namespace std;

int main(int argc, char *argv[])
{
	cout << "Hello World" << endl;
	return 0;
}
```

### 注释类型
* 单行注释
```c++
// 这个是一个单行注释
```

* 多行注释
```c++
/* 
	这个里面是一个注释
 */
```
### 变量

变量的存在意义:方便我们管理内存

变量创建的语法

```c++
数据类型 变量名 = 变量初始化;
```

### 常量
作用: 记录程序中不可以改变的数据
* define 宏常量(预编译期)
* const 修饰变量(编译期)

### 关键字

| 关键字        |           |               |      |
| ------------ | --------- | ---------------- | -------- |
| asm          | else      | new              | this     |
| auto         | enum      | operator         | throw    |
| bool         | explicit  | private          | true     |
| break        | export    | protected        | try      |
| case         | extern    | public           | typedef  |
| catch        | false     | register         | typeid   |
| char         | float     | reinterpret_cast | typename |
| class        | for       | return           | union    |
| const        | friend    | short            | unsigned |
| const_cast   | goto      | signed           | using    |
| continue     | if        | sizeof           | virtual  |
| default      | inline    | static           | void     |
| delete       | int       | static_cast      | volatile |
| do           | long      | struct           | wchar_t  |
| double       | mutable   | switch           | while    |
| dynamic_cast | namespace | template         |          |

###  标识符命名规则

1. 标识符不可以是关键字
2. 只能由字母、数字、下划线构成
3. 第一个字母只能是字母或者是下划线
4. 区分大小写

### 数据类型
指定类型,分配内存

#### 整形

#### 浮点型
1. 单精度`float`
2. 双精度`double`
#### 字符型

#### 转义字符

#### 字符串
1. C风格
```c++
char 变量名[] = "字符串值";
```
2. C++风格
```c++
string 变量名 = "字符串值";
```
#### 布尔类型
```C++
bool A = true;
bool B = false;
```
### 运算符
#### 基本运算符

#### 取模运算
就是取余数
#### 自增自减运算

```c++
a1++;
a2--;
```
### 赋值运算

| 运算符 | 术语 | 示例 | 结果 |
| ------ | ---- | ---- | ---- |
| =      |      |      |      |
| +=     |      |      |      |
| -=     |      |      |      |
| *=     |      |      |      |
| /=     |      |      |      |
| %=     |      |      |      |

### 比较运算符

### 逻辑运算符

### 程序流程结构
#### 顺序结构
##### if语句
##### 三目运算符
```c++
表达式1? 表达式2:表达式3
```
#### 选择结构
```c++
switch(condition)
{
case 条件1:
	break;
case 条件2:
	break;
default:
	break;
}
```
#### 循环结构
#### while循环
```c++
while(条件)
{
	循环体;
}
```
#### dowhile循环
```c++
do {

} while(条件)
```

#### for循环 
```c++
for (起始表达式; 条件表达式; 末尾循环体)
{
	循环体;
}
```

#### 跳转语句
##### break

##### continue

##### goto

### 数组

### 函数定义
1. 返回值类型
2. 函数名
3. 参数列表
4. 函数体语句
5. return表达式

```c++
返回值类型 函数名字(参数列表)
{
	函数体语句;
	return 表达式;
}
```

#### 值传递

类似数值拷贝

### 函数的常见样式

1. 无参无返
2. 有参无返
3. 无参有反
4. 有参有返

### 函数的声明

作用: 告诉编译器函数名以及调用方式,函数实体可以单独实现;

### 函数的分文件编写

###  指针

#### 指针的定义和使用

#### 指针所占用空间

#### 空指针
含义: 指针指向内存空间为0的指针;
用途: 初始化指针变量
注意: 空指针的地址是不可以访问的

#### 野指针
指针指向非法的内存空间

#### const与指针
1. const修饰指针
2. const修饰常量
3. const既修饰指针又修饰常量

```c++
const int *p = &a;

int const *p = &a;

const int *const p = &a;
```

#### 指针与数组

#### 指针与函数

### 结构体
#### 结构体数组
#### 结构体指针
#### 结构体嵌套



## C++核心编程

本阶段主要对面向对象进行详细讲解

### C++内存分区
c++程序在运行时,将内存分为4个区域
1. 代码区: 存放程序的二进制代码,由操作系统管理
2. 全局区: 存放全局变量、静态变量和常量
3. 栈区: 编译器自动分配
4. 堆区: 程序负责分配和释放

### new/delete操作符
C++利用new操作符在堆区开辟内存

### 引用
作用: 给变量起别名
语法: 数据类型 &别名 = 原名;

#### 引用做参数
```c++
#include <iostream>
void swap(int &a, int &b) 
{
    int t; t = a;a = b;b = t;
}
int main(int argc, char *argv[])
{
    int a = 10;int b = 12;
    std::cout << "交换前" << a << '\t' << b << std::endl;
    swap(a, b);
    std::cout << "交换后" << a << '\t' << b << std::endl;
    return 0;
}
```
执行结果

![image-20211003222910241](https://raw.githubusercontent.com/mengdemao/picture/master/image-20211003222910241.png)

#### 引用做返回值


#### 引用的本质
引用的本质是C++内部实现的一个指针常量

#### 常量引用
```c++
const int &ref = 10;
```

### 函数提高
#### 函数默认值

1. 某个位置有默认值，那么后面的参数也必须由默认值
2. 如果声明了默认值，那么实现不可以有默认值(默认参数会产生冲突)

```c++
void test_default_param(int a = 0, int b = 0, int c = 0)
{
    std::cout << a + b + c << std::endl; 
}
```
#### 函数的占位参数

占位参数还可以有默认值

```c++
void test(int a, int = 10) {
    std::cout << a << std::endl;
}
```
#### 函数重载
作用:函数名相同,提高复用性

重载的条件:
1. 相同作用域

2. 函数名相同

3. 参数不同(类型, 个数,顺序)

注意事项:

1. 引用作为重载条件
2. 函数重载碰到默认参数

### 类和对象
类的访问属性
1. public:
2. protected:
3. private:

#### class与struct的区别
class默认权限是private
struct默认权限是public

#### 构造函数和析构函数
对象的初始化和清理
* 构造函数有参数
* 析构函数没有参数
* 二者都没有返回值

#### 拷贝构造函数

```c++
class Person {
public:
    /* 构造函数 */
    Person(std::string name, int age) {
        std::cout << "构造函数" << std::endl;
    }
    /* 析构函数 */
    ~Person() {
        std::cout << "析构函数" << std::endl;
    }
    /* 拷贝构造函数 */
    Person(const Person &p) {
        std::cout << "拷贝构造函数" << std::endl;
    }
};
```
* 调用无参构造函数的时候不可以添加();否则就会产生函数声明的效果
```c++
Person testPerson();	// 表面上是执行构造函数
int func();				// 类似函数声明
```

#### 拷贝构造函数的调用时机
* 使用一个已经创建完毕的对象初始化一个新对象
* 值传递的方式给函数进行参数传递
* 以值的方式返回局部对象

#### 构造函数的调用规则
默认情况下:C++编译器至少给一个类添加3个函数
1. 默认构造函数(无参)
2. 默认析构函数(无参)
3. 默认拷贝函数,对属性值进行拷贝

构造函数构造规则如下:
* 用户定义有参构造,C++默认不提供无参构造，但是提供默认拷贝构造
* 用户定义拷贝构造,C++不提供其他构造函数

#### 深拷贝和浅拷贝
* 浅拷贝: 简单的复制操作
* 深拷贝: 在堆区重新申请空间，进行复制操作

### 初始化列表

作用:C++提供了初始化列表语法,用来初始化属性;

语法:

```c++
构造函数(): 属性1(值1),属性2(值2),属性3(值3)
{
	/* 函数体 */
}
```
### 类对象作为类成员

### 静态成员
静态成员就是在静态成员变量和成员函数前加上static,称为静态成员;

- 静态成员变量
	+ 所有对象共享一份数据
	+ 编译阶段分配内存
	+ 类内声明,类外初始化
- 静态成员函数
	+ 所有对象共享同一个函数
	+ 静态成员函数只能访问静态成员变量
```c++
class Person {
public:
    static int age;
    static void func()
    {
        std::cout << "静态成员函数" << std::endl;
    }
};
/* 通过对象访问 */
Person p;
p.func();
/* 通过类访问 */
Person::func();
```
### 成员变量和成员函数分开存储
1. 非静态成员,		属于类的对象
2. 静态成员,		不属于类的对象
3. 非静态成员函数,	    不属于类的对象
4. 静态成员函数,          不属于类的对象

**空对象大小为1**

### C++对象模型

### this指针
**this指针指向被调用成员函数所属的对象**
this指针本质：指针常量

### 空指针访问成员函数
C++空指针也是可以访问成员函数的,但是要注意的this指针;

### const修饰成员函数
**常函数:**
+  常函数不可以修改成员属性
+  成员属性加上mutable,常函数也可以修改
** 常对象**
+ 对象之前加const表示常对象
+ 常对象只能调用函数

执行原理

```c++
this ==> Person * const this;
后面新追加的const则会造成
const Person * const this;
```
```c++
class Person {
public:
    int m_A;
    mutable int m_B;
    void showPerson() const
    {
        m_A = 10; /* 错误,不可修改 */
        m_B = 10; /* 正确,可以修改 */
    }
};
```
### 友元
+ 全局函数
+ 全局类
+ 成员函数

### 运算符重载

重载的原理:对已有的运算符进行重新定义,赋予新的功能含义;

#### 通过成员函数重载运算符
```c++
class Person {
public:
    int m_A;
    int m_B;

    /* 使用成员函数实现 */
    Person PersonAddPerson(Person &p)
    {
        Person t;
        t.m_A = this->m_A + p.m_A;
        t.m_B = this->m_B + p.m_B;
        return t;
    }

    /* 重载+ */
    Person operator+(Person &p)
    {
        Person t;
        t.m_A = this->m_A + p.m_A;
        t.m_B = this->m_B + p.m_B;
        return t;
    }
};

int main(int argc, char *argv[])
{
    Person p1;
    Person p2;

    Person p3 = p1.PersonAddPerson(p2);
    
    Person p4 = p1.operator+(p2);
    
    Person p5 = p1 + p2;

    return 0;
}
```
![image-20211004211414127](https://raw.githubusercontent.com/mengdemao/picture/master/image-20211004211414127.png)

#### 通过全局函数重载
```c++
Person operator+(Person &p1, Person &p2) 
{
    Person t;
    t.m_A = p1.m_A + p2.m_A;
    t.m_B = p2.m_B + p2.m_B;
    return t;
}
```
#### 重载左移运算符
```c++
std::ostream &operator<<(std::ostream &cout, Person &p)
{
    cout << p.m_A << p.m_B;
    return cout;
} 
```

#### 递增重载++
注意:
+ 前置递增  **p++**
+ 后置递增 **++p**

#### 重载例子(复数)

```c++
#include <iostream>

class Complex {
    friend std::ostream &operator<<(std::ostream &cout, Complex p);

public:
    Complex(int i, int j);
    
    Complex();

    /* 重载+ */
    Complex operator+(Complex &p) 
    {
        Complex t;
        t.i = this->i + p.i;
        t.j = this->j + p.j;
        return t;
    }
    /* 重载前置++ */
    Complex& operator++()
    {
        this->i++;
        this->j++;
        return *this; 
    }
    
    /* 重载后置++ */
    Complex operator++(int)
    {
        Complex t;
        
        /* 记录 */
        t.i = this->i;
        t.j = this->j;

        /* 递增 */
        this->i++;
        this->j++;

        return t; 
    }

    /* 重载= */
    Complex& operator=(Complex &p)
    {
        this->i = p.i;
        this->j = p.j;

        return *this;
    }
private:
    int i;  /* 实部 */
    int j;  /* 虚部 */
};

/* 构造函数 */
Complex::Complex(int i, int j)
{
    this->i = i;
    this->j = j;
}

Complex::Complex()
{
    this->i = 0;
    this->j = 0;
}

std::ostream &operator<<(std::ostream &cout, Complex p)
{
    cout << p.i << "+" << p.j << "i"; 
    return cout;
} 

int main(int argc, char *argv[])
{
    Complex p1(1, 2);
    Complex p2(3, 4);

    std::cout << p1 << std::endl;
    std::cout << p2 << std::endl;
    std::cout << p1 + p2 << std::endl;

    std::cout << ++p1 << std::endl;
    std::cout << p2++ << std::endl;

    Complex p3 = p2 = p1;
    std::cout << p1 << " " << p2 << " " << p3 << std::endl;

    return 0;
}
```

### 继承
减少重复代码

```c++
class 子类 : 继承方式 父类
```

父类:基类
子类:派生类
#### 继承方式
+ 公共继承
+ 保护继承
+ 私有继承

#### 继承中的对象模型

#### 构造和析构的顺序
> 先构造父类再构造子类
> 先析构子类再析构父类
#### 继承中同名成员处理
+ 访问子类中同名成员,直接访问即可   s.m_A
+ 访问父类中同名成员,需要加上作用域 s.Base:m_A

#### 多重继承
C++允许一个类继承多个基类
```c++
class 子类 : 继承方式 父类1, 继承方式 父类2...
```
> 冲突解决：加上类名
#### 菱形继承

![菱形继承.drawio](https://raw.githubusercontent.com/mengdemao/picture/master/%E8%8F%B1%E5%BD%A2%E7%BB%A7%E6%89%BF.drawio.svg)

孙子类继承了子类1和子类2,但是继承了两次父类。

+ 多重继承数据会产生二义性
+ 数据只需要一份即可

```c++
/* 动物类 */
class Animal {
public:
    int m_age;
};
class Sheep : public Animal {}; /* 羊类 */
class Camel : public Animal {}; /* 驼类 */
class Alpaca : public Sheep, public Camel {}; /* 羊驼 */
int main(int argc, char *argv[])
{
    Alpaca a;
    a.Sheep::m_age = 18;
    a.Camel::m_age = 18;
    return 0;
}
```
> 虚继承
```c++
class Sheep : virtual public Animal {}; /* 羊类 */
class Camel : virtual public Animal {}; /* 驼类 */
```
#### 虚基类指针(vbptr)

vbptr --> vbtable

### 多态
+ 分类
	+ 静态多态: 重载
	+ 动态多态:虚函数
+ 区别
	+ 静态多态函数地址早绑定:编译期确定函数地址
	+ 动态多态函数地址晚绑定:运行期确定函数地址

父类接收子类的对象,在程序运行期间确定具体改调用那个函数;
+ 有继承关系

+ 子类重写父类的虚函数
  重写：函数完全一致

#### 纯虚函数
> 只要有一个纯虚函数，就称为抽象类
1. 抽象类无法直接实例化对象
2. 抽象子类必须重写父类的纯虚函数,否则也是抽象类
#### 原理

### 虚析构和纯虚析构
