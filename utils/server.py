import os
import re
import yaml
import json
from datetime import date
daysOfWeek={"R":0,"M":1,"T":2, "W":3 ,"U":4, "F":5,"S":6}
PATH = ""

def delete_keys(dic:dict):
    del dic["startTime"]
    del dic["endTime"]
    del dic["date"]

def update_db():
    pattern = re.compile(r'---\n(.*?)\n---', re.DOTALL)
    files = [f for f in os.listdir(PATH) if os.path.isfile(PATH+f)]
    
    for file in files:
        full = PATH+file
        with open(full, "r") as f:
            contents = f.read()
            matches = pattern.findall(contents)
            if matches:
                res:dict = yaml.safe_load(matches[0])
                for key in res: # Yaml returns as a date obj
                    if type(res[key] ) == date:
                        res[key] = res[key].strftime("%Y-%m-%d")
                try: #Obsidian parses dates as chars
                    res["daysOfWeek"]=[daysOfWeek[element] for element in res["daysOfWeek"]]
                except:
                    pass
    
                if "startTime" in res.keys():#For some reason FullCallendar cant handle the "startTime" if its for only one day
                    if type(res["startTime"])==str:
                        day = res["date"]
                        res["start"]=day+"T"+res["startTime"]+":00"
                        res["end"]=day+"T"+res["endTime"]+":00"
                        delete_keys(res)
                r = json.dumps(res)
                print(r)
