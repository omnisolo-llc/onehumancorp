# Unify Customer Messages from Instagram and WhatsApp

**Problem Statement:** Fatima (a baker) gets cake orders via WhatsApp, Instagram DMs, and Facebook. She misses orders because she has to check 3 different apps on her phone constantly. She needs one simple inbox where all customer messages appear together.

**Research Report:** Connecting Meta's Graph API (for FB/IG) and WhatsApp Business API provides unified messaging. Meta's official tools are reliable but their developer setup is extremely complex for a non-technical user. Alternatively, using an open-source hub like Chatwoot as an intermediary can simplify this. Free tier exists for Meta APIs. Reputation is solid but support is lacking.

**Design Doc:** The business owner connects their Facebook/Instagram account via a simple "Connect Socials" button in OHC. Once authenticated, new messages from IG/WhatsApp appear in the OHC unified inbox. Replying in OHC sends the message back to the customer's native app.

**Implementation Prompt:** Provide a UI button to authenticate with Meta. Once connected, display incoming Instagram and WhatsApp messages in the unified chat interface. Allow the user to type a reply and send it back to the customer's social app seamlessly.

**Priority:** P1

**Estimated Scope:** Medium
