/* event.h */
#ifndef EVENT_H
#define EVENT_H

#include <stdlib.h>
#include <string.h>
#include "queue.h"

typedef int Event;

int receive_event(Queue *events, const Event *event);
int process_event(Queue *events, int (*dispatch)(Event *event));

#endif
