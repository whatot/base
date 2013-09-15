#include <stdio.h>
#include <unistd.h>
#include <string.h>

int main() {
	char *c="abcdef";
	char d[]="abcdef";
	char e[]={'a','b','c','d','e','f'};

	printf("sizeof(c):%d\tstrlen(c):%d\n", sizeof(c), strlen(c));
	printf("sizeof(d):%d\tstrlen(d):%d\n", sizeof(d), strlen(d));
	printf("sizeof(e):%d\tstrlen(e):%d\n", sizeof(e), strlen(e));

	return 0;
}

/*
	sizeof(c):4	strlen(c):6
	sizeof(d):7	strlen(d):6
	sizeof(e):6	strlen(e):12
 */

/*
sizeof(c):4,strlen(c):6。
c为指针，sizeof结果和sizeof(int)相同（32位系统中为4）。
c指向了长度为6的字符串字面量，所以strlen(c)返回6。

sizeof(d):7,strlen(d):6。
d是char[]数组，初始化中可以确定它的类型是char[7]，每个元素（char型）占1个字节，所以共7字节。strlen理由同上。
注意初始化列表中的两个双引号之间的字符串字面量填充到数组中时，会自动在最后补0。

sizeof(e):6,strlen(e)。
由初始化可见e是char[6]型的。
strlen(e)理论上来说是不确定的。如果LZ给的代码放在函数体内，定义的是自动变量，这里以字符串形式输出e可以发现后面有d的内容。（不是 GhostWzf 说的“自动分配”，LZ可以自己改变d的内容然后再输出看看。）原因是C实现过程中，自动变量存储在栈空间上，内存是高内存地址向低内存地址分配的。所以可以通过之前定义的变量来估计可能的结果。但是实际一般不会用这种方法，变数太多（例如受到对齐设置的影响）。相对而言，strlen作用于非字符串，结果无法预料，本身就是错误的（非语法错误，是语义错误/逻辑错误），所以，结果无法预料。

*/
