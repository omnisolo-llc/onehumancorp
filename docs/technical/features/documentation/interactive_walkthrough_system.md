# Interactive Walkthrough System

## Overview
Interactive walkthroughs provide step-by-step, in-app tours for key OHC flows (e.g., "Set up your store", "Accept your first payment"). To respect the user experience and avoid intrusive popups or modals that block the application, walkthroughs are implemented using a lightweight, state-machine-driven highlight and speech bubble system.

## Design Constraints
1. **No Modals**: Walkthroughs must never use blocking modal dialogs.
2. **Contextual Spotlights**: The UI element being discussed must be subtly highlighted (e.g., using a darkened backdrop with a clear "cutout" around the target element).
3. **Interruptible**: The user must be able to dismiss the walkthrough at any time by clicking a close button or pressing Escape.
4. **Resumable**: Walkthrough state must be persisted so a user can resume a tour later if they are interrupted.

## Architecture: The Walkthrough State Machine

Walkthroughs are managed by a centralized, distributed state machine (built on top of the KAIROS Distributed State Machine infrastructure).

### State Definition (JSON Representation)
Each walkthrough is defined as a linear sequence of steps:

```json
{
  "walkthrough_id": "setup_store",
  "title": "Set up your store in 3 minutes",
  "target_audience": "new_users",
  "steps": [
    {
      "step_index": 0,
      "target_selector": "#nav-store-settings",
      "speech_bubble": {
        "title": "Welcome to your Store!",
        "content": "Let's get your business online. First, click here to open your store settings.",
        "position": "bottom-right"
      },
      "advance_trigger": "click"
    },
    {
      "step_index": 1,
      "target_selector": "#input-store-name",
      "speech_bubble": {
        "title": "Name your business",
        "content": "Type the name your customers know you by.",
        "position": "top"
      },
      "advance_trigger": "input_change"
    }
  ]
}
```

## Rendering the Speech Bubble Overlay

The overlay is rendered using React Portals to append the element to the root `<body>` node, ensuring it is not affected by the parent component's CSS `overflow` or `z-index` properties.

1. **Target Identification**: The state machine uses `document.querySelector` to find the target element defined in the current step.
2. **Position Calculation**: The exact coordinates and dimensions of the target element are calculated using `getBoundingClientRect()`.
3. **Backdrop Generation**: An SVG overlay or a `box-shadow` hack (e.g., `box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5)`) is applied to create a "spotlight" effect around the target element.
4. **Speech Bubble Placement**: The speech bubble component is positioned relative to the target element based on the `position` preference (e.g., `bottom-right`), using a library like Floating UI to handle edge-collision detection and viewport boundaries.

## Persistence and Tracking

User progress through walkthroughs is tracked and synced to the backend to ensure a consistent experience across devices.

- **Local State**: Progress is immediately saved to `localStorage` (Cloud) or SQLite (Standalone).
- **Backend Sync**: A debounced background sync sends the `current_step_index` to the OHC telemetry server.
- **Analytics**: Completion rates, drop-off points, and time-spent-per-step are recorded to help technical writers optimize the walkthrough content.
