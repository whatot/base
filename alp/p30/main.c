/*
 * =====================================================================================
 *
 *       Filename:  main.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月03日 12时04分08秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>
#include <stdlib.h>
#include "reciprocal.hpp"

int main(int argc, const char **argv)
{
	int i;

	i = atoi (argv[1]);
	printf ("The reciprocal of %d is %g\n", i, reciprocal (i));

	return 0;
}
