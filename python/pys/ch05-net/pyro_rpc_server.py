#!/usr/bin/python2

import Pyro.core
import os
# from xmlrpc_pyro_diff import PSACB


class PASExample(Pyro.core.ObjBase):

    def ls(self, directory):
        try:
            return os.listdir(directory)
        except OSError:
            return []

    def ls_boom(self, directory):
        return os.listdir(directory)

    def cb(self, obj):
        print("OBJECT:", obj)
        print("OBJECT.__class__:", obj.__class__)
        return obj.cb()

if __name__ == '__main__':
    Pyro.core.initServer()
    deamon = Pyro.core.Deamon()
    uri = deamon.connect(PASExample(), "pasexample")

    print("The deamon runs on port:", deamon.port)
    print("The object's uri is:", uri)
    deamon.requestLoop()
