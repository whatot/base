/*
 *    Description:
 *    type *ptr "一个叫做ptr的type型指针"
 *    *ptr "ptr 所指向地址对应的值"
 *    *(ptr + i) "ptr 所指地址加i的位置的值"
 *    &thing "thing 的地址"
 *    type *ptr = &thing "将名为ptr的type型指针设置到thing的地址"
 *    ptr++ "增进ptr的指向位置"
 *
 *        Created:  2013年01月10日 22时27分38秒
 */

#include <stdio.h>

int main()
{
	//create two arrays we care about
	int ages[] = {23, 43, 12, 89, 2};
	char *names[] = {
		"Alan", "Frank",
		"Mary", "John", "Lisa"
	};
	//safely get the size of ages
	int count = sizeof(ages) / sizeof(int);
	int i = 0;

	//first way using indexing
	for (i = 0; i < count; i++){
		printf("%s has %d years alive.\n",
				names[i], ages[i]);
	}

	printf("--------\n");

	// setup the pointers to the start of the arrays
	int *cur_age = ages;
	char **cur_name = names;

	// second way using pointers
	for (i = 0; i < count; i++){
		printf("%s is %d years old.\n",
				*(cur_name + i), *(cur_age + i));
	}

	printf("--------\n");


	// third way, pointers are just arrays
	for (i = 0; i < count; i++){
		printf("%s is %d years old again .\n",
				cur_name[i], cur_age[i]);
	}

	printf("--------\n");

	// fourth way with pointers in a stupid complex way
	for(cur_name = names, cur_age = ages;
			(cur_age - ages) < count;
			cur_name++, cur_age++)
	{
		printf("%s lives %d years so far.\n",
				*cur_name, *cur_age);
	}

	return 0;
}
