#!/usr/bin/python2
# -*- coding:utf-8 -*-

import httplib
import sys


def check_webserver(address, port, resource):
    #create connection
    if not resource.startswith('/'):
        resource = '/' + resource
    try:
        conn = httplib.HTTPConnection(address, port)
        print('HTTP Connection created successfully'
        # make request
        conn.request('GET', resource)
        print('request for %s successful' % resource)
        # get response
        response = conn.getresponse()
        print('response status: %s' % response.status)
    except socket.error as e:
        print('HTTP conection failed: %s' % e)
        return False
    finally:
        conn.close()
        print('HTTP connection closed successfully')
    if response.status in [200, 301]:
        return True
    else:
        return False


if __name__ == '__main__':
    from optparse import OptionParser
    parser = OptionParser()
    parser.add_option("-a", "--address", dest="address", default='localhost',
                      help="ADDRESS for webserver", metavar="ADDRESS")
    parser.add_option("-p", "--port", type="int", default=80,
                      help="PORT for webserver", metavar="PORT")
    parser.add_option("-r", "--resource", dest="resource",
                      default='index.html', help="RESOURCE to check",
                      metavar="RESOURCE")
    (options, args) = parser.parse_args()
    print('options: %s, args: %s' % (options, args))
    check = check_webserver(options.address, options.port, options.resource)
    print('check_webserver returned %s' % check)
    sys.exit(not check)
