# Bounded dependency maintenance, 2026-10-05

This change follows the library-reuse review of main
`ee926e0a4aaa179b86ac23d6e6070b81e30e63a8` and is based on
`405733038a8541b5cdaab70f88ad8b5e91a30c52`. It updates compatible release
lines without replacing application frameworks or changing the repository license.

| Dependency | Previous | Selected | Primary reason |
| --- | --- | --- | --- |
| Tauri | 2.11.5 | 2.11.6 | [GHSA-w28w-mhc8-qvjv](https://github.com/tauri-apps/tauri/security/advisories/GHSA-w28w-mhc8-qvjv) |
| h2 | 0.4.14 (some focused locks 0.4.13) | 0.4.16 | [RUSTSEC-2026-0258](https://rustsec.org/advisories/RUSTSEC-2026-0258.html) |
| rustls | 0.23.40 | 0.23.45 | [RUSTSEC-2026-0285](https://rustsec.org/advisories/RUSTSEC-2026-0285.html) |
| rustls-webpki | 0.103.13 | 0.103.15 | Compatible resolution of updated rustls's `^0.103.14` dependency |
| anyhow | 1.0.102 | 1.0.103 | [RUSTSEC-2026-0190](https://rustsec.org/advisories/RUSTSEC-2026-0190.html), informational unsoundness |
| crossbeam-epoch | 0.9.18 | 0.9.20 | [RUSTSEC-2026-0204](https://rustsec.org/advisories/RUSTSEC-2026-0204.html) |
| event-listener | 5.4.1 | 5.4.2 | [RUSTSEC-2026-0221](https://rustsec.org/advisories/RUSTSEC-2026-0221.html), informational unsoundness |
| plist | 1.9.0 | 1.10.0 | Its declared `quick-xml ^0.41.0` permits the fixed parser line |
| quick-xml | 0.39.4 | 0.41.0 | [RUSTSEC-2026-0194](https://rustsec.org/advisories/RUSTSEC-2026-0194.html) and [0195](https://rustsec.org/advisories/RUSTSEC-2026-0195.html) |

The root and all 11 tracked standalone focused-harness locks receive applicable
updates. Forty focused-lock update operations completed successfully. Registry
checksums and declared dependency ranges were independently checked. These package
releases have permissive license records; their notices remain applicable. The
largest newly selected MSRV is plist's Rust 1.88, below the repository's pinned
1.95. This is dependency-selection evidence, not a complete legal or security audit.

Cargo also resolves some edges between versions already present in the locks:
socket2, windows-sys and proc-macro-crate. Their upstream ranges permit these
selections. Those are real graph changes requiring platform validation, even though
they introduce no additional registry versions. Event-listener no longer needs
concurrent-queue. The source-extractor workspace member and TaxJar's stronger
focused harness account for separately reviewed local-package and harness changes.

`src/server/Cargo.lock` remains historical and inactive: it has no neighboring
Cargo manifest or identified build consumer. It is not one of the maintained
active graphs. `rkyv 0.7.46` remains an optional lock entry and is **not patched**
for RUSTSEC-2026-0235. Reviewed rust_decimal features do not activate it; keep that
feature disabled unless a compatible migration is reviewed. A lock inventory alone
does not establish compiled feature or target reachability.

The Next development/build tree still requires triage of
[GHSA-vfj7-8cjw-p6xm](https://github.com/advisories/GHSA-vfj7-8cjw-p6xm), for which
the inspected advisory lists no patched braces release. No arbitrary override or
unrelated Tailwind major upgrade is used to conceal this report.

The companion Node change pins 22.23.3 and verifies all six official distribution
checksums and the license. It does not certify every native release platform.
Neither lock review nor focused tests replace `make lint`, `make test`, complete
required hosted checks, or separate release platform/signing verification.
