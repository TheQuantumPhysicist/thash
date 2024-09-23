import hashlib

def hash_n_times(data, n):
    for _ in range(n):
        data = hashlib.sha3_384(data).digest()
    return data.hex()

if __name__ == "__main__":
    input_data = b"abc"
    for i in range(1, 6):
        result = hash_n_times(input_data, i)
        print(f"{i}x: {result}")

    print("[")
    for i in range(1, 6):
        result = hash_n_times(input_data, i)
        print(f"\"{result}\",")
    print("]")

