<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# OHC Walkthrough: Custom Agent Creation

Welcome to the Custom Agent Creation walkthrough!

## Flow
```mermaid
sequenceDiagram
    participant CEO as Human CEO
    participant Hub as Orchestration Hub
    participant Agent as Custom Agent
    CEO->>Hub: Define Custom Agent Role & Skills
    Hub->>Agent: Provision via OHC-HA
    Agent-->>Hub: Acknowledges
    Hub-->>CEO: Custom Agent Ready
```

## Steps
1. Define Role.
2. Assign domain expertise.
3. Deploy!
</div>
