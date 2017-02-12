#include "ch_exit.h"

#include "ch_log.h"
#include "ch_error.h"

void ch_exit()
{
	ch_log_emerg("ch_log emergency");
	ch_log_err("ch_log error");

	ch_log_info("ch_log to exit ...");
	ch_log_finish();

	system_error_exit();
}
