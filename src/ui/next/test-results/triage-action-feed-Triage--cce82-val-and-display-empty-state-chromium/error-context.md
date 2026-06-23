# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: triage-action-feed.spec.ts >> Triage Action Feed UI >> should render triage feed properly, show distinct card types, allow approval, and display empty state
- Location: src/e2e/triage-action-feed.spec.ts:6:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByTestId('triage-feed-empty')
Expected: visible
Timeout: 15000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for getByTestId('triage-feed-empty')

```

```yaml
- button "⚠️ AI Actions Left"
- complementary:
  - text: O
  - navigation "Primary":
    - link:
      - /url: /dashboard
    - link:
      - /url: /assistant
    - link:
      - /url: /onboarding
    - link:
      - /url: /triage
    - link:
      - /url: /orders
    - link:
      - /url: /inbox
    - link:
      - /url: /inventory
    - link:
      - /url: /kairos
    - link:
      - /url: /agents
    - link:
      - /url: /business-analytics
    - link:
      - /url: /dashboard/campaigns
    - link:
      - /url: /lead-magnet-generator
    - link:
      - /url: /settings
    - link:
      - /url: /ai-usage-paywall
    - link:
      - /url: /changelog
  - navigation "System":
    - link:
      - /url: /calendar
    - link:
      - /url: /langgraph
    - link:
      - /url: /integrations
    - link:
      - /url: /cost-dashboard
    - link:
      - /url: /diagnostics
    - link:
      - /url: /help
    - link:
      - /url: /api-docs
- banner:
  - text: "Site: default"
  - heading "Dashboard" [level=1]
  - paragraph: Network-style command center for your store operations.
  - text: API Online Orders 0 Stock 0 Growth Active
  - link "Campaigns":
    - /url: /dashboard/campaigns
  - link "New Product":
    - /url: /products/new
  - link "Help Center":
    - /url: /help
    - text: "?"
- main:
  - heading "Welcome back, Human." [level=2]
  - paragraph: Your agents are working on your behalf.
  - text: ⏱️ Weekly Insight
  - heading "You saved 0 hours this week" [level=3]
  - paragraph: "Your AI agents handled 0 customer inquiries (Auto-Replied: 0), scheduled 0 appointments, and recovered 0 abandoned carts."
  - button "Share to get 7 Days Pro":
    - img
    - text: Share to get 7 Days Pro
  - paragraph: Unlock premium tools by sharing your success.
  - button "+"
  - button "Start Tour"
  - button "Launch Site"
  - button "Migrate Existing Store"
  - region "Unified Agent Feed":
    - button "Proposals (0)"
    - button "Activity Feed"
  - heading "Viral Loop Performance" [level=2]
  - text: Track your referral program and team growth. Active Loop
  - link "View Referral Details":
    - /url: /referrals
  - img "shopping cart": 🛒
  - text: Cart Recovery Agent Active
  - link "Configure Agent →":
    - /url: /cart-recovery
  - heading "💰 Viral Growth" [level=3]
  - link "⚡ Powered by OHC":
    - /url: /onboarding?ref=default&source=footer_widget
    - text: ⚡ Powered by OHC
    - img
  - button "Report Incident"
  - heading "2024 Store Wrapped" [level=2]
  - text: A shareable snapshot of your strongest store moments. Viral Loop
  - paragraph: Turn your sales, products, and milestones into a referral-friendly recap.
  - link "View Your Wrapped 🎁":
    - /url: /wrapped
  - main:
    - link "Assistant Tasks Open the dashboard task workspace for conversations, artifacts, and assistant actions. →":
      - /url: /assistant
      - heading "Assistant Tasks" [level=3]
      - paragraph: Open the dashboard task workspace for conversations, artifacts, and assistant actions.
      - text: →
    - text: 🚀
    - heading "Grow Business" [level=3]
    - paragraph: Deploy a zero-config edge-cached storefront for instant consumer discovery.
    - link "Review Storefront":
      - /url: /edge-storefront-setup
    - text: 📣
    - heading "The Promoter Agent" [level=3]
    - paragraph: Let OHC's AI write engaging social media posts to drive traffic to your storefront.
    - link "Create Posts":
      - /url: /promoter
    - text: 🎉
    - heading "Milestone Unlocked!" [level=3]
    - paragraph: You completed your first 5 orders!
    - button "Share & Claim Reward"
    - heading "Business Analytics" [level=2]
    - paragraph: Live performance, orders, and inbox activity.
    - link "Business Analytics":
      - /url: /business-analytics
    - text: Total Sales $0.00 Loading your data... Customers 0 Customer records Pending Orders 0 Open fulfillment workload Low Stock 0 Materials below threshold Operations Map Live overview of your store workflow.
    - link "Open Orders":
      - /url: /orders
    - text: Orders 0 Rows returned Inbox 0 Messages returned Vendors 0 Supply partners Action Required
    - link "Inventory":
      - /url: /inventory
    - text: Recent Orders
    - link "View All":
      - /url: /orders
    - text: Loading your orders... Inbox Activity
    - link "Open Inbox":
      - /url: /inbox
    - text: Loading your messages...
    - heading "Growth & Virality" [level=2]
    - paragraph: Unlock new customers and track milestones.
    - link "↗ Orchestrate Campaign Orchestration Plan, generate, review, and launch customer campaigns from live dashboard data.":
      - /url: /feed
      - text: ↗ Orchestrate
      - heading "Campaign Orchestration" [level=3]
      - paragraph: Plan, generate, review, and launch customer campaigns from live dashboard data.
    - link "📈 ROI Pro Plan ROI Calculator See how much extra revenue you could generate by unlocking the Pro Plan.":
      - /url: /upgrade-roi
      - text: 📈 ROI
      - heading "Pro Plan ROI Calculator" [level=3]
      - paragraph: See how much extra revenue you could generate by unlocking the Pro Plan.
    - link "⚡ Growth Zero-Click Builder Generate a business in 30 seconds to show friends how fast OHC is.":
      - /url: /zero-click-builder
      - text: ⚡ Growth
      - heading "Zero-Click Builder" [level=3]
      - paragraph: Generate a business in 30 seconds to show friends how fast OHC is.
    - link "🤝 Earn $50 Referrals Invite other business owners to OHC and earn premium credits.":
      - /url: /referrals
      - text: 🤝 Earn $50
      - heading "Referrals" [level=3]
      - paragraph: Invite other business owners to OHC and earn premium credits.
    - link "🏆 Viral Affiliate Badge Builder Create an embeddable badge to grow your affiliate network.":
      - /url: /affiliate-badge-builder
      - text: 🏆 Viral
      - heading "Affiliate Badge Builder" [level=3]
      - paragraph: Create an embeddable badge to grow your affiliate network.
    - link "💰 Finance Finance & Invoicing Manage cash flow, invoices, and automated payment follow-ups.":
      - /url: /finance
      - text: 💰 Finance
      - heading "Finance & Invoicing" [level=3]
      - paragraph: Manage cash flow, invoices, and automated payment follow-ups.
    - link "🧾 Billing AI Invoice Generator Generate professional, shareable invoices that bring new customers to OHC.":
      - /url: /invoice-generator
      - text: 🧾 Billing
      - heading "AI Invoice Generator" [level=3]
      - paragraph: Generate professional, shareable invoices that bring new customers to OHC.
    - link "📝 Sales AI Proposal Generator Create smart, shareable proposals with an interactive approval flow to win clients faster.":
      - /url: /proposal-generator
      - text: 📝 Sales
      - heading "AI Proposal Generator" [level=3]
      - paragraph: Create smart, shareable proposals with an interactive approval flow to win clients faster.
    - link "Share Milestones Track and share your business achievements with your audience.":
      - /url: /milestone-alerts
      - text: Share
      - heading "Milestones" [level=3]
      - paragraph: Track and share your business achievements with your audience.
    - link "🤝 Loyalty Customer Loyalty Set up a 'Give X, Get Y' referral program and generate campaigns.":
      - /url: /loyalty-program
      - text: 🤝 Loyalty
      - heading "Customer Loyalty" [level=3]
      - paragraph: Set up a 'Give X, Get Y' referral program and generate campaigns.
    - link "💸 Referrals Customer Referral Program Launch a Give $10, Get $10 program to turn your customers into advocates.":
      - /url: /customer-referral-program
      - text: 💸 Referrals
      - heading "Customer Referral Program" [level=3]
      - paragraph: Launch a Give $10, Get $10 program to turn your customers into advocates.
    - link "🎴 Cards Social Share Cards Generate Share Cards to promote your brand on social media.":
      - /url: /share-cards
      - text: 🎴 Cards
      - heading "Social Share Cards" [level=3]
      - paragraph: Generate Share Cards to promote your brand on social media.
    - link "🌐 Widget Storefront Widget Embed a mini storefront on your blog or website to boost sales.":
      - /url: /storefront-widget
      - text: 🌐 Widget
      - heading "Storefront Widget" [level=3]
      - paragraph: Embed a mini storefront on your blog or website to boost sales.
    - link "🔌 Widget Interactive Embed Build custom intake, booking, or quote widgets for your site.":
      - /url: /embed-builder
      - text: 🔌 Widget
      - heading "Interactive Embed" [level=3]
      - paragraph: Build custom intake, booking, or quote widgets for your site.
    - link "📦 Recurring Subscriptions & Fulfillments Manage recurring products, subscribers, and shipping batches.":
      - /url: /subscriptions
      - text: 📦 Recurring
      - heading "Subscriptions & Fulfillments" [level=3]
      - paragraph: Manage recurring products, subscribers, and shipping batches.
    - link "🚀 Proof Social Proof Nudge Show visitors that others are buying to increase conversions.":
      - /url: /social-proof-nudge
      - text: 🚀 Proof
      - heading "Social Proof Nudge" [level=3]
      - paragraph: Show visitors that others are buying to increase conversions.
    - link "📋 Leads Work-Intake Widget Embed a smart lead capture form with a viral loop directly on your site.":
      - /url: /work-intake-widget
      - text: 📋 Leads
      - heading "Work-Intake Widget" [level=3]
      - paragraph: Embed a smart lead capture form with a viral loop directly on your site.
    - link "🔗 Bio Create Link-in-Bio Page Publish a lightweight social profile page for your storefront and offers.":
      - /url: /link-in-bio-generator
      - text: 🔗 Bio
      - heading "Create Link-in-Bio Page" [level=3]
      - paragraph: Publish a lightweight social profile page for your storefront and offers.
    - link "💬 Social WhatsApp Link Generator Create shareable WhatsApp links to start conversations instantly.":
      - /url: /whatsapp-link-generator
      - text: 💬 Social
      - heading "WhatsApp Link Generator" [level=3]
      - paragraph: Create shareable WhatsApp links to start conversations instantly.
    - link "📅 Schedule Calendar View upcoming appointments and sync with your AI operations assistant.":
      - /url: /calendar
      - text: 📅 Schedule
      - heading "Calendar" [level=3]
      - paragraph: View upcoming appointments and sync with your AI operations assistant.
    - link "🎁 Viral Viral Giveaway Generator Launch a viral sweepstakes to capture emails and drive social shares.":
      - /url: /giveaway
      - text: 🎁 Viral
      - heading "Viral Giveaway Generator" [level=3]
      - paragraph: Launch a viral sweepstakes to capture emails and drive social shares.
    - link "🔓 Growth Share-to-Unlock Generator Require customers to share your page on social media to reveal a discount code.":
      - /url: /share-to-unlock-generator
      - text: 🔓 Growth
      - heading "Share-to-Unlock Generator" [level=3]
      - paragraph: Require customers to share your page on social media to reveal a discount code.
    - link "🧮 Growth Interactive Quote Widget Build a pricing calculator to embed on your site to capture leads and drive referrals.":
      - /url: /interactive-quote-generator
      - text: 🧮 Growth
      - heading "Interactive Quote Widget" [level=3]
      - paragraph: Build a pricing calculator to embed on your site to capture leads and drive referrals.
    - link "💌 Retain Customer Win-back Re-engage inactive customers with AI-generated email campaigns.":
      - /url: /win-back
      - text: 💌 Retain
      - heading "Customer Win-back" [level=3]
      - paragraph: Re-engage inactive customers with AI-generated email campaigns.
    - link "⭐️ Reviews Automated Reviews Generate highly-converting, personalized review request emails.":
      - /url: /review-campaigns
      - text: ⭐️ Reviews
      - heading "Automated Reviews" [level=3]
      - paragraph: Generate highly-converting, personalized review request emails.
    - link "✨ Promo Seasonal Promo Generator Create AI campaigns and promo codes for special occasions instantly.":
      - /url: /seasonal-promo
      - text: ✨ Promo
      - heading "Seasonal Promo Generator" [level=3]
      - paragraph: Create AI campaigns and promo codes for special occasions instantly.
    - link "🛒 Recover Cart Recovery Recover abandoned carts with personalized AI follow-ups.":
      - /url: /cart-recovery
      - text: 🛒 Recover
      - heading "Cart Recovery" [level=3]
      - paragraph: Recover abandoned carts with personalized AI follow-ups.
    - link "⚡ Urgency Flash Sale Generator Create high-converting flash sale countdown widgets.":
      - /url: /flash-sale-generator
      - text: ⚡ Urgency
      - heading "Flash Sale Generator" [level=3]
      - paragraph: Create high-converting flash sale countdown widgets.
    - link "⏳ Virality Pre-Order Waitlist Launch an omnichannel pre-order engine with tiered waitlist capabilities.":
      - /url: /pre-order-widget
      - text: ⏳ Virality
      - heading "Pre-Order Waitlist" [level=3]
      - paragraph: Launch an omnichannel pre-order engine with tiered waitlist capabilities.
    - link "🎯 Leads Discount Code Generator Create discount code widgets for your customers.":
      - /url: /discount-code-generator
      - text: 🎯 Leads
      - heading "Discount Code Generator" [level=3]
      - paragraph: Create discount code widgets for your customers.
    - link "🔗 Growth Link in Bio Generator One link to rule them all. Drive social traffic to your store.":
      - /url: /link-in-bio-generator
      - text: 🔗 Growth
      - heading "Link in Bio Generator" [level=3]
      - paragraph: One link to rule them all. Drive social traffic to your store.
    - link "🎡 Gamification Spin to Win Generator Create interactive discount wheels to capture emails.":
      - /url: /spin-to-win-generator
      - text: 🎡 Gamification
      - heading "Spin to Win Generator" [level=3]
      - paragraph: Create interactive discount wheels to capture emails.
    - link "🎁 Extension Interactive Trial Extension Share your setup on X to instantly unlock 7 extra days of Pro.":
      - /url: /trial-extension
      - text: 🎁 Extension
      - heading "Interactive Trial Extension" [level=3]
      - paragraph: Share your setup on X to instantly unlock 7 extra days of Pro.
    - link "📍 Operations Field Ops Route Offline-first mobile route management for field service workers.":
      - /url: /field-ops/jobs
      - text: 📍 Operations
      - heading "Field Ops Route" [level=3]
      - paragraph: Offline-first mobile route management for field service workers.
    - link "💳 Billing My Plan Manage your subscription, usage, and billing.":
      - /url: /plan
      - text: 💳 Billing
      - heading "My Plan" [level=3]
      - paragraph: Manage your subscription, usage, and billing.
    - link "⚙️ Config Settings Manage your account and preferences.":
      - /url: /settings
      - text: ⚙️ Config
      - heading "Settings" [level=3]
      - paragraph: Manage your account and preferences.
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
```

# Test source

```ts
  1  | import { expect, test } from '@playwright/test';
  2  |
  3  | test.describe('Triage Action Feed UI', () => {
  4  |   test.use({ viewport: { width: 375, height: 812 } });
  5  |
  6  |   test('should render triage feed properly, show distinct card types, allow approval, and display empty state', async ({ page }) => {
  7  |     test.setTimeout(180000);
  8  |
  9  |     await page.goto('/login');
  10 |     await page.getByPlaceholder('Email or Username').fill('test@example.com');
  11 |     await page.getByPlaceholder('Password').fill('password123');
  12 |     await page.getByRole('button', { name: 'Log In' }).click();
  13 |     await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
  14 |
  15 |     const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');
  16 |
  17 |     const seedData = [
  18 |       { source: 'Instagram DM Message', priority: 'high', context: 'Message: Customer asked about vegan cakes.', action_type: 'Draft Reply', action_payload: 'Yes, we have vegan options.' },
  19 |       { source: 'Website Booking Request', priority: 'medium', context: 'Booking: Sarah wants an estimate tomorrow at 2PM.', action_type: 'Accept Booking', action_payload: 'Booked for 2PM tomorrow.' },
  20 |       { source: 'Inventory Alert', priority: 'urgent', context: 'Alert: Low stock on flour.', action_type: 'Reorder', action_payload: 'Order 50lbs of flour.' }
  21 |     ];
  22 |
  23 |     for (const data of seedData) {
  24 |       await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
  25 |         data: {
  26 |           customer_id: 'cust_test',
  27 |           source: data.source,
  28 |           priority: data.priority,
  29 |           context: data.context,
  30 |           action_type: data.action_type,
  31 |           action_payload: data.action_payload
  32 |         }
  33 |       });
  34 |     }
  35 |
  36 |     await page.goto('/triage');
  37 |     await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });
  38 |
  39 |     // Let's use a simpler locator for list items
  40 |     let listItems = page.locator('div[data-testid^="triage-card-"]');
  41 |
  42 |     await page.waitForTimeout(2000);
  43 |     await expect(page.locator('body')).toContainText(/Work Triage|All caught up/);
  44 |
  45 |     let count = await listItems.count();
  46 |
  47 |     while (count > 0) {
  48 |       // Removed list item click
  49 |
  50 |       const approveBtn = listItems.nth(0).getByTestId('approve-btn');
  51 |       const dismissBtn = listItems.nth(0).getByTestId('dismiss-btn');
  52 |
  53 |       if (await approveBtn.isVisible()) {
  54 |         await approveBtn.click();
  55 |       } else if (await dismissBtn.isVisible()) {
  56 |         await dismissBtn.click();
  57 |       } else {
  58 |         break;
  59 |       }
  60 |
  61 |       await page.waitForTimeout(1000);
  62 |
  63 |       count = await listItems.count();
  64 |     }
  65 |
  66 |     await page.waitForTimeout(3000);
  67 |
  68 |     // Look for the specific empty state element
  69 |     const emptyState = page.getByTestId('triage-feed-empty');
  70 |     if (await emptyState.isVisible()) {
  71 |         await expect(emptyState).toContainText(/caught up/i);
  72 |     }
  73 |
  74 |     await page.goto('/dashboard');
  75 |     await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });
  76 |
  77 |     const proposalsTab = page.locator('button', { hasText: /Proposals/ });
  78 |     if (await proposalsTab.isVisible()) {
  79 |        await proposalsTab.click();
  80 |     }
  81 |
  82 |     const triageFeedEmpty = page.getByTestId('triage-feed-empty');
> 83 |     await expect(triageFeedEmpty).toBeVisible({ timeout: 15000 });
     |                                   ^ Error: expect(locator).toBeVisible() failed
  84 |     await expect(triageFeedEmpty).toContainText(/caught up/i);
  85 |   });
  86 | });
  87 |
```