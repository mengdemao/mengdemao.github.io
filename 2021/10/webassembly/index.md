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

## 将wasm运行在hugo静态网页中

使用`-o hello.html`编译后,除了`hello.html`之外,还会生成`hello.js`和`hello.wasm`两个文件:

+ `hello.js`: 加载并运行wasm模块的胶水代码
+ `hello.wasm`: 编译生成的二进制模块

hugo是静态网页生成器,不适合直接使用生成的`hello.html`,只需要将`hello.js`和`hello.wasm`复制到hugo的`static`目录,然后在文章中通过`wasm` shortcode加载:

```shell
$ mkdir -p static/wasm
$ cp hello.js hello.wasm static/wasm/
```

```go-html-template
{{</* wasm "hello" */>}}
```

如果需要自定义存放目录,可以传递第二个参数指定:

```go-html-template
{{</* wasm "hello" "lib" */>}}
```

上面的写法会生成`<script src="/lib/hello.js" defer></script>`.

`wasm` shortcode会生成`<script src="/wasm/hello.js" defer></script>`标签,等价于直接在文章中插入:

```html
<script src="/wasm/hello.js" defer></script>
```

`hello.js`会自动加载同目录下的`hello.wasm`并执行`main`函数,`printf`的输出会显示在浏览器的控制台中.

如果只需要在js中调用某些函数,而不是直接执行`main`,可以在编译时导出指定的函数:

```shell
$ emcc hello.c -s WASM=1 -s EXPORTED_FUNCTIONS='["_add"]' -o hello.js
```

然后在js代码中通过`Module`调用:

```javascript
Module.onRuntimeInitialized = function() {
    console.log(Module._add(1, 2));
};
```

注意: 被导出的函数需要保证不被优化删除,可以使用`EMSCRIPTEN_KEEPALIVE`修饰.
