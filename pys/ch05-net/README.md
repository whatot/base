## ch05 network

###网络客户端

* socket
	>* socket()
	>* connect()
	>* send()
	>* recv()
	>* close()

* httplib
* ftplib
* urllib
* urllib2


###远程过程调用

* XML-RPC - SimpleXMLRPCServer
* Pyro ? - Pyro

### SSH

* paramiko

### Twisted

* python2-zope-interface  twisted python2-pyopenssl python2-soappy


### Scapy

> $sudo scapy

> >>>res,unans = traceroute(["www.microsoft.com","www.cisco.com","www.yahoo.com","www.wanadoo.fr","www.pacsec.com"],dport=[80,443],maxttl=20,retry=-2)

> >>>res.graph(target=">./graph.svg")
> >>>res.graph(type="ps", target="| lp")
