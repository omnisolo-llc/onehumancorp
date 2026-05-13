# OHC Visual Excellence & Premium Design Standards (Q4 2024)

## 1. The Visual Excellence Mandate

In the Small Business Platform market, trust is often established entirely through visual design. A platform that looks dated, cluttered, or cheap will not be trusted to handle a business's revenue or customer data. OHC's visual design must communicate premium quality, cutting-edge technology, and effortless simplicity.

This document outlines the core design tokens and principles that constitute the OHC Visual Excellence Mandate, specifically tailored to appeal to our target personas (Maya, Carlos, Priya, Leo, Fatima) while maintaining an enterprise-grade aesthetic.

## 2. Core Design Tokens & Principles

The OHC design system is built on a foundation of "Glassmorphism," precise typography, and strict mobile-first constraints.

### 2.1 Glassmorphism & Depth
OHC eschews flat design for a layered, depth-oriented approach that feels modern and lightweight.
*   **Backdrop Filter:** `backdrop-filter: blur(20px) saturate(200%);` is the standard for modals, floating action bars, and sticky headers. This ensures context is never lost, even when menus are open.
*   **Backgrounds:** Semi-transparent white or dark modes (e.g., `rgba(255, 255, 255, 0.7)`) overlaid on subtle, dynamic gradients.
*   **Borders:** 1px solid, low-opacity borders (e.g., `rgba(255, 255, 255, 0.2)`) to define edges on glassmorphic elements.
*   **Shadows:** Soft, diffuse shadows (`box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.07);`) to lift elements off the page.

### 2.2 Typography
Typography must balance personality with absolute legibility, particularly on small screens.
*   **Headings:** **Outfit** font family. Used for H1-H4. It provides a geometric, modern, and friendly aesthetic that feels approachable (crucial for overcoming "Setup Complexity" fears).
*   **Body & UI Text:** **Inter** font family. The industry standard for legible, neutral, and highly readable UI text at small sizes.
*   **Hierarchy:** Strict adherence to size and weight contrast. Headings should be bold (`700` or `800`), while body text remains regular (`400`) or medium (`500`).

### 2.3 Motion & Animation
Animations must feel snappy and responsive. Sluggish animations convey poor performance.
*   **Entrance Animations:** Maximum duration of **300ms**.
*   **Exit Animations:** Maximum duration of **200ms**.
*   **Easing:** The standard easing curve is `cubic-bezier(0.4, 0, 0.2, 1)`. This provides a natural, physics-based "snap" that feels premium.
*   **Purpose:** Motion should only be used to guide the user's eye (e.g., drawing attention to the "Action Feed" when a new AI draft is ready) or to confirm an action.

### 2.4 Mobile-First Constraints
The OHC platform must be 100% functional and beautiful on a mobile device.
*   **Target Viewport:** The design baseline is a **375px width** (standard iPhone SE/older models). If it works perfectly at 375px, it will scale up gracefully.
*   **Touch Targets:** Minimum touch target size of 44x44px for all interactive elements to accommodate "fat fingers" and rushed usage.
*   **No "Desktop-Only" Features:** Every feature, from inventory management to complex reporting, must be fully accessible and usable on the mobile client.

### 2.5 Accessibility
Premium design is inclusive design.
*   **Contrast:** Minimum WCAG 2.1 AA contrast ratios (4.5:1 for normal text, 3:1 for large text).
*   **Color Independence:** Information must not be conveyed by color alone (e.g., a "Low Stock" warning must have a text label or icon, not just turn red).
*   **Screen Readers:** Proper ARIA labels and semantic HTML/UI components.

## 3. Application to AI Interfaces (Progressive Disclosure)

The challenge of presenting complex AI operations (like the Omnichannel Assistant) to a non-technical user is solved through "Progressive Disclosure."

*   **Simple Mode (Default):** The user sees plain language. No jargon. They see the drafted message and a large "Approve" button.
*   **Advanced Mode (Hidden):** Behind a sticky session toggle or an "Advanced Settings" gear, power users can access raw configurations, JSON editors for AI prompts, or CLI commands.
*   **The Goal:** The UI should never intimidate a new user, but it should never restrict a power user who wants to dig deeper.

## 4. Competitive Advantage via Design

*   **vs. Shopify:** Shopify's backend feels like a massive, utilitarian spreadsheet. OHC feels like a consumer app (like Apple or Airbnb).
*   **vs. Wix:** Wix's editor is cluttered with hundreds of toolbars and options. OHC's interface is focused entirely on the *next required action*.
*   **vs. Durable:** Durable's generated sites are fast but often feel generic. OHC's adherence to strict typography and glassmorphic tokens ensures the generated output feels bespoke and premium.

By rigorously applying these design standards, OHC not only builds a functional platform but establishes the emotional trust necessary to become the central operating system for a small business.
