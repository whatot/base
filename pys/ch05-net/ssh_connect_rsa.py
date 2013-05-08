#!/usr/bin/python2

import paramiko


hostname = '192.168.1.15'
port = 22
username = 'hciud'
pkey_file = '/home/hciud/.ssh/id_rsa'

if __name__ == "__main__":
    key = paramiko.RSAKey.from_private_key(pkey_file)
    s = paramiko.SSHClient()
    s.load_system_host_keys()
    s.connect(hostname, port, pkey=key)
    stdin, stdout, stderr = s.exec_command('ifconfig')
    print(stdout.read())
    s.close()
