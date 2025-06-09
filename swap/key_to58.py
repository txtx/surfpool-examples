import json
import base58
import sys

with open(sys.argv[1]) as f:
    keypair = json.load(f)
    secret_key = bytes(keypair)  # 前32字节是私钥
    print(base58.b58encode(secret_key).decode())