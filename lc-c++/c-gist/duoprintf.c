#include <stdio.h>

int main()
{
    int i = 43;

    printf("%d\n",i);
    printf("%d\n",printf("%d",i));

    printf("%d\n",printf("%d",printf("%d",i)));

    /* Upon successful return, these functions return the number of characters printed (excluding the null byte used to end output to strings).
    �������printf���43������2
    �����ڶ���printf�����һ��printf�ķ���ֵ2,����1
    �����printf��������ڶ���printf�ķ���ֵ1,���ķ���ֵû�����
    ���ԣ���ӡ����4321 */

    return 0;
}


