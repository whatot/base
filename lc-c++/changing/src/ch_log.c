#include "ch_log.h"

#include <sys/time.h>			/* gettimeofday */
//#include <time.h>  /* localtime_r */
#include <stdio.h>				/* fclose, fwrite, fflush */
#include <strings.h>			/* strcasecmp */
#include <pthread.h>			/* for ch_log rw safety */
#include "ch_sds.h"				/* replace the old string-operation */
#include "ch_memory.h"			/* calloc1 */

/* fix the warning about `implicit declaration of function 'localtime_r'` */
extern struct tm *localtime_r(const time_t * timep, struct tm *result);

/* global_log but static */
typedef struct ch_log_s {
	int log_level;
	LOG_TYPE type;				/* bind output way */
	FILE *file;
	pthread_mutex_t *file_mutex;
} *ch_log_t;

static ch_log_t global_log;

typedef struct {
	const char *facility;
	int number;
} log_facility_t;

static log_facility_t _log_facilities[] = {
	{"local0", LOG_LOCAL0},
	{"local1", LOG_LOCAL1},
	{"local2", LOG_LOCAL2},
	{"local3", LOG_LOCAL3},
	{"local4", LOG_LOCAL4},
	{"local5", LOG_LOCAL5},
	{"local6", LOG_LOCAL6},
	{"local7", LOG_LOCAL7},
	{NULL, -1}
};

static int _log_facility(const char *facility)
{
	log_facility_t *lp;

	if (facility == NULL) {
		return -1;
	}

	for (lp = _log_facilities; lp->facility; lp++) {
		if (!strcasecmp(lp->facility, facility)) {
			break;
		}
	}

	return lp->number;
}

/* LOG_TYPE: (1,2,3) log_STDOUT,log_SYSLOG,log_FILE */
static ch_log_t ch_log_new(int log_level, LOG_TYPE type, const char *ident,
						   const char *facility)
{
	ch_log_t log;
	int fnum = 0;

	log = (ch_log_t) calloc1(sizeof(struct ch_log_s));

	pthread_mutex_init(log->file_mutex, NULL);

	log->log_level = log_level;
	log->type = type;

	if (type == log_SYSLOG) {
		fnum = _log_facility(facility);
		if (fnum < 0) {
			fnum = LOG_LOCAL7;
		}
		openlog(ident, LOG_PID, fnum);
		return log;
	} else if (type == log_STDOUT) {
		log->file = stdout;
		return log;
	} else {
		log->file = fopen(ident, "a+");
		if (log->file == NULL) {
			log->type = log_STDOUT;
			log->file = stdout;
		}
		return log;
	}
}

void ch_log_init(int log_level, LOG_TYPE type, const char *ident,
				 const char *facility)
{
	global_log = ch_log_new(log_level, type, ident, facility);
	ch_log_info("ch_log init successfully");
	return;
}

static void ch_log_free(ch_log_t log)
{
	if (log->type == log_SYSLOG) {
		closelog();
	} else if (log->type == log_FILE) {
		pthread_mutex_lock(global_log->file_mutex);
		fclose(log->file);
		pthread_mutex_unlock(global_log->file_mutex);
		pthread_mutex_destroy(global_log->file_mutex);
	}

	free2(log);
}

void ch_log_finish()
{
	ch_log_free(global_log);
	return;
}

static sds get_nowtime_sds()
{
	/* product now_time */
	sds time_sds;
	struct timeval tv;
	struct tm tm;

	gettimeofday(&tv, NULL);
	localtime_r(&tv.tv_sec, &tm);

	time_sds = sdscatprintf(sdsempty(), "%4d-%02d-%02d %02d:%02d:%02d.%06d ",
							tm.tm_year + 1900,
							tm.tm_mon,
							tm.tm_mday,
							tm.tm_hour,
							tm.tm_min, tm.tm_sec, (int)(tv.tv_usec));

	return time_sds;
}

static sds generate_common_sds(const char *filename, int lineno,
							   const char *funcname)
{
	sds com_message;

	/* combine the filename,funcname,lineno to com_message */
	com_message =
		sdscatprintf(sdsempty(), "%s:%d %s: ", filename, lineno, funcname);

	return com_message;
}

/*
 *  level: 0-7
 *  LOG_EMERG：紧急状况
    LOG_ALERT：高优先级问题，比如说数据库崩溃等，必须要立即采取反应行动
    LOG_CRIT：重要状况发生，比如硬件故障
    LOG_ERR：错误发生
    LOG_WARNING：警告发生
    LOG_NOTICE：一般状况，需要引起注意
    LOG_INFO：信息状况
    LOG_DEBUG：调试消息
 */
static const char *_log_level[] = {
	"emerg",
	"alert",
	"criti",
	"error",
	"warn ",
	"notice",
	"info ",
	"debug"
};

void ch_log_write(int level, const char *filename, int lineno,
				  const char *funcname, const char *msgfmt, ...)
{
	if (level > global_log->log_level) {
		return;
	}
	va_list args;
	sds message;				/* shouldn't use sdsnew(NULL) to initialize */
	sds time_sds = get_nowtime_sds();
	sds com_message = generate_common_sds(filename, lineno, funcname);

	/* using sdsempty() init message and add level */
	message = sdscatprintf(sdsempty(), "[%s] ", _log_level[level]);

	/* deal with the situation for syslog */
	if (global_log->type == log_SYSLOG) {
		message = sdscatsds(message, com_message);

		va_start(args, msgfmt);
		message = sdscatvprintf(message, msgfmt, args);
		va_end(args);

		syslog(level, "%s", message);
		return;
	}

	/* put time, com_message into message */
	message = sdscatsds(message, time_sds);
	message = sdscatsds(message, com_message);

	/* put the main body of the message */
	va_start(args, msgfmt);
	message = sdscatvprintf(message, msgfmt, args);
	va_end(args);

	/* every single line need a "\n" */
	message = sdscatlen(message, "\n", 1);

	/* write the message into global_log->file */
	pthread_mutex_lock(global_log->file_mutex);
	fwrite(message, sdslen(message), 1, global_log->file);
	fflush(global_log->file);
	pthread_mutex_unlock(global_log->file_mutex);

	sdsfree(time_sds);
	sdsfree(com_message);
	sdsfree(message);
	return;
}
