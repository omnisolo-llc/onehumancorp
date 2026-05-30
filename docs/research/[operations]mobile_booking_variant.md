# [operations] Unified Mobile-Native Product & Booking Variant Creator

**Problem Statement**: Carlos (Handyman) and Priya (Boutique) struggle to add complex variants (Size/Color or Date/Time) from their phones.

**Research Report**: Competitor mobile apps handle basic text inputs well but fail at complex matrix inputs (variants/scheduling).

**Design Doc**:
- *Trigger*: User uploads a photo or describes a service.
- *AI Agent (Operations)*: Parses the input ("I'm offering plumbing fixes for $50/hr") and generates a booking variant matrix automatically.
- *Mobile UX (375px)*: Conversational UI utilizing the native numeric keypad and touch-friendly toggle switches for availability.

**Implementation Prompt**: Create a native Flutter mobile UI component for product/service creation that uses the Operations Agent to pre-fill variants, pricing, and scheduling slots from a natural language prompt or photo upload.

**Priority**: P1
**Estimated Scope**: Medium
