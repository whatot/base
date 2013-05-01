#!/usr/bin/python2

def readfile(filename):
    lines=[line for line in file(filename)]

    #第一列是列标题
    colnames=lines[0].strip().split('\t')[1:]
    rownames=[]
    data=[]
    for line in lines[1:]:
        p=line.strip().split('\t')
        #每行的第一列是行名
        rownames.append(p[0])
        #剩余部分就是该行对应的数据
        data.append([float(x) for x in p[1:]])
    return rownames,colnames,data
