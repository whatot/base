# -*- coding:utf-8 -*-
# %r是debug专用, 它显示的是原始表示出来的字符, 而%s是为了显示给用户.

age = raw_input("How old are you?")
height = raw_input("How tall are you?")
weight = raw_input("How much do you weigh?")

print "So, you're %r old, %r tall and %r heavy." % (
        age, height, weight)
