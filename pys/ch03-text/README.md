# Ch03-text

####subprocess.Popen; subprocess.PIPE; in, not in; index(); find();

    In [3]: import subprocess

    In [4]: res = subprocess.Popen(['uname', '-sv'], stdout=subprocess.PIPE)
    In [5]: uname = res.stdout.read().strip()

    In [6]: uname
    Out[6]: 'Linux #1 SMP Debian 3.2.41-2'

    In [7]: 'Linux' in uname
    Out[7]: True

    In [8]: 'Linux' not in uname
    Out[8]: False

    In [9]: uname.index('Debian')
    Out[9]: 13

    In [10]: uname.find('Linux')
    Out[10]: 0

    In [11]: uname.find('inux')
    Out[11]: 1

    In [12]: uname.find('linux')
    Out[12]: -1

    In [13]: uname.index('linux')
    ...
    ValueError: substring not found


####字符串切分, 返回一个新的字符串对象

    In [22]: smp_index = uname.index('SMP')

    In [23]: smp_index
    Out[23]: 9

    In [24]: uname[smp_index:]
    Out[24]: 'SMP Debian 3.2.41-2'


##### stratswith(); endswith()

    In [26]: uname.startswith('Li')
    Out[26]: True

    In [27]: uname
    Out[27]: 'Linux #1 SMP Debian 3.2.41-2'

    In [28]: uname.endswith(2)
    TypeError: endswith first arg must be str, unicode, or tuple, not int

    In [29]: uname.endswith('2')
    Out[29]: True


#### strip(); lstrip(); rstrip() 去除空白

    In [1]: spas = "\t\n some no-space--- text \n \t\n"

    In [2]: spas
    Out[2]: '\t\n some no-space--- text \n \t\n'

    In [3]: print spas
        
    some no-space--- text 


    In [4]: spas.lstrip()
    Out[4]: 'some no-space--- text \n \t\n'

    In [5]: spas.strip()
    Out[5]: 'some no-space--- text'

    In [6]: spas.rstrip()
    Out[6]: '\t\n some no-space--- text'

    In [8]: spas.strip().strip("so-pt")
    Out[8]: 'me no-space--- tex'


#### upper(); lower()

#### split(), 定界字符串, 默认以空格分割

    strings.split(',')
    strings.split(',', 1)  指定分割的最多次数

#### splitlines()返回一个字符串的每一行所组成的列表，并且保存为一组。

#### join() 输入为str, 非整数or others

    In [9]: some_join = ['aaa', 'bbb', 'ccc', 'ddd']

    In [10]: ', '.join(some_join)
    Out[10]: 'aaa, bbb, ccc, ddd'

    In [11]: ''.join(some_join)
    Out[11]: 'aaabbbcccddd'

    In [12]: '\t'.join(some_join)
    Out[12]: 'aaa\tbbb\tccc\tddd'

    In [13]: some_list = range(10)

    In [14]: some_list
    Out[14]: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

    In [15]: ",".join([str(i) for i in some_list])
    Out[15]: '0,1,2,3,4,5,6,7,8,9'


#### replace()

    In [16]: ''.join(some_join).replace("a", "b")
    Out[16]: 'bbbbbbcccddd'


#### unicode

    In [17]: unicode_string = u'jcjdchiduf'

    In [18]: unicode_string
    Out[18]: u'jcjdchiduf'

    In [20]: unicode('unicode_string')
    Out[20]: u'unicode_string'

    In [21]: unicode_string = u'abc_\u03a0\u03a3\u03a9_\u0414\u0424\u042F'

    In [22]: unicode_string
    Out[22]: u'abc_\u03a0\u03a3\u03a9_\u0414\u0424\u042f'

    In [23]: print unicode_string.encode('utf-8')
    abc_ΠΣΩ_ДФЯ


#### re

**直接使用re模块中的对象**

	In [1]: import re

	In [2]: re_string = "{{(.*?)}}"

	In [3]: some_string = "this is a string with {{word}} embedded in\
	...: {{curly brackets}} to show an {{example}} of {{regular expressions}}"

	In [4]: for match in re.findall(re_string, some_string):
	...:     print "MATCH->", match
	...:     
	MATCH-> word
	MATCH-> curly brackets
	MATCH-> example
	MATCH-> regular expressions


**编译后的正则表达式**

	In [5]: import re

	In [6]: re_obj = re.compile("{{(.*?)}}")

	In [7]: some_string = "this is a string with {{words}} embedded in\
	...:  {{curly brackets}} to show an {{example}} of {{regular expressions}}"

	In [9]: for match in re_obj.findall(some_string):
	print "MATCH->", match
	...:     
	MATCH-> words
	MATCH-> curly brackets
	MATCH-> example
	MATCH-> regular expressions


	import re_loop_nocompile
	timeit -n 5 re_loop_nocompile.run_re()
	$ time python re_loop_nocompile

	import re_loop_compile
	timeit -n 5 re_loop_compile.run_re()
	$ time python re_loop_compile


**原始字符串与正则**

raw_pattern将\b识别为两个字符

non_raw_pattern将\b识别为转义字符中的退格字符

	In [1]: import re

	In [2]: raw_pattern = r'\b[a-z]+\b'

	In [3]: non_raw_pattern = '\b[a-z]+\b'

	In [4]: some_string = 'a few little words'

	In [5]: re.findall(raw_pattern, some_string)
	Out[5]: ['a', 'few', 'little', 'words']

	In [6]: re.findall(non_raw_pattern, some_string)
	Out[6]: []

	In [7]: some_other_string = 'a few \blittle\b words'

	In [8]: re.findall(non_raw_pattern, some_other_string)
	Out[8]: ['\x08little\x08']


**findall()**

	In [9]: re_obj = re.compile(r'\bt.*?e\b')

	In [10]: re_obj.findall("time tame tune tint tire")
	Out[10]: ['time', 'tame', 'tune', 'tint tire']

	In [11]: re_obj = re.compile(r'\bt\w*e\b')

	In [12]: re_obj.findall("time tame tune tint tire")
	Out[12]: ['time', 'tame', 'tune', 'tire']


**finditer()**

**search,match**

	In [13]: re_obj = re.compile('FOO')

	In [14]: search_string = ' FOO'

	In [15]: re_obj.search(search_string)
	Out[15]: <_sre.SRE_Match at 0x8baf138>

	In [16]: re_obj.match(search_string)

	In [17]: re_obj.search(search_string, pos=1)
	Out[17]: <_sre.SRE_Match at 0x8baf218>

	In [18]: re_obj.match(search_string, pos=1)
	Out[18]: <_sre.SRE_Match at 0x8baf2f8>

	In [19]: re_obj.search(search_string, pos=1, endpos=3)

	In [20]: re_obj.match(search_string, pos=1, endpos=3)



####处理文件

**读取文件,创建文件对象**

	In [1]: infile = open("foo.txt", "r")

	In [2]: print infile.read()
	this is a test file.
	for file operations.

	In [3]: ls
	foo.txt  README.md  re_loop_compile.py  re_loop_nocompile.py

open (文件名， 文件打开模式， 缓冲区大小)   
模式 r(读取，default), w(写), a(附加模式), b(二进制)


**写入文件,关闭文件,覆盖原内容**

	In [4]: outputfile = open("foo.txt", "w")

	In [5]: outputfile.write("This is \nSome\nRandom\nOutput Text\n")

	In [7]: outputfile.close()

	In [8]: !cat foo.txt
	This is 
	Some
	Random
	Output Text

**使用try/finally写入并关闭文件**

	In [9]: try:
	...:     f = open('foo.txt', 'w')
	...:     f.write('quick line here\n')
	...: finally:
	...:     f.close()
	...:     

	In [10]: !cat foo.txt
	quick line here

**使用with-as写入并自动关闭文件**

	In [11]: with open('foo.txt', 'w') as f:
	....:     f.write('this is a writeable file\n')
	....:     

	In [12]: !cat foo.txt
	this is a writeable file

	In [13]: f
	Out[13]: <closed file 'foo.txt', mode 'w' at 0x95dec28>


**读取文件read(),readline(),readlines()**

	In [15]: f = open("foo.txt", "r")

	In [16]: f.read()
	Out[16]: 'this is a writeable file\n'

	In [17]: f.readline()
	Out[17]: ''

	In [22]: f = open("foo.txt", "r")

	In [23]: f.readline()
	Out[23]: 'this is a writeable file\n'

	In [25]: f.read(5)
	Out[25]: 'this '


**写文件write(),writelines()**

	In [26]: f = open("foo.txt", "w")

	In [27]: f.write("Test\nFile\n")

	In [29]: f.close()

	In [30]: g = open("foo.txt", "r")

	In [31]: g.read()
	Out[31]: 'Test\nFile\n'

使用列表生成式

	In [35]: f = open("foo.txt", "w")

	In [36]: f.writelines("%s\n" % i for i in range(10))

	In [37]: f.close()

	In [38]: g = open("foo.txt", "r")

	In [39]: g.read()
	Out[39]: '0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n'

使用迭代器(iterator) yield

	In [40]: def myRange(r):
	....:     i = 0
	....:     while i < r:
	....:         yield "%s\n" % i
	....:         i += 1
	....:         

	In [41]: f = open("foo.txt", "w") 

	In [42]: f.writelines(myRange(10))

	In [43]: f.close()

	In [44]: g= open("foo.txt", "r")

	In [45]: g.read()
	Out[45]: '0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n'


####标准输入和输出

**sys.stdin**

	In [50]: import sys

	In [51]: g
	Out[51]: <open file 'foo.txt', mode 'r' at 0x96487b0>

	In [52]: sys.stdin
	Out[52]: <open file '<stdin>', mode 'r' at 0xb75b2020>

	In [53]: type(sys.stdin) == type(f)
	Out[53]: True

**sys.stdout**

	In [1]: import sys

	In [2]: f = open('foo.txt', 'w')

	In [3]: sys.stdout
	Out[3]: <open file '<stdout>', mode 'w' at 0xb7500078>

	In [4]: f
	Out[4]: <open file 'foo.txt', mode 'w' at 0x99dcb20>

	In [5]: type(sys.stdout) == type(f)
	Out[5]: True


**StringIO**

	In [6]: from StringIO import StringIO

	In [8]: file_like_string = StringIO("This is a\nmultiline string.\nreadline() shoule see\nmultiple lines of \ninput")

	In [9]: file_like_string.readline()
	Out[9]: 'This is a\n'

	In [10]: file_like_string.readline()
	Out[10]: 'multiline string.\n'

	In [11]: file_like_string.readlines()
	Out[11]: ['readline() shoule see\n', 'multiple lines of \n', 'input']

	In [12]: dir(file_like_string)
	Out[12]: 
	['__doc__',
	'__init__',
	'__iter__',
	'__module__',
	'buf',
	'buflist',
	'close',
	'closed',
	'flush',
	'getvalue',
	'isatty',
	'len',
	'next',
	'pos',
	'read',
	'readline',
	'readlines',
	'seek',
	'softspace',
	'tell',
	'truncate',
	'write',
	'writelines']


**file与StringIO的区别-方法与属性**

	In [13]: f = open("foo.txt", 'r')

	In [14]: from sets import Set

	In [15]: sio_set = Set(dir(file_like_string))

	In [16]: file_set = Set(dir(f))

	In [17]: sio_set.difference(file_set)
	Out[17]: Set(['__module__', 'buflist', 'pos', 'len', 'getvalue', 'buf'])

	In [18]: file_set.difference(sio_set)
	Out[18]: Set(['__enter__', 'encoding', '__str__', '__getattribute__', 'xreadlines', '__sizeof__', 'newlines', '__setattr__', 'errors', '__new__', 'readinto', '__format__', '__class__', 'mode', '__exit__', '__reduce__', '__reduce_ex__', 'fileno', 'name', '__delattr__', '__subclasshook__', '__repr__', '__hash__'])


**urllib**

	In [1]: import urllib

	In [2]: url_file = urllib.urlopen("https://google.com.hk")

	In [3]: url_docs = url_file.read()

	In [4]: url_file.close()

	In [5]: len(url_docs)
	Out[5]: 10976

	In [6]: url_docs[:80]
	Out[6]: '<!doctype html><html itemscope="itemscope" itemtype="http://schema.org/WebPage">'

	In [7]: url_docs[-80:]
	Out[7]: 'n);google.timers.load.t.prt=e=(new Date).getTime();})();\n</script></body></html>'



####日志解析

####ElementTree

    In [1]: from xml.etree import ElementTree as ET

    In [2]: tcusers = ET.parse('./mime.xml')

    In [3]: tcusers
    Out[3]: <xml.etree.ElementTree.ElementTree at 0x8a9e02c>


