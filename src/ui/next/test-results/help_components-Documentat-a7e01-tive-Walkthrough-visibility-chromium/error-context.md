# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: help_components.spec.ts >> Documentation Components >> Interactive Walkthrough visibility
- Location: src/e2e/help_components.spec.ts:41:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('#help-widget-container')
Expected: visible
Error: strict mode violation: locator('#help-widget-container') resolved to 2 elements:
    1) <div class="relative " id="help-widget-container">…</div> aka locator('#help-widget-container').first()
    2) <div id="help-widget-container" class="fixed bottom-24 right-4 sm:right-6 w-[calc(100vw-32px)] sm:w-[380px] h-[75vh] sm:h-[550px] max-h-[700px] bg-white/70 backdrop-blur-[30px] saturate-200 rounded-3xl shadow-2xl flex flex-col overflow-hidden z-[90] border border-white/50 transition-all font-inter">…</div> aka getByText('HelpAsk AIVideosNewHelp')

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('#help-widget-container')

```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e2]:
    - complementary [ref=e3]:
      - generic [ref=e4]:
        - generic [ref=e5]: O
        - generic [ref=e6]:
          - generic [ref=e7]: OHC Network
          - generic [ref=e8]: Application
      - navigation "Primary" [ref=e9]:
        - link "Dashboard" [ref=e10] [cursor=pointer]:
          - /url: /dashboard
          - img [ref=e12]
          - generic [ref=e17]: Dashboard
        - link "Setup" [ref=e18] [cursor=pointer]:
          - /url: /onboarding
          - img [ref=e20]
          - generic [ref=e21]: Setup
        - link "Orders" [ref=e22] [cursor=pointer]:
          - /url: /orders
          - img [ref=e24]
          - generic [ref=e26]: Orders
        - link "Inbox" [ref=e27] [cursor=pointer]:
          - /url: /inbox
          - img [ref=e29]
          - generic [ref=e32]: Inbox
        - link "Inventory" [ref=e33] [cursor=pointer]:
          - /url: /inventory
          - img [ref=e35]
          - generic [ref=e39]: Inventory
        - link "Kairos" [ref=e41] [cursor=pointer]:
          - /url: /kairos
          - img [ref=e43]
          - generic [ref=e45]: Kairos
        - link "Agents" [ref=e46] [cursor=pointer]:
          - /url: /agents
          - img [ref=e48]
          - generic [ref=e53]: Agents
        - link "Analytics" [ref=e54] [cursor=pointer]:
          - /url: /business-analytics
          - img [ref=e56]
          - generic [ref=e57]: Analytics
        - link "Settings" [ref=e58] [cursor=pointer]:
          - /url: /settings
          - img [ref=e60]
          - generic [ref=e63]: Settings
      - generic [ref=e64]: System
      - navigation "System" [ref=e65]:
        - link "Calendar" [ref=e66] [cursor=pointer]:
          - /url: /calendar
          - img [ref=e68]
          - generic [ref=e70]: Calendar
        - link "Integrations" [ref=e71] [cursor=pointer]:
          - /url: /integrations
          - img [ref=e73]
          - generic [ref=e76]: Integrations
        - link "Cost" [ref=e77] [cursor=pointer]:
          - /url: /cost-dashboard
          - img [ref=e79]
          - generic [ref=e81]: Cost
        - link "Diagnostics" [ref=e82] [cursor=pointer]:
          - /url: /diagnostics
          - img [ref=e84]
          - generic [ref=e86]: Diagnostics
    - generic [ref=e87]:
      - banner [ref=e88]:
        - generic [ref=e89]:
          - generic [ref=e90]: "Site: default"
          - heading "Dashboard" [level=1] [ref=e91]
          - paragraph [ref=e92]: Network-style command center for database-backed store operations.
        - generic [ref=e93]:
          - generic [ref=e94]:
            - generic [ref=e95]:
              - generic [ref=e97]: API
              - generic [ref=e98]: Degraded
            - generic [ref=e99]:
              - generic [ref=e101]: Orders
              - generic [ref=e102]: "0"
            - generic [ref=e103]:
              - generic [ref=e105]: Stock
              - generic [ref=e106]: "0"
            - generic [ref=e107]:
              - generic [ref=e109]: Growth
              - generic [ref=e110]: Active
          - link "New Product" [ref=e111] [cursor=pointer]:
            - /url: /products/new
            - img [ref=e112]
            - text: New Product
      - main [ref=e113]:
        - generic [ref=e114]:
          - heading "Welcome back, Human." [level=2] [ref=e115]
          - paragraph [ref=e116]: Your agents are working on your behalf.
        - generic [ref=e117]:
          - button "Start Tour" [ref=e118] [cursor=pointer]
          - button "Launch Site" [ref=e119] [cursor=pointer]
          - button "Migrate Existing Store" [ref=e120] [cursor=pointer]
          - generic [ref=e121]: One or more database-backed UI endpoints failed
        - generic [ref=e123]:
          - heading "Refer & Earn $50" [level=3] [ref=e124]
          - paragraph [ref=e125]: Invite a friend to OHC and you both get rewarded!
          - generic [ref=e126]:
            - button "Copy Link" [ref=e127] [cursor=pointer]
            - link "WhatsApp" [ref=e128] [cursor=pointer]:
              - /url: https://wa.me/?text=Start%20your%20business%20on%20OHC!%20Use%20my%20link%3A%20https%3A%2F%2Fohc.store%2Fjoin%3Fref%3Ddefault%26source%3Ddashboard
        - generic [ref=e129]:
          - generic [ref=e130]:
            - generic [ref=e131]:
              - heading "2024 Store Wrapped" [level=2] [ref=e132]
              - generic [ref=e133]: A shareable snapshot of your strongest store moments.
            - generic [ref=e134]: Viral Loop
          - generic [ref=e135]:
            - paragraph [ref=e136]: Turn your sales, products, and milestones into a referral-friendly recap.
            - link "View Your Wrapped 🎁" [ref=e137] [cursor=pointer]:
              - /url: /wrapped
        - main [ref=e138]:
          - region "Unified Agent Feed" [ref=e139]:
            - generic [ref=e140]:
              - button "Proposals (0)" [ref=e141] [cursor=pointer]
              - button "Activity Feed" [ref=e142] [cursor=pointer]
            - generic [ref=e144]: Loading Agent Proposals...
          - generic [ref=e145]:
            - generic [ref=e147]:
              - generic [ref=e148]:
                - generic [ref=e149]: 🎉
                - generic [ref=e150]:
                  - heading "Milestone Unlocked!" [level=3] [ref=e151]
                  - paragraph [ref=e152]: You completed your first 5 orders!
              - button "Share & Claim Reward" [ref=e153] [cursor=pointer]
            - generic [ref=e154]:
              - generic [ref=e155]:
                - heading "Business Analytics" [level=2] [ref=e156]
                - paragraph [ref=e157]: "Loaded from `/api/ui/dashboard/metrics`."
              - link "Business Analytics" [ref=e158] [cursor=pointer]:
                - /url: /business-analytics
            - generic [ref=e159]:
              - generic [ref=e160]:
                - generic [ref=e161]:
                  - generic [ref=e163]: Total Sales
                  - generic [ref=e164]: $0.00
                  - generic [ref=e165]: All recorded orders
                - generic [ref=e166]:
                  - generic [ref=e167]: Customers
                  - generic [ref=e168]: "0"
                  - generic [ref=e169]: Database customer records
                - generic [ref=e170]:
                  - generic [ref=e171]: Pending Orders
                  - generic [ref=e172]: "0"
                  - generic [ref=e173]: Open fulfillment workload
                - generic [ref=e174]:
                  - generic [ref=e175]: Low Stock
                  - generic [ref=e176]: "0"
                  - generic [ref=e177]: Materials below threshold
              - generic [ref=e178]:
                - heading "✨ Advanced AI Insights" [level=4] [ref=e180]:
                  - generic [ref=e181]: ✨
                  - text: Advanced AI Insights
                - paragraph [ref=e182]: Unlock predictive analytics and AI-driven growth recommendations.
                - button "Upgrade to Pro" [ref=e183] [cursor=pointer]
          - generic [ref=e184]:
            - generic [ref=e185]:
              - generic [ref=e186]:
                - generic [ref=e187]:
                  - generic [ref=e188]: Operations Map
                  - generic [ref=e189]: Live database state across the store workflow.
                - link "Open Orders" [ref=e190] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e192]:
                - generic [ref=e193]:
                  - generic [ref=e194]: Orders
                  - generic [ref=e195]: "0"
                  - generic [ref=e196]: Rows returned
                - generic [ref=e197]:
                  - generic [ref=e198]: Inbox
                  - generic [ref=e199]: "0"
                  - generic [ref=e200]: Messages returned
                - generic [ref=e201]:
                  - generic [ref=e202]: Vendors
                  - generic [ref=e203]: "0"
                  - generic [ref=e204]: Supply partners
            - generic [ref=e205]:
              - generic [ref=e206]:
                - generic [ref=e207]: Action Required
                - link "Inventory" [ref=e208] [cursor=pointer]:
                  - /url: /inventory
              - generic [ref=e210]: No database-backed actions are currently open.
          - generic [ref=e211]:
            - generic [ref=e212]:
              - generic [ref=e213]:
                - generic [ref=e214]: Recent Orders
                - link "View All" [ref=e215] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e216]: No order rows found for this tenant.
            - generic [ref=e217]:
              - generic [ref=e218]:
                - generic [ref=e219]: Inbox Activity
                - link "Open Inbox" [ref=e220] [cursor=pointer]:
                  - /url: /inbox
              - generic [ref=e222]: No inbox message rows found for this tenant.
          - generic [ref=e223]:
            - generic [ref=e225]:
              - heading "Growth & Virality" [level=2] [ref=e226]
              - paragraph [ref=e227]: Unlock new customers and track milestones.
            - generic [ref=e228]:
              - link "🤝 Earn $50 Referrals Invite other business owners to OHC and earn premium credits." [ref=e229] [cursor=pointer]:
                - /url: /referrals
                - generic [ref=e230]:
                  - generic [ref=e231]: 🤝
                  - generic [ref=e232]: Earn $50
                - heading "Referrals" [level=3] [ref=e233]
                - paragraph [ref=e234]: Invite other business owners to OHC and earn premium credits.
              - link "🏆 Share Milestones Milestones 🏆 Track and share your business achievements with your audience." [ref=e235] [cursor=pointer]:
                - /url: /milestones
                - generic [ref=e236]:
                  - generic [ref=e237]: 🏆
                  - generic [ref=e238]: Share
                - heading "Milestones" [level=3] [ref=e239]
                - generic [ref=e240]: Milestones 🏆
                - paragraph [ref=e241]: Track and share your business achievements with your audience.
              - link "🎴 Cards Social Share Cards Generate Share Cards to promote your brand on social media." [ref=e242] [cursor=pointer]:
                - /url: /share-cards
                - generic [ref=e243]:
                  - generic [ref=e244]: 🎴
                  - generic [ref=e245]: Cards
                - heading "Social Share Cards" [level=3] [ref=e246]
                - paragraph [ref=e247]: Generate Share Cards to promote your brand on social media.
              - link "🌐 Widget Storefront Widget Embed a mini storefront on your blog or website to boost sales." [ref=e248] [cursor=pointer]:
                - /url: /storefront-widget
                - generic [ref=e249]:
                  - generic [ref=e250]: 🌐
                  - generic [ref=e251]: Widget
                - heading "Storefront Widget" [level=3] [ref=e252]
                - paragraph [ref=e253]: Embed a mini storefront on your blog or website to boost sales.
              - link "📦 Recurring Subscriptions & Fulfillments Manage recurring products, subscribers, and shipping batches." [ref=e254] [cursor=pointer]:
                - /url: /subscriptions
                - generic [ref=e255]:
                  - generic [ref=e256]: 📦
                  - generic [ref=e257]: Recurring
                - heading "Subscriptions & Fulfillments" [level=3] [ref=e258]
                - paragraph [ref=e259]: Manage recurring products, subscribers, and shipping batches.
              - link "🚀 Proof Social Proof Nudge Show visitors that others are buying to increase conversions." [ref=e260] [cursor=pointer]:
                - /url: /social-proof-nudge
                - generic [ref=e261]:
                  - generic [ref=e262]: 🚀
                  - generic [ref=e263]: Proof
                - heading "Social Proof Nudge" [level=3] [ref=e264]
                - paragraph [ref=e265]: Show visitors that others are buying to increase conversions.
              - link "🔗 Bio Create Link-in-Bio Page Publish a lightweight social profile page for your storefront and offers." [ref=e266] [cursor=pointer]:
                - /url: /link-in-bio-generator
                - generic [ref=e267]:
                  - generic [ref=e268]: 🔗
                  - generic [ref=e269]: Bio
                - heading "Create Link-in-Bio Page" [level=3] [ref=e270]
                - paragraph [ref=e271]: Publish a lightweight social profile page for your storefront and offers.
  - generic:
    - button "Help" [active] [ref=e274] [cursor=pointer]:
      - img [ref=e275]
    - generic [ref=e277]:
      - generic [ref=e278]:
        - button "Help" [pressed] [ref=e279] [cursor=pointer]
        - button "Ask AI" [ref=e280] [cursor=pointer]
        - button "Videos" [ref=e281] [cursor=pointer]
        - button "New" [ref=e282] [cursor=pointer]
      - generic [ref=e284]:
        - heading "Help Center" [level=3] [ref=e285]
        - textbox "Search for help..." [ref=e286]
        - heading "Interactive Tours" [level=3] [ref=e287]
        - generic [ref=e288]:
          - 'button "Tour: Set up your store" [ref=e290] [cursor=pointer]':
            - generic [ref=e291]: "Tour: Set up your store"
          - 'button "Tour: Accept your first payment" [ref=e292] [cursor=pointer]':
            - generic [ref=e293]: "Tour: Accept your first payment"
          - 'button "Tour: Activate your AI Support Agent" [ref=e294] [cursor=pointer]':
            - generic [ref=e295]: "Tour: Activate your AI Support Agent"
          - 'button "Tour: Virtual Meeting Room & UltraPlan" [ref=e296] [cursor=pointer]':
            - generic [ref=e297]: "Tour: Virtual Meeting Room & UltraPlan"
          - 'button "Tour: KAIROS AI OS Orchestration" [ref=e298] [cursor=pointer]':
            - generic [ref=e299]: "Tour: KAIROS AI OS Orchestration"
  - button "Open help chat" [ref=e301] [cursor=pointer]:
    - generic [ref=e302]: ✨
    - generic: Ask anything
  - generic: Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.
  - alert [ref=e303]
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Documentation Components', () => {
  4  |
  5  |   test('Help Center Search functionality', async ({ page }) => {
  6  |     await page.goto('/help');
  7  |     await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
  8  |
  9  |     const searchInput = page.getByPlaceholder('Search for help articles and videos...');
  10 |     await searchInput.fill('Getting Started');
  11 |
  12 |     // Should show the article
  13 |     await expect(page.locator('h2', { hasText: 'Getting Started' }).first()).toBeVisible();
  14 |
  15 |     // Fill with something not found
  16 |     await searchInput.fill('XYZNonExistent');
  17 |     await expect(page.locator('text=No results found matching "XYZNonExistent"')).toBeVisible();
  18 |   });
  19 |
  20 |   test('Tooltip rendering on hover', async ({ page }) => {
  21 |     // Go to api docs where tooltip exists
  22 |     await page.goto('/api-docs');
  23 |     const tooltipTarget = page.locator('span', { hasText: 'Advanced:' });
  24 |
  25 |     await expect(tooltipTarget).toBeVisible();
  26 |     await tooltipTarget.hover();
  27 |
  28 |     // Wait for tooltip to appear
  29 |     await expect(page.locator('text=Direct API access is only for custom integrations.')).toBeVisible();
  30 |   });
  31 |
  32 |   test('Video tutorials page rendering', async ({ page }) => {
  33 |     await page.goto('/help/videos');
  34 |     await expect(page.locator('h1', { hasText: 'Video Guides' })).toBeVisible();
  35 |
  36 |     // Wait for page to load.
  37 |     // We expect some videos to render or at least the back button
  38 |     await expect(page.locator('text=Back to Help Center')).toBeVisible();
  39 |   });
  40 |
  41 |   test('Interactive Walkthrough visibility', async ({ page }) => {
  42 |     // Enable walkthrough via query param test bypass
  43 |     await page.goto('/dashboard?test_walkthrough=true');
  44 |     // Wait for the WalkthroughTarget to be present
  45 |     // It's attached to 'help-widget-container' in layout
  46 |     // In order to test the walkthrough, we open the Help widget and click a walkthrough tour button.
  47 |     const helpBtn = page.getByRole('button', { name: 'Help', exact: true }).first();
  48 |     await expect(helpBtn).toBeVisible();
  49 |     await helpBtn.click();
  50 |
  51 |     // Now it expects exactly one Help widget container, ignoring hidden ones
  52 |     // Check if help widget is open
> 53 |     await expect(page.locator('#help-widget-container')).toBeVisible();
     |                                                          ^ Error: expect(locator).toBeVisible() failed
  54 |
  55 |     // Click the "Tour: Set up your store" button
  56 |     const tourBtn = page.getByRole('button', { name: 'Tour: Set up your store' });
  57 |     await expect(tourBtn).toBeVisible();
  58 |     await tourBtn.click();
  59 |
  60 |     // Check if the first step speech bubble dialog is visible
  61 |     const bubble = page.getByRole('dialog', { name: 'Quick Guide walkthrough step' });
  62 |     await expect(bubble).toBeVisible();
  63 |
  64 |     // Finish the tour
  65 |     const finishBtn = page.getByRole('button', { name: 'Next' }).or(page.getByRole('button', { name: 'Finish' }));
  66 |     await finishBtn.click();
  67 |   });
  68 |
  69 |   test('Help Chat widget visibility', async ({ page }) => {
  70 |     await page.goto('/dashboard?test_chat=true');
  71 |
  72 |     // The floating button should be visible
  73 |     const chatBtn = page.getByRole('button', { name: 'Open help chat' });
  74 |     await expect(chatBtn).toBeVisible();
  75 |
  76 |     // Open chat
  77 |     await chatBtn.click();
  78 |
  79 |     // Check if chat window is visible
  80 |     await expect(page.locator('h3', { hasText: 'Ask AI Help' })).toBeVisible();
  81 |
  82 |     // Check input
  83 |     const input = page.getByPlaceholder('Ask me anything...');
  84 |     await expect(input).toBeVisible();
  85 |
  86 |     // Close chat
  87 |     const closeBtn = page.getByRole('button', { name: 'Close help chat' });
  88 |     await closeBtn.click();
  89 |
  90 |     // Verify it's closed
  91 |     await expect(page.locator('h3', { hasText: 'Ask AI Help' })).not.toBeVisible();
  92 |   });
  93 |
  94 | });
  95 |
```