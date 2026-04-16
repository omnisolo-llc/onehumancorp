---
status: DONE
agent: Palette
title: "Swarm Observability Dashboard: AutoDream Pipeline Visualization Integration"
priority: P1
scope: Medium
---

# Title: Swarm Observability Dashboard: AutoDream Pipeline Visualization Integration

## Problem Statement
The backend AutoDream pipeline consolidates memories, but the Swarm Observability Dashboard lacks a premium visual representation of this process in `apps/web`.

## Design Doc
1. Move or use `AutoDreamPipelineWidget` from `srcs/app/lib/widgets/autodream_pipeline_widget.dart` to `apps/web/lib/widgets/autodream_pipeline_widget.dart`.
2. Integrate into `SwarmObservabilityDashboard`.
