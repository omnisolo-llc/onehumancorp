# Source-bound Swagger UI distribution

The API reference uses official Swagger UI 5.33.1, rebuilt from commit
`cac3d136b5e37bfbe3288c8591f6f4fcf9870599` with DOMPurify pinned to 3.4.16.
The stock distribution embedded 3.4.13. Its package manifest did not expose that
embedded version to application `npm audit`.

No Swagger application source is modified. The small upstream patch changes the
dependency pin and makes build metadata use the source commit's timestamp. The
completed recipe lock preserves every other upstream package version and adds
missing exact npm-registry tarball/integrity records. Install scripts and Scarf
analytics are disabled. This is a locally rebuilt distribution, not an upstream
claim that the stock release contains the patch.

The bundle, styles, source map and OAuth redirect files reproduced byte-for-byte
after a clean dependency reinstall on Linux x64 with official Node 24.21.0 and
its bundled npm 11.19.0. `manifest.json` binds the source, recipe, runtime
inventory and shipped assets. `runtime-inventory.json` is derived from emitted
webpack modules, not the broader source-build dependency graph. It includes
DOMPurify 3.4.16 and excludes the unused Remarkable CLI's argparse/sprintf-js
modules. Build-only dependencies remain in the recipe lock and are not silently
represented as deployed code or declared advisory-free.

`node scripts/audit-swagger-bundle.mjs` first verifies the asset/inventory hashes,
then checks Swagger UI itself and every recorded runtime dependency version against the npm registry advisory
endpoint. It fails on unavailable or malformed advisory data and on advisories
at any severity. The existing root and Next npm audits remain separate gates.
No emitted package, assertion, severity threshold or operation is excluded to obtain a
passing result.

Use `recipe/rebuild.sh /absolute/new/working-directory` from a trusted checkout to
rebuild and verify the distribution. The output directory must not already
exist. The script fetches only the pinned official repository and uses the
integrity-bound npm lock. It does not publish or modify the application assets.
Review and test a new source/lock/inventory before changing any pinned hashes.

The application serves these assets from its own authenticated origin. The
viewer lives in a same-origin iframe so normal page unmount destroys the bundled
renderer, which exposes no React-root disposal API. The parent fetches the
existing authenticated specification, cancels on navigation and checks message
source/origin. The viewer disables the external Swagger validator. Interactive
operations, keyboard controls and light/dark presentation remain available.

All original Swagger LICENSE/NOTICE and emitted license comments are retained;
THIRD_PARTY_LICENSES.txt preserves available package notices and license metadata.

The iframe is a lifecycle boundary, not a security sandbox: same-origin code can
access the parent. Security still relies on the reviewed local bundle, patched
sanitation, private route policy and exact source/origin message checks. No
credentials are interpolated into HTML or sent to another origin. API requests
retain upstream Swagger behavior; this repair does not change the server spec's
existing localhost server URL or grant additional backend access.
