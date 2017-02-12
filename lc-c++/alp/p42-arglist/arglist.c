/*
 * =====================================================================================
 *
 *       Filename:  arglist.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月03日 13时32分59秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>

int main(int argc, const char **argv)
{
	printf("The name of this program is '%s'.\n", argv[0]);
	printf("This program was invoked with %d arguments.\n", argc - 1);

	/* Were any command-line arguments specified? */
	if(argc > 1) {
		/* Yes, print them. */
		int i;
		printf("The arguments are:\n");
		for(i = 0; i < argc; ++i) {
			printf("\t%s\n", argv[i]);
		}
	}

	return 0;
}
