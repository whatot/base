
#include "ptest.h"
#include "ch_memory.h"

PT_FUNC(test_malloc2) {

	PT_ASSERT(1);

}

PT_SUITE(suite_memory) {

	PT_REG(test_malloc2);

}
