
import hashlib
from pathlib import Path

BASE_DIR = Path(__file__).parent

FILE = BASE_DIR / "my_project_test.exe"
HASH_FILE = BASE_DIR / "original_hash.txt"


def calculate_hash(path):
    sha256 = hashlib.sha256()

    with open(path, "rb") as file:
        for block in iter(lambda: file.read(4096), b""):
            sha256.update(block)

    return sha256.hexdigest().upper()


expected_hash = HASH_FILE.read_text(encoding="utf-8").strip().upper()
actual_hash = calculate_hash(FILE)

print("Expected SHA-256:", expected_hash)
print("Actual SHA-256:  ", actual_hash)

if actual_hash == expected_hash:
    print("Integrity verified. File is unchanged.")
else:
    print("WARNING: File modification detected!")
