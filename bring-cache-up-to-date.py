import time
import requests

local = False 
reset_from_zero = False # False to continue from where it left off  
fly_app_name = "devhub-cache-api-rs-2"
# ~120 calls for devhub
# ~20 calls for infra
# ~40 calls for events
max_calls = 120 # This is for devhub to catch up to the latest block

base_url = f"http://localhost:8080/" if local else f"https://{fly_app_name}.fly.dev/"

def call_api(count):
    url = f"{base_url}proposals"  # Replace with your API URL
    try:
        response = requests.get(url)
        if response.status_code == 200:
            print(f"{count} API call successful: - response length {response.json().get('total_records')}")
        else:
            print("API call failed with status code:", response.status_code)
    except requests.exceptions.RequestException as e:
        print("An error occurred:", e)
    except Exception as e:
        print("An error2 occurred:", e)
        print(response.json())

def reset_cache():
    url = f"{base_url}proposals/info/reset"  # Replace with your API URL
    try:
        response = requests.get(url)
        if response.status_code == 200:
            print("Cache reset successful")
        else:
            print("Cache reset failed with status code:", response.status_code)
    except requests.exceptions.RequestException as e:
        print("An error occurred:", e)

def main():
    if reset_from_zero:
        reset_cache()
    count = 0
    while count < max_calls: 
        call_api(count)
        count += 1
        # 6 calls/minute limit of near blocks https://nearblocks.io/apis#
        time.sleep(0.5)  # Wait for 11 seconds before the next call

if __name__ == "__main__":
    main()
