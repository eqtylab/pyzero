import sys

data = sys.argv[0].encode("utf-8")

sha = sys.risc0_sha.hash_bytes(data)

print(sha.hex())
