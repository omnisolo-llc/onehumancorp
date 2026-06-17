# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ambassador-auto-reply.spec.ts >> Ambassador Auto-Responder CUJ >> Owner sees AI Handled auto-replied message in inbox
- Location: src/e2e/ambassador-auto-reply.spec.ts:4:7

# Error details

```
Error: expect(received).toBeTruthy()

Received: false
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
        - link "Dashboard" [ref=e11] [cursor=pointer]:
          - /url: /dashboard
          - img [ref=e13]
          - generic [ref=e18]: Dashboard
        - link "Assistant" [ref=e19] [cursor=pointer]:
          - /url: /assistant
          - img [ref=e21]
          - generic [ref=e23]: Assistant
        - link "Setup" [ref=e24] [cursor=pointer]:
          - /url: /onboarding
          - img [ref=e26]
          - generic [ref=e27]: Setup
        - link "Triage" [ref=e28] [cursor=pointer]:
          - /url: /triage
          - img [ref=e30]
          - generic [ref=e33]: Triage
        - link "Orders" [ref=e35] [cursor=pointer]:
          - /url: /orders
          - img [ref=e37]
          - generic [ref=e39]: Orders
        - link "Inbox" [ref=e40] [cursor=pointer]:
          - /url: /inbox
          - img [ref=e42]
          - generic [ref=e45]: Inbox
        - link "Inventory" [ref=e47] [cursor=pointer]:
          - /url: /inventory
          - img [ref=e49]
          - generic [ref=e53]: Inventory
        - link "Kairos" [ref=e55] [cursor=pointer]:
          - /url: /kairos
          - img [ref=e57]
          - generic [ref=e59]: Kairos
        - link "AI Departments" [ref=e60] [cursor=pointer]:
          - /url: /agents
          - img [ref=e62]
          - generic [ref=e67]: AI Departments
        - link "Analytics" [ref=e68] [cursor=pointer]:
          - /url: /business-analytics
          - img [ref=e70]
          - generic [ref=e71]: Analytics
        - link "Campaigns" [ref=e72] [cursor=pointer]:
          - /url: /dashboard/campaigns
          - img [ref=e74]
          - generic [ref=e77]: Campaigns
        - link "Settings" [ref=e78] [cursor=pointer]:
          - /url: /settings
          - img [ref=e80]
          - generic [ref=e83]: Settings
        - link "AI Usage" [ref=e84] [cursor=pointer]:
          - /url: /ai-usage-paywall
          - img [ref=e86]
          - generic [ref=e88]: AI Usage
      - generic [ref=e89]: System
      - navigation "System" [ref=e90]:
        - link "Calendar" [ref=e91] [cursor=pointer]:
          - /url: /calendar
          - img [ref=e93]
          - generic [ref=e95]: Calendar
        - link "LangGraph" [ref=e96] [cursor=pointer]:
          - /url: /langgraph
          - img [ref=e98]
          - generic [ref=e100]: LangGraph
        - link "Integrations" [ref=e101] [cursor=pointer]:
          - /url: /integrations
          - img [ref=e103]
          - generic [ref=e106]: Integrations
        - link "Cost" [ref=e107] [cursor=pointer]:
          - /url: /cost-dashboard
          - img [ref=e109]
          - generic [ref=e111]: Cost
        - link "Diagnostics" [ref=e112] [cursor=pointer]:
          - /url: /diagnostics
          - img [ref=e114]
          - generic [ref=e116]: Diagnostics
    - generic [ref=e117]:
      - banner [ref=e118]:
        - generic [ref=e119]:
          - generic [ref=e120]: "Site: default"
          - heading "Dashboard" [level=1] [ref=e121]
          - paragraph [ref=e122]: Network-style command center for database-backed store operations.
        - generic [ref=e123]:
          - generic [ref=e124]:
            - generic [ref=e125]:
              - generic [ref=e127]: API
              - generic [ref=e128]: Online
            - generic [ref=e129]:
              - generic [ref=e131]: Orders
              - generic [ref=e132]: "0"
            - generic [ref=e133]:
              - generic [ref=e135]: Stock
              - generic [ref=e136]: "0"
            - generic [ref=e137]:
              - generic [ref=e139]: Growth
              - generic [ref=e140]: Active
          - link "Campaigns" [ref=e141] [cursor=pointer]:
            - /url: /dashboard/campaigns
            - img [ref=e142]
            - text: Campaigns
          - link "New Product" [ref=e145] [cursor=pointer]:
            - /url: /products/new
            - img [ref=e146]
            - text: New Product
          - link "Help Center" [ref=e148] [cursor=pointer]:
            - /url: /help
            - generic [ref=e149]: "?"
      - main [ref=e150]:
        - generic [ref=e151]:
          - heading "Welcome back, test@example.com." [level=2] [ref=e152]
          - paragraph [ref=e153]: Your agents are working on your behalf.
        - generic [ref=e155]:
          - generic [ref=e156]:
            - generic [ref=e157]:
              - generic [ref=e158]: 🚀
              - text: Grow Your Network
            - heading "Refer & Earn $50" [level=2] [ref=e159]
            - paragraph [ref=e160]: Invite another business owner to OHC. When they sign up, you both unlock a $50 credit.
          - generic [ref=e161]:
            - generic [ref=e162]:
              - textbox [ref=e163]: http://localhost:3000/onboarding?ref=default-team&source=dashboard_invite
              - button "Copy Link" [ref=e164]
            - link "Share on WhatsApp" [ref=e165] [cursor=pointer]:
              - /url: https://wa.me/?text=Start%20your%20business%20on%20OHC!%20It's%20super%20easy.%20Use%20my%20link%20to%20get%20%2450%20off%20your%20first%20month%3A%20http%3A%2F%2Flocalhost%3A3000%2Fonboarding%3Fref%3Ddefault-team%26source%3Ddashboard_invite
              - img [ref=e166]
              - text: Share on WhatsApp
        - generic [ref=e169]:
          - generic [ref=e170]:
            - generic [ref=e171]: ⏱️
            - generic [ref=e172]:
              - generic [ref=e173]: Weekly Insight
              - heading "You saved 0 hours this week" [level=3] [ref=e174]
              - paragraph [ref=e175]: "Your AI agents handled 0 customer inquiries (Auto-Replied: 0), scheduled 0 appointments, and recovered 0 abandoned carts."
          - generic [ref=e176]:
            - button "Share to get 7 Days Pro" [ref=e177]:
              - img [ref=e178]
              - text: Share to get 7 Days Pro
            - paragraph [ref=e180]: Unlock premium tools by sharing your success.
        - button "+" [ref=e182]
        - button "Voice Command Assistant" [ref=e184]:
          - generic [ref=e185]: 🎤
        - generic [ref=e188]:
          - generic [ref=e189]:
            - img [ref=e190]
            - text: Pro Feature
          - heading "Unlock Advanced AI Analytics" [level=2] [ref=e192]
          - paragraph [ref=e193]: See exactly which products are driving revenue, automate cross-selling, and get daily AI strategy briefings.
          - generic [ref=e194]:
            - link "Upgrade to Pro ($79/mo)" [ref=e195] [cursor=pointer]:
              - /url: /pricing
            - generic [ref=e196]: or
            - button "Refer a Friend to Unlock for 7 Days" [ref=e197]
        - generic [ref=e198]:
          - button "Start Tour" [ref=e199]
          - button "Launch Site" [ref=e200]
          - button "Migrate Existing Store" [ref=e201]
        - generic [ref=e202]:
          - region "Unified Agent Feed" [ref=e204]:
            - generic [ref=e205]:
              - button "Proposals (0)" [ref=e206]
              - button "Activity Feed" [ref=e207]
            - generic [ref=e208]:
              - generic [ref=e212]:
                - generic [ref=e213]:
                  - generic [ref=e214]: Action Needed
                  - generic [ref=e215]: Just now
                - heading "Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed." [level=3] [ref=e216]
              - generic [ref=e217]:
                - generic [ref=e218]:
                  - generic [ref=e219]:
                    - generic [ref=e220]: Approval
                    - generic [ref=e221]: 5 min ago
                  - heading "Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?" [level=3] [ref=e222]
                - generic [ref=e223]:
                  - button "Approve" [ref=e224]
                  - button "Edit" [ref=e225]
                  - button "Deny" [ref=e226]
              - generic [ref=e227]: Loading Agent Proposals...
          - generic [ref=e228]:
            - generic [ref=e229]:
              - generic [ref=e230]:
                - generic [ref=e231]:
                  - heading "Viral Loop Performance" [level=2] [ref=e232]
                  - generic [ref=e233]: Track your referral program and team growth.
                - generic [ref=e235]: Active Loop
              - generic [ref=e236]:
                - generic [ref=e237]:
                  - generic [ref=e238]:
                    - generic [ref=e239]: Invites Sent
                    - generic [ref=e240]: "0"
                  - generic [ref=e241]:
                    - generic [ref=e242]: Active Referrals
                    - generic [ref=e243]: "0"
                  - generic [ref=e244]:
                    - generic [ref=e245]: Revenue from Referrals
                    - generic [ref=e246]: $0.00
                  - generic [ref=e247]:
                    - generic [ref=e248]: Pending Rewards
                    - generic [ref=e249]: $0.00
                - link "View Referral Details" [ref=e251] [cursor=pointer]:
                  - /url: /referrals
            - generic [ref=e252]:
              - generic [ref=e254]:
                - generic [ref=e256]:
                  - generic [ref=e257]:
                    - img "shopping cart" [ref=e258]: 🛒
                    - generic [ref=e259]: Cart Recovery Agent
                  - generic [ref=e260]: Active
                - link "Configure Agent →" [ref=e262] [cursor=pointer]:
                  - /url: /cart-recovery
                  - text: Configure Agent
                  - generic [ref=e263]: →
              - heading "💰 Viral Growth" [level=3] [ref=e266]:
                - generic [ref=e267]: 💰
                - text: Viral Growth
        - generic [ref=e268]:
          - link "⚡ Powered by OHC" [ref=e270] [cursor=pointer]:
            - /url: /onboarding?ref=default&source=footer_widget
            - generic [ref=e271]: ⚡ Powered by
            - generic [ref=e272]: OHC
            - img [ref=e273]
          - button "Report Incident" [ref=e275]
        - generic [ref=e276]:
          - generic [ref=e277]:
            - generic [ref=e278]:
              - heading "2024 Store Wrapped" [level=2] [ref=e279]
              - generic [ref=e280]: A shareable snapshot of your strongest store moments.
            - generic [ref=e281]: Viral Loop
          - generic [ref=e282]:
            - paragraph [ref=e283]: Turn your sales, products, and milestones into a referral-friendly recap.
            - link "View Your Wrapped 🎁" [ref=e284] [cursor=pointer]:
              - /url: /wrapped
        - main [ref=e285]:
          - link "Assistant Tasks Open the dashboard task workspace for conversations, artifacts, and assistant actions. →" [ref=e287] [cursor=pointer]:
            - /url: /assistant
            - generic [ref=e288]:
              - generic [ref=e290]: A
              - generic [ref=e291]:
                - heading "Assistant Tasks" [level=3] [ref=e292]
                - paragraph [ref=e293]: Open the dashboard task workspace for conversations, artifacts, and assistant actions.
              - generic [ref=e294]: →
          - generic [ref=e297]:
            - generic [ref=e298]:
              - generic [ref=e299]: 📣
              - generic [ref=e300]:
                - heading "The Promoter Agent" [level=3] [ref=e301]
                - paragraph [ref=e302]: Let OHC's AI write engaging social media posts to drive traffic to your storefront.
            - link "Create Posts" [ref=e303] [cursor=pointer]:
              - /url: /promoter
          - generic [ref=e304]:
            - generic [ref=e306]:
              - generic [ref=e307]:
                - generic [ref=e308]: 🎉
                - generic [ref=e309]:
                  - heading "Milestone Unlocked!" [level=3] [ref=e310]
                  - paragraph [ref=e311]: You completed your first 5 orders!
              - button "Share & Claim Reward" [ref=e312]
            - generic [ref=e313]:
              - generic [ref=e314]:
                - heading "Business Analytics" [level=2] [ref=e315]
                - paragraph [ref=e316]: Live performance, orders, and inbox activity.
              - link "Business Analytics" [ref=e317] [cursor=pointer]:
                - /url: /business-analytics
            - generic [ref=e319]:
              - generic [ref=e320]:
                - generic [ref=e322]: Total Sales
                - generic [ref=e323]: $0.00
                - generic [ref=e324]: Loading database rows
              - generic [ref=e325]:
                - generic [ref=e326]: Customers
                - generic [ref=e327]: "0"
                - generic [ref=e328]: Database customer records
              - generic [ref=e329]:
                - generic [ref=e330]: Pending Orders
                - generic [ref=e331]: "0"
                - generic [ref=e332]: Open fulfillment workload
              - generic [ref=e333]:
                - generic [ref=e334]: Low Stock
                - generic [ref=e335]: "0"
                - generic [ref=e336]: Materials below threshold
          - generic [ref=e337]:
            - generic [ref=e338]:
              - generic [ref=e339]:
                - generic [ref=e340]:
                  - generic [ref=e341]: Operations Map
                  - generic [ref=e342]: Live database state across the store workflow.
                - link "Open Orders" [ref=e343] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e345]:
                - generic [ref=e346]:
                  - generic [ref=e347]: Orders
                  - generic [ref=e348]: "0"
                  - generic [ref=e349]: Rows returned
                - generic [ref=e350]:
                  - generic [ref=e351]: Inbox
                  - generic [ref=e352]: "0"
                  - generic [ref=e353]: Messages returned
                - generic [ref=e354]:
                  - generic [ref=e355]: Vendors
                  - generic [ref=e356]: "0"
                  - generic [ref=e357]: Supply partners
            - generic [ref=e359]:
              - generic [ref=e360]: Action Required
              - link "Inventory" [ref=e361] [cursor=pointer]:
                - /url: /inventory
          - generic [ref=e362]:
            - generic [ref=e363]:
              - generic [ref=e364]:
                - generic [ref=e366]: Recent Orders
                - link "View All" [ref=e367] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e368]: Loading orders from the database...
            - generic [ref=e369]:
              - generic [ref=e370]:
                - generic [ref=e372]: Inbox Activity
                - link "Open Inbox" [ref=e373] [cursor=pointer]:
                  - /url: /inbox
              - generic [ref=e375]: Loading inbox from the database...
          - generic [ref=e377]:
            - generic [ref=e378]:
              - heading "Invite & Earn" [level=2] [ref=e379]
              - paragraph [ref=e380]: Invite a fellow business owner to OHC. They get 1 month free, you get $50 credit.
            - button "Get My Invite Link" [ref=e381]
          - generic [ref=e382]:
            - generic [ref=e384]:
              - heading "Growth & Virality" [level=2] [ref=e385]
              - paragraph [ref=e386]: Unlock new customers and track milestones.
            - generic [ref=e387]:
              - link "↗ Orchestrate Campaign Orchestration Plan, generate, review, and launch customer campaigns from live dashboard data." [ref=e388] [cursor=pointer]:
                - /url: /feed
                - generic [ref=e389]:
                  - generic [ref=e390]: ↗
                  - generic [ref=e391]: Orchestrate
                - heading "Campaign Orchestration" [level=3] [ref=e392]
                - paragraph [ref=e393]: Plan, generate, review, and launch customer campaigns from live dashboard data.
              - link "📈 ROI Pro Plan ROI Calculator See how much extra revenue you could generate by unlocking the Pro Plan." [ref=e394] [cursor=pointer]:
                - /url: /upgrade-roi
                - generic [ref=e395]:
                  - generic [ref=e396]: 📈
                  - generic [ref=e397]: ROI
                - heading "Pro Plan ROI Calculator" [level=3] [ref=e398]
                - paragraph [ref=e399]: See how much extra revenue you could generate by unlocking the Pro Plan.
              - link "🤝 Earn $50 Referrals Invite other business owners to OHC and earn premium credits." [ref=e400] [cursor=pointer]:
                - /url: /referrals
                - generic [ref=e401]:
                  - generic [ref=e402]: 🤝
                  - generic [ref=e403]: Earn $50
                - heading "Referrals" [level=3] [ref=e404]
                - paragraph [ref=e405]: Invite other business owners to OHC and earn premium credits.
              - link "🏆 Viral Affiliate Badge Builder Create an embeddable badge to grow your affiliate network." [ref=e406] [cursor=pointer]:
                - /url: /affiliate-badge-builder
                - generic [ref=e407]:
                  - generic [ref=e408]: 🏆
                  - generic [ref=e409]: Viral
                - heading "Affiliate Badge Builder" [level=3] [ref=e410]
                - paragraph [ref=e411]: Create an embeddable badge to grow your affiliate network.
              - link "💰 Finance Finance & Invoicing Manage cash flow, invoices, and automated payment follow-ups." [ref=e412] [cursor=pointer]:
                - /url: /finance
                - generic [ref=e413]:
                  - generic [ref=e414]: 💰
                  - generic [ref=e415]: Finance
                - heading "Finance & Invoicing" [level=3] [ref=e416]
                - paragraph [ref=e417]: Manage cash flow, invoices, and automated payment follow-ups.
              - link "🧾 Billing AI Invoice Generator Generate professional, shareable invoices that bring new customers to OHC." [ref=e418] [cursor=pointer]:
                - /url: /invoice-generator
                - generic [ref=e419]:
                  - generic [ref=e420]: 🧾
                  - generic [ref=e421]: Billing
                - heading "AI Invoice Generator" [level=3] [ref=e422]
                - paragraph [ref=e423]: Generate professional, shareable invoices that bring new customers to OHC.
              - link "📝 Sales AI Proposal Generator Create smart, shareable proposals with an interactive approval flow to win clients faster." [ref=e424] [cursor=pointer]:
                - /url: /proposal-generator
                - generic [ref=e425]:
                  - generic [ref=e426]: 📝
                  - generic [ref=e427]: Sales
                - heading "AI Proposal Generator" [level=3] [ref=e428]
                - paragraph [ref=e429]: Create smart, shareable proposals with an interactive approval flow to win clients faster.
              - link "Share Milestones Track and share your business achievements with your audience." [ref=e430] [cursor=pointer]:
                - /url: /milestones
                - generic [ref=e431]:
                  - generic [ref=e432]: 🏆
                  - generic [ref=e433]: Share
                - heading "Milestones" [level=3] [ref=e434]
                - paragraph [ref=e435]: Track and share your business achievements with your audience.
              - link "🤝 Loyalty Customer Loyalty Set up a 'Give X, Get Y' referral program and generate campaigns." [ref=e436] [cursor=pointer]:
                - /url: /loyalty-program
                - generic [ref=e437]:
                  - generic [ref=e438]: 🤝
                  - generic [ref=e439]: Loyalty
                - heading "Customer Loyalty" [level=3] [ref=e440]
                - paragraph [ref=e441]: Set up a 'Give X, Get Y' referral program and generate campaigns.
              - link "💸 Referrals Customer Referral Program Launch a Give $10, Get $10 program to turn your customers into advocates." [ref=e442] [cursor=pointer]:
                - /url: /customer-referral-program
                - generic [ref=e443]:
                  - generic [ref=e444]: 💸
                  - generic [ref=e445]: Referrals
                - heading "Customer Referral Program" [level=3] [ref=e446]
                - paragraph [ref=e447]: Launch a Give $10, Get $10 program to turn your customers into advocates.
              - link "🎴 Cards Social Share Cards Generate Share Cards to promote your brand on social media." [ref=e448] [cursor=pointer]:
                - /url: /share-cards
                - generic [ref=e449]:
                  - generic [ref=e450]: 🎴
                  - generic [ref=e451]: Cards
                - heading "Social Share Cards" [level=3] [ref=e452]
                - paragraph [ref=e453]: Generate Share Cards to promote your brand on social media.
              - link "🌐 Widget Storefront Widget Embed a mini storefront on your blog or website to boost sales." [ref=e454] [cursor=pointer]:
                - /url: /storefront-widget
                - generic [ref=e455]:
                  - generic [ref=e456]: 🌐
                  - generic [ref=e457]: Widget
                - heading "Storefront Widget" [level=3] [ref=e458]
                - paragraph [ref=e459]: Embed a mini storefront on your blog or website to boost sales.
              - link "🔌 Widget Interactive Embed Build custom intake, booking, or quote widgets for your site." [ref=e460] [cursor=pointer]:
                - /url: /embed-builder
                - generic [ref=e461]:
                  - generic [ref=e462]: 🔌
                  - generic [ref=e463]: Widget
                - heading "Interactive Embed" [level=3] [ref=e464]
                - paragraph [ref=e465]: Build custom intake, booking, or quote widgets for your site.
              - link "📦 Recurring Subscriptions & Fulfillments Manage recurring products, subscribers, and shipping batches." [ref=e466] [cursor=pointer]:
                - /url: /subscriptions
                - generic [ref=e467]:
                  - generic [ref=e468]: 📦
                  - generic [ref=e469]: Recurring
                - heading "Subscriptions & Fulfillments" [level=3] [ref=e470]
                - paragraph [ref=e471]: Manage recurring products, subscribers, and shipping batches.
              - link "🚀 Proof Social Proof Nudge Show visitors that others are buying to increase conversions." [ref=e472] [cursor=pointer]:
                - /url: /social-proof-nudge
                - generic [ref=e473]:
                  - generic [ref=e474]: 🚀
                  - generic [ref=e475]: Proof
                - heading "Social Proof Nudge" [level=3] [ref=e476]
                - paragraph [ref=e477]: Show visitors that others are buying to increase conversions.
              - link "📋 Leads Work-Intake Widget Embed a smart lead capture form with a viral loop directly on your site." [ref=e478] [cursor=pointer]:
                - /url: /work-intake-widget
                - generic [ref=e479]:
                  - generic [ref=e480]: 📋
                  - generic [ref=e481]: Leads
                - heading "Work-Intake Widget" [level=3] [ref=e482]
                - paragraph [ref=e483]: Embed a smart lead capture form with a viral loop directly on your site.
              - link "🔗 Bio Create Link-in-Bio Page Publish a lightweight social profile page for your storefront and offers." [ref=e484] [cursor=pointer]:
                - /url: /link-in-bio-generator
                - generic [ref=e485]:
                  - generic [ref=e486]: 🔗
                  - generic [ref=e487]: Bio
                - heading "Create Link-in-Bio Page" [level=3] [ref=e488]
                - paragraph [ref=e489]: Publish a lightweight social profile page for your storefront and offers.
              - link "💬 Social WhatsApp Link Generator Create shareable WhatsApp links to start conversations instantly." [ref=e490] [cursor=pointer]:
                - /url: /whatsapp-link-generator
                - generic [ref=e491]:
                  - generic [ref=e492]: 💬
                  - generic [ref=e493]: Social
                - heading "WhatsApp Link Generator" [level=3] [ref=e494]
                - paragraph [ref=e495]: Create shareable WhatsApp links to start conversations instantly.
              - link "🎁 Viral Viral Giveaway Generator Launch a viral sweepstakes to capture emails and drive social shares." [ref=e496] [cursor=pointer]:
                - /url: /giveaway
                - generic [ref=e497]:
                  - generic [ref=e498]: 🎁
                  - generic [ref=e499]: Viral
                - heading "Viral Giveaway Generator" [level=3] [ref=e500]
                - paragraph [ref=e501]: Launch a viral sweepstakes to capture emails and drive social shares.
              - link "🔓 Growth Share-to-Unlock Generator Require customers to share your page on social media to reveal a discount code." [ref=e502] [cursor=pointer]:
                - /url: /share-to-unlock-generator
                - generic [ref=e503]:
                  - generic [ref=e504]: 🔓
                  - generic [ref=e505]: Growth
                - heading "Share-to-Unlock Generator" [level=3] [ref=e506]
                - paragraph [ref=e507]: Require customers to share your page on social media to reveal a discount code.
              - link "💌 Retain Customer Win-back Re-engage inactive customers with AI-generated email campaigns." [ref=e508] [cursor=pointer]:
                - /url: /win-back
                - generic [ref=e509]:
                  - generic [ref=e510]: 💌
                  - generic [ref=e511]: Retain
                - heading "Customer Win-back" [level=3] [ref=e512]
                - paragraph [ref=e513]: Re-engage inactive customers with AI-generated email campaigns.
              - link "⭐️ Reviews Automated Reviews Generate highly-converting, personalized review request emails." [ref=e514] [cursor=pointer]:
                - /url: /review-campaigns
                - generic [ref=e515]:
                  - generic [ref=e516]: ⭐️
                  - generic [ref=e517]: Reviews
                - heading "Automated Reviews" [level=3] [ref=e518]
                - paragraph [ref=e519]: Generate highly-converting, personalized review request emails.
              - link "✨ Promo Seasonal Promo Generator Create AI campaigns and promo codes for special occasions instantly." [ref=e520] [cursor=pointer]:
                - /url: /seasonal-promo
                - generic [ref=e521]:
                  - generic [ref=e522]: ✨
                  - generic [ref=e523]: Promo
                - heading "Seasonal Promo Generator" [level=3] [ref=e524]
                - paragraph [ref=e525]: Create AI campaigns and promo codes for special occasions instantly.
              - link "🛒 Recover Cart Recovery Recover abandoned carts with personalized AI follow-ups." [ref=e526] [cursor=pointer]:
                - /url: /cart-recovery
                - generic [ref=e527]:
                  - generic [ref=e528]: 🛒
                  - generic [ref=e529]: Recover
                - heading "Cart Recovery" [level=3] [ref=e530]
                - paragraph [ref=e531]: Recover abandoned carts with personalized AI follow-ups.
              - link "⚡ Urgency Flash Sale Generator Create high-converting flash sale countdown widgets." [ref=e532] [cursor=pointer]:
                - /url: /flash-sale-generator
                - generic [ref=e533]:
                  - generic [ref=e534]: ⚡
                  - generic [ref=e535]: Urgency
                - heading "Flash Sale Generator" [level=3] [ref=e536]
                - paragraph [ref=e537]: Create high-converting flash sale countdown widgets.
              - link "🎯 Leads Want more local jobs this week? [Tap here] Launch an autonomous hyper-local lead generation campaign." [ref=e538] [cursor=pointer]:
                - /url: /marketing/lead-gen
                - generic [ref=e539]:
                  - generic [ref=e540]: 🎯
                  - generic [ref=e541]: Leads
                - heading "Want more local jobs this week? [Tap here]" [level=3] [ref=e542]
                - paragraph [ref=e543]: Launch an autonomous hyper-local lead generation campaign.
              - link "🎁 Extension Interactive Trial Extension Share your setup on X to instantly unlock 7 extra days of Pro." [ref=e544] [cursor=pointer]:
                - /url: /trial-extension
                - generic [ref=e545]:
                  - generic [ref=e546]: 🎁
                  - generic [ref=e547]: Extension
                - heading "Interactive Trial Extension" [level=3] [ref=e548]
                - paragraph [ref=e549]: Share your setup on X to instantly unlock 7 extra days of Pro.
              - link "📍 Operations Field Ops Route Offline-first mobile route management for field service workers." [ref=e550] [cursor=pointer]:
                - /url: /field-ops/jobs
                - generic [ref=e551]:
                  - generic [ref=e552]: 📍
                  - generic [ref=e553]: Operations
                - heading "Field Ops Route" [level=3] [ref=e554]
                - paragraph [ref=e555]: Offline-first mobile route management for field service workers.
              - link "⚙️ Config Settings Manage your account and preferences." [ref=e556] [cursor=pointer]:
                - /url: /settings
                - generic [ref=e557]:
                  - generic [ref=e558]: ⚙️
                  - generic [ref=e559]: Config
                - heading "Settings" [level=3] [ref=e560]
                - paragraph [ref=e561]: Manage your account and preferences.
  - button "Help" [ref=e564]:
    - img [ref=e565]
  - button "Open help chat" [ref=e568]:
    - generic [ref=e569]: ✨
    - generic [ref=e570]: Ask anything
  - button "Voice Assistant" [ref=e571]:
    - img
  - alert [ref=e573]
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Ambassador Auto-Responder CUJ', () => {
  4  |   test('Owner sees AI Handled auto-replied message in inbox', async ({ page, request }) => {
  5  |     // 1. Connect Instagram via Integrations
  6  |     // Start from login to satisfy the rules
  7  |     await page.goto('/login');
  8  |     await page.getByPlaceholder('Email or Username').fill('test@example.com');
  9  |     await page.getByPlaceholder('Password').fill('password123');
  10 |     await page.getByRole('button', { name: 'Log In' }).click();
  11 |     await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  12 |
  13 |     // Set configuration for auto-reply in backend if possible, or trigger auto-reply
  14 |     const tenantId = 'test-tenant';
  15 |
  16 |     // 2. Trigger the Ambassador's draft reply via a real API call (no mocks)
  17 |     // The CustomerSuccess agent listens for tenant.message.received, which is triggered via the webhook endpoint
  18 |     const webhookPayload = {
  19 |       tenant_id: tenantId,
  20 |       sender_id: 'testuser',
  21 |       message: 'I would like to place an order.',
  22 |       source: 'instagram'
  23 |     };
  24 |
  25 |     const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
  26 |     const response = await request.post(`${apiBase}/api/inbox/webhook`, {
  27 |       data: webhookPayload,
  28 |     });
  29 |
> 30 |     expect(response.ok()).toBeTruthy();
     |                           ^ Error: expect(received).toBeTruthy()
  31 |
  32 |     // 3. Wait for background task to execute
  33 |     // In our test environment, we wait for a moment so the worker pool handles it
  34 |     await page.waitForTimeout(2000);
  35 |
  36 |     // 4. Check Inbox Page
  37 |     await page.goto('/inbox');
  38 |     await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
  39 |
  40 |     // Verify "AI Handled" badge shows up
  41 |     const messageLocator = page.locator('.app-list-item', { hasText: 'I would like to place an order.' }).first();
  42 |     await expect(messageLocator).toBeVisible({ timeout: 5000 });
  43 |
  44 |     // Click it
  45 |     await messageLocator.click();
  46 |
  47 |     // Verify detail shows AI Handled
  48 |     await expect(page.locator('.app-panel-body .app-badge', { hasText: 'AI Handled' })).toBeVisible();
  49 |   });
  50 | });
  51 |
```