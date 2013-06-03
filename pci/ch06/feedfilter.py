#!/usr/bin/python2
# -*- coding:utf-8 -*-

import feedparser
# import re


# 接受一个博客订阅源的 URL 文件名并对其内容项进行分类
def read(feed, classifier):
    # 得到订阅源的内容项并遍历循环
    f = feedparser.parse(feed)
    for entry in f['entries']:
        print()
        print('------')
        # 将内容项打印输出
        print('Title      ' + entry['title'].encode('utf-8'))
        print('Publisher: ' + entry['publisher'].encode('utf-8'))
        print()
        print(entry['summary'].encode('utf-8'))

        # 将所有文本组合在一起，为分类器构建一个内容项
        fulltext = '%s\n%s\n%s' % (entry['title'], entry['publisher'],
                                   entry['summary'])
        # 将当前分类的最佳猜测结果打印输出
        print('Guess:     ' + str(classifier.classify(fulltext)))
        # 请求用户给出正确分类，并据此进行训练
        cl = input('Enter category: ')
        classifier.train(fulltext, cl)
