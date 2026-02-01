#!/usr/bin/env python3
import os
import sys

print("Cache-Control: no-cache")
print("Content-Type: text/html\n")

protocol = os.environ.get('SERVER_PROTOCOL', '')
method = os.environ.get('REQUEST_METHOD', '')
query_string = os.environ.get('QUERY_STRING', '')

content_length = int(os.environ.get('CONTENT_LENGTH', 0))
form_data = sys.stdin.read(content_length) if content_length > 0 else ''

print("<!DOCTYPE html>")
print("<html><head><title>General Request Echo</title>")
print("</head><body><h1 align='center'>General Request Echo</h1>")
print("<hr>")
print(f"<p><b>HTTP Protocol:</b> {protocol}</p>")
print(f"<p><b>HTTP Method:</b> {method}</p>")
print(f"<p><b>Query String:</b> {query_string}</p>")
print(f"<p><b>Message Body:</b> {form_data}</p>")
print("</body></html>")
