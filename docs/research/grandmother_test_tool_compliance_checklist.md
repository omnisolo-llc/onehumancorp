# The Grandmother Test: Tool Integration Compliance Checklist

## Philosophy
If a non-technical small business owner cannot understand a tool's function or connect it within 60 seconds, the integration fails the "Grandmother Test."

---

## 1. Terminology Translation
Technical jargon is forbidden in the user interface.

| Technical Term | OHC Human Label |
| :--- | :--- |
| OAuth Flow | "Secure Connection" |
| Webhook | "Automatic Update" |
| API Key | "Connection Code" |
| Endpoint | "Tool Address" |
| Sync Latency | "Update Speed" |

## 2. Setup Friction Benchmarks
- **Default settings**: Every tool should have a "Recommended" setting that requires zero configuration.
- **One-Tap Connection**: Simple login is the primary path.
- **Celebratory Success**: Every successful connection must end with a positive "You're all set!" screen.

## 3. Error Experience (The "Helpful Friend" Tone)
Errors must be translated from technical codes into helpful advice.
- **Bad**: "Error 500: Internal Server Error"
- **Good**: "We're having trouble reaching the service right now. Your data is safe! We'll keep trying every few minutes."

## 4. Visual Feedback
- **Shimmers**: Always show a loading state while fetching external data.
- **Human Icons**: Use icons that represent the real-world action.

## 5. Help Integration
Every tool settings page must include a "?" link that takes the user to a plain-language help article.
