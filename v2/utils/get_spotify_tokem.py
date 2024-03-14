CLIENT_ID = ""
CLIENT_SECCRET = ""
TOKEN = ""

import base64
from requests import post,get
import json

def get_token():
    auth_string = CLIENT_ID+":"+CLIENT_SECCRET
    a_bytes = auth_string.encode("utf-8")
    a_base = str(base64.b64encode(a_bytes),"utf-8")

    url = "https://accounts.spotify.com/api/token"
    headers = {
            "Authorization": "Basic "+a_base,
            "Content-Type": "application/x-www-form-urlencoded"
            }
    data = {"grant_type":"client_credentials"}
    result = post(url,headers=headers,data=data)
    print(result.content)
    j_res = json.loads(result.content)
    token = j_res["access_token"]
    with open("result", "w") as f:
        f.write(token)
    print(token)

def get_auth_header(token):
    return {"Authorization": "Bearer "+token}

def search(name):
    url = "https://api.spotify.com/v1/search"
    headers = get_auth_header(TOKEN)
    query = f"?q={name}&type=artist&limit=1"

    query_url = url + query
    result = get(query_url, headers=headers)
    print(result.content)

def autorize():
    url = "https://accounts.spotify.com/authorize?"
search("ACDC")
