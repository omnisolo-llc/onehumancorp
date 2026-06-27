# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: pos-inventory-sync-optimistic.spec.ts >> POS Inventory Sync - Optimistic UI >> POS terminal immediately updates stock UI on cash sale before API returns
- Location: src/e2e/pos-inventory-sync-optimistic.spec.ts:179:7

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
  108 |     await page.getByRole('button', { name: '2', exact: true }).click();
  109 |     await page.getByRole('button', { name: '3', exact: true }).click();
  110 |     await page.getByRole('button', { name: '4', exact: true }).click();
  111 |
  112 |     await page.waitForTimeout(500);
  113 |     await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
  114 |     await page.waitForTimeout(500);
  115 |
  116 |     // Ensure product catalog is populated
  117 |     await expect(page.getByText('Vegan Celebration Cake').first()).toBeVisible({ timeout: 10000 });
  118 |
  119 |     const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' }).first();
  120 |     const descriptionText = await productButton.innerText();
  121 |
  122 |     const stockMatch = descriptionText.match(/Stock: (\d+)/);
  123 |     expect(stockMatch).toBeTruthy();
  124 |
  125 |     if (stockMatch) {
  126 |       // Simulate going offline
  127 |       await page.context().setOffline(true);
  128 |
  129 |       // Select the product
  130 |       await productButton.click();
  131 |
  132 |       // Click the "Charge" button to queue the mutation offline
  133 |       const collectBtn = page.locator('button', { hasText: /Collect Payment/i });
  134 |       await expect(collectBtn).toBeVisible();
  135 |       await collectBtn.click();
  136 |
  137 |       // Go back online
  138 |       await page.context().setOffline(false);
  139 |
  140 |       // Force a conflict by directly hitting the endpoint with a large quantity
  141 |       // so it triggers the conflict generation workflow in the backend
  142 |       const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
  143 |       const transactionId = `tx-conflict-${Date.now()}`;
  144 |       const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;
  145 |
  146 |       const res = await page.request.post('/api/v1/sync/offline', {
  147 |         headers: {
  148 |           'x-spiffe-id': spiffeId
  149 |         },
  150 |         data: {
  151 |           mutations: [
  152 |             {
  153 |               transaction_id: transactionId,
  154 |               product_id: 'e2e-product-cake', // Assumed to exist and have < 100 stock
  155 |               quantity_deducted: 100,
  156 |               amount: 5000,
  157 |               currency: 'usd',
  158 |             }
  159 |           ]
  160 |         }
  161 |       });
  162 |
  163 |       expect(res.ok()).toBeTruthy();
  164 |
  165 |       // Wait for async workers (pos_sync_worker, pos_conflict_worker, operations_agent)
  166 |       await page.waitForTimeout(5000);
  167 |     }
  168 |
  169 |     // Navigate to Action Center
  170 |     await page.goto('/dashboard');
  171 |
  172 |     // We expect the Triage task to show up from Operations Agent
  173 |     // Fallback LLM text or "oversold the item" should be visible
  174 |     if (stockMatch) {
  175 |       await expect(page.getByText(/We oversold the item/i).first()).toBeVisible({ timeout: 10000 });
  176 |     }
  177 |   });
  178 |
  179 |   test('POS terminal immediately updates stock UI on cash sale before API returns', async ({ page }) => {
  180 |     // 1. Log in to get token
  181 |     await page.goto('/login');
> 182 |     await page.getByPlaceholder('Email address').fill('admin@ohc.local');
      |                                                  ^ Error: locator.fill: Test timeout of 30000ms exceeded.
  183 |     await page.getByPlaceholder('Password').fill('admin');
  184 |     await page.getByRole('button', { name: 'Sign In' }).click();
  185 |     await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });
  186 |
  187 |     const response = await page.request.post('/api/v1/auth/login', {
  188 |         data: {
  189 |             email: 'admin@ohc.local',
  190 |             password: 'admin'
  191 |         }
  192 |     });
  193 |     expect(response.ok()).toBeTruthy();
  194 |     const { token } = await response.json();
  195 |
  196 |     // 2. Create the "Falafel" product
  197 |     const createProductRes = await page.request.post('/api/v1/catalog/products', {
  198 |         headers: { Authorization: `Bearer ${token}` },
  199 |         data: {
  200 |             title: 'Falafel',
  201 |             inventory_count: 50,
  202 |             price_cents: 800
  203 |         }
  204 |     });
  205 |     expect(createProductRes.ok()).toBeTruthy();
  206 |
  207 |     // Navigate to POS terminal
  208 |     await page.goto('/pos.html');
  209 |     await page.evaluate(() => { localStorage.setItem("tenant_id", "default"); });
  210 |
  211 |     // Login with PIN 1234
  212 |     await page.getByRole('button', { name: '1', exact: true }).click();
  213 |     await page.getByRole('button', { name: '2', exact: true }).click();
  214 |     await page.getByRole('button', { name: '3', exact: true }).click();
  215 |     await page.getByRole('button', { name: '4', exact: true }).click();
  216 |
  217 |     await page.waitForTimeout(500);
  218 |     await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
  219 |     await page.waitForTimeout(500);
  220 |
  221 |     // Wait for the product catalog to be populated
  222 |     await expect(page.getByText('Falafel').first()).toBeVisible({ timeout: 10000 });
  223 |
  224 |     // Extract current stock from the text
  225 |     const productButton = page.locator('button', { hasText: 'Falafel' }).first();
  226 |     const descriptionText = await productButton.innerText();
  227 |
  228 |     const stockMatch = descriptionText.match(/Stock: (\d+)/);
  229 |     expect(stockMatch).toBeTruthy();
  230 |
  231 |     if (stockMatch) {
  232 |       const initialStock = parseInt(stockMatch[1], 10);
  233 |
  234 |       // Select the product
  235 |       await productButton.click();
  236 |
  237 |       // Go offline
  238 |       await page.context().setOffline(true);
  239 |
  240 |       // Verify offline mode indicator
  241 |       await expect(page.getByText('Offline - Changes will sync later')).toBeVisible({ timeout: 5000 });
  242 |
  243 |       // Click the "Charge" button to open cart drawer
  244 |       const collectBtn = page.locator('button', { hasText: /Charge/i }).first();
  245 |       await expect(collectBtn).toBeVisible();
  246 |       await collectBtn.click();
  247 |
  248 |       await page.waitForTimeout(500);
  249 |
  250 |       // Click the "Record Cash Sale" button to queue the mutation offline
  251 |       const cashBtn = page.locator('button', { hasText: /Record Cash Sale/i });
  252 |       await expect(cashBtn).toBeVisible();
  253 |       await cashBtn.click();
  254 |
  255 |       // Immediately verify the stock decreased by 1 without waiting for API
  256 |       // Since it's optimistic, it should happen instantly.
  257 |       await page.waitForTimeout(500);
  258 |       const updatedButtonText = await productButton.innerText();
  259 |       const newMatch = updatedButtonText.match(/Stock:\s*(\d+)/);
  260 |       if (newMatch) {
  261 |           const newStock = parseInt(newMatch[1], 10);
  262 |           expect(newStock).toBe(initialStock - 1);
  263 |       }
  264 |
  265 |       // Restore network
  266 |       await page.context().setOffline(false);
  267 |
  268 |       // Wait to verify it syncs back
  269 |       await page.waitForTimeout(2000);
  270 |     }
  271 |   });
  272 | });
  273 |
```