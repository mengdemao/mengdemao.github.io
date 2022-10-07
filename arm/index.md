# Arm笔记


## ARM体系结构

参考文件

[arm指令手册1](./source/arm%E6%8C%87%E4%BB%A4%E6%89%8B%E5%86%8C1.pdf)
[arm指令手册2](./source/arm%E6%8C%87%E4%BB%A4%E6%89%8B%E5%86%8C2.pdf)
[arm指令手册3](./source/arm%E6%8C%87%E4%BB%A4%E6%89%8B%E5%86%8C3.pdf)

### 相关术语

+ 流水线
+ DSP
+ Jazelle
+ ThumbEE
+ Thumb-2
+ TrustZone
+ VFP
+ NEON
+ LAPE
+ big.LITTLE

### 工具链

| 文件名    | 详解                         |
| --------- | ---------------------------- |
| addr2line | 把程序地址转化为文件名和行号 |
| ar        | 建立、修改和提取归档文件     |
| as        | 汇编编译器                   |
| ld        | 链接器                       |
| nm        | 列出文件的符号               |
| objcopy   | 文件个数格式转换             |
| objdump   | 反汇编                       |
| ranlib    | 产生索引,并且保存进入文件中  |
| readelf   | 显示elf文件信息              |
| size      | 列出文件大小                 |
| string    | 打印文件可打印字符串         |
| strip     | 丢弃文件符号                 |

交叉工具链测试

```shell
arm-none-linux-gnueabihf-addr2line      arm-none-linux-gnueabihf-gdb          
arm-none-linux-gnueabihf-ar             arm-none-linux-gnueabihf-gdb-add-index
arm-none-linux-gnueabihf-as             arm-none-linux-gnueabihf-gfortran     
arm-none-linux-gnueabihf-c++            arm-none-linux-gnueabihf-gprof        
arm-none-linux-gnueabihf-c++filt        arm-none-linux-gnueabihf-ld           
arm-none-linux-gnueabihf-cpp            arm-none-linux-gnueabihf-ld.bfd       
arm-none-linux-gnueabihf-dwp            arm-none-linux-gnueabihf-ld.gold      
arm-none-linux-gnueabihf-elfedit        arm-none-linux-gnueabihf-lto-dump     
arm-none-linux-gnueabihf-g++            arm-none-linux-gnueabihf-nm           
arm-none-linux-gnueabihf-gcc            arm-none-linux-gnueabihf-objcopy      
arm-none-linux-gnueabihf-gcc-10.2.1     arm-none-linux-gnueabihf-objdump      
arm-none-linux-gnueabihf-gcc-ar         arm-none-linux-gnueabihf-ranlib       
arm-none-linux-gnueabihf-gcc-nm         arm-none-linux-gnueabihf-readelf      
arm-none-linux-gnueabihf-gcc-ranlib     arm-none-linux-gnueabihf-size         
arm-none-linux-gnueabihf-gcov           arm-none-linux-gnueabihf-strings      
arm-none-linux-gnueabihf-gcov-dump      arm-none-linux-gnueabihf-strip        
arm-none-linux-gnueabihf-gcov-tool
```

### ARMv7处理器模式

| 模式             | 编码  | 功能                          | 安全       | 优先级 |
| ---------------- | ----- | ----------------------------- | ---------- | ------ |
| User (USR)       | 10000 | 大多数运行的非特权模式        | Both       | PL0    |
| FIQ              | 10001 | FIQ中断                       | Both       | PL1    |
| IRQ              | 10010 | IRQ中断                       | Both       | PL1    |
| Supervisor (SVC) | 10011 | 设备重启或者SVC指令           | Both       | PL1    |
| Monitor (MON)    | 10110 | 安全扩展实现                  | only       | PL1    |
| Abort (ABT)      | 10111 | 内存权限异常                  | Both       | PL1    |
| Hyp (HYP)        | 11010 | 虚拟化扩展实现.               | Non-secure | PL2    |
| Undef (UND)      | 11011 | 未定义指令调用                | Both       | PL1    |
| System (SYS)     | 11111 | 特权模式,与用户模式共享寄存器 | Both       | PL1    |

不同的处理器模式上寄存器共享的情况

![ARM处理器](https://raw.githubusercontent.com/mengdemao/picture/master/ArmRegisterSet.png)

### 指令条件码

指令条件码由`CPSR/SPSR`同时确定

![image-20221007201037402](picture/image-20221007201037402.png)


## 寻址方式

+ 立即数寻址
+ 寄存器寻址
+ 寄存器间接寻址
+ 寄存器偏移寻址
+ 寄存器基址变址寻址
+ 批量寄存器寻址
+ 相对寻址
+ 堆栈寻址
+ 块拷贝寻址

### 立即数寻址

立即数为操作数,其中立即数前面加上`#`

```asm
    ;; 立即数寻址
	mov r0, #0	; r0 = 0
	mov r1, #64	; r1 = 64
```

### 寄存器寻址

### 寄存器偏移寻址

寄存器寻址就是直接将寄存器中的数值作为操作数

| 指令 | 作用 |
| ---- | ----|
| LSL | Logical Shift Left by 1-31 bits.|
| LSR | Logical Shift Right by 1-32 bits. |
| ASR | Arithmetic Shift Right by 1-32 bits. |
| ROR | Rotate Right by 1-31 bits.|
| RRX | Rotate Right with Extend. |

```asm
    mov r0, r1	; r0 = r1

	mov r1, #12         ; r1 = 12;
	mov r0, r1, LSL #3	; r0 = r1 << 3 = 512
	mov r0, r1, LSR #3  ; r0 = r1 >> 3 = 8
	mov r0, r1, ASR #3  ; r0 = r1 >> 3 = 8 		不改写符号位
	mov r0, r1, ROR #3	; r1 = 0x80000001
	mov r0, r1, RRX  	; r0 = r1 >> 1 | 空位 = C
```

### 寄存器间接寻址

获取内存地址中的数据,需要使用`LDR/STR`操作符;

首先确定,RAM的地址范围;

打开`simulator->Memory Configuration`

![RAM地址范围](picture/2022-10-07_16-18-50.png)

可以得到RAM的地址0x00900000 --> 0x0097FFFF,

```asm
ldr r0, =0x123		;; r0 = 0x123
ldr r1, =0x900000	;; r1 = 0x900000
str r0, [r1]		;; *((unsigned long *)r1) = r0;
mov r0, 0			;; r0 = 0
ldr r0, [r1]		;; r0 = *((unsigned long *)r1)
```

但是此时只能得到一些奇怪的数据,那如何确定程序是运行正常的?

```assembly
;; 读取异常地址,看看是否会崩溃
;; 如果获取一个异常的地址,
;; 看看会发生什么?
ldr r1, =0x00980000
ldr r0, [r1]
```

程序立即崩溃, 我们的测试是正确的;

![image-20221007163149155](picture/image-20221007163149155.png)



### 寄存器基址变址寻址

```assembly
LDR    R0,	[R1,#0X0F] ;; R0 = *(unsigned long *)(R1 + 0x0f)
LDR    R0,	[R1],#4	   ;; R0 = *(unsigned long *)(R1 + 0x04)
LDR    R0,	[R1,R2]	   ;; R0 = *(unsigned long *)(R1 + R2)
```

### 批量寄存器寻址

批量寻址来自于批量操作符`STM/LDM`,

```assembly
LDMIA    R0,{R1,R2,R3,R4}
STMIA    R0,{R3-R5,R10}
```

### 堆栈寻址

```assembly
LDMFD    SP!,{R1-R7,LR}
STMFD    SP!,{R1-R7,LR}
```

## 指令集

### 数据处理

#### 数据传送

+ mov
+ mvn

#### 算数指令

+ add
+ sub
+ rsb
+ adc
+ rsc

#### 逻辑指令

+ and
+ orr
+ eor
+ bic

### 分支指令

### 状态寄存器访问指令

### 杂项指令

### 异常生成与处理指令

### 协处理器指令

### SIMD指令

## 仿真工具

### IAR仿真设置

新建工程

![image-20221007165810492](picture/image-20221007165810492.png)

设置仿真

![configure-project](picture/configure-project.png)

构建仿真

![debug-project](picture/debug-project.png)
