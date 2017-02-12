# A Naive Bayesian Classifier
# p429 TP274/Y139
# Data Analysis with Open Source Tools, Philipp K.Janert, O'REILLY

total = {} # Training instances per class label
histo = {} # Histogram

# Read the trainiing set and build up a histogram
train = open( "iris.train" )
for line in train:
    # seplen, sepwid, petlen, petwid, label
    f = line.rstrip().split( ',' )
    label = f.pop()

    if not total.has_key( label ):
        total[ label ] = 0
        histo[ label ] = [ {}, {}, {}, {} ]

    # Count trainiing instances for the current label
    total[label] += 1

    # Iterate over features
    for i in range( 4 ):
        histo[label][i][f[i]] = 1 + histo[label][i].get( f[i], 0.0 )


train.close()

# Read the test set and evaluate the probabilities
hit, miss = 0, 0
test = open( "iris.test" )
for line in test:
    f = line.rstrip().split( ',' )
    true = f.pop()

    p = {} # probability for the class label, given the test features
    for label in total.keys():
        p[label] = 1
        for i in range( 4 ):
            p[label] *= histo[label][i].get(f[i], 0.0)/total[label]

    # find the label with the largest probability
    mx, predicted = 0, -1
    for k in p.keys():
        if p[k] >= mx:
            mx, predicted = p[k], k
        if true == predicted:
            flag = '+'
            hit += 1
        else:
            flag = '-'
            miss += 1

    print flag, "\t", true, "\t", predicted, "\t",
    for label in p.keys():
        print label, ":", p[label], "\t",
    print

print
print hit, "out of", hit+miss, "correct - Accuracy: ", hit/(hit+miss+0.0)

test.close()
