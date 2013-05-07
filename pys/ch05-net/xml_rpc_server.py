#!/usr/bin/python2

'''
a SimpleXMLRPCServer
$ python2 xml_rpc_server.py
after that visit 127.0.0.1:8765 using method below

x = xmlrpcserver.ServerProxy('http://localhost:8765')
x.ls('.')
x.ls_boom('.')
'''

import SimpleXMLRPCServer
import os


def ls(directory):
    try:
        return os.listdir(directory)
    except OSError:
        return []


def ls_boom(directory):
    return os.listdir(directory)


def cb(obj):
    print("OBJECT ::", obj)
    print("OBJECT.__class__ ::", obj.__class__)
    return obj.cb()

if __name__ == '__main__':
    s = SimpleXMLRPCServer.SimpleXMLRPCServer(('127.0.0.1', 8765))
    s.register_function(ls)
    s.register_function(ls_boom)
    s.register_function(cb)
    s.serve_forever()
