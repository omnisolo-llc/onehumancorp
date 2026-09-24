{
  "issue_title": "Research CI Caches (M04)",
  "issue_description": "Investigation into M04: CI caches keyed by toolchain, OS/architecture, dependencies and build role. Current implementation states: OS/architecture/role/compiler-aware caches; explicit Node dependency scopes; positive trusted-event cache-save rules; no PR cache publication. Cold-cache control disables project-cache restoration. Remaining gaps: Real fresh-runner cache hit/miss and runtime measurements; signed release cache isolation is configuration-reviewed, not release-certified. This is a research finding, not a code implementation. Skill provenance: Loaded superpowers skills using-superpowers (revision 5bf4e78011075bcfc0dc295f0724994cd123ee71)."
}
