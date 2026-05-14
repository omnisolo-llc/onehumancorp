# Contextual Tooltip Registry Design

## Overview
Every non-obvious UI element in the OHC Small Business App requires a contextual tooltip written in plain language. To prevent UI code from becoming bloated with hardcoded strings and to allow AI agents to independently manage tooltip copy, we utilize a centralized Tooltip Registry.

## Architecture

The Tooltip Registry is a decoupled system with three main components:
1. **The Registry Store**: A unified JSON/Proto definition containing all tooltip keys, text, and metadata.
2. **The Delivery Protocol**: How tooltips are fetched and cached by the frontend (React/Tauri).
3. **The Render Engine**: The UI component responsible for positioning and displaying the tooltip on desktop (hover) and mobile (long-press).

## Schema Definition (Protobuf)

All tooltips are defined using the following Protocol Buffer schema to ensure type safety and seamless cross-language interoperability (Rust backend, TypeScript frontend).

```protobuf
syntax = "proto3";
package ohc.documentation.tooltips;

// Represents a single contextual tooltip for a specific UI element
message TooltipEntry {
  // A unique identifier, typically namespaced by feature (e.g., "marketing.campaign.start_button")
  string key = 1;

  // The primary text to display. Must be plain language, max 2 sentences.
  string plain_language_text = 2;

  // Optional link to a full help center article
  string help_article_url = 3;

  // Target platforms where this tooltip applies
  repeated Platform target_platforms = 4;
}

enum Platform {
  PLATFORM_UNKNOWN = 0;
  PLATFORM_DESKTOP_WEB = 1;
  PLATFORM_MOBILE_WEB = 2;
  PLATFORM_IOS = 3;
  PLATFORM_ANDROID = 4;
  PLATFORM_STANDALONE_DESKTOP = 5;
}

// The complete registry loaded by the client at runtime
message TooltipRegistry {
  // Version hash for cache invalidation
  string version_hash = 1;

  // Map of UI keys to their tooltip definitions
  map<string, TooltipEntry> entries = 2;
}
```

## Frontend Integration (React)

UI developers never hardcode tooltip strings. Instead, they use the custom `useTooltip` hook or wrap elements in a `TooltipProvider`.

### Example Usage:

```tsx
import { TooltipWrapper } from '@ohc/ui/tooltips';

export const CreateCampaignButton = () => {
  return (
    <TooltipWrapper registryKey="marketing.campaign.start_button">
      <button className="ohc-primary-btn">
        Start Campaign
      </button>
    </TooltipWrapper>
  );
};
```

### Rendering Logic
The `TooltipWrapper` component automatically handles interaction logic based on the user's device type:
- **Desktop/Mouse**: Attaches `onMouseEnter` and `onMouseLeave` event listeners. The tooltip is rendered using a portal to avoid z-index clipping issues, positioned dynamically (top, bottom, left, right) based on available viewport space.
- **Mobile/Touch**: Attaches `onTouchStart` and `onTouchEnd` listeners. A long-press (duration > 500ms) triggers the tooltip. The tooltip is displayed as a larger, more legible overlay centered near the touched element, with a clear "dismiss" action.

## Agentic Updates

Because the Tooltip Registry is a standalone data structure, L5 Implementer agents and specialized Documentation agents can independently analyze the UI state (e.g., via Visual State Diffing) and propose PRs to update tooltip copy without modifying the core React components.

This ensures that the "plain language" constraint (Max 8th-grade reading level) is continuously enforced via automated checks running against the registry file during the CI pipeline.
