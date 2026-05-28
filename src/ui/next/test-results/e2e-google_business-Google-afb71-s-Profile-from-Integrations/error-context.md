# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: e2e/google_business.spec.ts >> Google Business Profile Integration Flow >> 1. Connect Google Business Profile from Integrations
- Location: e2e/google_business.spec.ts:5:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('div:has-text("Google Business Profile")').last()
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('div:has-text("Google Business Profile")').last()

```

```yaml
- text: Premium
- heading "Tool Integrations" [level=1]
- paragraph: Supercharge your workflow by connecting your favorite tools.
- text: 🧩
- main:
  - button "All"
  - button "Marketing"
  - button "Operations"
  - button "Finance"
  - text: 📅 disconnected
  - heading "Calendly" [level=3]
  - paragraph: Automated Booking widget for your store.
  - button "Connect"
  - text: 💬 disconnected
  - heading "Manychat" [level=3]
  - paragraph: Unified social media inbox for Instagram, Facebook, and WhatsApp.
  - button "Connect"
  - text: 📱 disconnected
  - heading "Ayrshare" [level=3]
  - paragraph: Unified API for posting and retrieving messages across social networks.
  - button "Connect"
  - text: 📅 disconnected
  - heading "Cal.com" [level=3]
  - paragraph: Zero-Config Booking & Calendar Sync.
  - button "Connect"
  - text: 📨 disconnected
  - heading "Listmonk" [level=3]
  - paragraph: Embedded, No-Jargon Email Campaigns.
  - button "Connect"
  - text: 🌎 disconnected
  - heading "Mercado Pago" [level=3]
  - paragraph: Accept credit cards and local payment methods in Latin America.
  - button "Connect"
  - text: 📦 disconnected
  - heading "EasyPost" [level=3]
  - paragraph: Painless Shipping Labels & Tracking.
  - button "Connect"
  - text: 🔔 disconnected
  - heading "Twilio" [level=3]
  - paragraph: Reliable SMS alerts for new orders and customer notifications.
  - button "Connect"
  - text: 📹 disconnected
  - heading "Jitsi Meet" [level=3]
  - paragraph: Zero-Setup Online Lessons and video conferencing.
  - button "Connect"
- button "Help":
  - img
- button "✨ Ask anything"
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Google Business Profile Integration Flow', () => {
  4  |
  5  |   test('1. Connect Google Business Profile from Integrations', async ({ page }) => {
  6  |     // Navigate to Integrations page
  7  |     await page.goto('http://localhost:3000/integrations');
  8  |
  9  |     // Ensure the Google Business Profile card is visible
  10 |     const card = page.locator('div:has-text("Google Business Profile")').last();
> 11 |     await expect(card).toBeVisible();
     |                        ^ Error: expect(locator).toBeVisible() failed
  12 |
  13 |     // Verify it is initially disconnected
  14 |     await expect(card.locator('span:has-text("disconnected")')).toBeVisible();
  15 |
  16 |     // Mock window.alert to not block the test
  17 |     page.on('dialog', dialog => dialog.accept());
  18 |
  19 |     // Click Connect
  20 |     await card.locator('button:has-text("Connect")').click();
  21 |
  22 |     // After connecting, it should redirect to the Inbox page
  23 |     await page.waitForURL('**/inbox');
  24 |
  25 |     // Verify we are on the Inbox page
  26 |     await expect(page.locator('h1:has-text("Customer Inbox")')).toBeVisible();
  27 |   });
  28 |
  29 |   test('2. Verify Google Business messages appear in the Customer Inbox with AI drafts', async ({ page }) => {
  30 |     await page.goto('http://localhost:3000/inbox');
  31 |
  32 |     // Check for Google Business message presence
  33 |     const googleMsg = page.locator('div:has-text("Google Search User")').first();
  34 |     await expect(googleMsg).toBeVisible();
  35 |
  36 |     // Verify message content
  37 |     await expect(googleMsg.locator('text="Are you open on Sundays?"')).toBeVisible();
  38 |
  39 |     // Verify AI drafted reply
  40 |     const aiDraft = page.locator('text="Hi there! Yes, we are open on Sundays from 10:00 AM to 4:00 PM. We hope to see you soon!"');
  41 |     // Using string matching since it might be split across DOM nodes
  42 |     await expect(page.locator('body')).toContainText('Hi there! Yes, we are open on Sundays from 10:00 AM to 4:00 PM. We hope to see you soon!');
  43 |   });
  44 |
  45 |   test('3. Edit and send a reply to a Google Business message', async ({ page }) => {
  46 |     await page.goto('http://localhost:3000/inbox');
  47 |
  48 |     // There are hidden buttons in the DOM, so select the specific visible one inside the message container
  49 |     const sendBtn = page.locator('.bg-\\[\\#805ad5\\]').filter({ hasText: 'Send' }).last();
  50 |     await sendBtn.click({ force: true });
  51 |
  52 |     // Verify the sent message appears from 'Me'
  53 |     await expect(page.locator('body')).toContainText('Hi there! Yes, we are open on Sundays');
  54 |
  55 |     // Check that 'Me' is displayed as the sender in the list
  56 |     await expect(page.locator('span:has-text("Me")').last()).toBeVisible();
  57 |   });
  58 |
  59 |   test('4. Navigate to Reputation page and verify Google Reviews', async ({ page }) => {
  60 |     await page.goto('http://localhost:3000/reputation');
  61 |
  62 |     await expect(page.locator('h1:has-text("Reputation Management")')).toBeVisible();
  63 |
  64 |     // Verify Sarah's review is pending
  65 |     const review1 = page.locator('div:has-text("Sarah Jenkins")').first();
  66 |     await expect(review1).toBeVisible();
  67 |     await expect(review1.locator('span:has-text("pending")')).toBeVisible();
  68 |     await expect(review1.locator('p:has-text("Absolutely wonderful service! Carlos was very professional and fixed my car in no time.")')).toBeVisible();
  69 |
  70 |     // Verify AI suggested reply is present
  71 |     await expect(review1.locator('text="AI Suggested Reply"')).toBeVisible();
  72 |   });
  73 |
  74 |   test('5. Approve and send an AI-drafted response to a Google Review', async ({ page }) => {
  75 |     await page.goto('http://localhost:3000/reputation');
  76 |
  77 |     // Find the pending review
  78 |     const review1 = page.locator('div:has-text("Sarah Jenkins")').first();
  79 |     await expect(review1.locator('span:has-text("pending")')).toBeVisible();
  80 |
  81 |     // Click Approve & Publish
  82 |     const approveBtn = review1.locator('button:has-text("Approve & Publish")');
  83 |     await approveBtn.click();
  84 |
  85 |     // Verify status changes to replied
  86 |     await expect(review1.locator('span:has-text("replied")').first()).toBeVisible();
  87 |
  88 |     // Verify the published reply section appears
  89 |     await expect(review1.locator('span:has-text("Your Reply")').first()).toBeVisible();
  90 |     await expect(review1.locator('span:has-text("Published")').first()).toBeVisible();
  91 |   });
  92 |
  93 | });
```