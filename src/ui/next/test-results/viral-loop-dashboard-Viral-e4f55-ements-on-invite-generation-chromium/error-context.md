# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: viral-loop-dashboard.spec.ts >> Viral Loop Dashboard Widget >> dashboard surfaces viral loop metrics correctly and increments on invite generation
- Location: src/e2e/viral-loop-dashboard.spec.ts:4:9

# Error details

```
Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:3000/login
Call log:
  - navigating to "http://localhost:3000/login", waiting until "load"

```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Viral Loop Dashboard Widget', () => {
  4  |     test('dashboard surfaces viral loop metrics correctly and increments on invite generation', async ({ page }) => {
  5  |         // Go through the login flow
> 6  |         await page.goto('/login');
     |                    ^ Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:3000/login
  7  |         await page.fill('input[type="text"]', 'test-user');
  8  |         await page.fill('input[type="password"]', 'test-pass');
  9  |         await page.click('button[type="submit"]');
  10 |
  11 |         // Look for the "Viral Loop Performance" section
  12 |         const widgetHeader = page.locator('text=Viral Loop Performance');
  13 |         await expect(widgetHeader).toBeVisible({ timeout: 15000 });
  14 |
  15 |         // Get the initial number of Invites Sent
  16 |         const invitesSentLabel = page.locator('text=Invites Sent');
  17 |         await expect(invitesSentLabel).toBeVisible();
  18 |         const numberLocator = invitesSentLabel.locator('..').locator('.text-3xl');
  19 |
  20 |         // Wait for it to not be empty (if it's a number)
  21 |         await expect(numberLocator).not.toBeEmpty();
  22 |         let initialInvitesSentText = await numberLocator.innerText();
  23 |
  24 |         // Sometimes innerText might be initially empty before hydrated, retry a bit
  25 |         for (let i = 0; i < 5; i++) {
  26 |             if (initialInvitesSentText.trim() !== '') break;
  27 |             await page.waitForTimeout(500);
  28 |             initialInvitesSentText = await numberLocator.innerText();
  29 |         }
  30 |
  31 |         const initialInvitesSent = parseInt(initialInvitesSentText, 10);
  32 |         expect(isNaN(initialInvitesSent)).toBe(false);
  33 |
  34 |         // Next, go to the Team page and generate an invite to trigger a change
  35 |         await page.goto('/team');
  36 |         const generateInviteBtn = page.locator('button:has-text("Invite to Cloud Team")');
  37 |         await expect(generateInviteBtn).toBeVisible();
  38 |         await generateInviteBtn.click();
  39 |
  40 |         // The copy link input should become visible
  41 |         const copyInput = page.locator('#cloud-bridge-invite-link');
  42 |         await expect(copyInput).toBeVisible();
  43 |
  44 |         // Go back to the dashboard and ensure the widget still renders
  45 |         await page.goto('/dashboard');
  46 |         await expect(widgetHeader).toBeVisible();
  47 |
  48 |         // Check if the number of invites sent has incremented
  49 |         const newInvitesSentLabel = page.locator('text=Invites Sent');
  50 |         await expect(newInvitesSentLabel).toBeVisible();
  51 |         const newNumberLocator = newInvitesSentLabel.locator('..').locator('.text-3xl');
  52 |
  53 |         // Use web-first assertion to wait for the incremented value
  54 |         const expectedCount = (initialInvitesSent + 1).toString();
  55 |         await expect(newNumberLocator).toHaveText(expectedCount, { timeout: 10000 });
  56 |     });
  57 | });
  58 |
```