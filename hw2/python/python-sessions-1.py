#!/usr/bin/python3
import cgi
import cgitb
import os
import http.cookies
import hashlib
import json
from urllib.parse import parse_qs

cgitb.enable()

# Get query string
query_string = os.environ.get('QUERY_STRING', '')
params = parse_qs(query_string)

# Get or create session
cookie = http.cookies.SimpleCookie()
if 'HTTP_COOKIE' in os.environ:
    cookie.load(os.environ['HTTP_COOKIE'])

session_id = None
if 'session_id' in cookie:
    session_id = cookie['session_id'].value
else:
    # Create new session ID
    session_id = hashlib.md5(os.urandom(16)).hexdigest()

# Session file path
session_file = f"/tmp/python_sessions/session_{session_id}.json"

# Load or create session data
session_data = {}
if os.path.exists(session_file):
    with open(session_file, 'r') as f:
        session_data = json.load(f)

# Update username if provided
if 'username' in params:
    session_data['username'] = params['username'][0]

# Save session data
os.makedirs("/tmp/python_sessions", exist_ok=True)
with open(session_file, 'w') as f:
    json.dump(session_data, f)

# Get username
username = session_data.get('username', '')

# Set cookie
cookie['session_id'] = session_id
cookie['session_id']['path'] = '/'

print("Content-Type: text/html")
print(cookie.output())
print()

print("""<!DOCTYPE html>
<html>
<head>
    <title>Python Sessions</title>
</head>
<body>
    <h1>Python Sessions Page 1</h1>
""")

if username:
    print(f"    <p><b>Name:</b> {username}</p>")
else:
    print("    <p><b>Name:</b> You do not have a name set</p>")

print("""
    <br><br>
    <a href="python-sessions-2.py">Session Page 2</a><br>
    <a href="python-cgiform.html">Python CGI Form</a><br>
    <form style="margin-top:30px" action="python-destroy-session.py" method="get">
        <button type="submit">Destroy Session</button>
    </form>
</body>
</html>
""")
