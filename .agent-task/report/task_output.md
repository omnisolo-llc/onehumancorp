issue_title: "Scout Tool Integration Research - Twilio SMS"
issue_description: |
  Target: Twilio SMS (Issue #38512)
  Persona: Food cart operators (Fatima) / Operations department
  Observed vs. Inferred gap: The issue requests direct API integration with Twilio to send SMS notifications for new orders/appointments.
  Source/Code evidence: Current codebase contains research docs (`docs/technical/research/sms_notifications.md`, etc.) proposing this feature, but no actual implementation or provider sandboxes exist.
  Current behavior: No SMS notifications are sent.
  Expected business result: Operators receive reliable SMS alerts.
  Scope/Non-goals: Do not manage 10DLC compliance automatically if blocked.
  Dependencies/Blockers: We lack a Twilio sandbox, API credentials, and explicit authorization to spend money or configure external provider services (as mandated by `RESEARCH.md`).
  Outcome: Blocked. Returning a no-work finding as required by the OneHumanCorp operating contract.
