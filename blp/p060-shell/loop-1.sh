#!/bin/sh

for foo in "bar fud 43"; do
	echo $foo
done
exit 0

# output:双引号意味着是一个string
# bar fud 43

# 去除双引号输出为:
# bar
# fud
# 43
