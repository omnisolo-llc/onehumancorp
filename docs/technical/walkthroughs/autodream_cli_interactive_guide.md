<style>
  .ohc-card {
    background: rgba(255, 255, 255, 0.65);
    backdrop-filter: blur(30px) saturate(210%);
    -webkit-backdrop-filter: blur(30px) saturate(210%);
    border-radius: 16px;
    padding: 24px;
    font-family: 'Inter', system-ui, -apple-system, sans-serif;
    color: #1a1a1a;
    border: 1px solid rgba(0, 0, 0, 0.05);
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.05);
    margin-bottom: 24px;
  }
  @media (prefers-color-scheme: dark) {
    .ohc-card {
      background: rgba(22, 22, 26, 0.7);
      color: #f5f5f5;
      border: 1px solid rgba(255, 255, 255, 0.1);
    }
  }
  .ohc-heading {
    font-family: 'Outfit', system-ui, -apple-system, sans-serif;
    font-weight: 600;
    margin-top: 0;
  }
  .ohc-button {
    border-radius: 8px;
    background: #007aff;
    color: white;
    padding: 10px 20px;
    border: none;
    font-family: 'Inter', sans-serif;
    font-weight: 500;
    cursor: pointer;
    display: inline-block;
    text-decoration: none;
  }
  details {
    margin-top: 20px;
    padding: 15px;
    background: rgba(0,0,0,0.03);
    border-radius: 8px;
  }
  @media (prefers-color-scheme: dark) {
    details { background: rgba(255,255,255,0.05); }
  }
  summary {
    font-family: 'Inter', sans-serif;
    font-weight: 500;
    cursor: pointer;
  }
</style>

<div class="ohc-card">
  <h1 class="ohc-heading">AutoDream: Your AI's Memory Assistant</h1>

  <p>Welcome! This guide shows you how to use AutoDream, the tool that helps your business's AI remember important details from past conversations, making it smarter over time. Think of it as a daily notebook where your AI assistant writes down what it learned to better serve you and your customers.</p>
</div>

<div class="ohc-card">
  <h2 class="ohc-heading">Visual Guide</h2>
  <p>Here is a simple look at how AutoDream works to remember your business details:</p>

```mermaid
graph TD
    CLI[You (Command Line)] -->|Check Status| Status[See if AI is learning]
    CLI -->|Force Learn| Run[Make AI memorize right now]
    CLI -->|Ask Question| Query[Ask what AI remembers]

    Status --> DB[(AI Memory Notebook)]
    Run --> LLM[Brain Processing]
    LLM --> DB
    Query --> DB
```
</div>

<div class="ohc-card">
  <h2 class="ohc-heading">Core Commands</h2>

  <p>You can type these simple commands into your terminal to check or update your AI's memory.</p>

  <h3 class="ohc-heading">1. Check AI Learning Status</h3>
  <p>See if your AI assistant is currently updating its memory.</p>
  <pre><code>$ autodream status
Status: Learning
Last Update: 5 mins ago
Pending Notes: 2</code></pre>

  <h3 class="ohc-heading">2. Force Immediate Learning</h3>
  <p>If you just taught your AI something very important and want it to remember immediately, use this command:</p>
  <pre><code>$ autodream run --force
[INFO] Looking for new notes...
[INFO] Found 2 new conversations.
[INFO] Thinking and saving...
[SUCCESS] 2 conversations saved to memory.</code></pre>

  <h3 class="ohc-heading">3. Ask Your AI's Memory</h3>
  <p>Want to check if your AI remembers something specific? Ask it directly!</p>
  <pre><code>$ autodream query "store closing time"
Top results:
- We close at 8 PM on weekdays (from conversation with manager)
- Weekend hours are 10 AM to 6 PM (from training document)</code></pre>

</div>

<div class="ohc-card">
  <details>
    <summary>Advanced Settings</summary>
    <p>AutoDream uses a vector database (<code>pgvector</code> in Cloud mode or <code>SQLite</code> in Standalone Desktop mode) to store high-dimensional embeddings generated from your agent's session contexts.</p>
    <p>By forcing memory consolidation (<code>autodream run --force</code>), you trigger an immediate embedding pass via your configured embedding provider (e.g., Ada or Minimax), generating 1536-dim embeddings that are then upserted to <code>autodream_memories</code>.</p>
    <p>Learn more about the core pipeline in the <a href="kairos_autodream_walkthrough.md">AutoDream Walkthrough</a>.</p>
  </details>
</div>
