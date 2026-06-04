# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: videos.spec.ts >> In-App Video Tutorials >> renders videos tab, fetches videos, and opens/closes the modal player
- Location: src/e2e/videos.spec.ts:4:9

# Error details

```
Error: locator.click: Element is outside of the viewport
Call log:
  - waiting for locator('#help-widget-container button').first()
    - locator resolved to <button aria-label="Help" class="w-14 h-14 bg-blue-600/90 backdrop-blur-[20px] saturate-200 text-white rounded-full shadow-[0_8px_32px_rgba(37,99,235,0.3)] flex items-center justify-center hover:bg-blue-700/90 active:scale-95 transition-all min-h-[44px] min-w-[44px]">…</button>
  - attempting click action
    - scrolling into view if needed
    - done scrolling

```

# Page snapshot

```yaml
- generic [active] [ref=e1]:
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
        - link "Kairos" [ref=e40] [cursor=pointer]:
          - /url: /kairos
          - img [ref=e42]
          - generic [ref=e44]: Kairos
        - link "Agents" [ref=e45] [cursor=pointer]:
          - /url: /agents
          - img [ref=e47]
          - generic [ref=e52]: Agents
        - link "Analytics" [ref=e53] [cursor=pointer]:
          - /url: /business-analytics
          - img [ref=e55]
          - generic [ref=e56]: Analytics
        - link "Settings" [ref=e57] [cursor=pointer]:
          - /url: /settings
          - img [ref=e59]
          - generic [ref=e62]: Settings
      - generic [ref=e63]: System
      - navigation "System" [ref=e64]:
        - link "Calendar" [ref=e65] [cursor=pointer]:
          - /url: /calendar
          - img [ref=e67]
          - generic [ref=e69]: Calendar
        - link "Integrations" [ref=e70] [cursor=pointer]:
          - /url: /integrations
          - img [ref=e72]
          - generic [ref=e75]: Integrations
        - link "Cost" [ref=e76] [cursor=pointer]:
          - /url: /cost-dashboard
          - img [ref=e78]
          - generic [ref=e80]: Cost
        - link "Diagnostics" [ref=e81] [cursor=pointer]:
          - /url: /diagnostics
          - img [ref=e83]
          - generic [ref=e85]: Diagnostics
    - generic [ref=e86]:
      - banner [ref=e87]:
        - generic [ref=e88]:
          - generic [ref=e89]: "Site: default"
          - heading "Dashboard" [level=1] [ref=e90]
          - paragraph [ref=e91]: Network-style command center for database-backed store operations.
        - generic [ref=e92]:
          - generic [ref=e93]:
            - generic [ref=e94]:
              - generic [ref=e96]: API
              - generic [ref=e97]: Degraded
            - generic [ref=e98]:
              - generic [ref=e100]: Orders
              - generic [ref=e101]: "0"
            - generic [ref=e102]:
              - generic [ref=e104]: Stock
              - generic [ref=e105]: "0"
            - generic [ref=e106]:
              - generic [ref=e108]: Growth
              - generic [ref=e109]: Active
          - link "New Product" [ref=e110] [cursor=pointer]:
            - /url: /products/new
            - img [ref=e111]
            - text: New Product
      - main [ref=e112]:
        - generic [ref=e113]:
          - heading "Welcome back, Human." [level=2] [ref=e114]
          - paragraph [ref=e115]: Your AI assistants are working on your behalf.
        - generic [ref=e116]:
          - button "Start Tour" [ref=e117]
          - generic [ref=e118]: One or more database-backed UI endpoints failed
        - generic [ref=e120]:
          - heading "Refer & Earn $50" [level=3] [ref=e121]
          - paragraph [ref=e122]: Invite a friend to OHC and you both get rewarded!
          - generic [ref=e123]:
            - button "Copy Link" [ref=e124]
            - link "WhatsApp" [ref=e125] [cursor=pointer]:
              - /url: https://wa.me/?text=Start%20your%20business%20on%20OHC!%20Use%20my%20link%3A%20https%3A%2F%2Fohc.store%2Fjoin%3Fref%3Ddefault%26source%3Ddashboard
        - main [ref=e126]:
          - generic [ref=e127]: Failed to load agent feed
          - generic [ref=e128]:
            - generic [ref=e130]:
              - heading "Business Analytics" [level=2] [ref=e131]
              - paragraph [ref=e132]: "Loaded from `/api/ui/dashboard/metrics`."
            - generic [ref=e133]:
              - generic [ref=e134]:
                - generic [ref=e136]: Total Sales
                - generic [ref=e137]: $0.00
                - generic [ref=e138]: All recorded orders
              - generic [ref=e139]:
                - generic [ref=e140]: Customers
                - generic [ref=e141]: "0"
                - generic [ref=e142]: Database customer records
              - generic [ref=e143]:
                - generic [ref=e144]: Pending Orders
                - generic [ref=e145]: "0"
                - generic [ref=e146]: Open fulfillment workload
              - generic [ref=e147]:
                - generic [ref=e148]: Low Stock
                - generic [ref=e149]: "0"
                - generic [ref=e150]: Materials below threshold
          - generic [ref=e151]:
            - generic [ref=e152]:
              - generic [ref=e153]:
                - generic [ref=e154]:
                  - generic [ref=e155]: Operations Map
                  - generic [ref=e156]: Live database state across the store workflow.
                - link "Open Orders" [ref=e157] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e159]:
                - generic [ref=e160]:
                  - generic [ref=e161]: Orders
                  - generic [ref=e162]: "0"
                  - generic [ref=e163]: Rows returned
                - generic [ref=e164]:
                  - generic [ref=e165]: Inbox
                  - generic [ref=e166]: "0"
                  - generic [ref=e167]: Messages returned
                - generic [ref=e168]:
                  - generic [ref=e169]: Vendors
                  - generic [ref=e170]: "0"
                  - generic [ref=e171]: Supply partners
            - generic [ref=e172]:
              - generic [ref=e173]:
                - generic [ref=e174]: Action Required
                - link "Inventory" [ref=e175] [cursor=pointer]:
                  - /url: /inventory
              - generic [ref=e177]: No database-backed actions are currently open.
          - generic [ref=e178]:
            - generic [ref=e179]:
              - generic [ref=e180]:
                - generic [ref=e181]: Recent Orders
                - link "View All" [ref=e182] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e183]: No order rows found for this tenant.
            - generic [ref=e184]:
              - generic [ref=e185]:
                - generic [ref=e186]: Inbox Activity
                - link "Open Inbox" [ref=e187] [cursor=pointer]:
                  - /url: /inbox
              - generic [ref=e189]: No inbox message rows found for this tenant.
          - generic [ref=e190]:
            - generic [ref=e192]:
              - heading "Growth & Virality" [level=2] [ref=e193]
              - paragraph [ref=e194]: Unlock new customers and track milestones.
            - generic [ref=e195]:
              - link "🤝 Earn $50 Referrals Invite other business owners to OHC and earn premium credits." [ref=e196] [cursor=pointer]:
                - /url: /referrals
                - generic [ref=e197]:
                  - generic [ref=e198]: 🤝
                  - generic [ref=e199]: Earn $50
                - heading "Referrals" [level=3] [ref=e200]
                - paragraph [ref=e201]: Invite other business owners to OHC and earn premium credits.
              - link "🏆 Share Milestones Track and share your business achievements with your audience." [ref=e202] [cursor=pointer]:
                - /url: /milestones
                - generic [ref=e203]:
                  - generic [ref=e204]: 🏆
                  - generic [ref=e205]: Share
                - heading "Milestones" [level=3] [ref=e206]
                - paragraph [ref=e207]: Track and share your business achievements with your audience.
              - link "🎴 Cards Social Share Cards Generate Share Cards to promote your brand on social media." [ref=e208] [cursor=pointer]:
                - /url: /share-cards
                - generic [ref=e209]:
                  - generic [ref=e210]: 🎴
                  - generic [ref=e211]: Cards
                - heading "Social Share Cards" [level=3] [ref=e212]
                - paragraph [ref=e213]: Generate Share Cards to promote your brand on social media.
  - button "Help" [ref=e216]:
    - img [ref=e217]
  - button "Open help chat" [ref=e220]:
    - generic [ref=e221]: ✨
    - generic [ref=e222]: Ask anything
  - button "Open Next.js Dev Tools" [ref=e228] [cursor=pointer]:
    - img [ref=e229]
  - alert [ref=e232]
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('In-App Video Tutorials', () => {
  4  |     test('renders videos tab, fetches videos, and opens/closes the modal player', async ({ page }) => {
  5  |         // Go to a page where HelpWidget is available (layout.tsx ensures it's on pages like dashboard)
  6  |         await page.goto('/dashboard'); // Use the dashboard or any public page where layout applies
  7  |
  8  |         // The help widget should be present.
  9  |         const helpButton = page.locator('#help-widget-container button').first();
  10 |         await expect(helpButton).toBeVisible();
  11 |
  12 |         // Click the help widget floating button to open the menu
  13 |         await page.waitForTimeout(2000);
> 14 |         await helpButton.click({ force: true, timeout: 60000 });
     |                          ^ Error: locator.click: Element is outside of the viewport
  15 |
  16 |         // Check that tabs are visible
  17 |         const videosTabButton = page.locator('button', { hasText: 'Videos' });
  18 |         await expect(videosTabButton).toBeVisible();
  19 |
  20 |         // Click the Videos tab
  21 |         await videosTabButton.click();
  22 |
  23 |         // Wait for the videos to be fetched and rendered
  24 |         // The API returns 10 videos. We'll wait for at least one to show up.
  25 |         // The videos are rendered with titles, like 'How to set up your first store easily'
  26 |         const firstVideoTitle = page.locator('p', { hasText: 'How to set up your first store easily' });
  27 |         await expect(firstVideoTitle).toBeVisible();
  28 |
  29 |         // Verify some other videos are present
  30 |         await expect(page.locator('p', { hasText: 'Adding staff to your account' })).toBeVisible();
  31 |
  32 |         // Click on the first video to open the modal player
  33 |         // The video container is a div parent of the title
  34 |         const videoContainer = firstVideoTitle.locator('..').locator('..'); // go up to the container
  35 |         await videoContainer.click();
  36 |
  37 |         // Verify the modal player opens
  38 |         const modalContainer = page.locator('div.fixed.z-\\[100\\]');
  39 |         await expect(modalContainer).toBeVisible();
  40 |
  41 |         // Verify the modal has the correct mobile constraints (max-w-[375px])
  42 |         await expect(modalContainer.locator('div.max-w-\\[375px\\]')).toBeVisible();
  43 |
  44 |         // Verify the video title is shown in the modal header
  45 |         await expect(modalContainer.locator('h3', { hasText: 'How to set up your first store easily' })).toBeVisible();
  46 |
  47 |         // Click the close button
  48 |         const closeButton = modalContainer.locator('button[aria-label="Close video"]');
  49 |         await closeButton.click();
  50 |
  51 |         // Verify the modal player closes
  52 |         await expect(modalContainer).not.toBeVisible();
  53 |     });
  54 | });
  55 |
```