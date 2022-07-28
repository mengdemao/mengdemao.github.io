# C++基础笔记


> 这个是我在学习`C++`语言中所记录的笔记,有可能会存在错误和遗漏,并且我有一点点C语言基础,
> 会大量的提及C语言与C++的不同,从而造成笔记晦涩;
> 另外C++的学习是一个长期且艰难的过程,因此本文进行了切分;

## 第一个程序

```c++
#include <iostream>

using namespace std;

int main(int argc, char *argv[])
{
	cout << "Hello World" << endl;
	return 0;
}
```

## 注释类型

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
## 变量

变量的存在意义:方便我们管理内存

变量创建的语法

```c++
数据类型 变量名 = 变量初始化;
```

## 常量
作用: 记录程序中不可以改变的数据
* define 宏常量(预编译期)
* const 修饰变量(编译期)

## 关键字

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

##  标识符命名规则

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
```c++
char 变量名[] = "字符串值";
```
2. C++风格
```c++
string 变量名 = "字符串值";
```
### 布尔类型

```C++
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
## 赋值运算

| 运算符 | 术语 | 示例 | 结果 |
| ------ | ---- | ---- | ---- |
| =      |      |      |      |
| +=     |      |      |      |
| -=     |      |      |      |
| *=     |      |      |      |
| /=     |      |      |      |
| %=     |      |      |      |

## 比较运算符

## 逻辑运算符

## 程序流程结构
### 顺序结构
#### if语句
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

#### break

#### continue

#### goto

## 数组

## 函数定义
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

### 值传递

类似数值拷贝

## 函数的常见样式

1. 无参无返
2. 有参无返
3. 无参有反
4. 有参有返

## 函数的声明

作用: 告诉编译器函数名以及调用方式,函数实体可以单独实现;

## 函数的分文件编写

##  指针

### 指针的定义和使用

### 指针所占用空间

### 空指针
含义: 指针指向内存空间为0的指针;
用途: 初始化指针变量
注意: 空指针的地址是不可以访问的

### 野指针
指针指向非法的内存空间

### const与指针
1. const修饰指针
2. const修饰常量
3. const既修饰指针又修饰常量

```c++
const int *p = &a;

int const *p = &a;

const int *const p = &a;
```

### 指针与数组

### 指针与函数

## 结构体

### 结构体数组

### 结构体指针

### 结构体嵌套

