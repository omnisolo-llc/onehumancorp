# Private link-bio browser prerequisites

The application crawl creates a dedicated actor through the existing guarded
native database fixture, preserving the seed actor's role and plan. It then uses
the rendered login and private-profile editor, requires the real save response,
and reloads the exact verified owner's persisted profile. Only that owner's
`tenant` route parameter changes in the route inventory. Route enumeration and
all HTTP failure assertions remain in place; a missing private configuration is
not treated as a successful public page.

The maintained private journey in `src/e2e/growth-link-in-bio.spec.ts` also checks
that a separate anonymous context cannot read the saved profile or display its
private name and description. This is a private configuration/read contract.

## Unresolved public acceptance requirement

`src/ui/next/src/e2e/link_in_bio.spec.ts` still describes “User can create and
publish link in bio, then view it publicly”. It is preserved unchanged as an open
acceptance requirement. The current private save does not publish a snapshot,
return a publication receipt, or authorize anonymous profile access. A green
private journey or crawl cannot close that requirement. Any public lifecycle
must be paired with the reviewed publication snapshot/worker/reader design;
there is no global alias, latest-profile fallback, or inferred visibility flag.

Local route-inventory tests validate fixture substitution and URL encoding.
They do not execute these browser journeys; hosted browser acceptance is a
separate gate.
