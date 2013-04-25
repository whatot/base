#include <stdio.h>
int main()
{
	printf("Enter 10 numbers and sort it(from small to large):\n");
	int t, i, k, a[10];

	for (i = 0; i < 10; i++)
		scanf("%d", &a[i]);

	for (i = 0; i < 9; i++)
		for (k = 0; k < 9 - i; k++)
			if (a[k] > a[k + 1]) {
				t = a[k];
				a[k] = a[k + 1];
				a[k + 1] = t;
			}

	for (i = 0; i < 10; i++)
		printf("%d ", a[i]);

	printf("\n");
	return 0;
}
