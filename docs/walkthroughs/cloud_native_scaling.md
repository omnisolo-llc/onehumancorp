<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Cloud-Native Scaling Visual Walkthrough

Welcome to the Cloud-Native Scaling guide. This document walks through scaling the OHC API backend dynamically based on orchestrator queues.

## Architecture

~~~mermaid
sequenceDiagram
    participant LoadBalancer as OHC Gateway
    participant APIGo as API Pod (Go)
    participant Queue as SQS / Redis
    participant AutoScaler as Kubernetes HPA

    LoadBalancer->>APIGo: Incoming Orchestration Request
    APIGo->>Queue: Push Task
    Queue-->>AutoScaler: High Queue Depth Metric
    AutoScaler->>APIGo: Scale Up Pods
~~~

</div>
