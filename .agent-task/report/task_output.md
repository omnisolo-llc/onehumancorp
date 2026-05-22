issue_title: "[Architecture] Autonomous Event Ticketing & Pop-Up Engine"
issue_description: |
  # Research Report: Autonomous Event Ticketing & Pop-Up Engine

  ## Findings
  Small business owners such as Maya (baker), Priya (boutique owner), and Leo (music tutor) frequently host pop-up shops, workshops, and ticketed events. Currently, they are forced to duct-tape external platforms like Eventbrite or use clunky third-party plugins. This leads to disjointed inventory, fragmented customer data, high ticketing fees, and a poor check-in experience.

  Leading platforms like Shopify rely on paid third-party apps, while Wix/Squarespace often gate native tools behind higher tiers and are not mobile-optimized. Eventbrite charges high fees and owns the customer relationship.

  ## Proposed Next Steps
  We need to deliver a native, zero-config event ticketing engine that unifies inventory, calendar, and CRM within OHC. This will allow merchants to create events, sell tickets that automatically generate digital wallet passes, and scan attendees in using an offline-first mobile scanner. All background complexity (revenue splitting, capacity management, CRM tagging) will be handled invisibly by the Operations, Marketing, and Finance AI agents.

  See the detailed design doc at `docs/research/[architecture]_autonomous_event_ticketing_and_popup_engine.md` for full implementation guidelines, UX flows, and Mermaid.js architectures.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
