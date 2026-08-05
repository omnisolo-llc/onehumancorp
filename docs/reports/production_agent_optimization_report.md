## CHAT-00 — Chatwoot removal
- Confirmation that no production/customer Chatwoot data existed and no data migration was performed.
- Exact removed application/deployment surfaces: `src/server/integrations/chatwoot/`, `deploy/helm/ohc/templates/chatwoot.yaml`, `deploy/helm/ohc/templates/chatwoot-service.yaml`, docker-compose services, and prometheus configurations.
- `bash deploy/tests/no_chatwoot_residue_test.sh` exited 0
- Negative residue checks (`rg -n -i 'chatwoot'`) passed.
- The native inbox remains in place; feature expansion belongs to later native-chat projects.
