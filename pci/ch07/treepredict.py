#!/usr/bin/python2
# -*- coding:utf-8 -*-

# wget http://kiwitobes.com/tree/decision_tree_example.txt
with open('decision_tree_example.txt', 'r') as tree:
    my_data = [line.split('\t') for line in tree]

print(my_data)
