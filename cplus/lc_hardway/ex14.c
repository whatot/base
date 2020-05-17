/*
 * =====================================================================================
 *
 *       Filename:  ex14.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年01月10日 19时59分21秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>
#include <ctype.h>

extern int isblank(int c);


/* forward declarations */
int can_print_it(char ch);
void print_letters(char arg[]);

void print_arguments(int argc, char *argv[])
{
	int i = 0;

	for(i = 0; i < argc; i++){
		print_letters(argv[i]);
	}
}

void print_letters(char arg[])
{
	int i = 0;

	for(i = 0; arg[i] != '\0'; i++){
		char ch = arg[i];

		if(can_print_it(ch)){
			printf("'%c' == %d ", ch, ch);
		}
	}

	printf("\n");
}

int can_print_it(char ch)
{
	return isalpha(ch) || isblank(ch);
}


int main(int argc, char *argv[])
{
	print_arguments(argc, argv);
	return 0;
}
