package main

/*

#include <stdio.h>

// The gateway function
int callOnMeGo_cgo(int in)
{
    printf("C.callOnMeGo_cgo(): called with arg = %d\n", in);
    int callOnMeGo(int);
    return callOnMeGo(in);
}
*/
import "C"
