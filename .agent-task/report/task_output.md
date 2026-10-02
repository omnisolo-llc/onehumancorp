outcome: no_work
issue_title: "OHC Native Omnichannel Customer Support & Chat Engine"
issue_description: |
  The requested feature #36325 proposes building a native multi-tenant omnichannel chat engine in Rust to replace external tools like Chatwoot.

  However, this directly violates the core strategic directive laid out in `RESEARCH.md`: "Integrate existing tools instead of rebuilding them." Specifically, it states: "Requests to build native replacements for external services (such as Chatwoot) without explicit authorization, evidence, and an expansion gate must be rejected with a no_work finding."

  Furthermore, the instructions also state: "Defer new viral generators, referral badges, paywalls, agent marketplaces, additional harness adapters, visual workflow builders, simultaneous HR/payroll/MRP coverage, generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome."

  As there is no explicit authorization, evidence, or an expansion gate provided for rebuilding this functionality natively, this task is rejected with a no_work finding.

  (Note: When attempting to validate the clean workspace, `make test` failed due to missing `next` system dependency: `sh: 1: next: not found` during the `build-web` phase.)
