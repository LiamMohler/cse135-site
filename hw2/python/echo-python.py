#!/usr/bin/env python3

import os
import sys
import json
from datetime import datetime
from urllib.parse import parse_qs
import socket

print("Cache-Control: no-cache")
print("Content-Type: text/html\n")

method = os.environ.get('REQUEST_METHOD', 'GET')
date = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
ip = os.environ.get('REMOTE_ADDR', 'Unknown')
user_agent = os.environ.get('HTTP_USER_AGENT', 'Unknown')
hostname = socket.gethostname()

data = {}
if method == 'GET':
    query_string = os.environ.get('QUERY_STRING', '')
    data = parse_qs(query_string)
    data = {k: v[0] if len(v) == 1 else v for k, v in data.items()}
else:
    content_length = int(os.environ.get('CONTENT_LENGTH', 0))
    content_type = os.environ.get('CONTENT_TYPE', '')
    if content_length > 0:
        input_data = sys.stdin.read(content_length)
        if 'application/json' in content_type:
            data = json.loads(input_data)
        else:
            data = parse_qs(input_data)
            data = {k: v[0] if len(v) == 1 else v for k, v in data.items()}

# Remove form control fields
data.pop('language', None)
data.pop('method', None)

print("<!DOCTYPE html>")
print("<html>")
print("<head><title>Python Echo</title></head>")
print("<body>")
print("<h1>Python Echo</h1>")
print(f"<p><strong>Method:</strong> {method}</p>")
print(f"<p><strong>Hostname:</strong> {hostname}</p>")
print(f"<p><strong>Date/Time:</strong> {date}</p>")
print(f"<p><strong>User Agent:</strong> {user_agent}</p>")
print(f"<p><strong>IP:</strong> {ip}</p>")

print("<h2>Data Received</h2>")
if data:
    print("<table border='1'>")
    print("<tr><th>Key</th><th>Value</th></tr>")
    for key, value in data.items():
        print(f"<tr><td>{key}</td><td>{value}</td></tr>")
    print("</table>")
else:
    print("<p>No data</p>")

print("</body></html>")
