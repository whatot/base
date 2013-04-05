/*
 * =====================================================================================
 *
 *       Filename:  ex8.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年01月09日 15时37分06秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>

int main(int argc, char *argv[])
{
	int areas[] = {10,'a',13,14,20};//此处可以使用当个字符代替
	char name[] = "Zed";
	char full_name[] = {
		'Z', 'e', 'd',
		' ', 'A', '.', ' ',
		'S', 'h', 'a', 'w', '\0'
	};

	//WARNING: ON some systems you may have to change the
	//%ld in this code to a %u since it will use unsigned ints
	printf("The size of an int: %u\n", sizeof(int));
	printf("The size of areas (int[]): %u\n",
			sizeof(areas));//输出数组占据的内存大小,而不只是数组头部
	printf("The number of ints in areas: %u\n",
			sizeof(areas) / sizeof(int));//没有包括最后的'\0'
	printf("The first area is %d, the 2nd is %d.\n",
			areas[0], areas[1]);
	printf("The size of a char: %u\n", sizeof(char));
	printf("The size of name (char[]): %u\n",
			sizeof(name));
	printf("The number of chars: %u\n",
			sizeof(name) / sizeof(char));//包括了最后的字符'\0'
	
	printf("The size of full_name (char[]): %u\n",
			sizeof(full_name));
	printf("The number of chars: %u\n",
			sizeof(full_name) / sizeof(char));

	printf("name=\"%s\" and full_name=\"%s\"\n",
			name,full_name);

	return 0;
}
