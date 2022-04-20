from urllib import request
from wsgiref import headers
from flask import Flask, request
import requests
import threading
import os

app = Flask(__name__)
dicMap = {}
threadLock = threading.Lock()


@app.route("/")
def index():
    return "server is ok"


# json {url:String, headers:Map}
@app.route("/m3u8", methods=["GET", "POST"])
def m3u8():
    url = request.json["url"]
    headers = request.json["headers"]
    if url in dicMap.keys():
        return {"code": 1}
    proc(url, headers)

    return {"code": 0}


class threading1(threading.Thread):
    def __init__(self, url, headers):
        threading.Thread.__init__(self)

        self.url = url
        self.headers = headers

    def run(self):
        url = self.url

        threadLock.acquire()
        dicMap[url] = 1
        threadLock.release()
        m3u8 = requests.get(url, headers=self.headers)

        idx = url.rindex("/")
        basePath = url[0:idx+1]
        baseName = url[idx+1:]+".ts"
        for line in m3u8.text.split("\n"):
            if line.startswith("#"):
                continue

            url1 = basePath+line
            print("begin downloading ", url1)
            r = requests.get(url1, headers=self.headers)
            if not os.path.exists("result"):
                os.makedirs("result")

            with open("result/%s" % baseName, "ab") as f:
                f.write(r.content)
        del dicMap[url]
        print("saved", baseName)


def proc(url, headers):
    threading1(url, headers).start()


app.run("0.0.0.0", 2000)
