# [Research] Multilingual Operations Agent for Non-Native Speakers

## Title
Multilingual Operations Agent for Non-Native English Speakers

## Problem Statement
SMB owners like Fatima (Food Cart Operator) who have limited English proficiency struggle to use English-centric backends (like Shopify). They need a system that translates incoming orders, reviews, and customer messages into their preferred language, and allows them to manage their business without language barriers.

## Research Report
Market research indicates that platforms like Shopify have complex backends that alienate non-technical, non-native English speakers. Providing an operations agent that acts as a real-time translator and simplified interface will capture a significant underserved market segment.

## Design Doc
**Architecture & Integration:**
*   **Translation Layer:** Operations Agent intercepts all incoming textual data (orders, messages).
*   **Mobile UX Flow (375px first):**
    1.  Order arrives in English.
    2.  Agent translates order to user's preferred language (e.g., Spanish, Arabic).
    3.  Push notification sent in user's language.
    4.  App dashboard displays all data translated natively.

## Implementation Prompt
**User-Facing Outcome:** The SMB owner views and manages all business operations, including incoming orders and messages, entirely in their native language, regardless of the language used by the customer.
**Critical User Journey:**
1. Customer orders "2 Beef Tacos" in English.
2. Agent translates order and notifies Fatima in Spanish: "Nueva orden: 2 Tacos de Res".
3. Fatima prepares the order and taps "Listo" (Ready).
**Acceptance Criteria:**
* All incoming text data is automatically translated to the owner's configured language.
* The mobile UI natively reflects the chosen language.

## Priority
P1

## Estimated Scope
Medium
