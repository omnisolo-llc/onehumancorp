outcome: blocked
issue_title: "Implement Hardware-Free P2P Offline Mesh Sync for Multi-Device POS"
issue_description: |
  As per the RESEARCH.md and capabilities/needs audit findings:

  1. The requested feature ("Hardware-Free P2P Offline Mesh Sync for Multi-Device POS") aims to build a complex P2P mesh network for multi-device POS usage, utilizing Apple Multipeer Connectivity and Android Nearby Connections with CRDTs.
  2. The OHC contract operating revision 2026-09-18-usage-audit explicitly dictates that "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require explicit evidence and authorization via the expansion gate in RESEARCH.md."
  3. Retail/manufacturing, POS, and supply-chain correctness are currently explicitly labeled as expansion paths in the RESEARCH.md strategy document, not authorized parallel launch requirements. The current focus is strictly on digital service workloads and resolving P0 correctness issues (tenant data scoping, payment integrity, usage metrics, offline simulation cleanup).
  4. "POS" is explicitly mentioned under the "Gated expansion" layer in `RESEARCH.md` requiring "retained-customer need, measurable value and explicit strategy approval".
  5. The current mandate explicitly states to "Implement only the current bounded slice or report a justified no-work/blocked outcome". Since POS multi-device mesh is part of gated expansion and lacks explicit strategy approval or retention evidence in this trace, this epic is blocked.

  Therefore, this request represents an unauthorized product expansion into physical retail POS domains. I am returning a `blocked` outcome as no implementation is authorized or required at this stage without passing the expansion gate.

  Skills loaded: using-superpowers
  Skills revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
