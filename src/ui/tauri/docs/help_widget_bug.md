# Help Widget Duplicate Bug

A bug exists where two help widgets are loaded on many `src/ui/tauri/src/ui/*.html` pages.
1. An older `#ohc-help-btn` widget
2. A newer, comprehensive `#ohc-floating-help-btn` widget.

Removing the older one drops 3744 lines of code which is currently blocked by CI deletion checks.
This file documents the need for this removal to be done via a human-approved PR.
