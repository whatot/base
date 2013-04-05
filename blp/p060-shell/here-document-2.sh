#!/bin/sh 

ed a_text_file << !PunkyStuff
3
d
.,\$s/is/was
w
q
!PunkyStuff!

exit 



#输入: a_text_file
#That is line 1
#That is line 2
#That is line 3
#That is line 4
#
#输出: a_text_file
#That is line 1
#That is line 2
#That was line 4

