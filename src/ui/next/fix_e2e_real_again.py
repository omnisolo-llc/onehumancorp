import sys

def fix_file(filepath, replacements):
    with open(filepath, 'r') as f:
        content = f.read()

    for old, new in replacements:
        content = content.replace(old, new)

    with open(filepath, 'w') as f:
        f.write(content)

fix_file('src/e2e/conversational_checkout.spec.ts', [
    ('test.describe.skip(\'Conversational Checkout', 'test.describe(\'Conversational Checkout')
])
