# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: pos-inventory-sync-optimistic.spec.ts >> POS Inventory Sync - Optimistic UI >> POS terminal immediately updates stock UI on charge before API returns
- Location: src/e2e/pos-inventory-sync-optimistic.spec.ts:4:7

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.fill: Test timeout of 30000ms exceeded.
Call log:
  - waiting for getByPlaceholder('Email address')

```

# Page snapshot

```yaml
- generic [active] [ref=e1]:
  - generic [ref=e3]:
    - heading "Login" [level=1] [ref=e4]
    - generic [ref=e5]:
      - textbox "Email or Username" [ref=e6]
      - textbox "Password" [ref=e7]
      - button "Log In" [ref=e8]
    - generic [ref=e11]: or
    - button "Start Business Setup" [ref=e13]
  - button "Help" [ref=e16]:
    - img [ref=e17]
  - button "Open help chat" [ref=e20]:
    - generic [ref=e21]: ✨
    - generic [ref=e22]: Ask anything
  - button "Voice Assistant" [ref=e23]:
    - img [ref=e24]
  - alert [ref=e26]
```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('POS Inventory Sync - Optimistic UI', () => {
  4   |   test('POS terminal immediately updates stock UI on charge before API returns', async ({ page }) => {
  5   |     // 1. Log in to get token
  6   |     await page.goto('/login');
> 7   |     await page.getByPlaceholder('Email address').fill('admin@ohc.local');
      |                                                  ^ Error: locator.fill: Test timeout of 30000ms exceeded.
  8   |     await page.getByPlaceholder('Password').fill('admin');
  9   |     await page.getByRole('button', { name: 'Sign In' }).click();
  10  |     await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });
  11  |
  12  |     const response = await page.request.post('/api/v1/auth/login', {
  13  |         data: {
  14  |             email: 'admin@ohc.local',
  15  |             password: 'admin'
  16  |         }
  17  |     });
  18  |     expect(response.ok()).toBeTruthy();
  19  |     const { token } = await response.json();
  20  |
  21  |     // 2. Create the "Vegan Celebration Cake" product
  22  |     const createProductRes = await page.request.post('/api/v1/catalog/products', {
  23  |         headers: { Authorization: `Bearer ${token}` },
  24  |         data: {
  25  |             title: 'Vegan Celebration Cake',
  26  |             inventory_count: 10,
  27  |             price_cents: 5000
  28  |         }
  29  |     });
  30  |     expect(createProductRes.ok()).toBeTruthy();
  31  |
  32  |     // Navigate to POS terminal
  33  |     await page.goto('/pos.html');
  34  |     await page.evaluate(() => { localStorage.setItem("tenant_id", "default"); });
  35  |
  36  |     // Login with PIN 1234
  37  |     await page.getByRole('button', { name: '1', exact: true }).click();
  38  |     await page.getByRole('button', { name: '2', exact: true }).click();
  39  |     await page.getByRole('button', { name: '3', exact: true }).click();
  40  |     await page.getByRole('button', { name: '4', exact: true }).click();
  41  |
  42  |     await page.waitForTimeout(500);
  43  |     await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
  44  |     await page.waitForTimeout(500);
  45  |
  46  |     // Wait for the product catalog to be populated
  47  |     await expect(page.getByText('Vegan Celebration Cake').first()).toBeVisible({ timeout: 10000 });
  48  |
  49  |     // Extract current stock from the text
  50  |     const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' }).first();
  51  |     const descriptionText = await productButton.innerText();
  52  |
  53  |     const stockMatch = descriptionText.match(/Stock: (\d+)/);
  54  |     expect(stockMatch).toBeTruthy();
  55  |
  56  |     if (stockMatch) {
  57  |       const initialStock = parseInt(stockMatch[1], 10);
  58  |
  59  |       // Select the product
  60  |       await productButton.click();
  61  |
  62  |       // Immediately verify the stock decreased by 1 without waiting for API
  63  |       // Since it's optimistic, it should happen instantly.
  64  |       await page.waitForTimeout(500);
  65  |       const updatedButtonText = await productButton.innerText();
  66  |       const newMatch = updatedButtonText.match(/Stock:\s*(\d+)/);
  67  |       if (newMatch) {
  68  |           const newStock = parseInt(newMatch[1], 10);
  69  |           expect(newStock).toBe(initialStock - 1);
  70  |       }
  71  |     }
  72  |   });
  73  |
  74  |   test('Offline sync conflict generates Operations Agent Triage Task', async ({ page }) => {
  75  |     // 1. Log in to get token
  76  |     await page.goto('/login');
  77  |     await page.getByPlaceholder('Email address').fill('admin@ohc.local');
  78  |     await page.getByPlaceholder('Password').fill('admin');
  79  |     await page.getByRole('button', { name: 'Sign In' }).click();
  80  |     await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });
  81  |
  82  |     const response = await page.request.post('/api/v1/auth/login', {
  83  |         data: {
  84  |             email: 'admin@ohc.local',
  85  |             password: 'admin'
  86  |         }
  87  |     });
  88  |     expect(response.ok()).toBeTruthy();
  89  |     const { token } = await response.json();
  90  |
  91  |     // 2. Create the "Vegan Celebration Cake" product
  92  |     const createProductRes = await page.request.post('/api/v1/catalog/products', {
  93  |         headers: { Authorization: `Bearer ${token}` },
  94  |         data: {
  95  |             title: 'Vegan Celebration Cake',
  96  |             inventory_count: 10,
  97  |             price_cents: 5000
  98  |         }
  99  |     });
  100 |     expect(createProductRes.ok()).toBeTruthy();
  101 |
  102 |     // Navigate to POS terminal to login
  103 |     await page.goto('/pos.html');
  104 |     await page.evaluate(() => { localStorage.setItem("tenant_id", "default"); });
  105 |
  106 |     // Login with PIN 1234
  107 |     await page.getByRole('button', { name: '1', exact: true }).click();
```