# OmniSolo OneHumanCorp naming and compatibility contract

**OmniSolo** is the brand. **OmniSolo OneHumanCorp** is the product name.
Use the full product name in application titles, product descriptions, onboarding,
and documentation. Brand attribution, such as “Powered by OmniSolo,” uses the brand name.
Existing installations
may still expose legacy identifiers; these are compatibility aliases, not
user-facing branding.

| Legacy identifier | Compatibility boundary | Canonical replacement |
| --- | --- | --- |
| `OHC_*` environment variables | Server configuration and existing Helm/Secret manifests | `OMNISOLO_*` for new deployments |
| `OHC_DATABASE_URL` | Database URL loader | `DATABASE_URL` |
| `/api/v1/ohc_job_queue` | API route alias | Keep until a versioned OmniSolo route is published |
| `ohc.app`, `onehumancorp.com` | Existing tenant/share links | `omnisolo.co` / `cloud.omnisolo.co` |
| `OHC_WEB_SESSION_*` | Browser session-key aliases | `OMNISOLO_WEB_SESSION_*` |
| Kubernetes Secret keys containing `OHC` | Existing cluster Secret contract | OmniSolo-named keys in new manifests |

Compatibility identifiers must remain confined to configuration, migration,
transport, and deployment boundaries. Product UI, metadata, generated help,
share text, and documentation name the product **OmniSolo OneHumanCorp**.
Technical package names, URLs, and compatibility aliases are separate from display wording.
