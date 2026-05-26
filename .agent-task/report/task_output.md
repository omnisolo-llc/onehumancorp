issue_title: "Architect and Implement Ambient Voice Commerce and Hands-Free POS Engine"
issue_description: |
  **Problem Statement**
  For high-velocity, physically demanding small businesses—like Fatima's food cart, Carlos's hands-on repair jobs, or Priya managing a busy checkout line—interacting with a screen is a bottleneck. When Fatima is cooking and has gloves on, she cannot safely or quickly tap a 375px screen to accept a new pre-order, mark an item as sold out, or ring up a walk-up customer. Traditional POS systems (Square, Shopify) and even our current mobile-first OHC platform require physical touch, pulling the business owner away from their core craft. The gap is the lack of a secure, always-on, hands-free conversational interface that can orchestrate business operations in real-time.

  **Research Report**
  Competitor & Market Analysis:
  *   Square / Shopify POS: Highly optimized for touch interfaces and dedicated hardware. They offer some basic voice search for products, but lack ambient conversational AI to drive end-to-end checkout, inventory updates, or order management.
  *   Voice Assistants (Alexa, Google Assistant, Siri): These are consumer-focused. While they have "skills" or integrations, they are clunky for real-time, multi-turn business operations (e.g., "Siri, charge the next customer $15 for the Halal Plate and print a receipt").
  *   Market Opportunity: Ambient computing in the enterprise/SMB space is an untapped frontier. By leveraging advanced speech-to-text (STT), large language models (LLMs) with low latency, and our AI Swarm architecture, OHC can become the first truly invisible POS.

  User Sentiment & Pain Points:
  *   "I always have flour on my hands, touching my iPad POS is a nightmare." (Baker)
  *   "When the lunch rush hits, I can't look at my phone to accept DoorDash or online pre-orders, I just need to yell 'Accept' to my system." (Food Cart Operator)
  *   "I want to tell my phone 'Schedule Carlos for a plumbing quote tomorrow at 2pm' without taking my hands off the pipes." (Handyman)

  **Next Steps**
  - Read `docs/research/[architecture]_ambient_voice_commerce_and_hands_free_pos.md` for the full design doc and implementation prompt.
  - Dispatch to implementers to establish audio streaming layer.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []