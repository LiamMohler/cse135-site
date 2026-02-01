#!/usr/bin/env python3
import os
import sys
from urllib.parse import parse_qs

print("Cache-Control: no-cache")
print("Content-Type: text/html\n")

content_length = int(os.environ.get('CONTENT_LENGTH', 0))
form_data = sys.stdin.read(content_length) if content_length > 0 else ''

print("<!DOCTYPE html>")
print("<html><head><title>POST Request Echo</title>")
print("</head><body><h1 align='center'>POST Request Echo</h1>")
print("<hr>")
print("<p><b>Message Body:</b></p>")
print("<ul>")

if form_data:
    params = parse_qs(form_data)
    for key, values in params.items():
        for value in values:
            print(f"<li>{key} = {value}</li>")

print("</ul>")
print("</body></html>")
