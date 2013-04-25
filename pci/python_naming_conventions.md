(http://hi.baidu.com/mecfan/item/c9064c66ec3b642369105b8e)

##【转】Python 命名规范

- 文件名

	全小写,可使用下划线

- 包

	应该是简短的、小写的名字。如果下划线可以改善可读性可以加入。如mypackage。

- 模块

	与包的规范同。如mymodule。

- 类

	总是使用首字母大写单词串。如MyClass。内部类可以使用额外的前导下划线。

- 函数与方法

	函数名应该为小写，可以用下划线风格单词以增加可读性。如：myfunction，my_example_function。
	*注意*：混合大小写仅被允许用于这种风格已经占据优势的时候，以便保持向后兼容。

- 函数和方法的参数

	总使用“self”作为实例方法的第一个参数。总使用“cls”作为类方法的第一个参数。
	如果一个函数的参数名称和保留的关键字冲突，通常使用一个后缀下划线好于使用缩写或奇怪的拼写。

- 全局变量

	对于from M import *导入语句，如果想阻止导入模块内的全局变量可以使用旧有的规范，在全局变量上加一个前导的下划线。
	*注意*:应避免使用全局变量

- 变量

	变量名全部小写，由下划线连接各个单词。如color = WHITE，this_is_a_variable = 1
	*注意*：

>1. 不论是类成员变量还是全局变量，均不使用 m 或 g 前缀。
2. 私有类成员使用单一下划线前缀标识，多定义公开成员，少定义私有成员。
3. 变量名不应带有类型信息，因为Python是动态类型语言。如 iValue、names_list、dict_obj 等都是不好的命名。

- 常量

	常量名所有字母大写，由下划线连接各个单词如MAX_OVERFLOW，TOTAL。

- 异常

	以“Error”作为后缀。

- 缩写

	命名应当尽量使用全拼写的单词，缩写的情况有如下两种：
>1. 常用的缩写，如XML、ID等，在命名时也应只大写首字母，如XmlParser。
>2. 命名中含有长单词，对某个单词进行缩写。这时应使用约定成俗的缩写方式。

**例如:**
>1. function 缩写为 fn
2. text 缩写为 txt
3. object 缩写为 obj
4. count 缩写为 cnt
5. number 缩写为 num，等。

- 前导后缀下划线

>1. 一个前导下划线：表示非公有。
>2. 一个后缀下划线：避免关键字冲突。
>3. 两个前导下划线：当命名一个类属性引起名称冲突时使用。
>4. 两个前导和后缀下划线：“魔”（有特殊用图）对象或者属性，例如__init__或者__file__。绝对不要创造这样的名字，而只是使用它们。

>*注意*：关于下划线的使用存在一些争议。

- 特定命名方式

主要是指 __xxx__ 形式的系统保留字命名法。项目中也可以使用这种命名，它的意义在于这种形式的变量是只读的，这种形式的类成员函数尽量不要重载。如

>* class Base(object):
* def __init__(self, id, parent = None):
* self.__id__ = id
* self.__parent__ = parent
* def __message__(self, msgid):

>其中 __id__、__parent__ 和 __message__ 都采用了系统保留字命名法。


##附:Google Python命名规范
module_name, package_name, ClassName, method_name, ExceptionName, function_name, GLOBAL_VAR_NAME, instance_var_name, function_parameter_name, local_var_name.


##附:另一个命名规范

命名规范

* 变量：unix风格，小写加下划线，如：foo_bar。
* 常量：大写加下划线，如：ERROR_FOO_BAR。
* 函数：和 变量 一致，如：def foo_bar(): pass
* 类：大写C开头的驼峰法，如：CFooBar。(是的，与python lib风格不同)
* 模块：小写无下划线，如：foobar.py。
* 包：与 模块 一致。

* “私有化”命名约定： 一个且只一个下划线开始的命名。

表示本命名不能超过最近命名空间索引。

(不使用python的Private name mangling风格)

参考:
http://www.cnblogs.com/onm123/archive/2010/10/10/study-python-summary-three-style-guide.html
http://blog.csdn.net/lanphaday/archive/2008/08/26/2834883.aspx
http://google-styleguide.googlecode.com/svn/trunk/pyguide.html
http://blog.csdn.net/akara/archive/2010/08/09/5797974.aspx

转自：http://www.likestudy.org/150.html


## 另附两个简单的命名规则：

####Python变量的命名规则

Python与C#的变量（以及函数、类等其它标识符）的命名规则基本一样，同样对大小写敏感。不一样的地方是，Python中以下划线开始或者结束的标识符通常有特殊的意义。例如以一个下划线开始的标识符(如 _foo)不能用from module import *语句导入。前后均有两个下划线的标识符，如__init__，被特殊方法保留。前边有两个下划线的标识符，如__bar，被用来实现类私有属性，这个将在“类和面向对象编程”中再说。

最后，Python的关键字不能作为标识符（这个大家都知道），不过Python的关键字比C#要少得多，可以google一下，这里就不列出了。

转自：http://developer.51cto.com/art/201003/187668.htm


####简明Python命名规范

* 命名

	一致的命名可以给开发人员减少许多麻烦，而恰如其分的命名则可以大幅提高代码的可读性，降低维护成本。

* 常量

	常量名所有字母大写，由下划线连接各个单词，如

	WHITE = 0XFFFFFF
	
	THIS_IS_A_CONSTANT = 1

* 变

	变量名全部小写，由下划线连接各个单词，如
	
	color = WHITE
	
	this_is_a_variable = 1
	
	不论是类成员变量还是全局变量，均不使用 m 或 g 前缀。私有类成员使用单一下划线前缀标识，多定义公开成员，少定义私有成员。
	
	变量名不应带有类型信息，因为 Python 是动态类型语言。如 iValue、names_list、dict_obj 等都是不好的命名。

* 函数

	函数名的命名规则与变量名相同。

* 类

	类名单词首字母大写，不使用下划线连接单词，也不加入 C、T 等前缀。如：

	class ThisIsAClass(object):   
		passs
* 模块

     模块名全部小写，对于包内使用的模块，可以加一个下划线前缀，如

     module.py

     _internal_module.py

* 包

    包的命名规范与模块相同。

转自：(http://hi.baidu.com/sartre/blog/item/318ff1114474a0cca7ef3f9d.html)
