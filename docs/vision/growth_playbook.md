# OHC Small Business Growth Playbook

## 1. Referral Program
**"Share OHC with a friend, both get 1 month free Pro."**

The referral loop is our most powerful organic acquisition channel. Business owners who succeed on OHC naturally want to share their secret.
* **Mechanics:**
  * One-tap share from the mobile app (link + pre-filled message).
  * Dashboard tracks invites sent, pending, and converted.
  * Credits are automatically applied upon conversion.
* **Architecture (`src/server/services/growth/referrals.rs`):** Uses an in-memory `ReferralTracker` to assign hex-encoded codes to users, track usage by channel (e.g., 'twitter'), and assign tiers (Bronze to Platinum).

## 2. Business Share & Embed
Every OHC business gets a beautifully designed shareable link card.
* **Implementation:** The "Share my business" button on the growth dashboard generates a link that is optimized for social sharing (OpenGraph previews including logo, name, and tagline).
* **Cross-posting:** Native integrations to post directly to Instagram/WhatsApp/X.

## 3. Social Media Auto-Posting
An AI agent feature where the business owner connects Instagram, Facebook, or X.
* **Workflow:**
  1. Agent detects a new product, sale, or milestone.
  2. Agent auto-generates a post and schedules it.
  3. Owner approves posts from their phone with one tap.

## 4. Email Marketing (Simple)
Built-in email campaign tool for business owners.
* **Features:** Select contacts, pick an AI-generated template ("New arrivals", "Flash sale", "Thank you"), preview, and send. Track open rates.

## 5. Free Tier & Upgrade Funnel
We design a compelling free tier that naturally leads to Starter/Pro upgrades.
* **Limits:** 1 AI agent, 10 products, OHC subdomain.
* **Paywalls:** Soft paywalls with clear upgrade CTAs appear when limits are reached, explaining the value of upgrading.

## 6. Viral Storefront
* **Mechanism:** Every OHC business's public storefront automatically includes a subtle "Built with OHC — Start your free business →" link in the footer.
* **Impact:** This creates a continuous free-tier viral loop as businesses drive traffic to their own stores.

## 7. Success Milestones
Push notifications that celebrate business owner milestones.
* **Examples:** "🎉 You just got your 10th order!", "🚀 Your store has 100 visitors today!"
* **Philosophy:** These should feel like wins and celebrations, not just dry metrics.
