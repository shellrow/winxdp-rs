"""Bounded isolated-network peer for the ignored UDP roundtrip tests."""
import argparse
import socket
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--bind", required=True, help="local peer IPv4 address")
parser.add_argument("--target", required=True, help="test adapter IPv4 address")
parser.add_argument("--os-probe", action="store_true")
args = parser.parse_args()
with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as peer:
    peer.bind((args.bind, 49002))
    peer.settimeout(0.25)
    deadline = time.monotonic() + 25
    os_replies = 0
    for sequence in range(256):
        payload = sequence.to_bytes(8, "big") + bytes([sequence % 256]) * (sequence % 89 * 16)
        while True:
            assert time.monotonic() < deadline, f"timeout at sequence {sequence}"
            peer.sendto(payload, (args.target, 49000))
            if args.os_probe:
                peer.sendto(b"winxdp-os-pass", (args.target, 49003))
            try:
                reply, address = peer.recvfrom(2048)
            except socket.timeout:
                continue
            if reply == payload and address == (args.target, 49000):
                break
            if reply == b"winxdp-os-pass" and address == (args.target, 49003):
                os_replies += 1
    if args.os_probe:
        assert os_replies > 0, "no unmatched UDP response reached the peer"
        print(f"PASS: {os_replies} unmatched UDP responses via the target OS stack")
    print("PASS: 256 UDP payloads echoed byte-for-byte through the target XDP RX/TX path")
