issue_title: "[Research] AI Voice Ordering for Small Business Competitor Analysis"
issue_description: |
  # Research Report: AI Voice Ordering for Small Business & Gap Analysis

  ## 1. Market Mapping & Competitor Discovery (Track 1)

  Small business owners in the food and beverage industry (like Fatima the food cart operator) often struggle to take orders over the phone during busy hours. A growing market of AI voice ordering competitors has emerged to handle this.

  **Top Competitors in Voice Ordering for SMBs:**
  1. **Slang.ai**: Specifically targets restaurants with AI voice receptionists that can answer FAQs, take reservations, and handle basic orders.
  2. **Kea**: Cloud-based AI system that takes phone orders and integrates directly into POS systems for pizzerias and quick-service restaurants.
  3. **Popmenu (Answering)**: Bundled with their website/menu platform, Popmenu offers an AI answering service that routes calls and can text ordering links to customers.
  4. **SoundHound (Smart Answering)**: Uses conversational AI to answer calls, handle FAQs, and send SMS links for ordering or reservations.
  5. **Bland AI**: A more general-purpose programmable voice AI that some operators use to build custom phone agents.
  6. **PolyAI**: While targeting larger enterprises, they set the standard for natural, human-like voice interactions in customer service.

  **Success Factors:**
  - **Reducing missed calls:** Restaurants can lose 10-20% of revenue from unanswered calls during peak hours.
  - **SMS Handoff:** If the AI cannot complete the complex order via voice, successful tools text a link to the online ordering system.
  - **POS Integration:** Kea and Slang's ability to inject orders directly into the kitchen display system (KDS) is a massive advantage.

  ## 2. OHC Gap & Pain Point Identification (Track 3)

  **Persona Focus:** Fatima (Food Cart Operator)
  Fatima struggles with English-speaking customers on the phone while cooking. She needs a way to capture phone demand without stopping her operations.

  **The Gap:**
  Currently, OHC has basic Twilio Voice integration (Twilio voice integration service) and voice commands for the owner (audio command service), but it lacks a dedicated, multi-lingual AI Voice Receptionist capable of completing customer orders or seamlessly handing off to an SMS ordering flow. The current implementation only transcribes and summarizes calls into the unified inbox, creating a task for the owner to review later. It does not actively close the sale.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Architecture Diagram

  ```mermaid
  graph TD
      Caller[Customer Phone] -->|Call| Twilio[Twilio Voice Webhook]
      Twilio -->|Audio Stream| OHC_Voice[OHC Voice Gateway]
      OHC_Voice -->|Transcribe & Intent| LLM[LLM / Intent Extraction]
      LLM -->|Intent: Order Request| OperationsAgent[Operations Agent]
      OperationsAgent -->|Check Stock| Inventory[PostgreSQL / Redis]
      Inventory -->|Available| OperationsAgent
      OperationsAgent -->|Generate SMS Link| SMS[Twilio SMS]
      SMS -->|Order Link| Caller
      OperationsAgent -->|Draft Order| AgentFeed[Agent Feed]
  ```

  ### Proposed Enhancements

  1. **The "Voice-to-SMS Handoff" Flow:** Instead of forcing the AI to perfectly parse a complex food order with modifiers (which is prone to error and frustration), the AI should detect the ordering intent and immediately say, "I can send you a secure link to place your order right now. Would you like me to text that to the number you are calling from?"
  2. **Multilingual Capabilities:** The intent extraction layer needs explicit instructions to detect the caller's language and respond in kind, or translate the intent back to English for Fatima's Agent Feed summary.
  3. **Integration with Agent Feed:** If the customer completes the order via the SMS link, the Operations Agent should update the original "Missed Call" task in the Agent Feed to "Resolved - Order Placed."

  ## 4. Implementation Prompt & Issue Dispatch

  **Feature Name:** Voice-to-SMS Ordering Handoff for SMBs

  **Target Persona:** Fatima (Food Cart Operator)

  **Outcome:** An AI voice receptionist that answers incoming customer calls, detects ordering intent, and automatically texts the customer a link to the OHC online storefront, capturing the sale without owner intervention.

  **Critical User Journey (CUJ):**
  1. Customer calls Fatima's Twilio-provisioned OHC number.
  2. The OHC Voice Gateway (via Twilio Webhook) answers the call and uses the LLM to process the customer's speech.
  3. Customer says, "Hi, I'd like to place an order for pickup."
  4. The LLM identifies the `ORDER_FOOD` intent.
  5. The AI Voice responds: "Absolutely. I am an automated assistant. I'm texting you a link to our online menu right now so you can place your order. Please check your messages!"
  6. OHC dispatches an SMS via Twilio to the caller's number with a link to Fatima's OHC storefront.
  7. Fatima sees a summary in her Agent Feed: "Automated receptionist handled a call from [Number] and sent the ordering link."

  **Next Actions for Engineering:**
  - **Step 1:** Update Twilio voice integration service to include semantic intent routing for `ORDER_FOOD` or `GENERAL_INQUIRY`.
  - **Step 2:** Implement the Twilio SMS dispatch function when the `ORDER_FOOD` intent is triggered.
  - **Step 3:** Ensure the interaction is logged properly to the `omni_inbox_messages` and the task in the Agent Feed accurately reflects the SMS handoff.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
