#!/usr/bin/python2
# -*- coding:utf-8 -*-

# wget http://kiwitobes.com/tree/decision_tree_example.txt
with open('decision_tree_example.txt', 'r') as tree:
    my_data = [line.split('\t') for line in tree]

# print(my_data)


class decisionnode:
    def __init__(self, col=-1, value=None, results=None, tb=None, fb=None):
        # col 是待检验的判断条件所对应的列索引值
        # value 对应于为了使结果为true，当前列必须匹配的值
        # results 保存的是针对于当前分支的结果，是一个dict。
        #         除叶节点外，在其他节点上该值都为None
        # tb, fb 也是decisionnode，他们对应结果分别为true或false时，
        #       树上相对于当前节点的子树上的节点
        self.col = col
        self.value = value
        self.results = results
        self.tb = tb
        self.fb = fb
