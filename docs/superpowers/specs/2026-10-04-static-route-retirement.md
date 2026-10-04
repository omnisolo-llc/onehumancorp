# Retire redundant integrations and API reference pages

The owner approved removing confirmed obsolete static pages, preserving old URLs through verified canonical replacements, and publishing a cleanup/E2E draft PR. No merge or deployment is authorized. Retained features must use real implementations; this change does not certify every feature in the application.

The visual audit pinned to `40754bf82a9667025b0d440dad9b6bd4a533825c` found broken compiled integration assets and missing legacy Swagger assets. The implementation base is `814b969e68b72cbeebbe66109fe6d5056758e418`; intervening changes do not alter application sources.

Remove only these Next public implementations:

| Old paths | Canonical route |
| --- | --- |
| `/integrations.html`, `/ui/integrations.html` | `/integrations` |
| `/api-docs.html`, `/ui/api-docs.html`, `/api/ui/api-docs.html`, `/api/v1/ui/api-docs.html` | `/api-docs` |

Keep the existing authentication decision first. Only verified-session GET/HEAD navigation redirects, with private/no-store headers and same-origin canonical destinations. Keep query parameters and browser fragments; never accept an arbitrary destination. Other methods and unrelated paths retain existing behavior. Anonymous API-shaped aliases remain 401; anonymous page aliases retain login/return-path behavior.

Update shipped links to canonical URLs. Keep reusable help/voice assets, Tauri bootstrap and historical standalone sources, public authentication routes, reviewed publication documents, provider callbacks, and protected backend APIs. Do not add a public HTML allowlist.

Prove the removed files are absent, old authorized URLs reach real canonical content, required assets load, stale shipped links are absent, and public/authentication boundaries remain intact. Browser coverage must use the real isolated backend and seeded login, not fulfilled API responses. Retain meaningful tooltip/help contracts through canonical component/browser coverage and retained historical-source tests.

Calendar, assistant, dashboard, cost/billing, query-ID documents and customer embeds remain outside deletion scope pending parity/truthfulness work. Do not redirect them merely because names resemble another route.
