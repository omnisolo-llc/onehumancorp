# Assistant observation masking boundary

The receipt-backed Assistant admits one text-only analysis without tools,
workspace access, conversation history, or tool observations. Its former
observation-masking control submitted a setting that the settings endpoint only
echoed. That response was not evidence of persistence or runtime behavior.

The control is now explicitly unavailable, with the reason shown in the UI.
`PATCH /api/v1/assistant/settings` rejects any `observationMasking` field,
including mixed name/masking requests, before writing anything. Actual assistant
name settings remain persisted, tenant-keyed, readable and usable after reload.

The built-in agent's observation-masking algorithm and operator configuration are
unchanged. A user-configurable tenant masking policy wired into a tool-observation
execution path remains an open feature gap; this repair does not implement or
claim that integration. No unused preference is stored as a substitute.

Coverage includes the authenticated browser rejection/no-write boundary, real
name save and PostgreSQL readback, a mounted native settings regression, and UI
checks that the unavailable controls cannot submit a mutation. Native formatting,
compilation and the new browser case require the published-head CI run.
