
#include "ptest.h"

void suite_memory(void);

int main() {

	pt_add_suite(suite_memory);

	pt_run();

}
