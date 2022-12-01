# WebAssembly


## 前言

## 安装工具链

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

## 入门

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



