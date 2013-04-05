/*
 * =====================================================================================
 *
 *       Filename:  print-pid.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月09日 19时06分43秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>
#include <unistd.h>

int main(int argc, const char **argv)
{
	printf ("The process ID is %d\n", (int) getpid ());
	printf ("The parent process ID is %d\n", (int) getppid ());

	return 0;
}
