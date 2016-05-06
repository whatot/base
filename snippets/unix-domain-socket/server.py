
import socket
import os
import string
from time import sleep
from random import choice


def id_generator(size=10, chars=string.ascii_uppercase + string.digits):
    return ''.join(choice(chars) for _ in range(size))


class SocketService:

    def __init__(self):
        self._sock_file = "./communicate.sock"
        if os.path.exists(self._sock_file):
            os.remove(self._sock_file)
        print("Opening socket...")
        self._server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self._server.bind(self._sock_file)
        self._server.listen(5)
        print("Listening...")

    def loop(self):
        while True:
            conn, address = self._server.accept()
            print('accepted connection from: ', address)
            while True:
                data = conn.recv(1024)
                if not data:
                    break
                else:
                    print(data)
                    if data == "DONE" or data == b'DONE':
                        conn.close()
                        break
                conn.send(bytes(id_generator().encode(encoding='UTF-8')))
                sleep(1)

    def close(self):
        print("-" * 20)
        print("Shutting down...")
        self._server.close()
        os.remove(self._sock_file)
        print("Done")


def main():
    service = SocketService()
    service.loop()
    service.close()

if __name__ == "__main__":
    main()
