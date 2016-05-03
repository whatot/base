# 编译链接全过程

## 1.预编译

`$gcc -E hello.c -o hello.i`
or
`$cpp hello.c > hello.i`

## 2.编译

`$gcc -S hello.i -o hello.s`
现在gcc将预编译与编译合并为一个步骤，使用ccl
`$gcc -S hello.c -o hello.s`

## 3.汇编

* `$as hello.s -o hello.o`
* `$gcc hello.s -o hello.o`
* `$gcc -c hello.c -o hello.o`

## 4.链接

`$ld -static crt1.o crti.o crtbeginT.o hello.o -start-group -lgcc -lgcc_eh -lc --end-group crtend.o crtn.o`

# 编译过程

`**Source Code**---Scanner(lex词法扫描)--->**Tokens记号**---Grammar Parser(yacc语法分析器)(上下文无关语法context-free)--->**Syntax Tree**---Semantic Analyzer(语义分析器)--->**Commented Syntax Tree**---Source Code Optimizer(源码级优化器)--->**Intermediate Representation**(中间代码)---Code Generator--->**Target Code**---Code Optimizer--->**Final Target Code**`

yacc = Yet Another Compiler Compiler
编译器只能分析静态语义(Static Semantic)(在编译期间可以确定的语义)

> 动态语义(Dynamic Semantic)：只有在运行期间才能确定的语义

中间代码使得编译器可以被分成前端与后端，前端负责产生机器无关的中间代码，后端将中间代码转换成目标机器代码。

# Linking 链接

链接的主要内容就是把各个模块之间相互引用的部分都处理好，使得各个模块之间能够正确地链接

# 目标文件

1. 文件头(包括段表Section Table)

* 段表其实是一个描述文件中各个段的数组。段表描述了文件中各个段在文件中的偏移位置以及段的属性等，从段表里可以得到每个段的所有信息。
* 执行语句保存在.text段
* 已初始化的全局变量和局部静态变量保存在.data段
* 未初始化的全局变量和局部静态变量默认值为0,保存在.bss段,只是预留位置没有内容

## 指令与数据分离的原因

1. 分离后便于在虚存区域设置不同的读写权限
2. cpu中指令与数据cache分离
3. 共享内存--指令

## gcc自定义段

__attribute__((section("FOO"))) int golbal = 42;
__attribute__((section("BAR"))) void foo() {}

## strip命令-去掉ELF文件中的调试信息

```
$ strip foo
```
