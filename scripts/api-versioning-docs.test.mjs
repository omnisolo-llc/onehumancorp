import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
const rootUrl = new URL("../", import.meta.url);
// Keep this list to maintained references and walkthroughs. Historical plans,
// archives, and third-party integration research intentionally document their
// original contracts and are not rewritten by this compatibility guard.
const currentApiDocs = [
  "README.md",
  "docs/api/playbook.md",
  "docs/technical/api/api-reference.md",
  "docs/technical/api/edge-llm-offloading-api.md",
  "docs/technical/api/kairos-master-api-guide.md",
  "docs/technical/developer/developer-guide.md",
  "docs/technical/walkthroughs/agent_lifecycle.md",
  "docs/technical/walkthroughs/api_playbook_visual_walkthrough.md",
  "docs/technical/walkthroughs/edge_llm_handoff_walkthrough.md",
  "docs/technical/walkthroughs/help_portal.md",
  "docs/technical/walkthroughs/interactive_api_docs.md",
  "docs/technical/walkthroughs/kairos_interactive_api_playbook.md",
  "docs/technical/walkthroughs/swarm_intelligence_protocol.md",
  "docs/technical/walkthroughs/thin_client_integration.md",
];
const unversionedOHCApiPath = /(^|[^A-Za-z0-9_.])\/api\/(?!v1(?:\/|\b))/g;
const versionedPathWithoutApiNamespace = /https?:\/\/(?:api\.ohc\.local|localhost(?::\d+)?|127\.0\.0\.1(?::\d+)?)\/v1(?:\/|\b)/g;

test("current OHC API documentation uses the /api/v1 namespace", async () => {
  const failures = [];

  for (const relativePath of currentApiDocs) {
    const content = await readFile(new URL(relativePath, rootUrl), "utf8");
    const matches = [...content.matchAll(unversionedOHCApiPath)];
    if (matches.length > 0) {
      failures.push(`${relativePath}: ${matches.length} unversioned /api/ path(s)`);
    }

    const namespaceMatches = [...content.matchAll(versionedPathWithoutApiNamespace)];
    if (namespaceMatches.length > 0) {
      failures.push(`${relativePath}: ${namespaceMatches.length} /v1 path(s) missing /api`);
    }
  }

  assert.deepEqual(failures, []);
});
