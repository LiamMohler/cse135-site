#!/usr/bin/env python3

import os
import http.cookies
import json

SESSION_DIR = '/tmp/python_sessions'

print("Cache-Control: no-cache")
print("Content-Type: text/html\n")

# Get session
cookie = http.cookies.SimpleCookie()
cookie_string = os.environ.get('HTTP_COOKIE', '')
cookie.load(cookie_string)

session_data = None
if 'session_id' in cookie:
    session_id = cookie['session_id'].value
    session_file = os.path.join(SESSION_DIR, f'sess_{session_id}')
    
    if os.path.exists(session_file):
        with open(session_file, 'r') as f:
            session_data = json.load(f)

print("<!DOCTYPE html>")
print("<html>")
print("<head><title>Python State View</title></head>")
print("<body>")
print("<h1>Python State - View</h1>")

if session_data and session_data.get('username'):
    print(f"<p><strong>Username:</strong> {session_data['username']}</p>")
    print(f"<p><strong>Favorite Color:</strong> {session_data['favorite_color']}</p>")
    print(f"<p><strong>Notes:</strong> {session_data['notes']}</p>")
    print("<form method='POST' action='state-clear-python.py'>")
    print("<button type='submit'>Clear</button>")
    print("</form>")
else:
    print("<p>No data</p>")

print("<p><a href='state-python.py'>Back</a></p>")
print("</body></html>")
