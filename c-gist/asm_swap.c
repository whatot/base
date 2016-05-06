#include <stdio.h>

void(*swap)() = (void(*)()) "\\x8b\\x44\\x24\\x04\\x8b\\x5c\\x24\\x08\\x8b\\x00\\x8b\\x1b\\x31\\xc3\\x31\\xd8\\x31\\xc3\\x8b\\x4c\\x24\\x04\\x89\\x01\\x8b\\x4c\\x24\\x08\\x89\\x19\\xc3";

int main(){ /* works on GCC 3+4 */
	int a = 37, b = 13;
	swap(&a, &b);

	printf("%d %d\\n",a,b);
	return 0;
}

