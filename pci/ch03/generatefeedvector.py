# -*- coding:utf-8 -*-
#!/usr/bin/python

import feedparser
import re

# 返回一个 RSS 订阅源的标题和包含单词计数情况的字典
def getwordcounts(url):
    # 解析订阅源
    d=feedparser.parse(url)
    wc={}

    # 循环遍历所有的文章条目
    for e in d.entries:
        if 'summary' in e: summary=e.summary
        else: summary=e.description

        # 提取一个单词列表
        words=getwords(e.title+' '+summary)
        for word in words:
            wc.setdefault(word,0)
            wc[word]+=1
    return d.feed.title,wc

def filter_tags(htmlstr):
    re_cdata=re.compile('//<!\[CDATA\[[^>]*//\]\]>',re.I) #匹配CDATA
    re_script=re.compile('<\s*script[^>]*>[^<]*<\s*/\s*script\s*>',re.I)#Script
    re_style=re.compile('<\s*style[^>]*>[^<]*<\s*/\s*style\s*>',re.I)#style
    re_p=re.compile('<P\s*?/?>')#处理换行
    re_h=re.compile('</?\w+[^>]*>')#HTML标签
    re_comment=re.compile('<!--[^>]*-->')#HTML注释
    s=re_cdata.sub('',htmlstr)#去掉CDATA
    s=re_script.sub('',s) #去掉SCRIPT
    s=re_style.sub('',s)#去掉style
    s=re_p.sub('\r\n',s)#将<p>转换为换行
    s=re_h.sub('',s) #去掉HTML 标签
    s=re_comment.sub('',s)#去掉HTML注释  
    blank_line=re.compile('\n+')#去掉多余的空行
    s=blank_line.sub('\n',s)
    return s


def getwords(htmlstr):
    # 去除所有html标记
    txt=re.compile(r'<[^>]>+').sub('',htmlstr)
    #txt=filter_tags(htmlstr)
    
    # 利用所有非字母字符拆分出单词
    words=re.compile(r'[^A-Z^a-z]+').split(txt)

    # 转化为小写形式
    return [word.lower() for word in words if words!='']


apcount={}
wordcounts={}
feedlist=[line for line in file('feedlist.txt')]
for feedurl in feedlist:
    title,wc=getwordcounts(feedurl)
    wordcounts[title]=wc
    for word,count in wc.items():
        apcount.setdefault(word,0)
        if count>1:
            apcount[word]+=1


wordlist=[]
for w,bc in apcount.items():
    frac=float(bc)/len(feedlist)
    if frac>0.1 and frac<0.5:
        wordlist.append(w)

out=file('blogdata.txt','w')
out.write('Blog')
for word in wordlist:
    out.write('\t%s' % word)
out.write('\n')
for blog,wc in wordaccounts.items():
    out.write(blog)
    for word in wordlist:
        if word in wc:
            out.write('\t%d' % wc[word])
        else:
            out.write('\t0')
        out.write('\n')
