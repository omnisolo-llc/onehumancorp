# Multi-Language Voice Ordering

**Priority:** P2
**Scope:** Large

## Problem Statement
Fatima (food cart) struggles with English-first text interfaces. She needs to manage orders in her native language, and her customers want to order easily.

## Research Report
- **Competitors:** Mostly English-first, text-heavy POS systems.
- **Conclusion:** Voice-to-text ordering and multi-language admin interfaces open up underserved demographics.

## Design Doc
- **Architecture:** `VoiceInput` -> `TranslationService` -> `OrderProcessor`. Admin UI localized via `i18nManager`.
- **UX Flow:** Customer sends voice note. System transcribes, translates, and places order. Admin sees order in preferred language.

## Implementation Prompt
Implement voice-based order ingestion and real-time translation for the merchant dashboard. Acceptance Criteria: A customer must be able to place an order via an audio message, and the merchant must see the translated text and structured order details.
