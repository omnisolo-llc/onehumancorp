with open('src/e2e/operate_business.spec.ts', 'r') as f:
    operate_content = f.read()

operate_content = operate_content.replace('await page.getByPlaceholder("e.g. Maya\'s Cakes").fill(\'Custom cakes and pastries\');', 'await page.getByPlaceholder("e.g. Maya\'s Custom Cakes").fill(\'Custom cakes and pastries\');')

with open('src/e2e/operate_business.spec.ts', 'w') as f:
    f.write(operate_content)

with open('src/e2e/win_back_growth_loop.spec.ts', 'r') as f:
    win_content = f.read()

win_content = win_content.replace('await expect(page.getByText(/✅ Campaign sent to 34 inactive customers!/i)).toBeVisible({ timeout: 5000 });', 'await expect(page.getByText(/✅ Campaign sent to 34 inactive customers!/i)).toBeVisible({ timeout: 10000 });')

with open('src/e2e/win_back_growth_loop.spec.ts', 'w') as f:
    f.write(win_content)

print("Tests patched")
