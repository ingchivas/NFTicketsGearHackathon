# -*- coding: utf-8 -*-
"""
Created on Wed Dec 14 18:50:23 2022

@author: Caludio
"""

from Generator import generator
import requests
from flask import Flask, request, render_template, redirect, url_for, flash, session, jsonify
import random
import os

app = Flask(__name__)
app.config["DEBUG"] = True


directory = "./utils/flk"

def genNFTs(n, wallet):
    generator_object = generator(directory,wallet)
    generator_object.generate_multiple_nfts(n)
    generator_object.write_combinations()
    


@app.route("/api/v1/generate", methods=["GET"])
def generate():
    if request.method == "GET":
        n = 1
        wallet = request.json["wallet"]
        print(wallet)
        genNFTs(n, wallet)
        
        # Get random image path from output directory
        random_image = random.choice(os.listdir("./output/utils/flk"))
        # Get filename of random image
        random_image_name = random_image.split(".")[0]
        
        # Upload image to IPFS
        pinataURL = "https://api.pinata.cloud/pinning/pinFileToIPFS"

        payload={'pinataOptions': '{"cidVersion": 1}',
        'pinataMetadata': '{"name": "st", "keyvalues": {"company": "Pinata"}}'}
        files=[
        ('file',('test.png',open(random_image,'rb'),'application/octet-stream'))
        ]
        headers = {
        'Authorization': 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI4ZTJhMjQ1OC03ZjZiLTQ3N2UtYTY1Yy1iMmZkZjQ3NDExOTUiLCJlbWFpbCI6ImpwY2FsZnNlY0BnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwicGluX3BvbGljeSI6eyJyZWdpb25zIjpbeyJpZCI6IkZSQTEiLCJkZXNpcmVkUmVwbGljYXRpb25Db3VudCI6MX0seyJpZCI6Ik5ZQzEiLCJkZXNpcmVkUmVwbGljYXRpb25Db3VudCI6MX1dLCJ2ZXJzaW9uIjoxfSwibWZhX2VuYWJsZWQiOmZhbHNlLCJzdGF0dXMiOiJBQ1RJVkUifSwiYXV0aGVudGljYXRpb25UeXBlIjoic2NvcGVkS2V5Iiwic2NvcGVkS2V5S2V5IjoiYjY2MDY2ODA0NWE4MWU0ZDZjMTIiLCJzY29wZWRLZXlTZWNyZXQiOiIxMDNmYWU5YWM4ZmVmODU3MzY3MTIwN2YwY2ZlNDMwODczODBkYTgyNWNjMTVmMjU3ZjJjNmYyYTAyOTg2NDcyIiwiaWF0IjoxNjcxMjIyNTM1fQ.H7_3xGQxCkKHkXwJZD9K7y1vQMRg7dN2UBvBj1CP74k'
        }
        
        return jsonify({"message": "Success", "image": random_image_name})

    else:
        return jsonify({"message": "Error"})

if __name__ == "__main__":
    app.run(host='0.0.0.0', port=5000)