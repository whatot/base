#include <stdio.h>

int main()
{
	int i = 43;

	printf("%d\n",i);
	printf("%d\n",printf("%d",i));

	printf("%d\n",printf("%d",printf("%d",i)));

	/* Upon successful return, these functions return the number of characters printed (excluding the null byte used to end output to strings).
	最里面的printf输出43，返回2
	倒数第二个printf输出上一个printf的返回值2,返回1
	最外层printf输出倒数第二个printf的返回值1,它的返回值没被输出
	所以，打印的是4321 */

	return 0;
}


