/*
 * =====================================================================================
 *
 *       Filename:  ex22.c
 *
 *    Description:
 *
 *        Version:  1.0
 *        Created:  2013年01月18日 21时11分19秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (),
 *   Organization:
 *
 * =====================================================================================
 */

#include <stdio.h>
#include "ex22.h"
#include "dbg.h"

int THE_SIZE = 1000;

int get_age()
{
	return THE_SIZE;
}

void set_age(int age)
{
	THE_SIZE = age;
}

double update_ratio(double new_ratio)
{
	static double ratio = 1.0;

	double old_ratio = ratio;
	ratio = new_ratio;

	return old_ratio;
}

void print_size()
{
	log_info("I think size is: %d", THE_SIZE);
}
