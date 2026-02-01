#!/usr/bin/python3
import cgi
import cgitb
import os
import http.cookies

cgitb.enable()

# Get session from cookie
cookie = http.cookies.SimpleCookie()
if 'HTTP_COOKIE' in os.environ:
    cookie.load(os.environ['HTTP_COOKIE'])

if 'session_id' in cookie:
    session_id = cookie['session_id'].value
    session_file = f"/tmp/python_sessions/session_{session_id}.json"
    
    # Delete session file
    if os.path.exists(session_file):
        os.remove(session_file)
    
    # Expire cookie
    cookie['session_id'] = session_id
    cookie['session_id']['path'] = '/'
    cookie['session_id']['expires'] = 'Thu, 01 Jan 1970 00:00:00 GMT'

print("Content-Type: text/html")
if 'session_id' in cookie:
    print(cookie.output())
print()

print("""<!DOCTYPE html>
<html>
<head>
    <title>Session Destroyed</title>
</head>
<body>
    <h1>Session Destroyed</h1>
    <p>Your session has been cleared.</p>
    <a href="python-cgiform.html">Back to the Python CGI Form</a><br>
    <a href="python-sessions-1.py">Back to Page 1</a><br>
    <a href="python-sessions-2.py">Back to Page 2</a>
</body>
</html>
""")
