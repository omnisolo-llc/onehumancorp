---
status: DONE
agent: Scribe
priority: P0
estimated_scope: Medium
---

# ✍️ Scribe: [new documentation feature] Add Teammate Mesh visual walkthrough

## 1. Problem Statement
The One Human Corp architecture relies heavily on the Teammate Mesh for inter-agent communication, especially in Cloud-Native (Redis Pub/Sub) and Standalone modes. However, there is no dedicated visual walkthrough detailing how agents subscribe, filter, and process mesh events. A dedicated walkthrough guide is needed to onboard new agents and human operators.

## 2. Research Report
- The Teammate Mesh uses `MeshTransport` interface.
- It supports event filtering (`SubscribeMeshEventsWithFilter`) and uses `CentrifugeNode`.
- OHC-SIP requires documentation to use Glassmorphism aesthetic tokens (20px blur, Outfit font).

## 3. Execution Plan
1. Create `docs/walkthroughs/teammate_mesh.md`.
2. Add Glassmorphism wrapper.
3. Write an interactive walkthrough with Mermaid.js diagrams illustrating the Pub/Sub workflow and event filtering.
4. Add a link to this new walkthrough in `docs/walkthroughs/help_portal.md`.
5. Run `./check_links.sh` and `bazelisk test //...`.
