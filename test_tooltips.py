import os
import re

for root, dirs, files in os.walk('srcs/app/lib'):
    for file in files:
        if file.endswith('.dart'):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            # The regex `IconButton\((.*?)\)` stops at the FIRST `)` which is often the end of `Icon(Icons.refresh)`!
            # That's why my test script is broken. It is only matching `IconButton(onPressed: _refresh, icon: const Icon(Icons.refresh`
