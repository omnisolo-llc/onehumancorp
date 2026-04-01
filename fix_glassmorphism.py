import os
import re

files_to_fix = [
    "srcs/app/lib/screens/dashboard_screen.dart",
    "srcs/app/lib/screens/landing_screen.dart",
    "srcs/app/lib/screens/login_screen.dart",
    "srcs/app/lib/screens/settings_screen.dart",
    "srcs/app/lib/screens/user_management_screen.dart",
    "srcs/app/lib/screens/wizard_screen.dart",
]

glassmorphism_code = """ImageFilter.compose(
            outer: ColorFilter.matrix(<double>[
              1.168, -0.153, -0.015, 0, 0,
              -0.046, 1.061, -0.015, 0, 0,
              -0.046, -0.152, 1.198, 0, 0,
              0, 0, 0, 1, 0,
            ]),
            inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          )"""

for file_path in files_to_fix:
    with open(file_path, 'r') as f:
        content = f.read()

    # Check if file has just ImageFilter.blur as filter
    if "filter: ImageFilter.blur(" in content and "ImageFilter.compose" not in content:
        content = re.sub(r'filter:\s*ImageFilter\.blur\(sigmaX:\s*20\.0,\s*sigmaY:\s*20\.0\),', f'filter: {glassmorphism_code},', content)
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed {file_path}")
