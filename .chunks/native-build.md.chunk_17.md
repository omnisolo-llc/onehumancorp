ailure tests must not make real charges. Signing, public publishing, infrastructure changes and customer actions require explicit applicable authority.

## Migration status

See `docs/research/native_migration_and_remediation.md` for the detailed finding-to-code-to-test ledger. A source fix, a passing unit test, a running sandbox flow and a live owner outcome are different evidence levels. Keep those distinctions in release notes and research.

### Browser spec ownership

The required native browser runner sets `PLAYWRIGHT_TEST_DIR=./src`; it includes
Next's nested browser specs as well as `src/e2e`. Fresh discovery is partitioned
into all twelve logical shards and the existing three physical groups.
`native-discovery.test.mjs` compares every repository `*.spec.ts` path with that
actual discovery result. A new spec outside the maintained scope, or a file that
registers no browser test, fails the required Node gate instead of disappearing.

Unit tests keep their explicit `*.test.ts`, `*.test.tsx` or `scripts/*.test.mjs`
names and their Vitest/Node owners. Historical non-runnable browser contracts use
`*.mock-contract.ts`. Those naming contracts are the explicit exclusions from
browser-spec ownership, not directory-wide exceptions for new `*.spec.ts` files.

The browser naming policy permits `.spec.ts` only. Other runnable spec suffixes
(`.spec.tsx`, `.spec.js`, `.spec.jsx`, `.spec.mjs`, `.spec.cjs`, `.spec.mts`,
`.spec.cts`) fail the ownership guard until an actual additional runner is wired
and verified. They cannot silently fall between browser and unit discovery.
