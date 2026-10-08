ve distribution patterns, not two new products we must immediately build.

## 6. BYOK, subscription access and who pays

Do not use “BYOK” to imply that a consumer subscription supplies a general API credential. Technical support, commercial permission, privacy terms, quotas and owner authorization must all be checked for the exact mode.

| Mode under evaluation | AI provider payer | OHC charging basis / unresolved question |
|---|---|---|
| Managed API | OHC's contracted API account | OHC compute/resources plus explicitly priced API consumption, subject to the provider's commercial terms. Do not assume permission to resell bare access or pool consumer subscriptions. |
| Customer API key / cloud account | Customer directly | OHC hosting, storage, tools and other own costs; do not rebill customer-paid inference as OHC consumption. Inventory any embeddings/judges/background calls that still use an OHC-funded key. |
| Provider-native client with an eligible subscription | Customer subscription/credits | Potentially OHC execution-environment resources, only within provider-permitted integration/auth patterns. Quota is not unlimited autonomous capacity and a token is not transferable general API access. |
| Customer machine / local model | Customer hardware and applicable provider | OHC only charges for actual optional cloud services it provides. An owner-machine workflow cannot promise always-on hosted execution while that machine is offline. |

### Provider-specific evidence

**OpenAI:** [Business overview](https://help.openai.com/en/articles/8792828-chatgpt-business-overview) distinguishes Business from separately billed API usage. [Codex authentication](https://learn.chatgpt.com/docs/auth) supports ChatGPT subscription or API-key login; recommends API keys for programmatic CLI work; and documents Enterprise access tokens for trusted private Codex automation, not general API calls. This establishes different access modes, not approval for arbitrary hosted subscription