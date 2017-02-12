#!/usr/bin/python2
# -*- coding:utf-8 -*-

'''
the code below has some error that need to understand and fix

http://blog.csdn.net/jgood/article/details/5493824

    urllib中还提供了一些辅助方法，用于对url进行编码、解码。
    url中是不能出现一些特殊的符号的，有些符号有特殊的用途。
    我们知道以get方式提交数据的时候，会在url中添加key=value这样的字符串，
    所以在value中是不允许有'='，因此要对其进行编码；
    与此同时服务器接收到这些参数的时候，要进行解码，还原成原始的数据。
    这个时候，这些辅助方法会很有用：

urllib.quote(string[, safe])：对字符串进行编码。参数safe指定了不需要编码的字符;
urllib.unquote(string) ：对字符串进行解码；
urllib.quote_plus(string [ , safe ] ) ：与urllib.quote类似，
        但这个方法用'+'来替换' '，而quote用'%20'来代替' '
urllib.unquote_plus(string ) ：对字符串进行解码；
urllib.urlencode(query[, doseq])：
        将dict或者包含两个元素的元组列表转换成url参数。
        例如:字典{'name': 'dark-bull', 'age': 200}将被转换为
                "name=dark-bull&age=200"
urllib.pathname2url(path)：将本地路径转换成url路径；
urllib.url2pathname(path)：将url路径转换成本地路径；
'''

import urllib


data = 'name = ~a+3'

data1 = urllib.quote(data)
print(data1)    # result: name%20%3D%20%7Ea%2B3
print(urllib.unquote(data1)     # result: name = ~a+3)

data2 = urllib.quote_plus(data)
print(data2)    # result: name+%3D+%7Ea%2B3
print(urllib.unquote_plus(data2))     # result: name = ~a+3

data3 = urllib.urlencode({'name': 'dark-bull', 'age': 200})
print(data3)    # result: age=200&name=dark-bull

data4 = urllib.pathname2url(r'd:/a/b/c/23.php')
print(data4)    # result: ///D|/a/b/c/23.php
print(urllib.url2pathname(data4))    # result: D:/a/b/c/23.php
