# STL基础笔记


> STL称为标准模板库(Standard Template Library)
> 广义上可以分为容器,算法,迭代器
> 容器和算法通过迭代器进行无缝连接
> STL几乎所有的代码都采用了函数模版或者类模板

## STL分类

| 序号 | 名称       | 解释                     |
| ---- | ---------- | ------------------------ |
| 1    | 容器       | 各种数据结构             |
| 2    | 算法       | 各种常用的算法           |
| 3    | 迭代器     | 容器域算法的胶合         |
| 4    | 仿函数     | 行为类似函数             |
| 5    | 适配器     | 修饰容器或者仿函数迭代器 |
| 6    | 空间配置器 | 负责空间的配置和管理     |

## 空间管理


### 前言

SGI STL将new的申请空间和调用构造函数的两个功能分开实现, 如果对new不太清楚的, 可以先去看看这一篇[new实现](https://blog.csdn.net/Function_Dou/article/details/84526761)再来看配置器也不迟. 本节是STL分析的第一篇, 主要分析STL各个部分都会出现的`alloc`实现, 虽然每个部分都只会默认调用它, 不了解它也可以看懂分析,  但是他又是不可缺少的, 我们就以它做为开篇进行分析.



### "new"的实现

这里直接我们直接来看STL的`construct`实现吧

```c++
// 这里的construct调用的是placement new, 在一个已经获得的内存里建立一个对象
template <class T1, class T2>
inline void construct(T1* p, const T2& value)
{
  new (p) T1(value);
}
```

可以明白这里就只是一个`placement new`的调用, 只是用了泛型来实现一个对象分配的模板, 并实现初始化.

既然已经看到了对象的分配, 那再接再厉看看空间的分配, 充分了解STL是怎么将new分开执行的. allocate函数实现空间的申请, 但是这里有一点看不出来, 申请内存是有分为一级配置器和二级配置器, 分配的空间小于128字节的就调用二级配置器, 大于就直接使用一级配置器, 一级配置器直接调用`malloc`申请, 二级使用内存池.

```c++
template<class T>
inline T* allocate(ptrdiff_t size, T*)
{
    set_new_handler(0);
    T* tmp = (T*)(::operator new(size)(size * sizeof(T)));
    if(!tmp)
    {
        cerr << "out of memort" << endl;
        exit(1);
    }
    return tmp;
}
```

内存分配果然是调用`operator new`来执行空间分配, 这里allocate和construct都只是简单的对`operator new`进行封装.

```c++
const int N = 4;
int main()
{
	allocator<string>  alloc;
	auto str_ve = alloc.allocate(N);
	auto p = str_ve;	// vector<string> *p = str_ve;
	alloc.construct(p++);
	alloc.construct(p++, 10, 'a');
	alloc.construct(p++, "construct");
    cout << str_ve[0] << endl;
	cout << str_ve[1] << endl;
	cout << str_ve[2] << endl;

	while(p != str_ve)
	{
		alloc.destroy(--p);
	}
	alloc.deallocate(str_ve, N);

	exit(0);
}
```

输出结果为

```shell
rpz@0505:stl3.0$ ./a.out

aaaaaaaaaa
construct
```

这个程序首先调用`allocate`申请N个大小的空间, 在依次`construct`调用构造函数, 这里就先初始化3个结构, 紧接着通过`destory`调用析构函数, 最后`deallocate`释放申请的空间. 整个过程很容易理解, 但是这里还要深入是dealllocate和destroy两个函数.



### "delete"实现

先是看`destroy`调用析构函数. 而destroy有两个版本.

**版本一:**

需要传入的参数 : **一个指针**

```c++
// 第一版本, 接收指针
template <class T> inline void destroy(T* pointer)
{
    pointer->~T();
}
```

版本一直接就调用了析构函数, 不用过多的分析.



**版本二:**

需要传入的参数 : **两个迭代器**

```c++
// 第二个版本的, 接受两个迭代器, 并设法找出元素的类型. 通过__type_trais<> 找出最佳措施
template <class ForwardIterator>
inline void destroy(ForwardIterator first, ForwardIterator last)
{
  __destroy(first, last, value_type(first));
}

// 接受两个迭代器, 以__type_trais<> 判断是否有traival destructor
template <class ForwardIterator, class T>
inline void __destroy(ForwardIterator first, ForwardIterator last, T*)
{
  typedef typename __type_traits<T>::has_trivial_destructor trivial_destructor;
  __destroy_aux(first, last, trivial_destructor());
}
```

destroy直接调用`__destroy`, 前者只是一个接口, 所以重点是在后者.

分析`__type_traits<>` : 它是用于获取迭代器所指对象的类型,运用traits技法实现的.只要记住我们用来获取对对象类型就可以了. 然后通过类型的不一样选择执行不同的析构调用.



当`__type_traits `为`__false_type`时, 调用的是下面这个函数, 通过迭代所有的对象并调用版本一的函数执行析构函数进行析构. 而这个是被称为`non-travial destructor `

```c++
// 没有non-travial destructor
template <class ForwardIterator>
inline void __destroy_aux(ForwardIterator first, ForwardIterator last, __false_type)
{
  for ( ; first < last; ++first)
    destroy(&*first);
}
```



当`__type_traits `为`__true_type`时, 什么也不做, 因为这样效率很高效, 并不需要执行析构函数. 而这个是被称为`travial destructor `.

```c++
// 有travial destructor
template <class ForwardIterator>
inline void __destroy_aux(ForwardIterator, ForwardIterator, __true_type) {}
```



最后是版本二的特化版, 同样也什么都不用做, 没有必要做析构.

```c++
inline void destroy(char*, char*) {}
inline void destroy(wchar_t*, wchar_t*) {}
```

destroy分为这么几个版本和几个不同的函数执行都是为了提升效率, 较小的调用并不能看出什么, 但是如果是范围析构的话这样不同的选择析构能很节约时间和效率.



讲解完了destory后应该就能明白上面代码循环执行析构函数了.

---

### 小结

这里用一个小小的例子来理解"new"和"delete"运算符, 理解new, delete每步分开执行, 内存释放(deallocate)这里没有讲解, 也只是简单的调用free函数. STL这样做1. 为了效率, 2. 为了构建内存池.

最后将所有的函数进行封装到`allocator` , 所以例子中都是调用的构造析构等都是封装在该类中.

## 一级配置器
# 第一级配置器

### 前言

上一节我们分析了空间配置器对new的配置, 而STL将空间配置器分为了两级, 第一级是直接调用`malloc`分配空间, 调用`free`释放空间, 第二级三就是建立一个内存池, 小于128字节的申请都直接在内存池申请, 不直接调用`malloc`和`free`.

本节我们就先分析第一种空间配置器, 直接调用malloc, free, 而STL有是怎样的封装处理.



### 一级配置器

一级配置器的类. 它无template型别参数. 这里我将public定义的函数和私有成员函数成对分离出来讲解.

```c++
// 一级配置器
template <int inst>
class __malloc_alloc_template
{
  // 这里private里面的函数都是在内存不足的时候进行调用的
  private:
    static void *oom_malloc(size_t);        // 分配不足
    static void *oom_realloc(void *, size_t);   // 重新分配不足
#ifndef __STL_STATIC_TEMPLATE_MEMBER_BUG
    static void (* __malloc_alloc_oom_handler)();   // 内存不足设置的处理例程, 默认设置的是0, 表示没有设置处理例程, 这个处理例程是由用户手动设置的
#endif
  public:
};
```



唯一比较麻烦的就是`set_malloc_handler` 它就是接受一个函数指针, 用来保存用户自定义的处理函数, 如果用户没有设置的话, 默认就设置为0. 因为处理函数会跟后面的内存不足有关系.

```c++
// 这里是模仿c++的set_new_handler. 是由用户自己定义的处理函数, 没有设置默认为0
static void (* set_malloc_handler(void (*f)()))()
{
      void (* old)() = __malloc_alloc_oom_handler;
      __malloc_alloc_oom_handler = f;
      return(old);
}
```

默认将处理例程设置为0, 只有用户自己设置.

```c++
template <int inst>
void (* __malloc_alloc_template<inst>::__malloc_alloc_oom_handler)() = 0;
```



### allocate

`allocate` : 很明显, 这里直接调用`malloc`分配内存, 当内存不足的时候, 程序继续调用`oom_malloc`来选择抛出异常还是一直申请内存, 直到申请内存成功.

```c++
// 在分配和再次分配中, 都会检查内存不足, 在不足的时候直接调用private中相应的函数
static void * allocate(size_t n)
{
      void *result = malloc(n);
      if (0 == result) result = oom_malloc(n);
      return result;
}
```

`oom_malloc`函数功能 : 除非用户自定义了处理例程, 否则当内存不足的时候直接输出内存不足的提示然后直接调用exit(1);
用户定义了处理程序, 函数会一直进行内存申请, 直到申请到内存为止

```c++
template <int inst>
void * __malloc_alloc_template<inst>::oom_malloc(size_t n)
{
  void (* my_malloc_handler)();
  void *result;
    // 用户自定义处理例程, 就一直申请内存, 否则抛出异常
  for (;;)
  {
    my_malloc_handler = __malloc_alloc_oom_handler;
    if (0 == my_malloc_handler) { __THROW_BAD_ALLOC; }
    (*my_malloc_handler)();
    result = malloc(n);
    if (result) return(result);
  }
}
```



### deallocate

一级配置器直接调用free释放内存

```c++
static void deallocate(void *p, size_t /* n */)
{
      free(p);
}
```



### reallocate

下面的函数都是很简单的或是重复的功能, 就一笔带过.

这里reallocate和oom_realloc和上面`allocate`一样的, 这里就不做过多的解释了.

```c++
static void * reallocate(void *p, size_t /* old_sz */, size_t new_sz)
{
      void * result = realloc(p, new_sz);
      if (0 == result) result = oom_realloc(p, new_sz);
      return result;
}
```

```c++
template <int inst>
void * __malloc_alloc_template<inst>::oom_realloc(void *p, size_t n)
{
  void (* my_malloc_handler)();
  void *result;

  for (;;) {
    my_malloc_handler = __malloc_alloc_oom_handler;
    if (0 == my_malloc_handler) { __THROW_BAD_ALLOC; }
    (*my_malloc_handler)();
    result = realloc(p, n);
    if (result) return(result);
  }
}
```



---

程序默认定义`mallo_alloc`函数, 并且设置统一的调用接口, 默认的的接口为第二级配置器

```c++
// 默认将malloc_alloc设为0;
typedef __malloc_alloc_template<0> malloc_alloc;
```



### 统一的接口

定义符合STL规格的配置器接口, 不管是一级配置器还是二级配置器都是使用这个接口进行分配的

```c++
// 定义符合STL规格的配置器接口, 不管是一级配置器还是二级配置器都是使用这个接口进行分配的
template<class T, class Alloc>
class simple_alloc {
  public:
    static T *allocate(size_t n)
    { return 0 == n? 0 : (T*) Alloc::allocate(n * sizeof (T)); }
    static T *allocate(void)
    { return (T*) Alloc::allocate(sizeof (T)); }
    static void deallocate(T *p, size_t n)
    { if (0 != n) Alloc::deallocate(p, n * sizeof (T)); }
    static void deallocate(T *p)
    { Alloc::deallocate(p, sizeof (T)); }
};
```



### 总结

本节对STL的第一级配置器做了分析, STL对malloc和free用函数重新进行了封装, 同时一级还是二级都做了统一的接口. 接下来我们继续分析第二级配置器.

## 二级配置器


### 前言

第一级是直接调用`malloc`分配空间, 调用`free`释放空间, 第二级三就是建立一个内存池, 小于128字节的申请都直接在内存池申请, 不直接调用`malloc`和`free`. 本节分析第二级空间配置器, STL将第二级配置器设置为默认的配置器, 所以只要一次申请的空间不超过128字节就默认在内存池中申请空间, 超过才会调用第一级配置器.



### 第二级配置器

首先先来介绍3个常量.

>   1.   `__ALIGN` : 以8字节进行对齐
>   2.   `__MAX_BYTES` : 二级分配器最大分配的内存大小
>   3.   `__NFREELISTS` : 128字节能分配的的链表个数, 并且从每个链表保存的内存大小都是8的倍数, 而且都比前一个大8字节, 也就是分别是8, 16, 32...128字节

```c++
// 二级配置器
enum {__ALIGN = 8}; // 设置对齐要求. 对齐为8字节, 没有8字节自动补齐
enum {__MAX_BYTES = 128};   // 第二级配置器的最大一次性申请大小, 大于128就直接调用第一级配置器
enum {__NFREELISTS = __MAX_BYTES/__ALIGN};  // 链表个数, 分别代表8, 16, 32....字节的链表
```

再介绍一个宏操作, 这是进行对齐操作, 将不满8的倍数的填充成8的倍数.

```c++
static size_t FREELIST_INDEX(size_t bytes) \
{\
    return (((bytes) + ALIGN-1) / __ALIGN - 1);\
}
```



### 从allocate先切入分析

1.  先判断申请的字节大小是不是大于128字节, 是, 则交给第一级配置器来处理. 否, 继续往下执行
2.  找到分配的地址对齐后分配的是第几个大小的链表.
3.  获得该链表指向的首地址, 如果链表没有多余的内存, 就先填充链表.
4.  返回链表的首地址, 和一块能容纳一个对象的内存, 并更新链表的首地址

```c++
static void * allocate(size_t n)
{
      obj * __VOLATILE * my_free_list;
      obj * __RESTRICT result;

      if (n > (size_t) __MAX_BYTES)
      {
        return(malloc_alloc::allocate(n));
      }
      my_free_list = free_list + FREELIST_INDEX(n);
      result = *my_free_list;
      if (result == 0) 	// 没有多余的内存, 就先填充链表.
      {
        void *r = refill(ROUND_UP(n));
        return r;
      }
      *my_free_list = result -> free_list_link;
      return (result);
};
```

`refill`内存填充.

>   1.  向内存池申请空间的起始地址
>   2.  如果只申请到一个对象的大小, 就直接返回一个内存的大小, 如果有更多的内存, 就继续执行
>   3.  从第二个块内存开始, 把从内存池里面分配的内存用链表给串起来, 并返回一个块内存的地址给用户

```c++
// 内存填充
template <bool threads, int inst>
void* __default_alloc_template<threads, inst>::refill(size_t n)
{
  	int nobjs = 20;
  	char * chunk = chunk_alloc(n, nobjs);             // 向内存池申请空间的起始地址
  	obj * __VOLATILE * my_free_list;
  	obj * result;
  	obj * current_obj, * next_obj;
  	int i;

  	// 如果只申请到一个对象的大小, 就直接返回一个内存的大小
  	if (1 == nobjs) return(chunk);
  	my_free_list = free_list + FREELIST_INDEX(n);

  	// 申请的大小不只一个对象的大小的时候
  	result = (obj *)chunk;
  	// my_free_list指向内存池返回的地址的下一个对齐后的地址
  	*my_free_list = next_obj = (obj *)(chunk + n);
  	// 这里从第二个开始的原因主要是第一块地址返回给了用户, 现在需要把从内存池里面分配的内存用链表给串起来
  	for (i = 1; ; i++)
  	{
	    current_obj = next_obj;
    	next_obj = (obj *)((char *)next_obj + n);
	    if (nobjs - 1 == i)
        {
			current_obj -> free_list_link = 0;
      		break;
    	}
        else
        {
      		current_obj -> free_list_link = next_obj;
    	}
  		}
  	return(result);
}
```



### 再从deallocate结束

>   1.  释放的内存大于128字节直接调用一级配置器进行释放
>   2.  将内存直接还给对应大小的链表就行了, 并不用直接释放内存, 以便后面分配内存的时候快速.

```c++
static void deallocate(void *p, size_t n)
{
      obj *q = (obj *)p;
      obj * __VOLATILE * my_free_list;

      // 释放的内存大于128字节直接调用一级配置器进行释放
      if (n > (size_t) __MAX_BYTES)
      {
        malloc_alloc::deallocate(p, n);
        return;
      }
      my_free_list = free_list + FREELIST_INDEX(n);
      q -> free_list_link = *my_free_list;
      *my_free_list = q;
}
```



### 统一的接口

定义符合STL规格的配置器接口, 不管是一级配置器还是二级配置器都是使用这个接口进行分配的

```c++
// 定义符合STL规格的配置器接口, 不管是一级配置器还是二级配置器都是使用这个接口进行分配的
template<class T, class Alloc>
class simple_alloc {
  public:
    static T *allocate(size_t n)
    { return 0 == n? 0 : (T*) Alloc::allocate(n * sizeof (T)); }
    static T *allocate(void)
    { return (T*) Alloc::allocate(sizeof (T)); }
    static void deallocate(T *p, size_t n)
    { if (0 != n) Alloc::deallocate(p, n * sizeof (T)); }
    static void deallocate(T *p)
    { Alloc::deallocate(p, sizeof (T)); }
};
```



---

### 总结

用链表来保存不同字节大小的内存块, 就很容易的进行维护, 而且每次的内存分配都直接可以从链表或者内存池中获得, 提升了我们申请内存的效率, 毕竟每次调用malloc和free效率是很低的, 特别是很小内存的时候.

**STL默认的就是第二级配置器, 它会自动判断我们使用哪一个配置器.**

## 内存池管理


### 前言

上一节只分析了第二级配置器是由多个链表来存放相同内存大小, 当没有空间的时候就向内存池索取就行了, 却没有具体分析内存池是怎么保存空间的, 是不是内存池真的有用不完的内存, 本节我们就具体来分析一下



### 内存池



**static data template的初始化**

```c++
template <bool threads, int inst>
char *__default_alloc_template<threads, inst>::start_free = 0;	// 内存池的首地址
template <bool threads, int inst>
char *__default_alloc_template<threads, inst>::end_free = 0;	// 内存池的结束地址
template <bool threads, int inst>
size_t __default_alloc_template<threads, inst>::heap_size = 0;	// 多次调用内存池, 就会更多的是给链表分配内存, 这就是一个增量.
```



这里代码注释写的很清楚了, 我就提取出来分析一下吧

>   1.  内存池的大小大于需要的空间, 直接返回起始地址(nobjs默认设置为20, 所以每次调用都会给链表额外的19个内存块)
>   2.  内存池的内存不足以马上分配那么多内存, 但是还能满足分配一个即以上的大小, 那就全部分配出去
>   3.  如果一个对象的大小都已经提供不了了, 先将零碎的内存块给一个小内存的链表来保存, 然后就准备调用malloc申请40块+额外大小的内存块(额外内存块就由heap_size决定), 如果申请失败跳转到步骤4, 成功跳转到步骤6
>   4.  充分利用更大内存的链表, 通过递归来调用他们的内存块
>   5.  如果还是没有内存块, 直接调用一级配置器来申请内存, 还是失败就抛出异常, 成功申请就继续执行
>   6.  重新修改内存起始地址和结束地址为当前申请的地址块, 重新调用chunk_alloc分配内存

```c++
// 内存池
template <bool threads, int inst>
char* __default_alloc_template<threads, inst>::chunk_alloc(size_t size, int& nobjs)
{
  	char * result;
  	size_t total_bytes = size * nobjs;            // 链表需要申请的内存大小
  	size_t bytes_left = end_free - start_free;    // 内存池里面总共还有多少内存空间

	  // 内存池的大小大于需要的空间, 直接返回起始地址
  	if (bytes_left >= total_bytes)
  	{
	    result = start_free;
    	start_free += total_bytes;  // 内存池的首地址往后移
	    return(result);
  	}
  	// 内存池的内存不足以马上分配那么多内存, 但是还能满足分配一个即以上的大小, 那就按对齐方式全部分配出去
  	else if (bytes_left >= size)
  	{
	    nobjs = bytes_left/size;
    	total_bytes = size * nobjs;
    	result = start_free;
    	start_free += total_bytes;  // 内存池的首地址往后移
    	return(result);
  	}
  	else
  	{
	    // 如果一个对象的大小都已经提供不了了, 那就准备调用malloc申请两倍+额外大小的内存
	    size_t bytes_to_get = 2 * total_bytes + ROUND_UP(heap_size >> 4);
    	// Try to make use of the left-over piece.
    	// 内存池还剩下的零头内存分给给其他能利用的链表, 也就是绝不浪费一点.
    	if (bytes_left > 0)
    	{
      		// 链表指向申请内存的地址
      		obj * __VOLATILE * my_free_list = free_list + FREELIST_INDEX(bytes_left);
      		((obj *)start_free) -> free_list_link = *my_free_list;
      		*my_free_list = (obj *)start_free;
    	}
    	start_free = (char *)malloc(bytes_to_get);
    	// 内存不足了
    	if (0 == start_free)
    	{
      		int i;
      		obj * __VOLATILE * my_free_list, *p;
      		// 充分利用剩余链表的内存, 通过递归来申请
      		for (i = size; i <= __MAX_BYTES; i += __ALIGN)
      		{
	        	my_free_list = free_list + FREELIST_INDEX(i);
	        	p = *my_free_list;
	        	if (0 != p)
	            {
	          		*my_free_list = p -> free_list_link;
          			start_free = (char *)p;
	          		end_free = start_free + i;
          			return(chunk_alloc(size, nobjs));
    	    	}
      		}
      		// 如果一点内存都没有了的话, 就只有调用一级配置器来申请内存了, 并且用户没有设置处理例程就抛出异常
      		end_free = 0;	// In case of exception.
      		start_free = (char *)malloc_alloc::allocate(bytes_to_get);
	    }
	    	// 申请内存成功后重新修改内存起始地址和结束地址, 重新调用chunk_alloc分配内存
	    	heap_size += bytes_to_get;
	    	end_free = start_free + bytes_to_get;
    		return(chunk_alloc(size, nobjs));
  	}
}
```

---

### 总结

内存池的存在就是为了能快速的提供我们做需要的内存并且保存多余的空间, 让STL分配空间不再每次都进行malloc和free的操作, 效率又很有保障. 有时用户申请的块更小, 我们也能充分的利用起来. 唯一可能不足的是我们每次只申请`char`个大小, 但是内存池获得的确是8字节的大小.
