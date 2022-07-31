# C++基础笔记


<!--more-->

> 这个是我在学习`C++`语言中所记录的笔记,有可能会存在错误和遗漏,并且我有一点点C语言基础,
> 会大量的提及C语言与C++的不同,从而造成笔记晦涩;
> 另外C++的学习是一个长期且艰难的过程,因此本文进行了切分;

[C++基础笔记]({{< ref "cxx_basic.md ">}})</br>
[C++提高笔记]({{< ref "cxx_enhance.md ">}})</br>
[C++增强笔记]({{< ref "cxx_advance.md ">}})</br>

<!--more-->

## 第一个程序

```cpp
#include <iostream>

using namespace std;

int main(int argc, char *argv[])
{
    cout << "Hello World" << endl;
    return 0;
}
```

编译指令

![Hello world编译指令](HelloWorld.gif)

从生成的汇编指令来看,复杂不少

```asm
__cxx_global_var_init:
        push    {r11, lr}
        mov     r11, sp
        sub     sp, sp, #8
        ldr     r0, .LCPI0_0
.LPC0_0:
        add     r0, pc, r0
        str     r0, [sp, #4]                    @ 4-byte Spill
        bl      std::ios_base::Init::Init() [complete object constructor]
        ldr     r1, [sp, #4]                    @ 4-byte Reload
        ldr     r0, .LCPI0_1
.LPC0_1:
        ldr     r0, [pc, r0]
        ldr     r2, .LCPI0_2
.LPC0_2:
        add     r2, pc, r2
        bl      __cxa_atexit
        mov     sp, r11
        pop     {r11, lr}
        bx      lr
.LCPI0_0:
        .long   _ZStL8__ioinit-(.LPC0_0+8)
.LCPI0_1:
.Ltmp2:
        .long   _ZNSt8ios_base4InitD1Ev(GOT_PREL)-((.LPC0_1+8)-.Ltmp2)
.LCPI0_2:
        .long   __dso_handle-(.LPC0_2+8)
main:
        push    {r11, lr}
        mov     r11, sp
        sub     sp, sp, #16
        mov     r2, #0
        str     r2, [sp]                        @ 4-byte Spill
        str     r2, [r11, #-4]
        str     r0, [sp, #8]
        str     r1, [sp, #4]
        ldr     r0, .LCPI1_0
.LPC1_0:
        ldr     r0, [pc, r0]
        ldr     r1, .LCPI1_1
.LPC1_1:
        add     r1, pc, r1
        bl      std::basic_ostream<char, std::char_traits<char> >& std::operator<< <std::char_traits<char> >(std::basic_ostream<char, std::char_traits<char> >&, char const*)
        ldr     r1, .LCPI1_2
.LPC1_2:
        ldr     r1, [pc, r1]
        bl      std::basic_ostream<char, std::char_traits<char> >::operator<<(std::basic_ostream<char, std::char_traits<char> >& (*)(std::basic_ostream<char, std::char_traits<char> >&))
        ldr     r0, [sp]                        @ 4-byte Reload
        mov     sp, r11
        pop     {r11, lr}
        bx      lr
.LCPI1_0:
.Ltmp5:
        .long   _ZSt4cout(GOT_PREL)-((.LPC1_0+8)-.Ltmp5)
.LCPI1_1:
        .long   .L.str-(.LPC1_1+8)
.LCPI1_2:
.Ltmp6:
        .long   _ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_(GOT_PREL)-((.LPC1_2+8)-.Ltmp6)
_GLOBAL__sub_I_example.cpp:
        push    {r11, lr}
        mov     r11, sp
        bl      __cxx_global_var_init
        pop     {r11, lr}
        bx      lr
.L.str:
        .asciz  "Hello World"
```

## 注释

### C++注释

```c++
// 这个是一个单行注释
```

### C注释

```c++
/* 
这个里面是一个注释
*/
```

> 在C/C++中两种注释都是可以使用的
> 并不是绝对的,只因为提出者是C或者C++

1. 注释不影响程序的编译-->(预编译删除所有注释)
2. 推荐使用[doxygen](https://www.doxygen.nl/index.html)-->(使用doxygen漂亮的注释)

## 标识符

### 变量

变量的存在意义:方便我们管理内存

> 变量创建的语法

```cpp
存储类型 数据类型 变量名 = 变量初始化;
```

+ 自动,不使用标识符
+ 寄存器(register)
+ 静态static
+ 外部extern

```cxx
/* auto */ int a = 0;       // C++不在添加自动变量标识符    
static int a = 0;           // 静态
register int a = 0;         // 寄存器
extern int a = 0;           // 外部
```

> 总结

| 存储类型 | 持续性 | 作用域        | 链接性 | 定义                  |
| -------- | ------ | ------------- | :------: | --------------------- |
| 自动变量 | 自动   | 函数内        | :negative_squared_cross_mark: | 无标志符,定义在函数内 |
| 寄存器   | 自动   | 函数内        | :negative_squared_cross_mark: | `register`            |
| 静态     | 全局   | 函数内&#124;文件内 | :negative_squared_cross_mark: | `static`              |
| 外部     | 全局   | 文件内        | &#x2705; | 无标志符,定义在文件内 |

### 常量

作用: 记录程序中不可以改变的数据

* `define `宏常量(预编译期)
* `const `修饰变量(编译期)

### 关键字

| 关键字        |          |                   |          |
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

### 命名规则

1. 标识符不可以是关键字
2. 只能由字母、数字、下划线构成
3. 第一个字母只能是字母或者是下划线
4. 区分大小写

## 数据类型

指定类型,分配内存

### 整形

### 浮点型

1. 单精度`float`
2. 双精度`double`
   
### 字符型

### 转义字符

### 字符串

1. C风格
   
```c
char 变量名[] = "字符串值";
```

2. C++风格
   
```cpp
string 变量名 = "字符串值";
```

### 布尔类型

```cpp
bool A = true;
bool B = false;
```

## 运算符

### 基本运算符

### 取模运算

就是取余数

### 自增自减运算

```c++
a1++;
a2--;
```

### 赋值运算

| 运算符 | 术语  | 示例  | 结果   |
| :---: | :---: | :---: | :---: | 
| =     |       |       |       | 
| +=    |       |       |       | 
| -=    |       |       |       | 
| *=    |       |       |       | 
| /=    |       |       |       | 
| %=    |       |       |       | 

### 比较运算符

### 逻辑运算符

## 流程控制

### 顺序结构

#### if语句

```c++
// 情景1
if (条件) {

}

// 情景2
if (条件) {

} else {

}

// 情景3
if (条件1) {

} else if (条件2) {

} else {

}
```

#### 三目运算符

```c++
表达式1? 表达式2:表达式3
```

### 选择结构

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

### 循环结构

### while循环

```c++
while(条件)
{
    循环体;
}
```

###`do...while`循环

```c++
do {

} while(条件)
```

### for循环

```c++
for (起始表达式; 条件表达式; 末尾循环体)
{
    循环体;
}
```

### 跳转语句

* break
* continue

#### goto

> 绝对跳转语句

## 函数定义

1. 返回值类型
2. 函数名
3. 参数列表
4. 函数体语句
5. `return`表达式

```c++
返回值类型 函数名字(参数列表)
{
    函数体语句;
    return 表达式;
}
```

### 值传递

> 类似数值拷贝

### 函数的常见样式

1. 无参无返
2. 有参无返
3. 无参有反
4. 有参有返

### 声明

作用: 告诉编译器函数名以及调用方式,函数实体可以单独实现;

### 多文件

## 复合数据结构

### 数组

### 指针

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

### 枚举

### 联合

### 位域

## C++内存分区
c++程序在运行时,将内存分为4个区域
1. 代码区: 存放程序的二进制代码,由操作系统管理
2. 全局区: 存放全局变量、静态变量和常量
3. 栈区: 编译器自动分配
4. 堆区: 程序负责分配和释放

## new/delete操作符

+ new操作符在堆区开辟内存
+ delete释放内存对象

## 引用

作用: 给变量起别名
语法: 数据类型 &别名 = 原名;

### 引用做参数
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

### 引用做返回值

### 引用的本质

引用的本质是C++内部实现的一个指针常量

### 常量引用
```c++
const int &ref = 10;
```

## 函数提高

### 函数默认值

1. 某个位置有默认值，那么后面的参数也必须由默认值
2. 如果声明了默认值，那么实现不可以有默认值(默认参数会产生冲突)

```c++
void test_default_param(int a = 0, int b = 0, int c = 0)
{
    std::cout << a + b + c << std::endl; 
}
```
### 函数的占位参数

占位参数还可以有默认值

```c++
void test(int a, int = 10) {
    std::cout << a << std::endl;
}
```
### 函数重载
作用:函数名相同,提高复用性

重载的条件:
1. 相同作用域

2. 函数名相同

3. 参数不同(类型, 个数,顺序)

注意事项:

1. 引用作为重载条件
2. 函数重载碰到默认参数

