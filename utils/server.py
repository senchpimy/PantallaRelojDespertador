import os
import re
import yaml
import json
from datetime import date, datetime
from flask import Flask
app = Flask(__name__)

import os
daysOfWeek={"R":0,"M":1,"T":2, "W":3 ,"U":4, "F":5,"S":6}
CONFIG_PATH = ".obsidian/plugins/obsidian-full-calendar/data.json"
PATH = "/home/plof/Documents/4to-semstre-fes/4toSemestre/"

def delete_keys(dic:dict):
    del dic["startTime"]
    del dic["endTime"]
    del dic["date"]

def update_db():
    pattern = re.compile(r'---\n(.*?)\n---', re.DOTALL)
    arr={}
    arr["events"]=[]
    calendarios = get_paths()
    for cal in calendarios:
        route = cal["path"]+"/"
        files = [f for f in os.listdir(PATH+cal["path"]) if os.path.isfile(PATH+route+f)]
        
        for file in files:
            full = PATH+route+file
            with open(full, "r") as f:
                contents = f.read()
                matches = pattern.findall(contents)
                if matches:
                    res:dict = yaml.safe_load(matches[0])
                    for key in res: # Yaml returns as a date obj
                        if type(res[key] ) == date:
                            res[key] = res[key].strftime("%Y-%m-%d")
                            continue
                        if type(res[key] ) == datetime:
                            res[key] = res[key].strftime("%Y-%m-%dT%H:%M:%S")
                    try: #Obsidian parses dates as chars
                        res["daysOfWeek"]=[daysOfWeek[element] for element in res["daysOfWeek"]]
                    except:
                        pass
        
                    if "startTime" in res.keys():#For some reason FullCallendar cant handle the "startTime" if its for only one day
                        if type(res["startTime"])==str:
                            if type(res["endTime"])==int:
                                hora = int(res["endTime"] / 60)
                                min = int(res["endTime"] % 60)
                                res["endTime"]= "{:02d}:{:02d}".format(hora,min)
                            day = res["date"]
                            res["start"]=day+"T"+res["startTime"]+":00"
                            res["end"]=day+"T"+res["endTime"]+":00"
                            delete_keys(res)
                        elif type(res["startTime"])==int:
                            hora = int(res["startTime"] / 60)
                            min = int(res["startTime"] % 60)
                            res["startTime"]= "{:02d}:{:02d}".format(hora,min)
                            day = res["date"]
                            nday = res["endDate"]
                            res["start"]=day+"T"+res["startTime"]+":00"
                            res["end"]=nday+"T"+res["endTime"]+":00"
                            delete_keys(res)
                    arr["events"].append(res.copy())
    r = json.dumps(arr)
    return r

def get_paths():
    doc = json.loads(open(PATH+CONFIG_PATH,"r").read())
    calendarios = []
    for element in doc["calendarSources"]:
        cal = {}
        cal["path"]=element["directory"]
        cal["color"]=element["color"]
        calendarios.append(cal)
    return calendarios


hostname="localhost"
port = 8080

@app.route('/')
def ret_data():
    return update_db()

if __name__=="__main__":
    app.run(debug=True,port=port) # Run the Flask app
    try:
        print("Server Started")
    except KeyboardInterrupt:
        pass
    print("Server STOPPED")
