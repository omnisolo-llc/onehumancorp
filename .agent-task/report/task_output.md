```yaml
issue_title: "[feature] Implement Automated Cart Recovery Agent"
issue_priority: "P0"
issue_description: "Implement the Automated Cart Recovery Agent to re-engage users who abandon shopping carts. The agent will monitor sessions and trigger personalized follow-ups via email/SMS, potentially generating single-use discount codes."
issue_todo_list:
  - [ ] Add delayed job execution support to the PostgreSQL SKIP LOCKED queue
  - [ ] Integrate transactional email and SMS infrastructure into the Agent framework
  - [ ] Create an internal API for the agent to generate and track single-use discount codes
  - [ ] Implement the Phase 1 basic email recovery sequence
  - [ ] Integrate LLM to generate personalized recovery messages based on cart contents
issue_label: ["feature", "revenue", "agent"]
```
