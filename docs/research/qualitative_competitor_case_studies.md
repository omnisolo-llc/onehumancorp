# Qualitative Competitor Case Studies

## Introduction
To supplement our macro-level competitive audit, we conducted deep-dive qualitative case studies of three hypothetical but highly representative OHC personas attempting to launch their businesses on leading competitor platforms. These narratives highlight the exact moments of friction that cause churn.

## Case Study 1: Maya (Baker) vs. Shopify
**Goal:** Sell 20 custom cakes per week locally. No shipping.
**Platform Chosen:** Shopify (Basic Plan, $39/mo)

### The Journey
1. **Account Creation (Minutes 1-10):** Smooth. Shopify asks standard questions. Maya feels optimistic.
2. **Theme Selection (Minutes 10-30):** Maya struggles to find a theme that looks "warm and inviting" for a bakery. Most free themes look like drop-shipping tech stores. She settles for 'Dawn'.
3. **Product Creation (Minutes 30-90):** Maya uploads photos of her cakes. She realizes she needs different variants (Size: 6-inch, 8-inch; Flavor: Vanilla, Chocolate). Setting up variants in Shopify's matrix is confusing. She spends 20 minutes Googling "how to add cake flavors Shopify".
4. **The Critical Failure: Shipping/Delivery (Hours 2-4):** Maya *only* does local delivery. She goes to Settings -> Shipping and Delivery. Shopify assumes she wants to ship via USPS. She deletes the general shipping rates. She tries to set up Local Delivery. Shopify asks for a delivery radius. Maya prefers to deliver only to specific zip codes because a 5-mile radius crosses a bridge she hates driving over. Setting up zip code specific delivery rules requires a third-party app.
5. **The "App Store Tax" (Hour 5):** Maya searches the app store for "Zip code local delivery". The top app costs $15/month. She installs it. The app interface looks completely different from Shopify. She is overwhelmed and abandons the setup.
**Result:** Churn.

### OHC Solution Matrix
- **Theme:** AI auto-generates a bakery-specific layout based on her initial prompt.
- **Product:** AI extracts variants from a single photo and short description.
- **Delivery:** Native local delivery primitive that asks "Which zip codes do you deliver to?" in plain English during onboarding. No apps required.

## Case Study 2: Carlos (Handyman) vs. Wix
**Goal:** Get a professional online presence to stop handing out paper business cards. Needs people to request a quote.
**Platform Chosen:** Wix

### The Journey
1. **Account Creation & ADI (Minutes 1-15):** Carlos uses Wix ADI. He answers questions and gets a decent-looking website quickly. He is impressed.
2. **Mobile Editing (Minutes 15-45):** Carlos looks at the site on his phone. The text on the hero image is hard to read. He tries to edit it using the Wix Owner App. He can change the text, but he cannot change the contrast or font size easily. He has to wait until he gets home to his laptop.
3. **The Critical Failure: Booking/Quoting (Hours 1-3):** Carlos wants a form where people can upload a photo of a broken pipe and request a quote. He adds a Wix Form. It sends an email. However, Carlos rarely checks email while working; he prefers text messages.
4. **The Disconnect:** A customer fills out the form. It goes to Carlos's email. He sees it 8 hours later. He emails back a quote. The customer has already hired someone else who answered their phone.
**Result:** The website becomes a static, dead asset. It provides zero operational value.

### OHC Solution Matrix
- **Mobile First:** 100% of editing can be done on the phone.
- **Communications:** Native SMS routing. When a customer submits a quote request, the OHC Agent texts Carlos: "New job request: Broken Pipe. Photo attached. Reply with quote amount or type 'Call'." Carlos replies "$150". The Agent generates an invoice link and texts it to the customer.

## Case Study 3: Priya (Boutique) vs. Squarespace
**Goal:** Expand her physical retail store online. Maintain unified inventory.
**Platform Chosen:** Squarespace

### The Journey
1. **Aesthetics (Hours 1-5):** Priya loves the design process. The site looks beautiful and matches her brand perfectly.
2. **Product Upload (Hours 5-10):** She manually uploads 50 items. It is tedious but manageable.
3. **The Critical Failure: Inventory Sync (Ongoing Nightmare):** Priya uses Square for her physical POS. She realizes Squarespace does not natively sync bi-directionally with Square POS for inventory.
4. **The Workaround:** She has to manually adjust the stock level on Squarespace every time someone buys a shirt in her physical store.
5. **The Breaking Point:** On a busy Saturday, she sells the last medium blue shirt in-store but forgets to update the website. Someone buys it online an hour later. She has to email the online customer to cancel the order, resulting in a bad review.
**Result:** Priya unpublishes the online store because the operational risk of overselling is too high.

### OHC Solution Matrix
- **Native Sync:** OHC integrates bi-directionally with major physical POS systems (Square, Clover) out of the box. The database acts as the single source of truth.

## Summary of Case Studies
The common thread across all failures is not the *inability to build a website*. It is the **failure of the platform to map to the actual operational realities of the small business owner.** Legacy platforms build e-commerce software; OHC must build business operations software.
