import re

files_to_fix = [
    "srcs/app/lib/screens/dashboard_screen.dart",
    "srcs/app/lib/screens/landing_screen.dart",
    "srcs/app/lib/screens/login_screen.dart",
    "srcs/app/lib/screens/settings_screen.dart",
    "srcs/app/lib/screens/user_management_screen.dart",
    "srcs/app/lib/screens/wizard_screen.dart",
]

for file_path in files_to_fix:
    with open(file_path, 'r') as f:
        content = f.read()

    print(f"--- {file_path} ---")
    matches = re.findall(r'BackdropFilter\((.*?)\s*child:', content, re.DOTALL)
    for m in matches:
        if "ImageFilter.blur" in m and "ColorFilter.matrix" not in m:
            print("MISSING MATRIX!")
        else:
            print("HAS MATRIX!")
