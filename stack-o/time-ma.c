#include "stdio.h"
#include "time.h"

int main()
{
	time_t lt; /*define a longint time varible*/
	lt=time(NULL);/*system time and date*/
	printf("%s \n", ctime(&lt)); /*english format output*/
	printf("%s \n", asctime(localtime(&lt)));/*tranfer to tm*/
	printf("%s \n", asctime(gmtime(&lt))); /*tranfer to Greenwich time*/

	return 1;
} 
