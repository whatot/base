#include <stdio.h>
#include <stdlib.h>
#include <string.h>


typedef unsigned char *byte_pointer;

void show_bytes(byte_pointer start, int len) {
	int i;
	for (i = 0; i < len; i++)
		printf(" %.2x",start[i]);
	printf("\n");
}

void show_int(int x){
	show_bytes((byte_pointer) &x, sizeof(int));
}

void show_float(float x){
	show_bytes((byte_pointer) &x, sizeof(float));
}

void show_pointer(void *x) {
	show_bytes((byte_pointer) &x, sizeof(void *));
}

void test_show_bytes(int val) {
	int ival = val;
	float fval = (float) ival;
	int *pval = &ival;
	show_int(ival);
	show_float(fval);
	show_pointer(pval);
}

int main(int argc, char *argv[]) {
	int val = 0x87654321;
	byte_pointer valp = (byte_pointer)&val;
	
	show_bytes(valp,1);
	show_bytes(valp,2);
	show_bytes(valp,3);

//	test_show_bytes(val);
	
	const char *s = "abcdef";
	show_bytes((byte_pointer) s, strlen(s));

	printf("0x%x\n",0x87654321^0xFFFFFF00); //末位字节不变,其余取反
	printf("0x%x\n",0x87654321|0x000000FF); //末位字节变全1,其余不变
}
