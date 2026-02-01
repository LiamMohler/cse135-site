#!/usr/bin/env python3

import os
import json
from datetime import datetime

print("Cache-Control: no-cache")
print("Content-Type: application/json\n")

date = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
ip = os.environ.get('REMOTE_ADDR', 'Unknown')

data = {
    'title': 'Hello, Python!',
    'heading': 'Hello, Python!',
    'message': 'This page was generated with the Python programming language',
    'team': 'Liam',
    'time': date,
    'ip': ip
}

print(json.dumps(data))
