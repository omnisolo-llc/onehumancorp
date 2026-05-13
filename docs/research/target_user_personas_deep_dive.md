# Target User Personas & Journey Mapping

## Introduction
To ensure our engineering and design decisions are deeply rooted in user empathy, this document provides comprehensive profiles of our five core target personas. It expands upon the initial summaries to detail their psychographics, technical proficiency, and specific failure modes with existing technology.

## Persona 1: Maya, The "Side-Hustle Creator"
**Age:** 28 | **Occupation:** Specialty Baker | **Location:** Urban (Chicago, IL)
**Primary Platform:** Instagram
**Technical Proficiency:** High social media literacy; low traditional IT literacy.

### Psychographics & Motivations
- **Motivation:** Maya started baking for friends. Word of mouth spread. She wants to monetize her passion without it feeling like a "corporate job."
- **Anxiety:** She is terrified of the business side—taxes, incorporating, and managing complex inventory. She just wants to bake and share photos.
- **Current Tech Stack:** Instagram for marketing and DMs. Venmo/Zelle for payments. Apple Notes for keeping track of orders.

### The "Job to be Done" (JTBD)
- "Help me turn my Instagram followers into paying customers without making me manage a complex website."

### OHC Success Scenario
- Maya uses the OHC app to connect her Instagram.
- The **Architect Agent** reads her bio and creates a simple storefront.
- When a follower DMs her "How much for a dozen cupcakes?", the **Support Agent** auto-replies: "They are $35! We currently have slots open for this weekend. You can order here: [OHC Link]".
- Maya never leaves her kitchen; her phone manages the sales funnel.

---

## Persona 2: Carlos, The "Analog Service Provider"
**Age:** 42 | **Occupation:** Handyman / Contractor | **Location:** Suburban (Dallas, TX)
**Primary Platform:** Word of Mouth / Phone Calls
**Technical Proficiency:** Low. Uses a smartphone for calls, texts, and basic web browsing. Hates typing on small keyboards.

### Psychographics & Motivations
- **Motivation:** Wants to grow his business to hire a helper, but he is constantly bottlenecked by his inability to manage the admin work while on a job site.
- **Anxiety:** Missed calls equal lost revenue. He feels guilty when he takes a day to reply to a quote request. He finds traditional software completely alienating.
- **Current Tech Stack:** Cellular Phone, SMS, Paper invoices, Cash/Check.

### The "Job to be Done" (JTBD)
- "Give me a professional front door that handles inquiries when I am under a sink, and let me manage it via text."

### OHC Success Scenario
- Carlos's OHC site has a simple "Request a Quote" form.
- A customer uploads a photo of a broken fence.
- The OHC **Operations Agent** texts Carlos the photo and asks, "What's the quote for this repair?"
- Carlos replies with a simple text: "$250".
- The Agent instantly formats a professional PDF quote, emails/texts it to the customer, and handles the deposit payment. Carlos never opened an app.

---

## Persona 3: Priya, The "Hybrid Retailer"
**Age:** 35 | **Occupation:** Boutique Owner | **Location:** High-Street Retail (London, UK)
**Primary Platform:** Physical Storefront
**Technical Proficiency:** Medium. Comfortable with modern POS systems but intimidated by setting up complex digital integrations.

### Psychographics & Motivations
- **Motivation:** She pays high rent for her physical store and needs an online presence to supplement revenue, especially during off-seasons.
- **Anxiety:** Overselling. She is terrified of selling a dress online that someone just bought in her physical store because the inventory systems didn't sync fast enough.
- **Current Tech Stack:** Square POS in-store, Mailchimp (rarely used).

### The "Job to be Done" (JTBD)
- "Give me an online store that perfectly mirrors my physical store's inventory in real-time, without requiring manual data entry."

### OHC Success Scenario
- Priya authorizes OHC to connect to her Square POS.
- The **Architect Agent** instantly builds her online store, importing all 500 items, images, and current stock levels.
- The **Growth Agent** notices she has 50 winter coats left in March. It sends Priya a push notification: "Shall I run a 20% off End of Season sale online for the winter coats?" Priya taps "Approve."

---

## Persona 4: Leo, The "Subscription Tutor"
**Age:** 22 | **Occupation:** Music Tutor | **Location:** Remote / Online
**Primary Platform:** Zoom / Google Meet
**Technical Proficiency:** High. Very comfortable with digital tools.

### Psychographics & Motivations
- **Motivation:** Leo wants predictable, recurring income. He hates chasing students for payments.
- **Anxiety:** The awkward conversation of asking a student to pay their past-due invoice before starting the lesson.
- **Current Tech Stack:** Calendly for booking, PayPal for manual invoicing, Zoom for delivery.

### The "Job to be Done" (JTBD)
- "Automate my scheduling and make sure I get paid *before* I show up."

### OHC Success Scenario
- Leo sets up his OHC profile with a "Subscription" service type.
- Students subscribe for $100/month (4 lessons).
- OHC handles the recurring billing.
- The **Scheduling Agent** automatically integrates with Leo's Google Calendar and sends students their Zoom links. If a payment fails, the Agent automatically handles the dunning process (retry emails) and cancels the upcoming Zoom meeting until payment is resolved.

---

## Persona 5: Fatima, The "High-Volume / Low-Tech Operator"
**Age:** 50 | **Occupation:** Food Cart Operator | **Location:** Urban (New York City)
**Primary Platform:** Walk-ups
**Technical Proficiency:** Very Low. English is her second language. Relies heavily on family members for technical help.

### Psychographics & Motivations
- **Motivation:** Efficiency. During the lunch rush, she processes 100 orders an hour. She wants to accept pre-orders to increase volume but cannot manage a complex tablet interface while cooking.
- **Anxiety:** Technology slowing her down. Language barriers in complex SaaS dashboards.
- **Current Tech Stack:** Cash register, maybe a basic Zettle/Square terminal for card swipes.

### The "Job to be Done" (JTBD)
- "Let my regular customers pre-order so I can prep their food, and notify me in a way that doesn't interrupt my cooking."

### OHC Success Scenario
- The OHC interface is localized natively into Arabic/Spanish.
- Customers pre-order via a simple web link or WhatsApp message.
- Instead of requiring Fatima to look at a tablet, OHC integrates with a cheap $30 Bluetooth receipt printer.
- When an order is placed and paid for online, the printer simply spits out a ticket: "Order #42: Chicken over rice, no white sauce. Pickup at 12:15." Fatima cooks the ticket.
