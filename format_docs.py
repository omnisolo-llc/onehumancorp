import os
import re

wrapper_start = '<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">'
wrapper_end = '</div>'

# Regex to check if file already has the wrapper
wrapper_pattern = re.compile(r'<div[^>]*backdrop-filter:\s*blur\(20px\).*?>', re.IGNORECASE)

docs_dir = 'docs'
for root, dirs, files in os.walk(docs_dir):
    for file in files:
        if file.endswith('.md'):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            if not wrapper_pattern.search(content):
                print(f"Applying wrapper to {filepath}")
                new_content = f"{wrapper_start}\n\n{content}\n\n{wrapper_end}\n"
                with open(filepath, 'w') as f:
                    f.write(new_content)
