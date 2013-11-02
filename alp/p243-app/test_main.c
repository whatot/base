
#include "ptest.h"

extern void suite_common(void);


int main() {

	pt_add_suite(suite_common);

	pt_run();

}
