# Core User Journey (CUJ): Expanding OHC via Capability Plugins

**Goal:** Allow the Human CEO to seamlessly expand the AI workforce's capabilities without manual architectural changes.

**Persona:** The Human CEO (Startup Founder)
**Context:** The CEO currently orchestrates a "Software Company" with PM, SWE, and Marketing agents. They now want their company to launch a custom hardware component, requiring the agents to interface with a new "CAD Modeling Tool".

### Steps:

1.  **Deployment**: The CEO or their IT Administrator deploys a new K8s service, `cad-mcp-server`, to the cluster.
2.  **Discovery**: The OHC MCP Gateway automatically discovers the new service's `CapabilityManifest` and registers it in the `capability_plugins` database table.
3.  **Intent Matching**: The CEO prompts their PM Agent: "Design a new casing for our edge device."
4.  **Semantic Search**: The PM Agent queries the MCP Gateway for tools related to "hardware design" and "casing".
5.  **Dynamic Binding**: The Gateway returns the `cad-mcp-server` schema to the PM Agent. The agent automatically ingests this new tool and its required context without prior hardcoded knowledge.
6.  **Execution & Collaboration**: The PM Agent uses the new CAD tool to generate initial schematics and opens a Meeting Room with the SWE Agent to discuss integrating the hardware with the software stack.
7.  **Visibility**: The CEO views this process on the Dashboard. The UI dynamically populates the newly acquired capability with smooth transitions, rendered through the glass-like interface (`blur(15px) saturate(180%)`), visually indicating that the organization has learned a new skill.
