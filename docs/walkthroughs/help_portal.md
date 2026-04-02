<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OHC Help Portal: Visual Walkthroughs

Welcome to the One Human Corp Help Portal. This guide will walk you through setting up and orchestrating your swarm of agents seamlessly.

## Getting Started

1. **Initialize the Orchestration Hub**
   Start by configuring your base environment. The system operates on an `OHC-HA` (Hybrid Architecture). Set your mode in the configuration variables.

2. **Hiring Agents**
   Use the UI or API to assemble your team. Agents are automatically onboarded using zero-trust SPIFFE identity protocols.

3. **Virtual Meeting Rooms**
   Initiate a session by inviting the PM and Engineering Director agents to a Virtual Meeting Room to debate scope before execution.

## Troubleshooting

- **Redis Connections in Standalone Mode**: In Standalone mode, OHC falls back gracefully to SQLite. Ensure `DATABASE_URL` is configured for your local sqlite database.
- **Teammate Mesh Not Syncing**: Verify the connection to the Centrifuge realtime pub/sub system and your `mesh:tasks` channels.

*For more advanced topics, see the [API Playbook](../api/playbook.md).*

</div>
