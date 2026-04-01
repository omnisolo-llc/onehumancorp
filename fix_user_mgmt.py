import re

file_path = "srcs/app/lib/screens/user_management_screen.dart"

glassmorphism_code = """ImageFilter.compose(
              outer: ColorFilter.matrix(<double>[
                1.168, -0.153, -0.015, 0, 0,
                -0.046, 1.061, -0.015, 0, 0,
                -0.046, -0.152, 1.198, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            )"""

with open(file_path, 'r') as f:
    content = f.read()

content = re.sub(r'filter:\s*const\s*ColorFilter\.matrix\(\[\s*1,\s*0,\s*0,\s*0,\s*0,\s*0,\s*1,\s*0,\s*0,\s*0,\s*0,\s*0,\s*1,\s*0,\s*0,\s*0,\s*0,\s*0,\s*1,\s*0,\s*//\s*Using\s*backdrop\s*filter\s*for\s*glassmorphism\s*\]\)', f'filter: {glassmorphism_code}', content)

with open(file_path, 'w') as f:
    f.write(content)
print(f"Fixed {file_path}")
