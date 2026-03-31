import os
import re

filepath = 'srcs/app/lib/screens/service_screen.dart'
with open(filepath, 'r') as f:
    content = f.read()

content = content.replace("icon: Icon(_isRunning ? Icons.stop : Icons.play_arrow),", "icon: _isLoading ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2)) : Icon(_isRunning ? Icons.stop : Icons.play_arrow),")

with open(filepath, 'w') as f:
    f.write(content)
