/*
 * =====================================================================================
 *
 *       Filename:  echobsd.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月23日 21时55分50秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */
#include <locale.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int main(int argc, const char *argv[])
{
	int nflag;

	(void)setlocale(LC_ALL, "");

	/* This utility may NOT do getopt(3) option parsing.  */
	if (*++argv && !strcmp(*argv, "-n")) {
		++argv;
		nflag = 1;
	}
	else
		nflag = 0;

	while (*argv) {
		(void)printf("%s", *argv);
		if (*++argv)
			(void)putchar(' ');
	}
	if (nflag == 0)
		(void)putchar('\n');
	fflush(stdout);
	if (ferror(stdout))
		exit(1);
	exit(0);
	/* NOTREACHED */
}
