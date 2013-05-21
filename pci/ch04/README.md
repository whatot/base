## ch04 信息检索 Information retrieval


### searchengine.py

#### crawler

#### searcher

* getmatchrows 构造查询字符串
* getscoredlist
* normalizescores 归一化函数
* frequencyscore 单词频度
* locationscore 单词在文档越早权值越高
* distancescore 单词距离
* inboundlinkscore 外部指回链接
* PageRank
    * calculatepagerank
    * pagerankscore
* linktextscore 链接文本


### nn.py 神经网络

输入层--隐藏层--输出层

#### searchnet

* maketables
* getstrength
* setstrength
* generatehiddennode
* feeding forward 前馈法
    * getallhiddenids
    * setupnetwork
    * feedforward
    * getresult
* training with backpropagation 反向传播法
    * dtanh
    * backPropagate
    * trainquery
    * updatedatabase

* nnscore : add to ``searcher``

