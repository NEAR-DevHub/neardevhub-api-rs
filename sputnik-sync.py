import time
import requests # type: ignore

local = False 
reset_from_zero = False # False to continue from where it left off  
fly_app_name = "testing-indexer-2"
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

def start_at_block0(sputnik_contract):
    url = f"{base_url}/dao/proposals/{sputnik_contract}/block/0"  # Replace with your API URL
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
    sputnik_contract = "testing-treasury.sputnik-dao.near" # shitzu "testing-treasury.sputnik-dao.near"
    if reset_from_zero:
        start_at_block0(sputnik_contract)
    count = 0
    while count < max_calls: 
        call_api(count, sputnik_contract) # ~ 2 minutes before we hit the max archival calls from fastnear
        count += 1
        time.sleep(3 * 60) # ~ +3 minutes to cool down from the rate limit.
        # It indexes about 30-60 proposals and their votes
        # so if we want to index 250 proposals per DAO,  it will take 5*5=25 minutes per DAO
        # 20 DAO = 500 minutes = 8.3 hours
        
        # not all DAO's have 250 proposals, so I will make a list of DAO's to index
        # and how long to run each one.

if __name__ == "__main__":
    main()
