/*
 *        Created:  2013年02月04日 13时12分12秒
 */
#include <stdio.h>

/* The ENVIRON variable contains the environment. */
extern char** environ;

int main()
{
	char** var;
	for(var = environ; *var != NULL; ++var) {
		printf ("%s\n", *var);
	}

	return 0;
}

/* use setenv and unsetenv to set or clear environment variables*/
