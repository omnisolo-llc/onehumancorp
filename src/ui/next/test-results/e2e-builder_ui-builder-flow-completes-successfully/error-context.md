# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: e2e/builder_ui.spec.ts >> builder flow completes successfully
- Location: e2e/builder_ui.spec.ts:3:5

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByText(/Let's build your store/i)
Expected: visible
Timeout: 15000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for getByText(/Let's build your store/i)

```

```yaml
- heading "What are you building today?" [level=1]
- button "🛍️ Selling Products"
- button "🛠️ Offering Services"
- button "✨ Showcasing Work"
- button "Help":
  - img
- button "✨ Ask anything"
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test('builder flow completes successfully', async ({ page }) => {
  4  |   // Use the baseURL from playwright config (or relative to it if Next is served there)
  5  |   await page.goto('http://localhost:3000/builder');
  6  |
  7  |   // 1. Onboarding Screen - Step 1: Basics
> 8  |   await expect(page.getByText(/Let's build your store/i)).toBeVisible({ timeout: 15000 });
     |                                                           ^ Error: expect(locator).toBeVisible() failed
  9  |
  10 |   const nameInput = page.getByPlaceholder(/e.g. Acme Corp/i);
  11 |   await nameInput.fill('My Awesome Store');
  12 |
  13 |   const categoryInput = page.getByPlaceholder(/e.g. Retail, Consulting, Tech/i);
  14 |   await categoryInput.fill('Retail');
  15 |
  16 |   await page.getByRole('button', { name: /Next: Choose Vibe/i }).click();
  17 |
  18 |   // Step 2: Vibe
  19 |   await expect(page.getByText(/Select Your Vibe/i)).toBeVisible();
  20 |   await page.getByRole('button', { name: 'Friendly' }).click();
  21 |   await page.getByRole('button', { name: /Next: Details/i }).click();
  22 |
  23 |   // Step 3: Final Details
  24 |   await expect(page.getByText(/Final Details/i)).toBeVisible();
  25 |   const textarea = page.getByPlaceholder(/e.g. I run a mobile dog grooming service/i);
  26 |   await expect(textarea).toBeVisible();
  27 |
  28 |   // The bio should be pre-filled, but we can append or replace it
  29 |   await textarea.fill('I run a friendly retail store selling amazing products');
  30 |
  31 |   // Click Generate
  32 |   const buildButton = page.getByRole('button', { name: /Build Store/i });
  33 |   await buildButton.click();
  34 |
  35 |   // 2. Generating Screen
  36 |   await expect(page.getByText(/The Promoter is picking colors/i)).toBeVisible();
  37 |
  38 |   // 3. Draft Preview Screen
  39 |   await expect(page.getByText(/Preview Mode/i)).toBeVisible({ timeout: 5000 });
  40 |   await expect(page.getByText(/1-Tap Launch/i)).toBeVisible();
  41 |
  42 |   // 4. Click Launch
  43 |   await page.getByRole('button', { name: /1-Tap Launch/i }).click();
  44 |
  45 |   // 5. Launch Screen
  46 |   await expect(page.getByText(/You're Live/i)).toBeVisible({ timeout: 5000 });
  47 | });
  48 |
```