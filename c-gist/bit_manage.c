#include <stdio.h>

/*
 * 位域虽然简单好用，但使用时需要注意：
1) 如果相邻位域字段的类型相同，且其位宽之和小于类型的sizeof大小，则后面的字段将紧邻前一个字段存储，直到不能容纳为止；
2) 如果相邻位域字段的类型相同，但其位宽之和大于类型的sizeof大小，则后面的字段将从新的存储单元开始，其偏移量为其类型大小的整数倍；
3) 整个结构体的总大小为最宽基本类型成员大小的整数倍。
4) 如果位域字段之间穿插着非位域字段，则不进行压缩；
5) 整个结构体的总大小为最宽基本类型成员大小的整数倍。
 * */

int main()
{
    struct test {
        unsigned a:10;
        unsigned b:10;
        unsigned c:6;
        unsigned:2;                /*this two bytes can't use */
        unsigned d:4;
    } data, *pData;

    data.a = 0x177;
    data.b = 0x111;
    data.c = 0x7;
    data.d = 0x8;

    pData = &data;
    printf("data.a = %x;  data.b = %x;  data.c = %x; data.d = %x \n",
           pData->a, pData->b, pData->c, pData->d);
    /* 位域可以使用指针 */

    printf("sizeof(data) = %ld \n", sizeof(data));    /* 4 bytes ，最常用的情况 */

    struct testLen {
        char a:5;
        char b:5;
        char c:5;
        char d:5;
        char e:5;
    } len;

    printf("sizeof(len) = %ld \n", sizeof(len));    /* 5bytes 规则2 */

    struct testLen1 {
        char a:5;
        char b:2;
        char d:3;
        char c:2;
        char e:7;
    } len1;
    printf("sizeof(len1) = %ld \n", sizeof(len1));    /* 3bytes 规则1 */

    struct testLen2 {
        char a:2;
        char:3;
        char b:7;
        long d:20;                /* 4bytes */
        char e:4;
    } len2;
    printf("sizeof(len2) = %ld \n", sizeof(len2));
    /* 12bytes 规则3,4,5，总长为4的整数倍，2+3 占1byte，b占1bye
     * 由于与long对其，2+3+7 占4字节，后面d与e进行了优化占一个4字节
     */

    struct testLen3 {
        char a:2;
        char:3;
        char b:7;
        long d:30;
        char e:4;
    } len3;
    printf("sizeof(len3) = %ld \n", sizeof(len3));
    /* 12bytes 规则3，4，5，总长为4的整数倍，2+3 占1byte，b占1byte
     * 由于与long对其，2+3+7 占4字节，后面 d占一个4字节，
     * 为了保证与long对其, e独占一个4字节
     */

    return 0;
}
