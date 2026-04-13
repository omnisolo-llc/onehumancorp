---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: A/B Experiment Tracker

## Problem Statement
The growth strategy audit indicates that an A/B test should highlight "Local Sovereignty" versus "Cloud Convenience". Currently, the growth service lacks an internal experiment tracker to log variant assignments and calculate conversion rates reliably.

## Research Report
Adding an `ExperimentTracker` in `services/growth` satisfies the Nova protocol to proactively implement growth-oriented features.

## Design Doc
1. Implement `ExperimentTracker` with `TrackAssignment` and `MarkConverted` in `services/growth/experiments.go`.
2. Compute metrics for conversion rate.

## Implementation Prompt
1. Check for proactive improvements.
2. Create PR with tests.
