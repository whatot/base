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


