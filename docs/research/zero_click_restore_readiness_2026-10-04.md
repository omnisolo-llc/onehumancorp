# Zero-click setup restore readiness

Baseline: `ef2e7f6617d1ae11343882af5f6dbe6830f58f8c`, PR #39757,
[run 37168335661](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37168335661),
browser shard 3 job `111337296209`. The shard ended with 149 passed and two failed;
the separate quote-refresh failure belongs to a different repair.

The `/onboarding/zero-click` click audit failed after six completed observations
and seven real navigations. Its historical inventory contained four initial
business prompt buttons. Two had already been clicked; the two remaining prompts
were reported missing when the next restored document temporarily exposed only
shell controls. The retained video in
[artifact 11290524214](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37168335661/artifacts/11290524214)
shows the setup restore spinner during that reset. This is not evidence that the
prompt buttons were deleted, persisted chat replaced them, or their clicks failed.

`OnboardingChatAgent` keeps `isLoaded` false until canonical identity and the
saved-state response body are read. However, its visible loading wrapper had no
`aria-busy` state. The existing audit navigation correctly waits for visible
busy elements, but this unmarked spinner bypassed that gate. The dynamic crawl
then found no unvisited controls and its unchanged historical-inventory assertion
reported the two remaining prompts.

The production change gives the existing wrapper `role="status"`, the accessible
name `Restoring setup`, and `aria-busy="true"`. The wrapper already disappears
when restoration completes or a truthful error is displayed. No artificial delay,
timeout increase, network substitution, inventory filtering or owner/session
behavior change is introduced. Every existing click assertion remains unchanged.

New regressions cover deferred canonical identity and a deferred saved-state body,
all four initial prompt controls after successful empty-state restoration, retained
owner conversation instead of fabricated initial prompts, and rejected/malformed
state reads ending in an alert. Existing owner invalidation and stale-response
regressions remain required.

Verification: initial RED had 3 failed / 1 passed because the loading status was
absent. The first GREEN attempt had 45 passed / 1 failed because the test fixture
held a repeated canonical identity verification; that fixture was corrected to
release subsequent identity reads. Final focused verification passed all 46 tests
across six files with one worker and a 768 MiB heap limit. Changed-file lint and
source hashes are recorded with the frozen patch. Full TypeScript, full repository
acceptance and real Chromium execution of this repair remain pending integration
and hosted CI; shared resource windows are serialized.
