# A Nearest-Neighbor Classifier
# p428 TP274/Y139
# Data Analysis with Open Source Tools, Philipp K.Janert, O'REILLY

from numpy import *

train = loadtxt( "iris.train", delimiter=',', usecols=(0,1,2,3) )
trainlabel = loadtxt( "iris.train", delimiter=',', usecols=(4,), dtype=str )

test = loadtxt( "iris.test", delimiter=',', usecols=(0,1,2,3) )
testlabel = loadtxt( "iris.test", delimiter=',', usecols=(4,), dtype=str )

hit, miss = 0, 0
for i in range( test.shape[0] ):
    dist = sqrt( sum( (test[i] - train)**2, axis=1 ) )
    k = argmin( dist )

    if trainlabel[k] == testlabel[i]:
        flag = '+'
        hit += 1
    else:
        flag = '-'
        miss += 1

print flag, "\t Predicted: ", trainlabel[k], "\t True: ", testlabel[i]
print
print hit, "out of", hit+miss, "correct - Accuracy: ", hit/(hit+miss+0.0)
