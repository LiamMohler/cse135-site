#!/usr/bin/env python3

import os

print("Cache-Control: no-cache")
print("Content-Type: text/html\n")

print("<!DOCTYPE html>")
print("<html>")
print("<head>")
print("<title>Python Environment</title>")
print("</head>")
print("<body>")
print("<h1>Python Environment Variables</h1>")
print("<table border='1'>")
print("<tr><th>Variable</th><th>Value</th></tr>")

for key, value in sorted(os.environ.items()):
    print(f"<tr><td>{key}</td><td>{value}</td></tr>")

print("</table>")
print("</body>")
print("</html>")
