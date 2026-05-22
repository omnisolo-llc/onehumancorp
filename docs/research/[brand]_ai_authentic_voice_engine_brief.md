# [brand]_ai_authentic_voice_engine

## Title
AI Authentic Voice Engine: Eliminating Generic Copy

## Problem Statement
When using AI website builders like Durable, the generated copy often sounds robotic, generic, and devoid of the business owner's actual personality. For a boutique owner like Priya, her brand's unique, welcoming voice is her primary differentiator against large retailers. She cannot use standard ChatGPT-style text on her site or in her emails.

## Research Report
Feedback from users of AI-native platforms highlights a critical flaw in current implementations.
- **Finding 1**: Reddit discussions on AI builders frequently mention spending hours rewriting the AI's output because "it doesn't sound like me."
- **Finding 2**: Competitors use basic prompts (e.g., "Write a description for a coffee shop") without capturing the user's specific brand ethos.
- **Finding 3**: True agentic systems must adapt to the user, rather than forcing the user to adapt to the system.

## Design Doc
**Architecture High-Level:**
- **Entities**: `BrandProfile`, `VoiceGuideline`, `ContentDraft`.
- **Key Relationships**: A `BrandProfile` contains multiple `VoiceGuideline`s. Every `ContentDraft` (product description, email, website copy) is generated using the `BrandProfile`.
- **Integration Points**: Core AI text generation services, Onboarding Flow.
- **AI Agent Integration**: The `BrandAgent` interviews the user during onboarding (or ingests their existing social media presence) to define the `VoiceGuideline` (e.g., Tone: Playful, Vocabulary: High-end, Emoji usage: Frequent). All subsequent AI text generation tasks pass through a middleware that enforces these guidelines.

**Mobile UX Flow (375px first):**
1. During setup, the app asks: "How do you talk to your customers? Pick the vibe that fits you best," showing 3 options (Professional, Casual/Friendly, Edgy/Bold).
2. Or, the user pastes their Instagram handle, and the AI analyzes their past posts to learn their voice.
3. The AI confirms: "Got it! Your brand voice is warm, uses emojis 😊, and focuses on community."
4. Whenever the user requests an email draft or product description later, the AI output natively matches this learned voice.

## Implementation Prompt
Implement the AI Authentic Voice Engine.
**User-Facing Outcome**: The AI generates copy that actually sounds like the business owner, requiring zero manual rewriting.
**Critical User Journey**:
1. User provides a sample of their writing or selects a tone preference during onboarding.
2. The system stores this as a Brand Voice Profile.
3. User asks the system to "Write an email announcing our summer sale."
4. The generated email utilizes the exact tone, style, and vocabulary specified in the Brand Voice Profile.
**Acceptance Criteria**:
- System can store and retrieve a Brand Voice Profile.
- Text generation prompts must inject the Brand Voice Profile as system instructions.
- Generated output must demonstrably change based on different voice profiles.

## Priority
P2

## Estimated Scope
Small