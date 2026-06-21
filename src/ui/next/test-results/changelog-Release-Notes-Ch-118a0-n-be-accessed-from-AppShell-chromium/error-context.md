# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: changelog.spec.ts >> Release Notes & Changelog >> renders Changelog page with screenshots and can be accessed from AppShell
- Location: src/e2e/changelog.spec.ts:4:9

# Error details

```
Error: expect(page).toHaveURL(expected) failed

Expected pattern: /\/changelog/
Received string:  "http://localhost:3000/dashboard"
Timeout: 5000ms

Call log:
  - Expect "toHaveURL" with timeout 5000ms
    13 × unexpected value "http://localhost:3000/dashboard"

```

```yaml
- button "⚠️ AI Actions Left"
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
    - link "Lead Magnets":
      - /url: /lead-magnet-generator
    - link "Settings":
      - /url: /settings
    - link "AI Usage":
      - /url: /ai-usage-paywall
    - link "What's New":
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
  - heading "Dashboard" [level=1]
  - paragraph: Network-style command center for database-backed store operations.
  - text: API Degraded Orders 0 Stock 0 Growth Active
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
  - img
  - text: AI Actions Limit
  - heading "Approaching Free Tier Limit" [level=2]
  - paragraph: Your AI agents are working hard. You have used 0 of your 100 free actions this month.
  - text: 0 / 100
  - link "Upgrade to Pro (Unlimited)":
    - /url: /pricing
  - text: or
  - button "Share on X to get +50 Actions"
  - heading "Invite & Earn" [level=2]
  - paragraph: Invite a fellow business owner to OHC. They get 1 month free, you get $50 credit.
  - button "Get My Invite Link"
  - text: ⏱️ Weekly Insight
  - heading "You saved 0 hours this week" [level=3]
  - paragraph: "Your AI agents handled 0 customer inquiries (Auto-Replied: 0), scheduled 0 appointments, and recovered 0 abandoned carts."
  - button "Share to get 7 Days Pro":
    - img
    - text: Share to get 7 Days Pro
  - paragraph: Unlock premium tools by sharing your success.
  - button "+"
  - button "Voice Command Assistant": 🎤
  - img
  - text: Pro Feature
  - heading "Unlock Advanced AI Analytics" [level=2]
  - paragraph: See exactly which products are driving revenue, automate cross-selling, and get daily AI strategy briefings.
  - link "Upgrade to Pro ($79/mo)":
    - /url: /pricing
  - text: or
  - button "Refer a Friend to Unlock for 7 Days"
  - button "Start Tour"
  - button "Launch Site"
  - button "Migrate Existing Store"
  - text: Unified UI feed endpoint failed
  - region "Unified Agent Feed":
    - button "Proposals (0)"
    - button "Activity Feed"
  - heading "🏆 100th Order Delivered! 🎉" [level=2]
  - text: You're growing fast. Share your success to unlock $50 in OHC credits. $50 Credit
  - paragraph: "\"I just hit my 100th order using OHC to run my business! 🚀 Check them out and get $50 off your first month: https://ohc.app/onboarding?ref=default&source=milestone_share ⚡ Powered by OHC\""
  - button "🔗 Copy & Share to Unlock"
  - link "🐦 Share on X":
    - /url: https://twitter.com/intent/tweet?text=I%20just%20hit%20my%20100th%20order%20using%20OHC%20to%20run%20my%20business!%20%F0%9F%9A%80%20Check%20them%20out%20and%20get%20%2450%20off%20your%20first%20month%3A%20https%3A%2F%2Fohc.app%2Fonboarding%3Fref%3Ddefault%26source%3Dmilestone_share%0A%0A%E2%9A%A1%20Powered%20by%20OHC
  - heading "Viral Loop Performance" [level=2]
  - text: Track your referral program and team growth. Active Loop Invites Sent 0 Active Referrals 0 Revenue from Referrals $0.00 Pending Rewards $0.00
  - link "View Referral Details":
    - /url: /referrals
  - img "shopping cart": 🛒
  - text: Cart Recovery Agent Active
  - paragraph: Your Sales Agent automatically followed up on abandoned carts this week.
  - paragraph: Recovered
  - paragraph: "3"
  - paragraph: Revenue Saved
  - paragraph: $120
  - link "Configure Agent →":
    - /url: /cart-recovery
  - heading "💰 Viral Growth" [level=3]
  - paragraph: Your affiliates are generating sales. Here is how your word-of-mouth engine is performing.
  - text: 0 Active Affiliates $0.00 Paid Commissions
  - link "Manage Affiliates":
    - /url: /referrals
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
    - text: Total Sales $0.00 All recorded orders Customers 0 Database customer records Pending Orders 0 Open fulfillment workload Low Stock 0 Materials below threshold Operations Map Live database state across the store workflow.
    - link "Open Orders":
      - /url: /orders
    - text: Orders 0 Rows returned Inbox 0 Messages returned Vendors 0 Supply partners Action Required
    - link "Inventory":
      - /url: /inventory
    - text: No database-backed actions are currently open. Recent Orders
    - link "View All":
      - /url: /orders
    - text: No order rows found for this tenant. Inbox Activity
    - link "Open Inbox":
      - /url: /inbox
    - text: No inbox message rows found for this tenant.
    - heading "Invite & Earn" [level=2]
    - paragraph: Invite a fellow business owner to OHC. They get 1 month free, you get $50 credit.
    - button "Get My Invite Link"
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
      - /url: /milestones
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
    - link "🎯 Leads Discount Code Generator Create discount code widgets for your customers.":
      - /url: /discount-code-generator
      - text: 🎯 Leads
      - heading "Discount Code Generator" [level=3]
      - paragraph: Create discount code widgets for your customers.
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
- text: See what's new in the latest updates.
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Release Notes & Changelog', () => {
  4  |     test('renders Changelog page with screenshots and can be accessed from AppShell', async ({ page }) => {
  5  |         // Go to dashboard to see AppShell
  6  |         await page.goto('/dashboard');
  7  |
  8  |         // Find and click the "What's New" link in the sidebar
  9  |         const whatsNewLink = page.locator('a.app-nav-link', { hasText: "What's New" });
  10 |         await expect(whatsNewLink).toBeVisible();
  11 |         await whatsNewLink.click();
  12 |
  13 |         // Verify we are on the changelog page
> 14 |         await expect(page).toHaveURL(/\/changelog/);
     |                            ^ Error: expect(page).toHaveURL(expected) failed
  15 |         await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
  16 |
  17 |         // Verify release notes content is rendered
  18 |         await expect(page.locator('h2').first()).toBeVisible();
  19 |
  20 |         // Wait for images to load, which proves the API correctly fetched the screenshot URL
  21 |         // In our manual test setup, we added a placeholder image to the top version
  22 |         const screenshot = page.locator('img[alt*="Screenshot"]').first();
  23 |         await expect(screenshot).toBeVisible();
  24 |
  25 |         // Check the website link
  26 |         const externalLink = page.locator('a', { hasText: 'Read the full technical changelog on our website' });
  27 |         await expect(externalLink).toBeVisible();
  28 |         await expect(externalLink).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  29 |     });
  30 | });
  31 |
```