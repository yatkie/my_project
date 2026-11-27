import bcrypt
import time

password = "pass: Кондратюк"

target_hash = bcrypt.hashpw(
    password.encode("utf-8"),
    bcrypt.gensalt(rounds=12)
)

candidates = [
    f"test_password_{i}" for i in range(1, 20)
] + [password]

start = time.perf_counter()

for attempt, candidate in enumerate(candidates, 1):
    if bcrypt.checkpw(candidate.encode("utf-8"), target_hash):
        end = time.perf_counter()
        print("Password:", candidate)
        print("Attempts:", attempt)
        break

print("Algorithm: bcrypt")
print("Rounds: 12")
print("Hash:", target_hash.decode())
print(f"Time: {end - start:.8f} seconds")