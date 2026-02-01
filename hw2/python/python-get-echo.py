#!/usr/bin/env python3
import os
import sys
from urllib.parse import parse_qs, unquote

print("Cache-Control: no-cache")
print("Content-Type: text/html\n")

query_string = os.environ.get('QUERY_STRING', '')

print("<!DOCTYPE html>")
print("<html><head><title>GET Request Echo</title>")
print("</head><body><h1 align='center'>GET Request Echo</h1>")
print("<hr>")
print(f"<p><b>Query String:</b> {query_string}</p>")

if query_string:
    params = parse_qs(query_string)
    for key, values in params.items():
        for value in values:
            print(f"{key} = {value}<br/>")

print("</body></html>")
