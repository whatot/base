/*
 *        Created:  2013年02月04日 13时21分52秒
 */

#include <stdio.h>
#include <stdlib.h>

int main()
{
	char* server_name = getenv ("SERVER_NAME");
	if (server_name == NULL)
		/* The SERVER_NAME environment variable was not set. 
		 * use the default*/
		server_name = "server.my-company.com";

	printf ("accessing server %s\n", server_name);
	/* Access the server here... */

	return 0;
}

/* $ export SERVER_NAME=backup-server.elsewhere.net */
/* $ unset SERVER_NAME */
