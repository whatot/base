#ifndef ch_log_h
#define ch_log_h

#include <sys/syslog.h>			/* syslog(3), for parameter in ch_log_init */
#include "ch_config.h"			/* temporarily useless */

/* http://blog.csdn.net/zmxiangde_88/article/details/8025907 */

#define MAX_LOG_LINE (1024)
#define MAX_NAMES (200)

typedef enum {
	log_STDOUT = 1,
	log_SYSLOG = 2,
	log_FILE = 3
} LOG_TYPE;

/*
 * NULL, NULL, NULL --> log_STDOUT
 * log_STDOUT, NULL, NULL --> log_STDOUT
 * log_FILE, file_name, NULL --> log_FILE
 * log_SYSLOG, NULL, facility --> log_SYSLOG
 * */
void ch_log_init(int log_level, LOG_TYPE type, const char *ident,
				 const char *facility);
void ch_log_finish();

void ch_log_write(int level, const char *filename, int lineno,
				  const char *funcname, const char *msgfmt, ...);

#if defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L

#define ch_log_emerg(...) \
	ch_log_write(LOG_EMERG, __FILE__, __LINE__, __func__, __VA_ARGS__)
#define ch_log_alert(...) \
	ch_log_write(LOG_ALERT, __FILE__, __LINE__, __func__, __VA_ARGS__)
#define ch_log_crit(...) \
	ch_log_write(LOG_CRIT, __FILE__, __LINE__, __func__, __VA_ARGS__)
#define ch_log_err(...) \
	ch_log_write(LOG_ERR, __FILE__, __LINE__, __func__, __VA_ARGS__)
#define ch_log_waring(...) \
	ch_log_write(LOG_WARNING, __FILE__, __LINE__, __func__, __VA_ARGS__)
#define ch_log_notice(...) \
	ch_log_write(LOG_NOTICE, __FILE__, __LINE__, __func__, __VA_ARGS__)
#define ch_log_info(...) \
	ch_log_write(LOG_INFO, __FILE__, __LINE__, __func__, __VA_ARGS__)
#define ch_log_debug(...) \
	ch_log_write(LOG_DEBUG, __FILE__, __LINE__, __func__, __VA_ARGS__)

#elif defined __GNUC__

#define ch_log_emerg(format, args...) \
	ch_log_write(LOG_EMERG, __FILE__, __LINE__, __func__, format, ##args)
#define ch_log_alert(format, args...) \
	ch_log_write(LOG_ALERT, __FILE__, __LINE__, __func__, format, ##args)
#define ch_log_crit(format, args...) \
	ch_log_write(LOG_CRIT, __FILE__, __LINE__, __func__, format, ##args)
#define ch_log_err(format, args...) \
	ch_log_write(LOG_ERR, __FILE__, __LINE__, __func__, format, ##args)
#define ch_log_waring(format, args...) \
	ch_log_write(LOG_WARNING, __FILE__, __LINE__, __func__, format, ##args)
#define ch_log_notice(format, args...) \
	ch_log_write(LOG_NOTICE, __FILE__, __LINE__, __func__, format, ##args)
#define ch_log_info(format, args...) \
	ch_log_write(LOG_INFO, __FILE__, __LINE__, __func__, format, ##args)
#define ch_log_debug(format, args...) \
	ch_log_write(LOG_DEBUG, __FILE__, __LINE__, __func__, format, ##args)

#endif

#endif
