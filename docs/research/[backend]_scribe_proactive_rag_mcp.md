# [backend]_scribe_proactive_rag_mcp.md

## Documentation Architecture

This document describes the architectural consolidation of the OneHumanCorp documentation UI elements into modular React/Slint components.

### 1. Tooltip Registry
The `TooltipRegistry` provides a centralized Rust repository for context-sensitive plain language hints. Instead of modifying `.slint` UI components and recompiling the frontend whenever a hint is changed, tooltips are fetched at runtime via a Slint global component.

* The global object is `TooltipGlobals` which has a `get_tooltip` pure callback.
* Any UI file can use the `TooltipArea` component to provide hover (`has-hover`) descriptions without explicitly hardcoding strings.

### 2. Help Center Modularity
All Help Center related UI structures (`HelpCenter`, `InteractiveWalkthrough`, `AiHelpChat`, `VideoTutorials`, `ReleaseNotes`, `ApiDocs`) have been separated into a main `Window` wrapper and an internal `*Content` component (which is a standard layout element inheriting `Rectangle`).

* This modularity ensures `HelpCenterContent` can be instantiated inside generic views (like overlays in the Dashboard) without triggering "nesting `Window`" errors in Slint, while maintaining backward compatibility for direct desktop test usage which expect base windows.

### Conclusion
By adopting these component abstractions, we achieve documentation-as-infrastructure. We keep technical noise isolated while empowering product agents to continuously deploy new tooltips and documentation content over backend APIs seamlessly.
