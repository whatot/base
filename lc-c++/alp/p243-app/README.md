
# A simple HTTP server

# compile and use

## compile

* ``make all`` or ``make`` compiles the main target ``server.bin`` and modules;
* ``make test`` compiles the test target ``test.bin`` and run it;
* ``make GTAGS`` generates the ``GTAGS`` files;
* ``make dist-clean`` includes ``make clean`` and ``make GTAGS-clean``
* ``make clean`` clean all *.bin *.o *.so
* ``make GTAGS-clean`` clean files about GTAGS

## usage

* ``./server.bin --help`` shows how to use it.

```
$ ./server.bin --help
Usage: ./server.bin [ options ]
    -a, --address ADDR        Bind to local address (by default, bind
                                  to all local addresses).
    -h, --help                Print this information.
    -m, --module-dir DIR      Load Modules from specified directory
                                  (by default, use executable directory).
    -p, --port PORT           Bind to specified port.
    -v, --verbose             Print verbose messages.
```

* examples
	1. ``./server.bin -a localhost -p 4000 -v``
	2. use brower to visit the URLs below:
		* ``http://127.0.0.1:4000/diskfree``
		* ``http://127.0.0.1:4000/processes``
		* ``http://127.0.0.1:4000/time``
		* ``http://127.0.0.1:4000/issue``
		they are visit the output from modules builded above.


# others
