<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# OHC Hybrid AI OS Features

This document outlines the design for the OHC Swarm core functionalities:

## Shared Task List
The shared task list enables scalable coordination by persisting assignments and their dependencies via a distributed DAG representation in PostgreSQL or SQLite.

## Teammate Mesh APIs
Teammate Mesh acts as a realtime broadcast channel across agent pods. Redis Pub/Sub drives cloud deployment message routing while SQLite fulfills Standalone deployment needs.

## AutoDream Pipeline
The AutoDream pipeline converts short-lived runtime memory files from `OHC_MEMORY_DIR` into semantic vectors stored in `pgvector` for exact Nearest Neighbor context lookups during agent execution.
</div>
