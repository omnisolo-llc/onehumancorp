<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Local LLM Inference Walkthrough

Welcome to the One Human Corp (OHC) interactive walkthrough for configuring Local LLM Inference in **Standalone Mode**.

## Architecture & Flow

In Standalone Mode, OHC runs entirely on your local machine, ensuring maximum privacy and zero latency by utilizing local LLMs via tools like Ollama or llama.cpp.

```mermaid
graph TD
    User[Human CEO] --> Standalone[OHC Standalone Hub]
    Standalone --> |API Calls| LocalLLM[Local LLM Process]
    LocalLLM --> |Ollama / llama.cpp| Model[Local Weights]
    Standalone --> |Sync| SQLite[(Local SQLite)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class User,Standalone,LocalLLM,Model,SQLite premium;
```

## Configuration

To configure OHC to use your local LLM, ensure you have Ollama or llama.cpp running and accessible. Configure your `.env` file with the local endpoint:

- `LLM_API_URL=http://localhost:11434/api/generate` (Example for Ollama)
- `LLM_MODEL=llama3`

When configured, the Orchestration Hub will automatically route inference requests locally instead of calling external Cloud providers.

</div>
