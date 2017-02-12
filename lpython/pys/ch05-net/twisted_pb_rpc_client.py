#!/usr/bin/python2

'''
if there is a terminal run below:
$ python2 twisted_pb_rpc_server.py

you can run this in another terminal:
$ python2 twisted_pb_rpc_client.py

if you want to see how twisted handle error, you can comment line 25
and uncomment line 25, and run again
'''

from twisted.spread import pb
from twisted.internet import reactor


def handle_err(reason):
    print("an error occurred", reason)
    reactor.stop()


def call_ls(def_call_obj):
    return def_call_obj.callRemote('ls', '/home')
    # return def_call_obj.callRemote('ls_boom', '/home/a')


def print_ls(print_result):
    print(print_result)
    reactor.stop()


if __name__ == '__main__':
    factory = pb.PBClientFactory()
    reactor.connectTCP("localhost", 9876, factory)
    d = factory.getRootObject()
    d.addCallback(call_ls)
    d.addCallback(print_ls)
    d.addErrback(handle_err)
    reactor.run()
