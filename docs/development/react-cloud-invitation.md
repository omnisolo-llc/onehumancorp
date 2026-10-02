# React cloud invitation receipt boundary

The Cloud Bridge Invite form on `/referrals` now calls the existing canonical invitation endpoint. The URL shown after creation comes only from its HTTP 200 receipt. The submitted body contains the captured invitee ID; verified owner headers are preconditions checked against the server session, not browser authority.

The React flow uses the same origin lock and owner-keyed pending/created metadata namespace as the maintained legacy dashboard bridge. Metadata is persisted before dispatch, and neither unknown outcomes nor confirmed reloads automatically retry. It stores no invitation capability URL or invitee email. Authentication/storage lifecycle changes, expiry and unmount retire old view updates; a late confirmed response may record completion for its original owner without displaying it in the new view.

Missing origin coordination, unreadable/corrupt history, storage failures and unrepresentable identity headers remain held. A missing reconciliation interface means a held browser cannot safely create another invitation here. This is not server or cross-device idempotency. GET listing tenant isolation and invitation redemption remain separate verification gaps.

The browser test uses the real isolated loopback acceptance stack, compares the exact response URL and owner headers, and asserts that reload does not POST again. Unit tests isolate the HTTP/platform boundaries without making live invitations. The rest of the referral page's counts, reward promises, export/log placeholders and separate shared referral widget are outside this correction and must not be treated as verified capabilities.
