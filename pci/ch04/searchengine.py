#!/usr/bin/python2
# -*- coding:utf-8 -*-


class crawler:
    # 初始化crawler类并传入数据库名称
    def __init__(self, dbname):
        pass

    def __del__(self):
        pass

    def dbconnect(self):
        pass

    # 辅助函数，用于获取条目的id，并且如果条目不存在，就将其加入数据库中
    def getentryid(self, table, field, value, createnew=True):
        return None

    # 为每个网页建立索引
    def addtoindex(self, url, soup):
        print('Indexing %s' % url)

    # 从一个HTML网页中提取文字（不带标签）
    def gettextonly(self, soup):
        return None

    # 根据任何非空白字符进行分词处理
    def separatewords(self, text):
        return None

    # 如果url已经建立过索引，则返回true
    def isindexed(self, url):
        return False

    # 添加一个关联两个网页的链接
    def addlinkref(self, urlFrom, urlTo, linkText):
        pass

    # 从一小组网页开始进行广度优先搜索，直至某一给定深度，
    # 期间为网页建立索引
    def crawl(self, pages, depth=2):
        pass

    # 创建数据库
    def createindextables(self):
        pass
