/*
 *        Created:  2013年02月05日 17时08分02秒
 */

int main()
{
	return f ();
}

//gcc -o libtest.o -c test.c
//ar cr libtest.a libtest.o
//gcc -o app.o -c app.c
//
//$ gcc -o app -L. -ltest app.o
//app.o: In function `main':
//app.c:(.text+0x7): undefined reference to `f'
//collect2: error: ld returned 1 exit status
//
//库编译进最终执行文件
//$ gcc -o app app.o -L. -ltest
//$ ./app
//$ echo $?
//3
//
//
//
//Position-Independent Code (PIC)
//使用test1.c作为共享库(shared object)的一部分
//$ gcc -c -fPIC test1.c
//合并objects生成共享库文件libtest.so
//$ gcc -shared -fPIC -o libtest.so test1.o test2.o
//链接共享库生成最终执行文件
//$ gcc -o app app.o -L. -ltest
//当目录同时存在libtest.so与libtest.a时,选择shared library version
//当使用-static时,使用libtest.a
//$ gcc -static -o app app.o -L. -ltest
//
//增加搜索目录
//$ gcc -o app app.o -L. -ltest -Wl,-rpath,/usr/local/lib
//
//$ gcc -o compute compute.c -lm
