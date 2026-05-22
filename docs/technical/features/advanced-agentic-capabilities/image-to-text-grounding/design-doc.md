<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Document: Image-to-Text Grounding

## 1. Executive Summary
**Objective:** Enable seamless multimodal inference by securely passing image payloads to capable LLMs and structuring the output for agent reasoning.
**Scope:** Update the Orchestration Hub's LLM routing layer to handle multipart and base64 encoded image requests.

## 2. Architecture & Components
- **Multimodal Router:** Parses agent requests containing images and routes to the appropriate model (e.g., GPT-4o).
- **Image Preprocessor:** Handles downsampling and format conversion.
- **Grounding Parser:** Structures the LLM's text output into a usable format.

## 3. Data Flow
1. Agent invokes `capture_screen()`.
2. Agent sends `ImagePrompt` to the Hub.
3. Hub routes the prompt to a multimodal LLM.
4. LLM returns text descriptions (bounding boxes, layout analysis).

## 4. API & Data Models
```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct MultimodalPayload {
    prompt: String,
    images_base64: Vec<String>,
}
```

## 5. Implementation Details
- Implement asynchronous handling for multimodal calls as they may incur higher latency.
- Ensure base64 strings are efficiently handled in memory to prevent OOM errors.
- Maintain Zero-Lock stack compatibility.

</div>
