# C++增强笔记


<!--more-->

> 这个是我在学习`C++`语言中所记录的笔记,有可能会存在错误和遗漏,并且我有一点点C语言基础,
> 会大量的提及C语言与C++的不同,从而造成笔记晦涩;
> 另外C++的学习是一个长期且艰难的过程,因此本文进行了切分;

[C++基础笔记]({{< ref "cxx_basic.md ">}})</br>
[C++提高笔记]({{< ref "cxx_enhance.md ">}})</br>
[C++增强笔记]({{< ref "cxx_advance.md ">}})</br>

<!--more-->

## 对象模型

### 汇编实现分析

```c++
class classParent {
private:
    int property;
public:
    int get_property(void)
    {
        return property;
    }
    void set_property(int property)
    {
        this->property = property;
    }
    classParent()
    {
        property = 0;
    }
    ~classParent()
    {
        property = 0;
    }
};

int classParentTest(void)
{
    classParent T;
    T.set_property(1);
    T.get_property();
}
```

分析生成的代码

{{< highlight armasm >}}

assemblyclassParent::get_property():
        push    {r7}
        sub     sp, sp, #12
        add     r7, sp, #0
        str     r0, [r7, #4]
        ldr     r3, [r7, #4]
        ldr     r3, [r3]
        mov     r0, r3
        adds    r7, r7, #12
        mov     sp, r7
        ldr     r7, [sp], #4
        bx      lr

// 高地址 | P1   | 
//       -------
//       | this |
//       -------
// 低地址 |      |
classParent::set_property(int):
        push    {r7}                    // 保存R7
        sub     sp, sp, #12             // 生成2个变量
        add     r7, sp, #0              // R7 = SP
        str     r0, [r7, #4] 
        str     r1, [r7]
        ldr     r3, [r7, #4]

        ldr     r2, [r7]                // r2 <= p1
        str     r2, [r3]                // r2 => [r3]
        
        nop
        adds    r7, r7, #12
        mov     sp, r7
        ldr     r7, [sp], #4
        bx      lr

classParent::classParent() [base object constructor]:
        push    {r7}
        sub     sp, sp, #12
        add     r7, sp, #0
        str     r0, [r7, #4]
        ldr     r3, [r7, #4]
        movs    r2, #0
        str     r2, [r3]
        ldr     r3, [r7, #4]
        mov     r0, r3
        adds    r7, r7, #12
        mov     sp, r7
        ldr     r7, [sp], #4
        bx      lr

classParent::~classParent() [base object destructor]:
        push    {r7}
        sub     sp, sp, #12
        add     r7, sp, #0
        str     r0, [r7, #4]
        ldr     r3, [r7, #4]
        movs    r2, #0
        str     r2, [r3]
        ldr     r3, [r7, #4]
        mov     r0, r3
        adds    r7, r7, #12
        mov     sp, r7
        ldr     r7, [sp], #4
        bx      lr

classParentTest():
        push    {r7, lr}                // 保存堆栈
        sub     sp, sp, #8              // 生成局部变量
        add     r7, sp, #0              // R7 = SP
        adds    r3, r7, #4              // R3 = R7 + 4 ==> T的地址
        mov     r0, r3                  // R0 = R3 = T的地址 ==> this指针
        bl      classParent::classParent() [complete object constructor]

        adds    r3, r7, #4              // R3 = R7 + 4 ==> T的地址
        movs    r1, #1                  // R1 = 1    第二个参数
        mov     r0, r3                  // R0 = this 第一个参数
        bl      classParent::set_property(int)
        
        adds    r3, r7, #4
        mov     r0, r3
        bl      classParent::get_property()
        
        adds    r3, r7, #4
        mov     r0, r3
        bl      classParent::~classParent() [complete object destructor]
        .inst   0xdeff

{{< /highlight >}}

通过上面的代码，我们可以确定出:

+ 对象生成规则
  - 先生成变量
  - 在调用构造函数
+ this指针传递
  - this使用第一个参数传递变量
+ 对象函数调用
  + 隐含传递this指针

## 运行期间

### 函数对象

### 右值引用

## 构造函数

+ 普通构造
+ 拷贝构造
+ 移动构造
+ 委托构造

## 智能指针

### RAII 与引用计数

### std::shared_ptr

### std::unique_ptr

### std::weak_ptr

## 强制类型转换

### C风格的强制转换

### const_cast

### static_cast 

### dynamic_cast 

### reinterpret_cast

## 模板类型

### class templates,

### function templates

### variable templates.
### alias templates
