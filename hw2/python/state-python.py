#!/usr/bin/env python3

import os
import sys
import http.cookies
import hashlib
import json
from urllib.parse import parse_qs
from pathlib import Path

SESSION_DIR = '/tmp/python_sessions'
Path(SESSION_DIR).mkdir(exist_ok=True)

print("Cache-Control: no-cache")

# Get or create session
cookie = http.cookies.SimpleCookie()
cookie_string = os.environ.get('HTTP_COOKIE', '')
cookie.load(cookie_string)

if 'session_id' in cookie:
    session_id = cookie['session_id'].value
else:
    session_id = hashlib.md5(str(os.getpid()).encode()).hexdigest()

session_file = os.path.join(SESSION_DIR, f'sess_{session_id}')

# Handle POST
if os.environ.get('REQUEST_METHOD') == 'POST':
    content_length = int(os.environ.get('CONTENT_LENGTH', 0))
    if content_length > 0:
        post_data = sys.stdin.read(content_length)
        data = parse_qs(post_data)
        
        session_data = {
            'username': data.get('username', [''])[0],
            'favorite_color': data.get('favorite_color', [''])[0],
            'notes': data.get('notes', [''])[0]
        }
        
        with open(session_file, 'w') as f:
            json.dump(session_data, f)
        
        print(f"Set-Cookie: session_id={session_id}; Path=/")
        print("Location: state-view-python.py\n")
        sys.exit(0)

# Display form
print(f"Set-Cookie: session_id={session_id}; Path=/")
print("Content-Type: text/html\n")

print("<!DOCTYPE html>")
print("<html>")
print("<head><title>Python State Input</title></head>")
print("<body>")
print("<h1>Python State - Input</h1>")
print("<form method='POST'>")
print("<label>Username:</label><br>")
print("<input type='text' name='username'><br><br>")
print("<label>Favorite Color:</label><br>")
print("<input type='text' name='favorite_color'><br><br>")
print("<label>Notes:</label><br>")
print("<textarea name='notes' rows='4'></textarea><br><br>")
print("<button type='submit'>Save</button>")
print("</form>")
print("<p><a href='state-view-python.py'>View Data</a></p>")
print("</body></html>")
