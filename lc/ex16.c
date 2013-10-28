/*
 *        Created:  2013年01月11日 12时49分39秒
 */

#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct Person {
	char *name;
	int age;
	int height;
	int weight;
};

struct Person *Person_create(char *name, int age, int height, int weight)
{
	struct Person *who = (struct Person *)malloc(sizeof(struct Person));
	assert(who != NULL);

	who->name = strdup(name);
	who->age = age;
	who->height = height;
	who->weight = weight;

	return who;
}

void Person_destroy(struct Person *who)
{
	assert(who != NULL);

	free(who->name);
	free(who);
}

void Person_print(struct Person *who)
{
	printf("Name: %s\n", who->name);
	printf("\tAge: %d\n", who->age);
	printf("\tHeight: %d\n", who->height);
	printf("\tWeight: %d\n", who->weight);
}

int main()
{
	// make two people structures
	char joe_name[20] = "Joe Alex";
	char frank_name[20] = "Frank Blank";

	struct Person *joe = Person_create(
			joe_name, 32, 64, 140);

	struct Person *frank = Person_create(
			frank_name, 20, 72, 180);

	// print them out and where they are in memory
	printf("Joe is at memory location %p:\n", joe);
	Person_print(joe);

	printf("Frank is at memory location %p:\n", frank);
	Person_print(frank);

	// make everyone age 20 years and print them again
	joe->age += 20;
	joe->height -= 2;
	joe->weight += 40;
	Person_print(joe);

	frank->age += 20;
	frank->weight += 20;
	Person_print(frank);

	// destroy them both so we clean up
	Person_destroy(joe);
	Person_destroy(frank);

	return 0;
}
