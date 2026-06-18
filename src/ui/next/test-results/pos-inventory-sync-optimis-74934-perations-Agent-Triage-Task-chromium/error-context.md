# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: pos-inventory-sync-optimistic.spec.ts >> POS Inventory Sync - Optimistic UI >> Offline sync conflict generates Operations Agent Triage Task
- Location: src/e2e/pos-inventory-sync-optimistic.spec.ts:45:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByRole('heading', { name: 'Manager' })
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for getByRole('heading', { name: 'Manager' })

```

```yaml
- complementary:
  - text: O OHC Network Application
  - navigation "Primary":
    - link "Dashboard":
      - /url: /dashboard
    - link "Assistant":
      - /url: /assistant
    - link "Setup":
      - /url: /onboarding
    - link "Triage":
      - /url: /triage
    - link "Orders":
      - /url: /orders
    - link "Inbox":
      - /url: /inbox
    - link "Inventory":
      - /url: /inventory
    - link "Kairos":
      - /url: /kairos
    - link "AI Departments":
      - /url: /agents
    - link "Analytics":
      - /url: /business-analytics
    - link "Campaigns":
      - /url: /dashboard/campaigns
    - link "Settings":
      - /url: /settings
    - link "AI Usage":
      - /url: /ai-usage-paywall
    - link "Changelog":
      - /url: /changelog
  - text: System
  - navigation "System":
    - link "Calendar":
      - /url: /calendar
    - link "LangGraph":
      - /url: /langgraph
    - link "Integrations":
      - /url: /integrations
    - link "Cost":
      - /url: /cost-dashboard
    - link "Diagnostics":
      - /url: /diagnostics
- banner:
  - text: "Site: default"
  - heading "Pos" [level=1]
  - paragraph: Use this workspace from the dashboard navigation.
  - link "Help Center":
    - /url: /help
    - text: "?"
- main:
  - heading "Quick Actions" [level=3]
  - heading "Product Catalog" [level=3]
  - paragraph: No products found in catalog
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- alert
```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('POS Inventory Sync - Optimistic UI', () => {
  4   |   test('POS terminal immediately updates stock UI on charge before API returns', async ({ page }) => {
  5   |     // Navigate to POS terminal
  6   |     await page.goto('/pos/terminal');
  7   |
  8   |     // Wait for the pin screen to be visible
  9   |     await expect(page.getByText('Terminal Locked')).toBeVisible();
  10  |
  11  |     // Login with PIN 1234
  12  |     await page.getByRole('button', { name: '1', exact: true }).click();
  13  |     await page.getByRole('button', { name: '2', exact: true }).click();
  14  |     await page.getByRole('button', { name: '3', exact: true }).click();
  15  |     await page.getByRole('button', { name: '4', exact: true }).click();
  16  |
  17  |     // Wait for the dashboard to load
  18  |     await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });
  19  |
  20  |     // Wait for the product catalog to be populated
  21  |     await expect(page.getByText('Vegan Celebration Cake')).toBeVisible();
  22  |
  23  |     // Extract current stock from the text
  24  |     const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' });
  25  |     const descriptionText = await productButton.innerText();
  26  |
  27  |     const stockMatch = descriptionText.match(/Stock: (\d+)/);
  28  |     expect(stockMatch).toBeTruthy();
  29  |
  30  |     if (stockMatch) {
  31  |       const initialStock = parseInt(stockMatch[1], 10);
  32  |
  33  |       // Select the product
  34  |       await productButton.click();
  35  |
  36  |       // Click the "Charge" button
  37  |       await page.getByRole('button', { name: /Collect Payment \$/ }).click();
  38  |
  39  |       // Immediately verify the stock decreased by 1 without waiting for API
  40  |       // Since it's optimistic, it should happen instantly.
  41  |       await expect(productButton).toContainText(`Stock: ${initialStock - 1}`);
  42  |     }
  43  |   });
  44  |
  45  |   test('Offline sync conflict generates Operations Agent Triage Task', async ({ page }) => {
  46  |     // Navigate to POS terminal to login
  47  |     await page.goto('/pos/terminal');
  48  |     await expect(page.getByText('Terminal Locked')).toBeVisible();
  49  |
  50  |     // Login with PIN 1234
  51  |     await page.getByRole('button', { name: '1', exact: true }).click();
  52  |     await page.getByRole('button', { name: '2', exact: true }).click();
  53  |     await page.getByRole('button', { name: '3', exact: true }).click();
  54  |     await page.getByRole('button', { name: '4', exact: true }).click();
  55  |
> 56  |     await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });
      |                                                                  ^ Error: expect(locator).toBeVisible() failed
  57  |
  58  |     // Ensure product catalog is populated
  59  |     await expect(page.getByText('Vegan Celebration Cake')).toBeVisible({ timeout: 5000 });
  60  |
  61  |     const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' });
  62  |     const descriptionText = await productButton.innerText();
  63  |
  64  |     const stockMatch = descriptionText.match(/Stock: (\d+)/);
  65  |     expect(stockMatch).toBeTruthy();
  66  |
  67  |     if (stockMatch) {
  68  |       // Simulate going offline
  69  |       await page.context().setOffline(true);
  70  |
  71  |       // Select the product
  72  |       await productButton.click();
  73  |
  74  |       // Click the "Charge" button to queue the mutation offline
  75  |       await page.getByRole('button', { name: /Collect Payment \$/ }).click();
  76  |
  77  |       // Go back online
  78  |       await page.context().setOffline(false);
  79  |
  80  |       // Force a conflict by directly hitting the endpoint with a large quantity
  81  |       // so it triggers the conflict generation workflow in the backend
  82  |       const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
  83  |       const transactionId = `tx-conflict-${Date.now()}`;
  84  |       const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;
  85  |
  86  |       const res = await page.request.post('/api/v1/sync/offline', {
  87  |         headers: {
  88  |           'x-spiffe-id': spiffeId
  89  |         },
  90  |         data: {
  91  |           mutations: [
  92  |             {
  93  |               transaction_id: transactionId,
  94  |               product_id: 'e2e-product-cake', // Assumed to exist and have < 100 stock
  95  |               quantity_deducted: 100,
  96  |               amount: 5000,
  97  |               currency: 'usd',
  98  |             }
  99  |           ]
  100 |         }
  101 |       });
  102 |
  103 |       expect(res.ok()).toBeTruthy();
  104 |
  105 |       // Wait for async workers (pos_sync_worker, pos_conflict_worker, operations_agent)
  106 |       await page.waitForTimeout(5000);
  107 |     }
  108 |
  109 |     // Navigate to Action Center
  110 |     await page.goto('/action-center');
  111 |
  112 |     // We expect the Triage task to show up from Operations Agent
  113 |     // Fallback LLM text or "oversold the item" should be visible
  114 |     if (stockMatch) {
  115 |       await expect(page.getByText(/We oversold the item/i).first()).toBeVisible({ timeout: 10000 });
  116 |     }
  117 |   });
  118 | });
  119 |
```