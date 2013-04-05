/*
 * =====================================================================================
 *
 *       Filename:  ex24.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年01月31日 20时11分37秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>
#include "dbg.h"

#define MAX_DATA 100

typedef enum Eyecolor {
	BLUE_EYES, GREEN_EYES, BROEN_EYES,
	BLACK_EYES, OTHER_EYES
} Eyecolor;

const char *EYE_COLOR_NAMES[] = {
	"Blue", "Green", "Brown", "Black", "Other"
};

typedef struct Person {
	int age;
	char first_name[MAX_DATA];
	char last_name[MAX_DATA];
	Eyecolor eyes;
	float income;
} Person;


int main(int argc, const char **argv){
	Person you = {.age = 0};
	int i = 0;
	char *in = NULL;

	printf("What's your first name?");
	in = fgets(you.first_name, MAX_DATA-1, stdin);
	check(in != NULL, "Failed to read first_name.");

	printf("What's your last name?");
	in = fgets(you.last_name, MAX_DATA-1, stdin);
	check(in != NULL, "Failed to read last_name.");

	printf("How old are you?");
	int rc = fscanf(stdin, "%d", &you.age);
	check(rc > 0, "You have to enter a nuber.");

	printf("What color are your eyes:\n");
	for(i = 0; i <= OTHER_EYES; i++) {
		printf("%d) %s\n", i+1, EYE_COLOR_NAMES[i]);
	}
	printf("> ");

	int eyes = -1;
	rc = fscanf(stdin, "%d", &eyes);
	check(rc > 0, "You have to enter a number.");

	you.eyes = eyes - 1;
	check(you.eyes <= OTHER_EYES && you.eyes >= 0, "Do it right, that's not an option.");

	printf("How much do you make an hour? ");
	rc = fscanf(stdin, "%f", &you.income);
	check(rc > 0, "Enter a floating point number.");

	printf("------ RESULTS ------\n");

	printf("First Name: %s", you.first_name);
	printf("Last Name: %s", you.last_name);
	printf("Age: %d\n", you.age);
	printf("Eyes: %s\n", EYE_COLOR_NAMES[you.eyes]);
	printf("Income: %f\n", you.income);

	return 0;
error:
	return -1;
}
