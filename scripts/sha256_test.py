import hashlib
import time

password = "pass: Кондратюк"

target_hash = hashlib.sha256(
    password.encode("utf-8")
).hexdigest()

candidates = [
    f"test_password_{i}" for i in range(1, 20)
] + [password]

start = time.perf_counter()

for attempt, candidate in enumerate(candidates, 1):
    candidate_hash = hashlib.sha256(
        candidate.encode("utf-8")
    ).hexdigest()

    if candidate_hash == target_hash:
        print("Password:", candidate)
        print("Attempts:", attempt)
        break

end = time.perf_counter()

print("Algorithm: SHA-256")
print("Hash:", target_hash)
print(f"Time: {end - start:.8f} seconds")