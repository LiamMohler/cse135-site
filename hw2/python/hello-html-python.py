#!/usr/bin/env python3

import os
from datetime import datetime

print("Cache-Control: no-cache")
print("Content-Type: text/html\n")

date = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
ip = os.environ.get('REMOTE_ADDR', 'Unknown')

print("<!DOCTYPE html>")
print("<html>")
print("<head>")
print("<title>Hello Python</title>")
print("</head>")
print("<body>")
print("<h1>Hello, Python!</h1>")
print("<p>Greeting from Team Liam</p>")
print("<p>This page was generated with the Python programming language</p>")
print(f"<p>Generated at: {date}</p>")
print(f"<p>Your IP Address: {ip}</p>")
print("</body>")
print("</html>")
