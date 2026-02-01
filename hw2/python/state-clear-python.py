#!/usr/bin/env python3

import os
import http.cookies

SESSION_DIR = '/tmp/python_sessions'

print("Cache-Control: no-cache")

# Get session
cookie = http.cookies.SimpleCookie()
cookie_string = os.environ.get('HTTP_COOKIE', '')
cookie.load(cookie_string)

if 'session_id' in cookie:
    session_id = cookie['session_id'].value
    session_file = os.path.join(SESSION_DIR, f'sess_{session_id}')
    
    if os.path.exists(session_file):
        os.remove(session_file)

print("Set-Cookie: session_id=; Path=/; Max-Age=0")
print("Location: state-python.py\n")
