# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: help.spec.ts >> Help Center >> should use backend search for filtering articles
- Location: src/e2e/help.spec.ts:35:9

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: page.waitForResponse: Test timeout of 30000ms exceeded.
```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e3]:
    - heading "Help Center" [level=1] [ref=e4]
    - textbox "Search for help articles and videos..." [active] [ref=e6]: My Store
    - generic [ref=e7]:
      - img [ref=e8]
      - paragraph [ref=e10]:
        - text: No results found matching
        - generic [ref=e11]: "\"My Store\""
      - paragraph [ref=e12]: Try adjusting your search terms or ask our AI assistant for help.
  - button "Help" [ref=e15]:
    - img [ref=e16]
  - button "Open help chat" [ref=e19]:
    - generic [ref=e20]: ✨
    - generic [ref=e21]: Ask anything
  - button "Open Next.js Dev Tools" [ref=e27] [cursor=pointer]:
    - img [ref=e28]
  - alert [ref=e31]
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Help Center', () => {
  4  |     test('renders help center and navigates to an article', async ({ page }) => {
  5  |         await page.goto('/help');
  6  |
  7  |         // Verify Help Center title
  8  |         await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
  9  |
  10 |         // Verify that categories are rendered (Getting Started, My Store, Payments)
  11 |         await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible();
  12 |         await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();
  13 |         await expect(page.locator('h2', { hasText: 'Payments' })).toBeVisible();
  14 |
  15 |         // Search for an article
  16 |         const searchInput = page.getByPlaceholder('Search for help articles and videos...');
  17 |         await searchInput.fill('Getting Started');
  18 |
  19 |         // Click on the article
  20 |         const articleLink = page.locator('a[href="/help/getting-started-1"]');
  21 |         await expect(articleLink).toBeVisible();
  22 |         await articleLink.click();
  23 |
  24 |         // Wait for navigation and API load
  25 |         await page.waitForURL('/help/getting-started-1');
  26 |
  27 |         // Verify article content
  28 |         await expect(page.locator('h1', { hasText: 'Getting Started with Your Store' })).toBeVisible();
  29 |         await expect(page.locator('p', { hasText: 'Welcome to OneHumanCorp!' })).toBeVisible();
  30 |
  31 |         await page.goto('/help');
  32 |         await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
  33 |     });
  34 |
  35 |     test('should use backend search for filtering articles', async ({ page }) => {
  36 |         await page.goto('/help');
  37 |
  38 |         // Verify Help Center title
  39 |         await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
  40 |
  41 |         // Search for an article that matches My Store
  42 |         const searchInput = page.getByPlaceholder('Search for help articles and videos...');
  43 |
  44 |         // Use Promise.all to wait for the request to the search endpoint
  45 |         const [response] = await Promise.all([
> 46 |             page.waitForResponse(response =>
     |                  ^ Error: page.waitForResponse: Test timeout of 30000ms exceeded.
  47 |                 response.url().includes('/api/help/search') && response.status() === 200
  48 |             ),
  49 |             searchInput.fill('My Store')
  50 |         ]);
  51 |
  52 |         // Wait for UI to update
  53 |         const articleLink = page.locator('a[href="/help/my-store"]');
  54 |         await expect(articleLink).toBeVisible();
  55 |     });
  56 |
  57 |     test('should open help chat and send a message', async ({ page }) => {
  58 |         await page.goto('/help');
  59 |
  60 |         // Find and click the floating Ask anything button
  61 |         const chatButton = page.locator('button[aria-label="Open help chat"]');
  62 |         await expect(chatButton).toBeVisible();
  63 |         await chatButton.click({ force: true });
  64 |
  65 |         // Wait for the chat to open and be visible
  66 |         const chatHeader = page.locator('#ai-chat-header');
  67 |         await expect(chatHeader).toBeVisible();
  68 |
  69 |         // Check if the chat input is present
  70 |         const chatInput = page.locator('input[placeholder="Ask me anything..."]');
  71 |         await expect(chatInput).toBeVisible();
  72 |
  73 |         // Type a message and send it
  74 |         const testMessage = 'How do I add a product?';
  75 |         await chatInput.fill(testMessage);
  76 |         const sendButton = page.locator('button[aria-label="Send message"]');
  77 |         await expect(sendButton).toBeVisible();
  78 |         await sendButton.click();
  79 |
  80 |         // Assert that the message appears in the chat
  81 |         const sentMessage = page.locator('div', { hasText: testMessage }).last();
  82 |         await expect(sentMessage).toBeVisible();
  83 |
  84 |         // Close the chat
  85 |         const closeButton = page.locator('button[aria-label="Close help chat"]');
  86 |         await closeButton.click();
  87 |         await expect(chatHeader).not.toBeVisible();
  88 |     });
  89 | });
  90 |
```