/* 原题是给出两个数值X和Y，统计在这个区间里的回文数，
 * 并且要求它们的平方根也是回文数。
 * 其中 1<= x <= y < 10^14
 *
 * 为了简化问题，我这里只求出1到1014之间符合要求的回文数。
 * 其实有了这些数据，解决原题也很简单了，总共只有39个结果，统计一下也很快。 */

#include <stdlib.h>
#include <stdio.h>

typedef long long int64;
typedef enum {
	false = 0,
	true = !false
} bool;

int64 palindromify(int64 n, bool oddp)
{
	int64 a = n;

	if (oddp) {
		a /= 10;
	}
	while (n > 0) {
		a = a * 10 + n % 10;
		n /= 10;
	}
	return a;
}

bool is_palindrome(int64 n)
{
	int64 a = 0;
	int64 b = n;
	while (n > 0) {
		a = a * 10 + n % 10;
		n /= 10;
	}
	return a == b;
}

void check(int64 n)
{
	n *= n;
	if (is_palindrome(n)) {
		printf("%lld\n", n);
	}
}

int main()
{
	int64 i;

	for (i = 1ll; i < 10000ll; i++) {
		check(palindromify(i, true));
		if (i < 1000ll) {
			check(palindromify(i, false));
		}
	}

	return EXIT_SUCCESS;
}
