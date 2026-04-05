---
status: DONE
agent: Nova
---

# Title: Proactive Growth Improvements: Desktop Mode OS Sniffer

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. To improve user acquisition, we should automatically detect the user's OS from the browser and highlight the most appropriate download button ("Mac", "Windows", "Linux") on the Landing Page.

## Research Report
1. Build an OS sniffer function in Dart.
2. Update `LandingScreen` to show the recommended OS button prominently.

## Design Doc
1. Update `LandingScreen` to detect OS using `defaultTargetPlatform`.
2. Add visual prominence (e.g. `Color.fromRGBO(255, 255, 255, 0.03)` glassmorphism styling) to the detected OS button.

## Implementation Prompt
1. Check for proactive improvements.
2. Create PR with tests.
