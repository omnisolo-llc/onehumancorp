# [operations] Multilingual Mobile Ops & Thermal Printing ("The Kitchen Manager")

## Problem Statement
Small food business owners like Fatima (food cart) face two major hurdles: language barriers and the lack of a physical "ticket" system for orders. When an order comes in via her phone, she has to manually translate the request and often loses track of orders because she has no way to print them for her cooking line. She needs a tool that speaks her language and connects to the physical world of her food cart.

## Research Report
- **Market Context**: POS systems like Toast or Square are expensive and hardware-heavy. Mobile-first platforms (Shopify) have "Order Lists" but poor support for cheap, portable Bluetooth thermal printers used in global markets.
- **Competitor Gap**: None of the major AI builders (Durable, Wix, 10Web) focus on the *physical* operational needs of a food stall or street vendor.
- **User Evidence**:
    - *Fatima (Persona)*: Limited English, needs mobile notifications and printed lists.
    - *Market Data*: Rise of "Ghost Kitchens" and food trucks in LATAM and SE Asia requiring low-cost mobile POS solutions.

## Design Doc
### Architecture
- **Entity Relationship**: `Order` <-> `TranslationService` <-> `PrintJob`.
- **Integration Points**: Browser Web Bluetooth API (for thermal printers), Google Translate API (for order notes).
- **Agent Integration**: "The Kitchen Manager" agent automatically translates order special instructions into the owner's preferred language and formats the order for a 58mm thermal printer.

### UI/UX Flow (375px Mobile)
1. **Order Received**: A high-contrast "New Order" screen appears with a large [Print Ticket] button.
2. **Auto-Translation**: The English note "No onions, extra spicy" is displayed in Arabic (Fatima's language) immediately below the original.
3. **Physical Action**: Tapping [Print] triggers a Bluetooth thermal printer to spit out the ticket.

## Implementation Prompt
Implement the "The Kitchen Manager" mobile ops workflow. The system must:
1. Detect the user's preferred language during onboarding and persist it.
2. Automatically translate all incoming "Customer Notes" in orders.
3. Implement a "Print Layout" view optimized for 58mm/80mm thermal printers.
4. Add a "Print" action to the Order Detail screen using the Web Bluetooth/Print API.
5. Ensure "Grandmother Test" compliance: order status is "Big, Bold, and Clear" for fast-paced environments.

**Acceptance Criteria**:
- Orders are translated in <2s upon arrival.
- User can trigger a print job with 1 tap.
- UI supports high-contrast "Sunlight Mode" for outdoor food stall use.

## Priority: P1
## Estimated Scope: Medium
