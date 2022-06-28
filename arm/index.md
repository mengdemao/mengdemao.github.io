# Arm

ARM笔记
====

## ARM体系结构

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

```
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

## ARM指令集

## 翻译结果

### 统一汇编语言 

### 分支指令

### 数据处理指令
| 指令 | 作用 |
| ---- | ----|
| LSL | Logical Shift Left by 1-31 bits.|
| LSR | Logical Shift Right by 1-32 bits. |
| ASR | Arithmetic Shift Right by 1-32 bits. |
| ROR | Rotate Right by 1-31 bits.|
| RRX | Rotate Right with Extend. |

### 状态寄存器访问指令

### 加载存储指令

### 加载存储多条指令

### 杂项指令

### 异常生成与处理指令

### 协处理器指令

### SIMD指令
