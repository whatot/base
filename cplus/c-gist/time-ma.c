#include <stdio.h>
#include <time.h>

int main()
{
    time_t lt = time(NULL);  /* current time */
    printf("localtime:\t%s", ctime(&lt));  /* english format output */
    printf("%s", asctime(localtime(&lt)));  /* tranfer to tm */
    printf("UTC time:\t%s", asctime(gmtime(&lt)));   /* tranfer to Greenwich time */

    return 0;
}
