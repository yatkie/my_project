import bcrypt
import random
import string

year = "2026"
group = input("Group: ")
surname = "Кондратюк"
variant = input("Variant: ")

original = f"{year}_{group}_{surname}_{variant}_"
original_length = len(original.encode("utf-8"))

if original_length > 72:
    raise ValueError("Original string exceeds 72 bytes")

base = original + "_" * (72 - original_length)

suffix = "".join(
    random.choices(string.ascii_letters + string.digits, k=10)
)

passwords = [
    base,
    base + "1",
    base + suffix,
    base[:-1]
]

print("\n=== Original string ===")
print(original)
print("Original bytes:", original_length)

print("\n=== Test strings ===")

for i, password in enumerate(passwords, 1):
    print(f"String {i}: {password}")
    print("UTF-8 bytes:", len(password.encode("utf-8")))

assert len(base.encode("utf-8")) == 72

# Emulate traditional bcrypt 72-byte truncation.
def bcrypt_input(password):
    return password.encode("utf-8")[:72]

hashes = [
    bcrypt.hashpw(
        bcrypt_input(password),
        bcrypt.gensalt(rounds=12)
    )
    for password in passwords
]

print("\n=== Hashes ===")

for i, password_hash in enumerate(hashes, 1):
    print(f"Hash {i}: {password_hash.decode()}")

print("\n=== Verification matrix ===")
print(f"{'String':<12} H1      H2      H3      H4")

for i, password in enumerate(passwords, 1):
    results = []

    for password_hash in hashes:
        valid = bcrypt.checkpw(
            bcrypt_input(password),
            password_hash
        )
        results.append(str(valid))

    print(
        f"String {i:<5} "
        + "  ".join(f"{result:<6}" for result in results)
    )

print("\nTest completed.")