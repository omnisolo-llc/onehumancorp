# Site publication receipts and public delivery

## Outcome

An authenticated owner can publish the reviewed site, recover its status after reload, and open a working public URL. A saved draft or queued request does not count as publication. Anonymous readers cannot retrieve drafts, arbitrary tenant catalog rows, or invoke cache invalidation.

## Existing defects and boundaries

- `builder/jobs.rs` inserts nonexistent `tasks.mission_type/payload/status`, discards the error, and starts an untracked task. Restart loses pending work.
- `publish_store_profile` commits draft rows before enqueueing and discards enqueue failure. The UI now truthfully calls this a saved site with unverified publishing.
- Ordinary page/block edits also enqueue publication. `published_at` therefore does not prove an explicit publication request.
- The inner builder router invents ADMIN claims when none exist; the real global strict middleware currently masks this fallback. Public delivery must not expose that router.
- Product delivery reads arbitrary tenant/product UUIDs and the first builder site without a publication check. Its broad router also includes a mutation endpoint. Anonymous access currently fails at the strict mount, which must remain until a safe public projection exists.
- Existing rendering can consume live draft rows and has unsafe JSON-LD/script contexts. It must not be made public unchanged.

## Proposed contract

1. Preserve the existing authenticated builder routes. Remove synthetic claim creation; use the current verified identity and normalized OWNER/ADMIN roles for publication. Editing does not create a new public version.
2. Add an owner/tenant-bound publication receipt identified by a client operation UUID. Bind it to a canonical digest of the reviewed layout and site identity. Replaying the same operation returns its recorded receipt; a changed payload under that UUID conflicts before effects.
3. Save the draft, immutable publication snapshot and durable pending job atomically. The publication receipt row also stores pending/leased job state, so this dedicated operation does not inherit the general queue's automatic provider retry rules.
4. A mounted worker claims each pending snapshot durably. It renders reviewed content deterministically without calling an LLM or other paid provider. Optional SEO generation remains a separate existing action. It records the exact rendered bytes/hash and committed public version before acknowledging `published`.
5. Restart may resume pending or expired claimed work because rendering has no external provider effects. A lease token fences late workers, and a monotonically allocated site publication version prevents an older job from replacing a newer reviewed publication. The final database transition is idempotent for the same snapshot/hash. A lost acknowledgement is reconciled by reading the durable receipt; no browser resend is required. Recheck the current owner/tenant authority before committing the public version. Cache refresh failures cannot fabricate readiness for a separate CDN or custom domain.
6. Add an authenticated receipt/status read. The two builder UIs retain the operation UUID before dispatch, read the receipt after reload, and show saved/pending/published/reconciliation separately. Only a committed published receipt supplies the public URL.
7. Mount a separate read-only public projection. Resolve an explicit published snapshot and current publishing authority before any cache lookup; missing, revoked, unpublished or cross-tenant references return404. Leave all builder editing and invalidation endpoints authenticated.

## Catalog publication scope

A published site may expose only product IDs explicitly present in its reviewed catalog blocks. Validate each ID against the current tenant before committing the snapshot. Literal reviewed catalog content remains literal content; it grants no lookup permission to another product. The public product endpoint requires membership in a published snapshot, with tenant and product bound together. Newly created unrelated products remain private until selected and republished. Prices remain snapshot values by default. A future live-price option must be explicitly selected during the owner's publication review, default off, and bound to the selected products and a policy version in the immutable snapshot and receipt. A private price edit must never become public solely to satisfy a cache-invalidation test. Descriptions, names, layout and product membership remain immutable until republished.

The existing SEO browser fixture must create and select its real product before publication, then retain its anonymous access, exact JSON-LD, tenant denial and cache invalidation assertions. An updated price requires a new explicit publication unless a separately tested live-price review/receipt contract is selected in the actual UI. The fixture must not manufacture `published_at` or bypass the real publish action.

## Rendering and compatibility

Render from the committed snapshot, never mutable draft rows. Escape text, attribute, URL and JSON-LD contexts correctly. Only supported, configured actions produce live controls; publishing this content does not assert that booking, checkout, custom DNS or third-party deployment has been configured. Default publication uses the application's real public route. Response headers prevent intermediary reuse until CDN revocation/purge has an independently verified contract; the guarded internal rendered-byte cache remains usable. Existing custom-domain fields remain a separately verified capability.

Existing legacy `published_at` values are insufficient to grant public access; an explicit reviewed publication creates the new receipt. Authenticated preview can continue reading owned drafts. Provider budgeting, payment execution and arbitrary scripts are outside this repair.

## Verification

Use source-bound PostgreSQL tests and real restricted LOGIN roles to prove atomic save/job/receipt commit, two-tenant denial, duplicate/concurrent operation reuse, changed-input conflict, restart recovery and deferred commit failure. Record the actual renderer bytes and exercise the public GET and protected mutation routes through their real middleware. Negative cases cover unpublished/foreign products, stale cached data after revocation, malformed snapshots, script-context payloads and unknown worker outcomes. Keep full source fingerprints and mandatory nonzero CI discovery.

Frontend tests cover operation persistence, owner retirement, delayed completions, unknown outcomes, exact receipt-bound URLs and reload status recovery. Hosted Playwright must execute a real owner publication followed by anonymous GET and edited-product invalidation. No live provider calls or deployed customer data are part of local verification.
