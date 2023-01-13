# C++提高笔记


<!--more-->

> 这个是我在学习`C++`语言中所记录的笔记,有可能会存在错误和遗漏,并且我有一点点C语言基础,
> 会大量的提及C语言与C++的不同,从而造成笔记晦涩;
> 另外C++的学习是一个长期且艰难的过程,因此本文进行了切分;

[C++基础笔记]({{< ref "cxx_basic.md ">}})</br>
[C++提高笔记]({{< ref "cxx_enhance.md ">}})</br>
[C++增强笔记]({{< ref "cxx_advance.md ">}})</br>

<!--more-->

## 类和对象

+ 类的访问属性
+ 类的继承属性

1. public:
2. protected:
3. private:

### 成员函数

+ 成员函数 + delete表示删除此函数
+ 成员函数 + default表示默认函数

+ explicit: 关闭隐式类型转换
  - 关键字只能作用域类构造函数
  - 只作用单个参数的构造函数

+ final:
  - 第一个用在类，用于说明该类是继承体系下最后的一个类，不要其他类继承我，当继承时就会报错。
  - 第二个用在虚函数，表示这个虚函数不能再被override了，再override就会报错

### 虚函数

+ override用于虚函数,上面的`virtual void func(int)`实际上不是重写父类的虚函数,而是定义一个新的虚函数;
+ 我们的本意是重写虚函数,当不加overrride的时候,这样写编译器不会报错
+ 那如果像下面加上override的话，则会报错，表示告诉了编译器，我确实要重写，但写错了，没有重写，于是就报错了
+ 这样就能给我们对虚函数的重写做检查!

### class与struct的区别

class默认权限是private
struct默认权限是public

### 构造函数和析构函数

对象的初始化和清理
* 构造函数有参数
* 析构函数没有参数
* 二者都没有返回值

### 拷贝构造函数

```cpp
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

```cpp
Person testPerson();	// 表面上是执行构造函数
int func();				// 类似函数声明
```

### 拷贝构造函数的调用时机
* 使用一个已经创建完毕的对象初始化一个新对象
* 值传递的方式给函数进行参数传递
* 以值的方式返回局部对象

### 构造函数的调用规则
默认情况下:C++编译器至少给一个类添加3个函数
1. 默认构造函数(无参)
2. 默认析构函数(无参)
3. 默认拷贝函数,对属性值进行拷贝

构造函数构造规则如下:

* 用户定义有参构造,C++默认不提供无参构造，但是提供默认拷贝构造
* 用户定义拷贝构造,C++不提供其他构造函数

### 深拷贝和浅拷贝

* 浅拷贝: 简单的复制操作
* 深拷贝: 在堆区重新申请空间，进行复制操作

## 初始化列表

作用:C++提供了初始化列表语法,用来初始化属性;

语法:

```c++
构造函数(): 属性1(值1),属性2(值2),属性3(值3)
{
	/* 函数体 */
}
```

## 类对象作为类成员

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

## C++对象模型

## this指针

**this指针指向被调用成员函数所属的对象**
this指针本质：指针常量

## 空指针访问成员函数
C++空指针也是可以访问成员函数的,但是要注意的this指针;

## const修饰成员函数

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
## 友元
+ 全局函数
+ 全局类
+ 成员函数

## 运算符重载

重载的原理:对已有的运算符进行重新定义,赋予新的功能含义;

### 通过成员函数重载运算符

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

### 通过全局函数重载

```c++
Person operator+(Person &p1, Person &p2) 
{
    Person t;
    t.m_A = p1.m_A + p2.m_A;
    t.m_B = p2.m_B + p2.m_B;
    return t;
}
```

### 重载左移运算符
```c++
std::ostream &operator<<(std::ostream &cout, Person &p)
{
    cout << p.m_A << p.m_B;
    return cout;
} 
```

### 递增重载++

注意:
+ 前置递增  **p++**
+ 后置递增 **++p**

### 重载例子(复数)

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

## 继承

减少重复代码

```c++
class 子类 : 继承方式 父类
```

父类:基类
子类:派生类

### 继承方式

+ 公共继承
+ 保护继承
+ 私有继承

### 继承中的对象模型

### 构造和析构的顺序

> 先构造父类再构造子类
> 先析构子类再析构父类

### 继承中同名成员处理

+ 访问子类中同名成员,直接访问即可   s.m_A
+ 访问父类中同名成员,需要加上作用域 s.Base:m_A

## 多重继承

C++允许一个类继承多个基类

```c++
class 子类 : 继承方式 父类1, 继承方式 父类2...
```
> 冲突解决：加上类名

### 菱形继承

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
### 虚基类指针(vbptr)

vbptr --> vbtable

## 多态

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

## 虚函数

> 只要有一个纯虚函数，就称为抽象类

1. 抽象类无法直接实例化对象
2. 抽象子类必须重写父类的纯虚函数,否则也是抽象类

### 虚析构和纯虚析构



