# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import socket
import struct
import time

SOCKET_PATH = "/tmp/barmd.sock"

def recv_exact(sock, n):
    buf = b""
    while len(buf) < n:
        chunk = sock.recv(n - len(buf))
        if not chunk:
            raise RuntimeError("socket closed without data")
        buf += chunk
        return buf

RESPONSE_REQUEST_INVALID_ERROR = (0x00, 0x00, 0x0001)

def send(x, y, z, s):
    print(f"sending: {x:.2f}, {y:.2f}, {z:.2f}")
    data = struct.pack("<ddd", x, y, z)
    s.sendall(data)
    resp = recv_exact(s, 4)
    a, b, c = struct.unpack("<BBH", resp)

    if (a, b, c) == RESPONSE_REQUEST_INVALID_ERROR:
        print(f"invalid request for ({x}, {y}, {z}), error {c}")
    else:
        print(f"response: shoulder {a}, elbow {b}, rotation {c}")

    time.sleep(0.05)

def main():
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
        s.connect(SOCKET_PATH)

        import numpy as np

        MAX_REACH = 12
        STEP = 1
        Z_STEPS = [0, 1, 2, 3, 4, 5, 10, 15]

        for z in Z_STEPS:
            for x in np.arange(-MAX_REACH, MAX_REACH + STEP, STEP):
                for y in np.arange(-MAX_REACH, MAX_REACH + STEP, STEP):
                    dt = (x**2 + y**2 + z**2)**0.5
                    if dt <= MAX_REACH:
                        send(float(x), float(y), float(z), s)

if __name__ == "__main__":
    main()
