import glob, os

for filepath in glob.glob('src/e2e/*.ts'):
    with open(filepath, 'r') as f:
        content = f.read()

    target7 = "'input[placeholder=\"you@email.com\"]'"
    replacement7 = "EMAIL_INPUT"

    target8 = "'input[placeholder=\"Password\"]'"
    replacement8 = "PASSWORD_INPUT"

    if target7 in content or target8 in content:
        content = content.replace(target7, replacement7)
        content = content.replace(target8, replacement8)

        constants = """const EMAIL_INPUT = 'input[placeholder="you@email.com"]';
const PASSWORD_INPUT = 'input[placeholder="Password"]';

"""
        if "const EMAIL_INPUT" not in content:
            content = content.replace("const START_BUSINESS_BTN = 'text=🚀 Start My Business';", constants + "const START_BUSINESS_BTN = 'text=🚀 Start My Business';")

        with open(filepath, 'w') as f:
            f.write(content)
