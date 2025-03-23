import time
import requests # type: ignore

local = False 
reset_from_zero = False # False to continue from where it left off  
fly_app_name = "sputnik-indexer-2"
max_calls = 120 # This is for devhub to catch up to the latest block

base_url = f"http://localhost:8080/" if local else f"https://{fly_app_name}.fly.dev"

def call_api(count, sputnik_contract):
    url = f"{base_url}/dao/proposals/{sputnik_contract}"  # Replace with your API URL
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

def reset_cache(sputnik_contract):
    url = f"{base_url}/dao/admin/{sputnik_contract}/reset"  # Replace with your API URL
    try:
        response = requests.get(url)
        if response.status_code == 200:
            print("Cache reset successful")
        else:
            print("Cache reset failed with status code:", response.status_code)
    except requests.exceptions.RequestException as e:
        print("An error occurred:", e)

def main():
    # TODO should be a list of sputnik contracts
    sputnik_contract = "testing-treasury.sputnik-dao.near"
    if reset_from_zero:
        reset_cache(sputnik_contract)
    count = 0
    while count < max_calls: 
        call_api(count, sputnik_contract)
        count += 1
        time.sleep(0.5)

if __name__ == "__main__":
    main()
