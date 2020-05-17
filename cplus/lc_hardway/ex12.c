/*
 * =====================================================================================
 *
 *       Filename:  ex12.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年01月10日 19时34分20秒
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
	int i = 0;

	if(argc == 1){
		printf("You only have one argument. You suck.\r");
	}else if(argc > 1 && argc < 4){
		printf("Here's your arguments:\n");

		for(i = 0; i < argc; i++){
			printf("%s ",argv[i]);
		}
		printf("\n");
	}else{
		printf("You have too many arguments. You suck.\n");
	}

	return 0;
}
