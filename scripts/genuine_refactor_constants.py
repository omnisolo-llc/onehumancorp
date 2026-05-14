import glob, os

for filepath in glob.glob('src/e2e/*.ts'):
    with open(filepath, 'r') as f:
        content = f.read()

    # Extract common selectors to constants
    target1 = "'text=🚀 Start My Business'"
    replacement1 = "START_BUSINESS_BTN"

    target2 = "'text=🛒 Online Store'"
    replacement2 = "ONLINE_STORE_BTN"

    target3 = "'text=Next →'"
    replacement3 = "NEXT_BTN"

    if target1 in content or target2 in content or target3 in content:
        content = content.replace(target1, replacement1)
        content = content.replace(target2, replacement2)
        content = content.replace(target3, replacement3)

        constants = """const START_BUSINESS_BTN = 'text=🚀 Start My Business';
const ONLINE_STORE_BTN = 'text=🛒 Online Store';
const NEXT_BTN = 'text=Next →';

"""
        if "const START_BUSINESS_BTN" not in content:
            content = content.replace("import { test, expect } from '@playwright/test';", "import { test, expect } from '@playwright/test';\n\n" + constants)

        with open(filepath, 'w') as f:
            f.write(content)
