<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Agentic OS Features CUJ

1. **Task Delegation:** A user delegates a complex task to the main agent.
2. **Sub-agent Spawning:** The main agent decomposes the task and spawns sub-agents via the Orchestration Queue.
3. **Mesh Coordination:** Sub-agents coordinate and share intermediate results over the Realtime Teammate Mesh.
4. **State Transitions:** As sub-tasks complete, the Shared Task List safely updates states using distributed locks.
5. **Memory Consolidation:** Once the entire task completes, the `AutoDream` pipeline automatically summarizes the context and persists it to long-term vector memory.

</div>
