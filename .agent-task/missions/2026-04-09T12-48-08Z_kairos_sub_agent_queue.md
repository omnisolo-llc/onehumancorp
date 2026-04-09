---
Title: "Implement Sub-Agent Orchestration Queue"
Problem Statement: "The Sub-Agent Orchestration Queue is a vital component of the KAIROS Orchestration layer, designed to handle the massive concurrency of sub-tasks delegated by primary agents. The queue seamlessly transitions between different storage backends depending on the operating mode."
Research Report: "Based on docs/features/kairos/sub_agent_queue.md, we need a distributed queue for agent tasks. In Cloud Mode, it uses Redis (rueidis) Lists and Sorted Sets. In Standalone Mode, it uses an internal SQLite table (sub_agent_jobs)."
Design Doc: "1. Cloud Mode: Implement Redis list-based queueing for high throughput. 2. Standalone Mode: Use SQLite table `sub_agent_jobs` with concurrent read/write locks (simulating `FOR UPDATE SKIP LOCKED`). 3. Worker Logic: Implement polling, execution, success/failure transitions, retry backoffs, and dead-letter logic. 4. Observability: Both implementations natively integrate with OpenTelemetry for queue length, processing time, and failure rates."
Implementation Prompt: "1. Read docs/features/kairos/sub_agent_queue.md. 2. Create the queue interface and workers in `srcs/server/orchestration/queue.go`. 3. Implement Redis backend. 4. Implement SQLite backend. 5. Integrate OpenTelemetry metrics for queue stats. 6. Write tests."
Priority: "P0"
Estimated Scope: "Large"
---
