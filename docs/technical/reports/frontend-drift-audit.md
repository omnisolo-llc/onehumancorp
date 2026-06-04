# Frontend UI/UX Drift Audit Report

This report outlines the discrepancies identified between the canonical end-to-end tests (`src/e2e/current_app_smoke.ts`) and the current implementation of the Next.js frontend application (`src/ui/next`). These drifts cause test failures and represent a deviation from the intended product design.

## Missing or Broken Screens

1.  **`/login`**:
    -   **Issue**: The route does not exist (returns HTTP 404).
    -   **Expected**: Tests expect a login page with a heading "Login".

2.  **`/agents`**:
    -   **Issue**: The agent cards are not implemented as buttons.
    -   **Expected**: The test expects an element matching `page.getByRole('button', { name: /The Ambassador/ })`. Currently, it's rendered as a `<div>`.

3.  **`/website-builder`**:
    -   **Issue**: The heading text has drifted.
    -   **Expected**: The test expects `page.getByRole('heading', { name: '10-Minute Setup Wizard' })`.
    -   **Actual**: The current heading is "Welcome to OHC Smart Builder" or "Your business, live in minutes.".

4.  **`/integrations`**:
    -   **Issue**: The heading text has drifted, and a section is missing.
    -   **Expected**: The test expects `page.getByRole('heading', { name: 'Connect Custom Software' })` and `page.getByRole('heading', { name: 'Social Media Accounts' })`.
    -   **Actual**: The current heading is "Tool Integrations". The "Social Media Accounts" heading is not present.

5.  **`/storefront-builder`**:
    -   **Issue**: The element with class `.builder-block` is missing.
    -   **Expected**: The test expects `page.locator('.builder-block')` to be visible.
    -   **Actual**: The page renders `DraggableBlock` and `SmartBlock` components, but the specific class `.builder-block` is not used in the DOM. It has a "1-Tap Launch" button instead.

6.  **`/api/v1/growth/storefront/og-card`**:
    -   **Issue**: The API endpoint returns the wrong Content-Type.
    -   **Expected**: The test expects the response header `content-type` to contain `image/svg+xml`.
    -   **Actual**: The endpoint uses `next/og` `ImageResponse`, which returns `image/png`.

## Recommendation
An implementation agent should restore the UI elements and API responses to match the expectations defined in `src/e2e/current_app_smoke.ts` to ensure the canonical baseline is maintained.
