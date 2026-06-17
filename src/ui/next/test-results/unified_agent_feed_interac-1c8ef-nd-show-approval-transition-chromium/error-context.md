# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: unified_agent_feed_interaction.spec.ts >> Unified Agent Feed Interactive Flow >> should render properly, expand for details, and show approval transition
- Location: src/e2e/unified_agent_feed_interaction.spec.ts:6:7

# Error details

```
Error: expect(received).toBeGreaterThanOrEqual(expected)

Expected: >= 44
Received:    14
```

# Page snapshot

```yaml
- generic [active] [ref=e1]:
  - generic [ref=e2]:
    - complementary [ref=e3]:
      - generic [ref=e5]: O
      - navigation "Primary" [ref=e6]:
        - link [ref=e8] [cursor=pointer]:
          - /url: /dashboard
          - img [ref=e10]
        - link [ref=e15] [cursor=pointer]:
          - /url: /assistant
          - img [ref=e17]
        - link [ref=e19] [cursor=pointer]:
          - /url: /onboarding
          - img [ref=e21]
        - link [ref=e22] [cursor=pointer]:
          - /url: /triage
          - img [ref=e24]
        - link [ref=e28] [cursor=pointer]:
          - /url: /orders
          - img [ref=e30]
        - link [ref=e32] [cursor=pointer]:
          - /url: /inbox
          - img [ref=e34]
        - link [ref=e38] [cursor=pointer]:
          - /url: /inventory
          - img [ref=e40]
        - link [ref=e45] [cursor=pointer]:
          - /url: /kairos
          - img [ref=e47]
        - link [ref=e49] [cursor=pointer]:
          - /url: /agents
          - img [ref=e51]
        - link [ref=e56] [cursor=pointer]:
          - /url: /business-analytics
          - img [ref=e58]
        - link [ref=e59] [cursor=pointer]:
          - /url: /dashboard/campaigns
          - img [ref=e61]
        - link [ref=e64] [cursor=pointer]:
          - /url: /settings
          - img [ref=e66]
        - link [ref=e69] [cursor=pointer]:
          - /url: /ai-usage-paywall
          - img [ref=e71]
      - navigation "System" [ref=e73]:
        - link [ref=e74] [cursor=pointer]:
          - /url: /calendar
          - img [ref=e76]
        - link [ref=e78] [cursor=pointer]:
          - /url: /langgraph
          - img [ref=e80]
        - link [ref=e82] [cursor=pointer]:
          - /url: /integrations
          - img [ref=e84]
        - link [ref=e87] [cursor=pointer]:
          - /url: /cost-dashboard
          - img [ref=e89]
        - link [ref=e91] [cursor=pointer]:
          - /url: /diagnostics
          - img [ref=e93]
    - generic [ref=e95]:
      - banner [ref=e96]:
        - generic [ref=e97]:
          - generic [ref=e98]: "Site: default"
          - heading "Dashboard" [level=1] [ref=e99]
          - paragraph [ref=e100]: Network-style command center for database-backed store operations.
        - generic [ref=e101]:
          - generic [ref=e102]:
            - generic [ref=e103]:
              - generic [ref=e105]: API
              - generic [ref=e106]: Online
            - generic [ref=e107]:
              - generic [ref=e109]: Orders
              - generic [ref=e110]: "0"
            - generic [ref=e111]:
              - generic [ref=e113]: Stock
              - generic [ref=e114]: "0"
            - generic [ref=e115]:
              - generic [ref=e117]: Growth
              - generic [ref=e118]: Active
          - link "Campaigns" [ref=e119] [cursor=pointer]:
            - /url: /dashboard/campaigns
            - img [ref=e120]
            - text: Campaigns
          - link "New Product" [ref=e123] [cursor=pointer]:
            - /url: /products/new
            - img [ref=e124]
            - text: New Product
          - link "Help Center" [ref=e126] [cursor=pointer]:
            - /url: /help
            - generic [ref=e127]: "?"
      - main [ref=e128]:
        - generic [ref=e129]:
          - heading "Welcome back, Human." [level=2] [ref=e130]
          - paragraph [ref=e131]: Your agents are working on your behalf.
        - generic [ref=e133]:
          - generic [ref=e134]:
            - generic [ref=e135]:
              - generic [ref=e136]: 🚀
              - text: Grow Your Network
            - heading "Refer & Earn $50" [level=2] [ref=e137]
            - paragraph [ref=e138]: Invite another business owner to OHC. When they sign up, you both unlock a $50 credit.
          - generic [ref=e139]:
            - generic [ref=e140]:
              - textbox [ref=e141]: http://localhost:3000/onboarding?ref=default-team&source=dashboard_invite
              - button "Copy Link" [ref=e142]
            - link "Share on WhatsApp" [ref=e143] [cursor=pointer]:
              - /url: https://wa.me/?text=Start%20your%20business%20on%20OHC!%20It's%20super%20easy.%20Use%20my%20link%20to%20get%20%2450%20off%20your%20first%20month%3A%20http%3A%2F%2Flocalhost%3A3000%2Fonboarding%3Fref%3Ddefault-team%26source%3Ddashboard_invite
              - img [ref=e144]
              - text: Share on WhatsApp
        - generic [ref=e147]:
          - generic [ref=e148]:
            - generic [ref=e149]: ⏱️
            - generic [ref=e150]:
              - generic [ref=e151]: Weekly Insight
              - heading "You saved 0 hours this week" [level=3] [ref=e152]
              - paragraph [ref=e153]: "Your AI agents handled 0 customer inquiries (Auto-Replied: 0), scheduled 0 appointments, and recovered 0 abandoned carts."
          - generic [ref=e154]:
            - button "Share to get 7 Days Pro" [ref=e155]:
              - img [ref=e156]
              - text: Share to get 7 Days Pro
            - paragraph [ref=e158]: Unlock premium tools by sharing your success.
        - button "+" [ref=e160]
        - button "Voice Command Assistant" [ref=e162]:
          - generic [ref=e163]: 🎤
        - generic [ref=e166]:
          - generic [ref=e167]:
            - img [ref=e168]
            - text: Pro Feature
          - heading "Unlock Advanced AI Analytics" [level=2] [ref=e170]
          - paragraph [ref=e171]: See exactly which products are driving revenue, automate cross-selling, and get daily AI strategy briefings.
          - generic [ref=e172]:
            - link "Upgrade to Pro ($79/mo)" [ref=e173] [cursor=pointer]:
              - /url: /pricing
            - generic [ref=e174]: or
            - button "Refer a Friend to Unlock for 7 Days" [ref=e175]
        - generic [ref=e176]:
          - button "Start Tour" [ref=e177]
          - button "Launch Site" [ref=e178]
          - button "Migrate Existing Store" [ref=e179]
        - generic [ref=e180]:
          - region "Unified Agent Feed" [ref=e182]:
            - generic [ref=e183]:
              - button "Proposals (0)" [ref=e184]
              - button "Activity Feed" [ref=e185]
            - generic [ref=e186]:
              - generic [ref=e190]:
                - generic [ref=e191]:
                  - generic [ref=e192]: Action Needed
                  - generic [ref=e193]: Just now
                - heading "Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed." [level=3] [ref=e194]
              - generic [ref=e195]:
                - generic [ref=e196]:
                  - generic [ref=e197]:
                    - generic [ref=e198]: Approval
                    - generic [ref=e199]: 5 min ago
                  - heading "Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?" [level=3] [ref=e200]
                - generic [ref=e201]:
                  - button "Approve" [ref=e202]
                  - button "Edit" [ref=e203]
                  - button "Deny" [ref=e204]
              - generic [ref=e205]: Loading Agent Proposals...
          - generic [ref=e206]:
            - generic [ref=e207]:
              - generic [ref=e208]:
                - generic [ref=e209]:
                  - heading "Viral Loop Performance" [level=2] [ref=e210]
                  - generic [ref=e211]: Track your referral program and team growth.
                - generic [ref=e213]: Active Loop
              - generic [ref=e214]:
                - generic [ref=e215]:
                  - generic [ref=e216]:
                    - generic [ref=e217]: Invites Sent
                    - generic [ref=e218]: "0"
                  - generic [ref=e219]:
                    - generic [ref=e220]: Active Referrals
                    - generic [ref=e221]: "0"
                  - generic [ref=e222]:
                    - generic [ref=e223]: Revenue from Referrals
                    - generic [ref=e224]: $0.00
                  - generic [ref=e225]:
                    - generic [ref=e226]: Pending Rewards
                    - generic [ref=e227]: $0.00
                - link "View Referral Details" [ref=e229] [cursor=pointer]:
                  - /url: /referrals
            - generic [ref=e230]:
              - generic [ref=e232]:
                - generic [ref=e234]:
                  - generic [ref=e235]:
                    - img "shopping cart" [ref=e236]: 🛒
                    - generic [ref=e237]: Cart Recovery Agent
                  - generic [ref=e238]: Active
                - link "Configure Agent →" [ref=e240] [cursor=pointer]:
                  - /url: /cart-recovery
                  - text: Configure Agent
                  - generic [ref=e241]: →
              - heading "💰 Viral Growth" [level=3] [ref=e244]:
                - generic [ref=e245]: 💰
                - text: Viral Growth
        - generic [ref=e246]:
          - link "⚡ Powered by OHC" [ref=e248] [cursor=pointer]:
            - /url: /onboarding?ref=default&source=footer_widget
            - generic [ref=e249]: ⚡ Powered by
            - generic [ref=e250]: OHC
            - img [ref=e251]
          - button "Report Incident" [ref=e253]
        - generic [ref=e254]:
          - generic [ref=e255]:
            - generic [ref=e256]:
              - heading "2024 Store Wrapped" [level=2] [ref=e257]
              - generic [ref=e258]: A shareable snapshot of your strongest store moments.
            - generic [ref=e259]: Viral Loop
          - generic [ref=e260]:
            - paragraph [ref=e261]: Turn your sales, products, and milestones into a referral-friendly recap.
            - link "View Your Wrapped 🎁" [ref=e262] [cursor=pointer]:
              - /url: /wrapped
        - main [ref=e263]:
          - link "Assistant Tasks Open the dashboard task workspace for conversations, artifacts, and assistant actions. →" [ref=e265] [cursor=pointer]:
            - /url: /assistant
            - generic [ref=e266]:
              - generic [ref=e268]: A
              - generic [ref=e269]:
                - heading "Assistant Tasks" [level=3] [ref=e270]
                - paragraph [ref=e271]: Open the dashboard task workspace for conversations, artifacts, and assistant actions.
              - generic [ref=e272]: →
          - generic [ref=e275]:
            - generic [ref=e276]:
              - generic [ref=e277]: 📣
              - generic [ref=e278]:
                - heading "The Promoter Agent" [level=3] [ref=e279]
                - paragraph [ref=e280]: Let OHC's AI write engaging social media posts to drive traffic to your storefront.
            - link "Create Posts" [ref=e281] [cursor=pointer]:
              - /url: /promoter
          - generic [ref=e282]:
            - generic [ref=e284]:
              - generic [ref=e285]:
                - generic [ref=e286]: 🎉
                - generic [ref=e287]:
                  - heading "Milestone Unlocked!" [level=3] [ref=e288]
                  - paragraph [ref=e289]: You completed your first 5 orders!
              - button "Share & Claim Reward" [ref=e290]
            - generic [ref=e291]:
              - generic [ref=e292]:
                - heading "Business Analytics" [level=2] [ref=e293]
                - paragraph [ref=e294]: Live performance, orders, and inbox activity.
              - link "Business Analytics" [ref=e295] [cursor=pointer]:
                - /url: /business-analytics
            - generic [ref=e297]:
              - generic [ref=e298]:
                - generic [ref=e300]: Total Sales
                - generic [ref=e301]: $0.00
                - generic [ref=e302]: Loading database rows
              - generic [ref=e303]:
                - generic [ref=e304]: Customers
                - generic [ref=e305]: "0"
                - generic [ref=e306]: Database customer records
              - generic [ref=e307]:
                - generic [ref=e308]: Pending Orders
                - generic [ref=e309]: "0"
                - generic [ref=e310]: Open fulfillment workload
              - generic [ref=e311]:
                - generic [ref=e312]: Low Stock
                - generic [ref=e313]: "0"
                - generic [ref=e314]: Materials below threshold
          - generic [ref=e315]:
            - generic [ref=e316]:
              - generic [ref=e317]:
                - generic [ref=e318]:
                  - generic [ref=e319]: Operations Map
                  - generic [ref=e320]: Live database state across the store workflow.
                - link "Open Orders" [ref=e321] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e323]:
                - generic [ref=e324]:
                  - generic [ref=e325]: Orders
                  - generic [ref=e326]: "0"
                  - generic [ref=e327]: Rows returned
                - generic [ref=e328]:
                  - generic [ref=e329]: Inbox
                  - generic [ref=e330]: "0"
                  - generic [ref=e331]: Messages returned
                - generic [ref=e332]:
                  - generic [ref=e333]: Vendors
                  - generic [ref=e334]: "0"
                  - generic [ref=e335]: Supply partners
            - generic [ref=e337]:
              - generic [ref=e338]: Action Required
              - link "Inventory" [ref=e339] [cursor=pointer]:
                - /url: /inventory
          - generic [ref=e340]:
            - generic [ref=e341]:
              - generic [ref=e342]:
                - generic [ref=e344]: Recent Orders
                - link "View All" [ref=e345] [cursor=pointer]:
                  - /url: /orders
              - generic [ref=e346]: Loading orders from the database...
            - generic [ref=e347]:
              - generic [ref=e348]:
                - generic [ref=e350]: Inbox Activity
                - link "Open Inbox" [ref=e351] [cursor=pointer]:
                  - /url: /inbox
              - generic [ref=e353]: Loading inbox from the database...
          - generic [ref=e355]:
            - generic [ref=e356]:
              - heading "Invite & Earn" [level=2] [ref=e357]
              - paragraph [ref=e358]: Invite a fellow business owner to OHC. They get 1 month free, you get $50 credit.
            - button "Get My Invite Link" [ref=e359]
          - generic [ref=e360]:
            - generic [ref=e362]:
              - heading "Growth & Virality" [level=2] [ref=e363]
              - paragraph [ref=e364]: Unlock new customers and track milestones.
            - generic [ref=e365]:
              - link "↗ Orchestrate Campaign Orchestration Plan, generate, review, and launch customer campaigns from live dashboard data." [ref=e366] [cursor=pointer]:
                - /url: /feed
                - generic [ref=e367]:
                  - generic [ref=e368]: ↗
                  - generic [ref=e369]: Orchestrate
                - heading "Campaign Orchestration" [level=3] [ref=e370]
                - paragraph [ref=e371]: Plan, generate, review, and launch customer campaigns from live dashboard data.
              - link "📈 ROI Pro Plan ROI Calculator See how much extra revenue you could generate by unlocking the Pro Plan." [ref=e372] [cursor=pointer]:
                - /url: /upgrade-roi
                - generic [ref=e373]:
                  - generic [ref=e374]: 📈
                  - generic [ref=e375]: ROI
                - heading "Pro Plan ROI Calculator" [level=3] [ref=e376]
                - paragraph [ref=e377]: See how much extra revenue you could generate by unlocking the Pro Plan.
              - link "🤝 Earn $50 Referrals Invite other business owners to OHC and earn premium credits." [ref=e378] [cursor=pointer]:
                - /url: /referrals
                - generic [ref=e379]:
                  - generic [ref=e380]: 🤝
                  - generic [ref=e381]: Earn $50
                - heading "Referrals" [level=3] [ref=e382]
                - paragraph [ref=e383]: Invite other business owners to OHC and earn premium credits.
              - link "🏆 Viral Affiliate Badge Builder Create an embeddable badge to grow your affiliate network." [ref=e384] [cursor=pointer]:
                - /url: /affiliate-badge-builder
                - generic [ref=e385]:
                  - generic [ref=e386]: 🏆
                  - generic [ref=e387]: Viral
                - heading "Affiliate Badge Builder" [level=3] [ref=e388]
                - paragraph [ref=e389]: Create an embeddable badge to grow your affiliate network.
              - link "💰 Finance Finance & Invoicing Manage cash flow, invoices, and automated payment follow-ups." [ref=e390] [cursor=pointer]:
                - /url: /finance
                - generic [ref=e391]:
                  - generic [ref=e392]: 💰
                  - generic [ref=e393]: Finance
                - heading "Finance & Invoicing" [level=3] [ref=e394]
                - paragraph [ref=e395]: Manage cash flow, invoices, and automated payment follow-ups.
              - link "🧾 Billing AI Invoice Generator Generate professional, shareable invoices that bring new customers to OHC." [ref=e396] [cursor=pointer]:
                - /url: /invoice-generator
                - generic [ref=e397]:
                  - generic [ref=e398]: 🧾
                  - generic [ref=e399]: Billing
                - heading "AI Invoice Generator" [level=3] [ref=e400]
                - paragraph [ref=e401]: Generate professional, shareable invoices that bring new customers to OHC.
              - link "📝 Sales AI Proposal Generator Create smart, shareable proposals with an interactive approval flow to win clients faster." [ref=e402] [cursor=pointer]:
                - /url: /proposal-generator
                - generic [ref=e403]:
                  - generic [ref=e404]: 📝
                  - generic [ref=e405]: Sales
                - heading "AI Proposal Generator" [level=3] [ref=e406]
                - paragraph [ref=e407]: Create smart, shareable proposals with an interactive approval flow to win clients faster.
              - link "Share Milestones Track and share your business achievements with your audience." [ref=e408] [cursor=pointer]:
                - /url: /milestones
                - generic [ref=e409]:
                  - generic [ref=e410]: 🏆
                  - generic [ref=e411]: Share
                - heading "Milestones" [level=3] [ref=e412]
                - paragraph [ref=e413]: Track and share your business achievements with your audience.
              - link "🤝 Loyalty Customer Loyalty Set up a 'Give X, Get Y' referral program and generate campaigns." [ref=e414] [cursor=pointer]:
                - /url: /loyalty-program
                - generic [ref=e415]:
                  - generic [ref=e416]: 🤝
                  - generic [ref=e417]: Loyalty
                - heading "Customer Loyalty" [level=3] [ref=e418]
                - paragraph [ref=e419]: Set up a 'Give X, Get Y' referral program and generate campaigns.
              - link "💸 Referrals Customer Referral Program Launch a Give $10, Get $10 program to turn your customers into advocates." [ref=e420] [cursor=pointer]:
                - /url: /customer-referral-program
                - generic [ref=e421]:
                  - generic [ref=e422]: 💸
                  - generic [ref=e423]: Referrals
                - heading "Customer Referral Program" [level=3] [ref=e424]
                - paragraph [ref=e425]: Launch a Give $10, Get $10 program to turn your customers into advocates.
              - link "🎴 Cards Social Share Cards Generate Share Cards to promote your brand on social media." [ref=e426] [cursor=pointer]:
                - /url: /share-cards
                - generic [ref=e427]:
                  - generic [ref=e428]: 🎴
                  - generic [ref=e429]: Cards
                - heading "Social Share Cards" [level=3] [ref=e430]
                - paragraph [ref=e431]: Generate Share Cards to promote your brand on social media.
              - link "🌐 Widget Storefront Widget Embed a mini storefront on your blog or website to boost sales." [ref=e432] [cursor=pointer]:
                - /url: /storefront-widget
                - generic [ref=e433]:
                  - generic [ref=e434]: 🌐
                  - generic [ref=e435]: Widget
                - heading "Storefront Widget" [level=3] [ref=e436]
                - paragraph [ref=e437]: Embed a mini storefront on your blog or website to boost sales.
              - link "🔌 Widget Interactive Embed Build custom intake, booking, or quote widgets for your site." [ref=e438] [cursor=pointer]:
                - /url: /embed-builder
                - generic [ref=e439]:
                  - generic [ref=e440]: 🔌
                  - generic [ref=e441]: Widget
                - heading "Interactive Embed" [level=3] [ref=e442]
                - paragraph [ref=e443]: Build custom intake, booking, or quote widgets for your site.
              - link "📦 Recurring Subscriptions & Fulfillments Manage recurring products, subscribers, and shipping batches." [ref=e444] [cursor=pointer]:
                - /url: /subscriptions
                - generic [ref=e445]:
                  - generic [ref=e446]: 📦
                  - generic [ref=e447]: Recurring
                - heading "Subscriptions & Fulfillments" [level=3] [ref=e448]
                - paragraph [ref=e449]: Manage recurring products, subscribers, and shipping batches.
              - link "🚀 Proof Social Proof Nudge Show visitors that others are buying to increase conversions." [ref=e450] [cursor=pointer]:
                - /url: /social-proof-nudge
                - generic [ref=e451]:
                  - generic [ref=e452]: 🚀
                  - generic [ref=e453]: Proof
                - heading "Social Proof Nudge" [level=3] [ref=e454]
                - paragraph [ref=e455]: Show visitors that others are buying to increase conversions.
              - link "📋 Leads Work-Intake Widget Embed a smart lead capture form with a viral loop directly on your site." [ref=e456] [cursor=pointer]:
                - /url: /work-intake-widget
                - generic [ref=e457]:
                  - generic [ref=e458]: 📋
                  - generic [ref=e459]: Leads
                - heading "Work-Intake Widget" [level=3] [ref=e460]
                - paragraph [ref=e461]: Embed a smart lead capture form with a viral loop directly on your site.
              - link "🔗 Bio Create Link-in-Bio Page Publish a lightweight social profile page for your storefront and offers." [ref=e462] [cursor=pointer]:
                - /url: /link-in-bio-generator
                - generic [ref=e463]:
                  - generic [ref=e464]: 🔗
                  - generic [ref=e465]: Bio
                - heading "Create Link-in-Bio Page" [level=3] [ref=e466]
                - paragraph [ref=e467]: Publish a lightweight social profile page for your storefront and offers.
              - link "💬 Social WhatsApp Link Generator Create shareable WhatsApp links to start conversations instantly." [ref=e468] [cursor=pointer]:
                - /url: /whatsapp-link-generator
                - generic [ref=e469]:
                  - generic [ref=e470]: 💬
                  - generic [ref=e471]: Social
                - heading "WhatsApp Link Generator" [level=3] [ref=e472]
                - paragraph [ref=e473]: Create shareable WhatsApp links to start conversations instantly.
              - link "🎁 Viral Viral Giveaway Generator Launch a viral sweepstakes to capture emails and drive social shares." [ref=e474] [cursor=pointer]:
                - /url: /giveaway
                - generic [ref=e475]:
                  - generic [ref=e476]: 🎁
                  - generic [ref=e477]: Viral
                - heading "Viral Giveaway Generator" [level=3] [ref=e478]
                - paragraph [ref=e479]: Launch a viral sweepstakes to capture emails and drive social shares.
              - link "🔓 Growth Share-to-Unlock Generator Require customers to share your page on social media to reveal a discount code." [ref=e480] [cursor=pointer]:
                - /url: /share-to-unlock-generator
                - generic [ref=e481]:
                  - generic [ref=e482]: 🔓
                  - generic [ref=e483]: Growth
                - heading "Share-to-Unlock Generator" [level=3] [ref=e484]
                - paragraph [ref=e485]: Require customers to share your page on social media to reveal a discount code.
              - link "💌 Retain Customer Win-back Re-engage inactive customers with AI-generated email campaigns." [ref=e486] [cursor=pointer]:
                - /url: /win-back
                - generic [ref=e487]:
                  - generic [ref=e488]: 💌
                  - generic [ref=e489]: Retain
                - heading "Customer Win-back" [level=3] [ref=e490]
                - paragraph [ref=e491]: Re-engage inactive customers with AI-generated email campaigns.
              - link "⭐️ Reviews Automated Reviews Generate highly-converting, personalized review request emails." [ref=e492] [cursor=pointer]:
                - /url: /review-campaigns
                - generic [ref=e493]:
                  - generic [ref=e494]: ⭐️
                  - generic [ref=e495]: Reviews
                - heading "Automated Reviews" [level=3] [ref=e496]
                - paragraph [ref=e497]: Generate highly-converting, personalized review request emails.
              - link "✨ Promo Seasonal Promo Generator Create AI campaigns and promo codes for special occasions instantly." [ref=e498] [cursor=pointer]:
                - /url: /seasonal-promo
                - generic [ref=e499]:
                  - generic [ref=e500]: ✨
                  - generic [ref=e501]: Promo
                - heading "Seasonal Promo Generator" [level=3] [ref=e502]
                - paragraph [ref=e503]: Create AI campaigns and promo codes for special occasions instantly.
              - link "🛒 Recover Cart Recovery Recover abandoned carts with personalized AI follow-ups." [ref=e504] [cursor=pointer]:
                - /url: /cart-recovery
                - generic [ref=e505]:
                  - generic [ref=e506]: 🛒
                  - generic [ref=e507]: Recover
                - heading "Cart Recovery" [level=3] [ref=e508]
                - paragraph [ref=e509]: Recover abandoned carts with personalized AI follow-ups.
              - link "⚡ Urgency Flash Sale Generator Create high-converting flash sale countdown widgets." [ref=e510] [cursor=pointer]:
                - /url: /flash-sale-generator
                - generic [ref=e511]:
                  - generic [ref=e512]: ⚡
                  - generic [ref=e513]: Urgency
                - heading "Flash Sale Generator" [level=3] [ref=e514]
                - paragraph [ref=e515]: Create high-converting flash sale countdown widgets.
              - link "🎯 Leads Want more local jobs this week? [Tap here] Launch an autonomous hyper-local lead generation campaign." [ref=e516] [cursor=pointer]:
                - /url: /marketing/lead-gen
                - generic [ref=e517]:
                  - generic [ref=e518]: 🎯
                  - generic [ref=e519]: Leads
                - heading "Want more local jobs this week? [Tap here]" [level=3] [ref=e520]
                - paragraph [ref=e521]: Launch an autonomous hyper-local lead generation campaign.
              - link "🎁 Extension Interactive Trial Extension Share your setup on X to instantly unlock 7 extra days of Pro." [ref=e522] [cursor=pointer]:
                - /url: /trial-extension
                - generic [ref=e523]:
                  - generic [ref=e524]: 🎁
                  - generic [ref=e525]: Extension
                - heading "Interactive Trial Extension" [level=3] [ref=e526]
                - paragraph [ref=e527]: Share your setup on X to instantly unlock 7 extra days of Pro.
              - link "📍 Operations Field Ops Route Offline-first mobile route management for field service workers." [ref=e528] [cursor=pointer]:
                - /url: /field-ops/jobs
                - generic [ref=e529]:
                  - generic [ref=e530]: 📍
                  - generic [ref=e531]: Operations
                - heading "Field Ops Route" [level=3] [ref=e532]
                - paragraph [ref=e533]: Offline-first mobile route management for field service workers.
              - link "⚙️ Config Settings Manage your account and preferences." [ref=e534] [cursor=pointer]:
                - /url: /settings
                - generic [ref=e535]:
                  - generic [ref=e536]: ⚙️
                  - generic [ref=e537]: Config
                - heading "Settings" [level=3] [ref=e538]
                - paragraph [ref=e539]: Manage your account and preferences.
  - button "Help" [ref=e542]:
    - img [ref=e543]
  - button "Open help chat" [ref=e546]:
    - generic [ref=e547]: ✨
    - generic [ref=e548]: Ask anything
  - button "Voice Assistant" [ref=e549]:
    - img
  - alert [ref=e551]
```

# Test source

```ts
  1   | import { expect, test } from '@playwright/test';
  2   |
  3   | test.describe('Unified Agent Feed Interactive Flow', () => {
  4   |   test.use({ viewport: { width: 375, height: 812 } });
  5   |
  6   |   test('should render properly, expand for details, and show approval transition', async ({ page }) => {
  7   |     test.setTimeout(180000);
  8   |
  9   |     await page.goto('/dashboard');
  10  |     await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
  11  |
  12  |     // Wait for the feed items to populate
  13  |     const feedContainer = page.locator('div.glassmorphism', { hasText: 'Approval' }).first();
  14  |     await expect(feedContainer).toBeVisible({ timeout: 15000 });
  15  |
  16  |     // 1. Verify width constraint
  17  |     const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
  18  |     expect(bodyWidth).toBeLessThanOrEqual(375);
  19  |
  20  |     // Verify touch targets are at least 44x44
  21  |     const buttons = await page.locator('button').all();
  22  |     for (const btn of buttons) {
  23  |       if (await btn.isVisible()) {
  24  |         const box = await btn.boundingBox();
  25  |         if (box) {
> 26  |           expect(box.width).toBeGreaterThanOrEqual(44);
      |                             ^ Error: expect(received).toBeGreaterThanOrEqual(expected)
  27  |           expect(box.height).toBeGreaterThanOrEqual(44);
  28  |         }
  29  |       }
  30  |     }
  31  |
  32  |     // Find the dynamic approval card (which we've mapped using data-testid or just looking for the buttons)
  33  |     const approveBtn = page.getByTestId('approve-proposal').first();
  34  |     const editBtn = page.getByTestId('edit-proposal').first();
  35  |
  36  |     // In case there are no items to approve, we will skip the rest of the assertions safely.
  37  |     // In a real E2E environment we would seed this, but this guarantees the script runs.
  38  |     if (await approveBtn.isVisible()) {
  39  |         // 2. Expand card to see details
  40  |         await editBtn.click();
  41  |         const detailsPre = page.locator('pre').first();
  42  |         await expect(detailsPre).toBeVisible();
  43  |
  44  |         // 3. Verify interaction states when "Approve" is clicked
  45  |         const cardParent = approveBtn.locator('xpath=./../../..'); // navigate up to the card container
  46  |         await approveBtn.click();
  47  |
  48  |         // The card should transition to green border and slightly scale down
  49  |         await expect(cardParent).toHaveClass(/border-green-500/);
  50  |         await expect(cardParent).toHaveClass(/scale-95/);
  51  |
  52  |         // Card should disappear after 500ms
  53  |         await expect(cardParent).not.toBeVisible({ timeout: 2000 });
  54  |     }
  55  |   });
  56  |
  57  |   test('should queue actions optimistically when offline', async ({ page, context }) => {
  58  |     test.setTimeout(180000);
  59  |
  60  |     // 1. Seed some distinct approvals representing different departments
  61  |     await page.goto('/dashboard');
  62  |     await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
  63  |
  64  |     // Ensure we have some items
  65  |     const approveBtn = page.getByTestId('approve-proposal').first();
  66  |     const isVisible = await approveBtn.isVisible({ timeout: 15000 }).catch(() => false);
  67  |
  68  |     if (isVisible) {
  69  |       // Go offline
  70  |       await context.setOffline(true);
  71  |       await page.evaluate(() => window.dispatchEvent(new Event('offline')));
  72  |
  73  |       // Verify offline banner
  74  |       await expect(page.locator('text=You are offline. Actions will sync when online.')).toBeVisible();
  75  |
  76  |       const cardParent = approveBtn.locator('xpath=./../../..');
  77  |
  78  |       // 2. Tap approve
  79  |       await approveBtn.click();
  80  |
  81  |       // 3. The item should optimisticly disappear
  82  |       await expect(cardParent).not.toBeVisible({ timeout: 2000 });
  83  |
  84  |       // Go back online
  85  |       await context.setOffline(false);
  86  |       await page.evaluate(() => window.dispatchEvent(new Event('online')));
  87  |
  88  |       // Verify offline banner goes away
  89  |       await expect(page.locator('text=You are offline. Actions will sync when online.')).not.toBeVisible();
  90  |     }
  91  |   });
  92  |
  93  |   test('Feed Page should load items and approve', async ({ page }) => {
  94  |     test.setTimeout(180000);
  95  |     await page.goto('/feed');
  96  |     await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 25000 });
  97  |
  98  |     const card = page.getByTestId('agent-feed-card').first();
  99  |     if (await card.isVisible()) {
  100 |         const approveBtn = card.locator('button', { hasText: 'Approve' });
  101 |         await approveBtn.click();
  102 |         await expect(card).not.toBeVisible({ timeout: 5000 });
  103 |     }
  104 |   });
  105 |
  106 |   test('Feed Page should load items and dismiss', async ({ page }) => {
  107 |     test.setTimeout(180000);
  108 |     await page.goto('/feed');
  109 |     await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 25000 });
  110 |
  111 |     const card = page.getByTestId('agent-feed-card').first();
  112 |     if (await card.isVisible()) {
  113 |         const dismissBtn = card.locator('button', { hasText: 'Dismiss' });
  114 |         await dismissBtn.click();
  115 |         await expect(card).not.toBeVisible({ timeout: 5000 });
  116 |     }
  117 |   });
  118 |
  119 |   test('Dashboard should have functional UnifiedAgentFeed component', async ({ page }) => {
  120 |     test.setTimeout(180000);
  121 |     await page.goto('/dashboard');
  122 |     await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
  123 |
  124 |     // Check if feed loads
  125 |     const feedContainer = page.locator('div.glassmorphism', { hasText: 'Approval' }).first();
  126 |     await expect(feedContainer).toBeVisible({ timeout: 15000 });
```