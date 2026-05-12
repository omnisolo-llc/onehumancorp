# Business Journey Architecture

## Overview
This document outlines the complete end-to-end user journey for each persona in the OneHumanCorp platform. The focus is to design the core flows from a non-technical small business owner's perspective, mapping how they acquire customers, onboard, activate, retain, generate revenue, and trigger viral referrals.

## Personas & Journey Diagrams

### 1. Maya (Baker, 28)
**Context:** Sells custom cakes via Instagram DMs. Needs a beautiful storefront with a photo catalog, deposit-based custom orders, and an AI agent to handle DM inquiries. Runs everything from an iPhone.

**Acquisition:** Discovers OHC via an Instagram ad highlighting seamless DM-to-storefront conversion.
**Onboarding:** Downloads the app, creates an account with Apple/Google, links her Instagram, and uploads cake photos. The AI agent auto-generates titles, descriptions, and a storefront layout based on the photos.
**Activation:** Custom order form is published. First customer pays a deposit via the form link in her Instagram bio. Success is measured by having the storefront live and receiving the first deposit.
**Retention:** Push notifications on new orders, AI agent summarizing daily DM inquiries handled, and weekly revenue reports.
**Revenue:** Upgrades from Free to Starter tier when she exceeds 10 products or needs a custom domain.
**Referral:** Word of mouth to other bakers in her local community, sharing her beautiful, high-converting storefront link.

```mermaid
sequenceDiagram
    participant Customer
    participant Maya (iPhone)
    participant OHC App
    participant AI Agent (Operations/Marketing)

    Maya (iPhone)->>OHC App: Downloads App & Registers
    Maya (iPhone)->>OHC App: Links Instagram & Uploads Photos
    OHC App->>AI Agent (Marketing): Generate product descriptions & storefront layout
    AI Agent (Marketing)-->>OHC App: Returns generated content
    Maya (iPhone)->>OHC App: Reviews and Publishes Storefront
    Note over Maya (iPhone), OHC App: Storefront is live (under 10 mins)
    Customer->>Maya (iPhone): Sends DM on Instagram "Do you do vegan cakes?"
    OHC App->>AI Agent (Operations): Intercepts DM & checks constraints
    AI Agent (Operations)-->>Customer: Replies "Yes! Here is the order form: [Link]"
    Customer->>OHC App: Opens link, fills custom order form
    Customer->>OHC App: Pays deposit
    OHC App-->>Maya (iPhone): Push Notification: New Order & Deposit Received!
```

### 2. Carlos (Handyman, 42)
**Context:** No website, relies on word of mouth. Needs service listings, a booking calendar with deposit payments, a customer inbox, and an AI quote generator. Uses an Android phone only.

**Acquisition:** Discovers OHC via organic search or referral when looking for a "simple scheduling app for contractors."
**Onboarding:** Downloads app, creates account, enters business name and typical services. AI suggests standard pricing and descriptions based on local averages. Sets up his availability in the calendar.
**Activation:** Sends first booking link via SMS to a recurring client. Success is securing the first deposit-backed appointment.
**Retention:** Daily schedule push notifications, simple inbox for client messages, AI agent drafting quotes for new requests.
**Revenue:** Upgrades to Starter when he needs to manage more than a few bookings a week or wants a custom domain to look more professional.
**Referral:** Mentions the easy booking system to other tradespeople he collaborates with on job sites.

```mermaid
sequenceDiagram
    participant Client
    participant Carlos (Android)
    participant OHC App
    participant AI Agent (Sales/Operations)

    Carlos (Android)->>OHC App: Downloads App & Registers
    Carlos (Android)->>OHC App: Enters business details & services
    OHC App->>AI Agent (Sales): Suggests standard pricing & service descriptions
    AI Agent (Sales)-->>Carlos (Android): Reviews & accepts suggestions
    Carlos (Android)->>OHC App: Connects calendar & sets availability
    Note over Carlos (Android), OHC App: Service booking page is live
    Client->>Carlos (Android): SMS "Can you fix a leaky pipe?"
    Carlos (Android)->>OHC App: Triggers AI Quote Generator
    OHC App->>AI Agent (Sales): Drafts quote based on description
    AI Agent (Sales)-->>Carlos (Android): Reviews & sends quote link to Client
    Client->>OHC App: Opens link, selects time, pays deposit
    OHC App-->>Carlos (Android): Push Notification: New Booking & Deposit Received!
```

### 3. Priya (Boutique Owner, 35)
**Context:** Sells clothing in-store and wants to go online. Needs storefront + inventory sync, product variants, in-person tap-to-pay, email newsletters, and daily mobile analytics.

**Acquisition:** Discovers OHC via a YouTube tutorial or blog post about moving a brick-and-mortar store online easily.
**Onboarding:** Creates account (likely on iPad/Desktop then uses phone). Imports basic inventory list. Sets up tap-to-pay on her phone.
**Activation:** Processes first in-person sale via tap-to-pay, inventory auto-syncs. First online order is received.
**Retention:** Daily mobile analytics dashboards, AI agent suggesting newsletter topics based on low inventory or seasonal trends.
**Revenue:** Quickly upgrades to Pro for unlimited products and advanced analytics, given her existing physical inventory scale.
**Referral:** Shares her success in Facebook groups for small boutique owners, highlighting the easy POS + online sync.

```mermaid
sequenceDiagram
    participant In-Store Customer
    participant Online Customer
    participant Priya (Mobile/Tablet)
    participant OHC App
    participant AI Agent (Marketing/Operations)

    Priya (Mobile/Tablet)->>OHC App: Registers & Imports Inventory
    Priya (Mobile/Tablet)->>OHC App: Enables Tap-to-Pay
    Note over Priya (Mobile/Tablet), OHC App: Online store & POS are ready
    In-Store Customer->>Priya (Mobile/Tablet): Buys item in store
    Priya (Mobile/Tablet)->>OHC App: Processes payment via Tap-to-Pay
    OHC App->>OHC App: Auto-updates unified inventory
    OHC App->>AI Agent (Marketing): Detects low inventory on an item
    AI Agent (Marketing)-->>Priya (Mobile/Tablet): Suggests "Flash Sale" newsletter for overstocked items
    Online Customer->>OHC App: Browses online store, adds to cart, purchases
    OHC App-->>Priya (Mobile/Tablet): Push Notification: New Online Order!
```

### 4. Leo (Music Tutor, 22)
**Context:** Teaches online + in-person. Needs lesson booking with calendar sync, auto-generated meeting links, subscription lesson packages, AI follow-up for inactive students, and a portfolio page.

**Acquisition:** Finds OHC via a TikTok showing "how to set up a tutoring business link-in-bio."
**Onboarding:** Sets up account, creates a portfolio page with YouTube video embeds of his playing, sets up calendar integration (Zoom/Google Meet), and creates subscription packages.
**Activation:** A student books a trial lesson via his link-in-bio.
**Retention:** Automated reminders to students, AI agent sending follow-up emails to students who haven't booked in a month.
**Revenue:** Upgrades from Free to Starter when he launches recurring subscription packages (which may be a premium feature).
**Referral:** Adds a "Powered by OHC" badge to his link-in-bio, driving other tutors/creators to the platform.

```mermaid
sequenceDiagram
    participant Student
    participant Leo (Mobile/Web)
    participant OHC App
    participant AI Agent (Customer Success)

    Leo (Mobile/Web)->>OHC App: Registers & creates Portfolio/Link-in-bio
    Leo (Mobile/Web)->>OHC App: Sets up calendar & Zoom integration
    Leo (Mobile/Web)->>OHC App: Creates subscription packages
    Note over Leo (Mobile/Web), OHC App: Booking page is live in bio
    Student->>OHC App: Clicks link in bio, books trial lesson
    OHC App->>OHC App: Generates Zoom link & sends invites
    OHC App-->>Leo (Mobile/Web): Notification: New trial lesson booked!
    Note over Student, OHC App: 1 month later, student hasn't booked again
    OHC App->>AI Agent (Customer Success): Identifies inactive student
    AI Agent (Customer Success)-->>Student: Sends friendly email: "Ready for your next lesson? Here's 10% off."
```

### 5. Fatima (Food Cart, 50)
**Context:** Takes halal food pre-orders. Needs a photo menu with sold-out toggles, pre-order/pickup with payment, phone notification on new orders, printable daily order lists, Arabic + English UI. Works on a low-end Android.

**Acquisition:** Community referral. Another local food vendor helps her set it up.
**Onboarding:** Very assisted onboarding. Uses Arabic UI. Takes photos of her dishes. AI translates descriptions to English automatically to create a bilingual menu.
**Activation:** Receives first pre-order for the next day. Prints the daily order list.
**Retention:** The app becomes essential to her daily operation (printing the order list every morning). Push notifications are loud and clear for immediate pickup orders.
**Revenue:** Remains on the Free or Starter tier. Value is derived from transaction fees or a low monthly fee for the ordering system.
**Referral:** High density referral—other food carts in the same plaza see her using it and want the same system.

```mermaid
sequenceDiagram
    participant Customer
    participant Fatima (Low-end Android)
    participant OHC App
    participant AI Agent (Operations/Translation)

    Fatima (Low-end Android)->>OHC App: Registers (Arabic UI)
    Fatima (Low-end Android)->>OHC App: Uploads dish photos
    OHC App->>AI Agent (Translation): Translates titles/descriptions to English
    AI Agent (Translation)-->>OHC App: Creates bilingual menu
    Note over Fatima (Low-end Android), OHC App: Menu is live for pre-orders
    Customer->>OHC App: Browses menu in English, places pre-order for tomorrow
    Customer->>OHC App: Pays online
    OHC App-->>Fatima (Low-end Android): Loud Push Notification: New Pre-Order!
    Fatima (Low-end Android)->>OHC App: (Next Morning) Generates daily order list
    OHC App-->>Fatima (Low-end Android): Displays simple, printable list of all pre-orders
```

## Architectural Friction Points to Avoid
- **Mandatory Desktop Setup:** Onboarding must be completable entirely on a mobile device (even low-end).
- **Complex Typography/Design Choices:** Users shouldn't need to choose fonts or padding. The platform enforces the premium design system (Outfit/Inter, Glassmorphism).
- **Manual AI Prompting:** Users should never see an empty text box asking them to "Prompt the AI." AI actions should be contextually suggested (e.g., "Draft a reply," "Generate description from photo").
- **Payment Gateway Complexity:** Stripe/payment onboarding must be abstracted or simplified to the absolute minimum required fields to start accepting money.

## Conclusion
The architecture must prioritize extreme simplicity, mobile-first operations, and invisible AI orchestration. The user should feel like they have a staff of experts working for them, rather than a complex software tool they need to learn.