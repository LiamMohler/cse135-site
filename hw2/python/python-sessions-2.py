#!/usr/bin/python3
import cgi
import cgitb
import os
import http.cookies
import json

cgitb.enable()

# Get session from cookie
cookie = http.cookies.SimpleCookie()
if 'HTTP_COOKIE' in os.environ:
    cookie.load(os.environ['HTTP_COOKIE'])

session_id = None
username = ''

if 'session_id' in cookie:
    session_id = cookie['session_id'].value
    session_file = f"/tmp/python_sessions/session_{session_id}.json"
    
    # Load session data
    if os.path.exists(session_file):
        with open(session_file, 'r') as f:
            session_data = json.load(f)
            username = session_data.get('username', '')

print("Content-Type: text/html")
print()

print("""<!DOCTYPE html>
<html>
<head>
    <title>Python Sessions Page 2</title>
</head>
<body>
    <h1>Python Sessions Page 2</h1>
""")

if username:
    print(f"    <p><b>Name:</b> {username}</p>")
else:
    print("    <p><b>Name:</b> You do not have a name set</p>")

print("""
    <br><br>
    <a href="python-sessions-1.py">Session Page 1</a><br>
    <a href="python-cgiform.html">Python CGI Form</a><br>
    <form style="margin-top:30px" action="python-destroy-session.py" method="get">
        <button type="submit">Destroy Session</button>
    </form>
</body>
</html>
""")
