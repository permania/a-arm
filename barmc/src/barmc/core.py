# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import socket
import struct
import time
import random

SOCKET_PATH = "/tmp/barmd.sock"

def recv_exact(sock, n):
    buf = b""
    while len(buf) < n:
        chunk = sock.recv(n - len(buf))
        if not chunk:
            raise RuntimeError("socket closed without data")
        buf += chunk
        return buf

ERROR_REQUEST_INVALID = (0x00, 0x00, 0x0001)

def send(x, y, z, s):
    print(f"sending: {x:.2f}, {y:.2f}, {z:.2f}")
    data = struct.pack("<ddd", x, y, z)
    s.sendall(data)
    resp = recv_exact(s, 4)
    a, b, c = struct.unpack("<BBH", resp)

    if (a, b, c) == ERROR_REQUEST_INVALID:
        print(f"invalid request for ({x}, {y}, {z}), error {c}")
    else:
        print(f"response: shoulder {a}, elbow {b}, rotation {c}")

def main():
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
        s.connect(SOCKET_PATH)

        send(0, 0, 0, s)

        while True:
            x = random.uniform(-10, 10)
            y = random.uniform(-10, 10)
            z = random.uniform(-10, 10)
            send(x, y, z, s)

            time.sleep(0.05)

if __name__ == "__main__":
    main()
