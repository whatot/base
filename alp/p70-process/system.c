/*
 * =====================================================================================
 *
 *       Filename:  system.c
 *
 *    Description:  一种不被推荐的创建进程的方法
 *
 *        Version:  1.0
 *        Created:  2013年02月09日 19时16分31秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdlib.h>

int main(int argc, const char **argv)
{
	int return_value;
	return_value = system ("ls -l /");

	return return_value;
}
