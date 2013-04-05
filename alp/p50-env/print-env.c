/*
 * =====================================================================================
 *
 *       Filename:  print-env.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月04日 13时12分12秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */
#include <stdio.h>

/* The ENVIRON variable contains the environment. */
extern char** environ;

int main(int argc, const char **argv)
{
	char** var;
	for(var = environ; *var != NULL; ++var) {
		printf ("%s\n", *var);
	}

	return 0;
}

/* use setenv and unsetenv to set or clear environment variables*/
