# Loading States and Perceived Latency in OHC UI

## Overview
Even with Bolt-optimized backends, some operations (like complex AI reasoning) will inherently take time. How we handle this time in the UI determines the perceived performance of the platform.

## 1. Skeleton Screens vs. Spinners
OHC mandates the use of Skeleton Screens (shimmering grey boxes that mimic the final layout) over generic spinners.
- **Why**: Skeletons reduce the "shock" of data appearing suddenly and make the transition feel smoother.
- **Bolt Pattern**: Skeletons must appear within 100ms of a navigation or search event.

## 2. Progressive Content Loading
Large screens (like the Business Insights page) are loaded progressively.
- **Critical Data First**: Total sales and current agent status load first (from cache).
- **Secondary Data**: Detailed charts and full histories load in background streams.

## 3. Interaction Stutter Mitigation
If a backend response takes longer than 150ms, the UI must provide a "Work in Progress" indicator (e.g., an agent "Thinking" animation).
- **Goal**: Maintain the user's sense of progress and prevent them from reloading the page.

## 4. Impact on Perceived Speed
Users report that OHC feels "instant" even when background tasks are running. By managing the visual transition from request to result, we bridge the gap between technical reality and user expectation.
