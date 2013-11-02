#include "ptest.h"
#include "server.h"
#include <stdlib.h>

PT_FUNC (test_xmalloc) {
	return;
}

PT_FUNC (test_xrealloc) {
	return;
}

PT_FUNC (test_xstrdup) {
	char* source = "ciuiucdjscnksj";
	char* dest = xstrdup (source);

	PT_ASSERT_STR_EQ (source, dest);
	free(dest);

	return;
}

PT_FUNC (test_system_error) {
	return;
}

PT_SUITE (suite_common) {
	
	PT_REG (test_xmalloc);
	PT_REG (test_xrealloc);
	PT_REG (test_xstrdup);
	PT_REG (test_system_error);

}
