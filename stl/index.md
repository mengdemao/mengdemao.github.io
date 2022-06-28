# STL学习笔记

> STL称为标准模板库(Standard Template Library)
> 广义上可以分为容器,算法,迭代器
> 容器和算法通过迭代器进行无缝连接
> STL几乎所有的代码都采用了函数模版或者类模板

## STL组件

| 序号 | 名称       | 解释                     |
| ---- | ---------- | ------------------------ |
| 1    | 容器       | 各种数据结构             |
| 2    | 算法       | 各种常用的算法           |
| 3    | 迭代器     | 容器域算法的胶合         |
| 4    | 仿函数     | 行为类似函数             |
| 5    | 适配器     | 修饰容器或者仿函数迭代器 |
| 6    | 空间配置器 | 负责空间的配置和管理     |

## 容器算法和迭代器
### vector

#### vector使用
```c++
/* 创建vector容器 */
vector<int> v;
/* 插入数据 */
v.push_back(10);
v.push_back(20);
v.push_back(30);
v.push_back(40);
```
#### 迭代器使用
##### 迭代器方案1
```c++
vector<int>::iterator itBegin = v.begin();
vector<int>::iterator itEnd   = v.end();
while (itBegin != itEnd) {
	cout << *itBegin << endl;
	itBegin += 1;
}
```

##### 迭代器2
```c++
for (vector<int>::iterator it = v.begin(); it != v.end(); it++)
{
	cout << *it << endl;
}
```

##### 遍历算法

```c++
template <class T>
void myPrint(T val)
{
	cout << val << endl;
}

/* 可惜回调函数不支持自动推导 */
for_each(v.begin(), v.end(), myPrint<int>);
```
#### 容器自定义数据

#### 容器嵌套容器
```c++
vector<vector<int>>v; // 外部大容器
vector<int> vx[10];   // 内部小容器

/* 插入容器 */
for (int i = 0; i < 10; i++)
{
	for (int j = 0; j < 30; j++)
	{
		vx[i].push_back(i + j + 10);
	}
	v.push_back(vx[i]);
}

/* 遍历容器 */
for (vector<vector<int>>::iterator it = v.begin(); it != v.end(); it++)
{
	for (vector<int>::iterator vit = it->begin(); vit != it->end(); vit++)
	{
		cout << *vit << " ";
	}
	cout << endl;
}
```

### string
string本质上是一个类,封装了char*,提供了许多的成员方法;
####  构造函数
```c++
string s1(str);
string s2 = "Hello World";
string s3(s2);
```

### 赋值操作

1. 重载操作符**=**

```c++
string s1;
s1 = "Hello World";
```

2. 成员函数**assign**

```c++
string str;
str.assign("Hello World");
```

### 追加操作

1. 重载操作符**+=**
2. 成员函数**append**

### 查找和替换 

#### find

#### replace

### 比较

#### compare

### 字符存取

1. []
2. at

### 插入和删除

#### insert

#### earse

### 子串

#### substr

## array

## deque

## hashtable

## map

## list

## queue

## stack

## set

## rbtree

