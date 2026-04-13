1. **Define Proactive Mission**:
   Since the Elastic Swarm Bursting API docs exist but it lacks a visual walkthrough in `docs/walkthroughs/help_portal.md`, I will create a new proactive mission file `$(date -u +"%Y-%m-%dT%H-%M-%SZ")_scribe_bursting_walkthrough.md` to add this visual walkthrough.

2. **Create Visual Walkthrough**:
   Create `docs/walkthroughs/elastic_swarm_bursting.md` using the exact Mermaid diagram found in the API playbook (`docs/api/playbook.md`) and the required Glassmorphism style.

3. **Update Help Portal**:
   Append the link to `docs/walkthroughs/help_portal.md` under the "Deep Dive Walkthroughs" section using sed: `sed -i '/- \*\*\[Hybrid Search MCP Protocol Walkthrough\]/a \- **[Elastic Swarm Bursting Walkthrough](elastic_swarm_bursting.md)**: Visual guide to offloading local compute to the Cloud-Native API.' docs/walkthroughs/help_portal.md`.

4. **Verify Links and Build**:
   Run `export PATH=$PATH:/home/jules/go/bin && ./check_links.sh` and `bazelisk test //...` to ensure all links and tests pass.

5. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit Changes**:
   Use the `submit` tool to submit the PR with the title `✍️ Scribe: [new documentation feature] Add Elastic Swarm Bursting Walkthrough` to the active branch.
