import os
import re

lib_dir = "srcs/app/lib"

def check_unused_imports(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # If it imports glass_card but doesn't use it, remove it
    if "import '../widgets/glass_card.dart';" in content or "import '../../widgets/glass_card.dart';" in content:
        if 'GlassCard(' not in content and 'GlassCard' not in content:
            content = re.sub(r"import\s+['\"](?:\.\./)+widgets/glass_card\.dart['\"];\n?", "", content)
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Removed from {filepath}")

for root, dirs, files in os.walk(lib_dir):
    for file in files:
        if file.endswith('.dart'):
            filepath = os.path.join(root, file)
            check_unused_imports(filepath)
