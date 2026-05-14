import glob, os

for filepath in glob.glob('src/e2e/*.ts'):
    with open(filepath, 'r') as f:
        content = f.read()

    target4 = "'text=📦 Physical products'"
    replacement4 = "PHYSICAL_PRODUCTS_BTN"

    target5 = "'text=🌐 Online only'"
    replacement5 = "ONLINE_ONLY_BTN"

    target6 = "'text=✨ Modern'"
    replacement6 = "MODERN_THEME_BTN"

    if target4 in content or target5 in content or target6 in content:
        content = content.replace(target4, replacement4)
        content = content.replace(target5, replacement5)
        content = content.replace(target6, replacement6)

        constants = """const PHYSICAL_PRODUCTS_BTN = 'text=📦 Physical products';
const ONLINE_ONLY_BTN = 'text=🌐 Online only';
const MODERN_THEME_BTN = 'text=✨ Modern';

"""
        if "const PHYSICAL_PRODUCTS_BTN" not in content:
            content = content.replace("const START_BUSINESS_BTN = 'text=🚀 Start My Business';", constants + "const START_BUSINESS_BTN = 'text=🚀 Start My Business';")

        with open(filepath, 'w') as f:
            f.write(content)
