import glob, os

target = """    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');"""

helper = """
// REFACTOR: Centralized common E2E setup instructions
async function performStandardSetup(page: any) {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');
}
"""

replacement = "    await performStandardSetup(page);"

for filepath in glob.glob('src/e2e/*.ts'):
    with open(filepath, 'r') as f:
        content = f.read()

    if target in content:
        content = content.replace(target, replacement)
        if 'performStandardSetup' not in content:
            content = content.replace("test.describe(", helper + "\n\ntest.describe(")
        with open(filepath, 'w') as f:
            f.write(content)
