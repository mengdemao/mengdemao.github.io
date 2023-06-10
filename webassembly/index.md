# WebAssembly


<!--more-->
WebAssembly 是一种新的编码方式，可以在现代的网络浏览器中运行 － 它是一种低级的类汇编语言，具有紧凑的二进制格式，可以接近原生的性能运行，并为诸如 C / C ++等语言提供一个编译目标，以便它们可以在 Web 上运行。它也被设计为可以与 JavaScript 共存，允许两者一起工作。
<!--more-->

## 安装编译器

```shell
$ git clone https://github.com/juj/emsdk.git
$ cd emsdk
$ ./emsdk install sdk-incoming-64bit binaryen-master-64bit
$ ./emsdk activate sdk-incoming-64bit binaryen-master-64bit

# 使用最新的代码
$ ./emsdk install latest
$ ./emsdk activate latest
```

> 但是因为llvm需要连接github进行下载,常常会失败,因此需要使用mirror,
> 修改`emsdk_manifest.json`,找到https://github.com/llvm/llvm-project.git,修改为
> https://mirrors.tuna.tsinghua.edu.cn/git/llvm-project.git,然后在执行下编译.


工具链编译结束,导入环境变量
```shell
source ./emsdk_env.sh
```

```c
#include <stdio.h>
int main(int argc, char *argv[])
{
  printf("Hello world\r\n");
  return 0;
}
```

```c++
#include <iostream>
int main(int argc, char *argv[])
{
  std::cout << "Hello world\r\n" << std::endl;
  return 0;
}
```

执行编译

```shell
emcc hello.c -s WASM=1 -o hello.html # 执行编译
emrun --no_browser --port 8080 .     # 运行
```

## 入门
