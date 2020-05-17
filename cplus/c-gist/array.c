#include <stdio.h>

int main()
{
    int i;
    int arr[] = { 0, 3, 4, 28, 1198};
    for(i = 0; i < 5; i++) {
        printf(" arr[i] %p\n" , (void *) (arr+i));
    }
    printf(" *******************\n");
    printf(" %p\n",(void *) (&arr+1));

    return 1;

}

/* output likes
 arr[i] 0xbf8d9c4c
 arr[i] 0xbf8d9c50
 arr[i] 0xbf8d9c54
 arr[i] 0xbf8d9c58
 arr[i] 0xbf8d9c5c
 *******************
 0xbf8d9c60


[want to understand a C array behaviour]
http://stackoverflow.com/questions/16133233/want-to-understand-a-c-array-behaviour

arr is the name for the array. C has a rule that an array expression is converted to a pointer to the first element (except in some particular situations that do not apply here). So arr is converted to &arr[0], which is the address of the first element. This is a pointer to int, so, when you add 1 to it, you get a pointer to the next int. Thus, successive increments to this pointer increment through elements of the array.

In contrast, &arr is a pointer to the array.* The starting address of the array and the starting address of the first element are the same, but they have different types. The type of &arr is ”pointer to array of five int”. When you add 1 to this, you get a pointer to the next array of five int. That is, the address is incremented by the size of an entire array of five int.

Incidentally, it is inappropriate to use a %u specifier to print addresses. You should use %p and convert the addresses to void *, such as:

printf("%p ", (void *) (&arr+1));


* This is one of those special situations: When an array is used with &, the conversion is not done. In &arr, arr is the array, not a pointer, and &arr is its address.
*/
