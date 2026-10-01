# Required browser click coverage

The canonical full native runner executes each per-route click audit once. Its required post-run verifier checks the actual completed evidence instead of repeating the same clicks in a second all-route sweep. The six other global contracts remain separate browser tests: route inventory size, page loads, internal links, external/protocol links, interactive usability, and desktop/mobile layouts. Every per-route purpose assertion remains required too.

The source route discovery is shared with the verifier and keeps the previous rules, including dynamic examples and ignored API/route-group segments. At the reconstruction base it finds213 routes and433 comprehensive assertions. These are observed inventory sizes, not hardcoded substitutes for current source discovery.

## What must pass

Each route attaches the union of target keys actually discovered, one completed effect/error observation for every attempted target, the exhausted-enumeration state, and the result of its original no-failures assertion. A discovered target that disappears before execution is a failure. The collector records actual Playwright test selection, terminal outcomes and attachment bodies; expected failures, skips and retries cannot satisfy any required comprehensive assertion. Alert-only feedback and unsuccessful selection restoration remain invalid. Original target selectors, actions,90-second enumeration budgets and other assertions are unchanged.

Receipts bind to actual Git HEAD, all tracked source-file/symlink bytes, the exact route inventory, GitHub run ID and run attempt. Source fingerprints must match before and after execution. The aggregate requires all12 distinct shards, every expected comprehensive test, every discovered route and every observed target. Missing, stale, duplicate, incomplete, symlinked, malformed or pending receipt files fail closed. Generated counts alone never establish completion.

CI keeps the original browser reports and adds separate per-shard receipt artifacts. The required native-click-coverage job downloads each artifact into a separate directory and validates their combined evidence. CI Required refuses a missing or failed aggregate. The unsharded complete local native runner performs the same verification directly. Required CI does not accept a narrowed selection or reporter override; explicit local focused selections remain partial checks. The inventory declaration itself refuses execution without the native runner context.

## Evidence and limits

The protocol tests include the installed Playwright runner executing real Node-only fixture assertions through the actual reporter, plus a deliberately failed assertion that must not aggregate. This verifies reporter delivery, not a browser or product workflow. Browser oracle and full real-stack CI execution are still required. The fragment oracle separately exercises an actual in-document destination while rejecting placeholders.

The reset-per-target crawler still has a known discovery limitation: a control revealed only by an earlier interaction may never be seen after document retirement. Receipts enforce completion for targets the crawler actually discovered; they do not certify unseen controls. The current performance and remount failures must be resolved with real browser evidence, without reducing discovery or increasing budgets.

This source was reconstructed after the earlier local workspace was lost. It is not claimed byte-identical to the historical ad571566 implementation. New validation and independent review bind to this reconstruction's own source manifest.
