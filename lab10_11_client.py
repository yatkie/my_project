
import os
import requests

BASE_URL = "http://127.0.0.1:8080"

CLIENT_ID = os.environ["LIBRARY_CLIENT_ID"]
CLIENT_SECRET = os.environ["LIBRARY_CLIENT_SECRET"]

def get_token():
    response = requests.post(
        f"{BASE_URL}/auth/token",
        json={
            "client_id": CLIENT_ID,
            "client_secret": CLIENT_SECRET
        },
        timeout=10
    )
    response.raise_for_status()
    return response.json()["access_token"]


def main():
    token = get_token()

    headers = {
        "Authorization": f"Bearer {token}"
    }

    print("=== WHOAMI ===")
    response = requests.get(
        f"{BASE_URL}/whoami",
        headers=headers,
        timeout=10
    )
    response.raise_for_status()
    print(response.json())

    print("\n=== BOOKS ===")
    response = requests.get(
        f"{BASE_URL}/api/books",
        headers=headers,
        timeout=10
    )
    response.raise_for_status()
    print(response.json())

    print("\n=== LOANS ===")
    response = requests.get(
        f"{BASE_URL}/api/loans",
        headers=headers,
        timeout=10
    )
    response.raise_for_status()
    print(response.json())

    print("\n=== SUMMARY ===")
    response = requests.get(
        f"{BASE_URL}/api/summary",
        headers=headers,
        timeout=10
    )
    response.raise_for_status()
    print(response.json())


if __name__ == "__main__":
    main()
