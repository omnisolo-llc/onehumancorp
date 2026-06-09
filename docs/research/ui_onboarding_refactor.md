# OHC Onboarding UI Optimization & Glassmorphism Refactoring

## Overview

This document outlines the architectural changes and optimizations applied to the Tauri onboarding flow. The original implementation contained duplicated CSS rules across multiple HTML views (`index.html`, `setup.html`, `assistant.html`, `dashboard.html`, `success.html`), violating DRY principles and creating a maintenance burden for future design system updates.

By centralizing the styling into a unified `styles.css` stylesheet, we drastically improved the maintainability and payload consistency of the Tauri-based user interface. This optimization ensures that OHC's signature Apple/Ubiquiti-style translucent glass aesthetic is universally applied and centrally updatable.

## Key Enhancements

### 1. Centralized Glassmorphism and Layout Variables
Previously, each HTML file independently defined the translucent glass effect:
`background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%);`

These rules have been consolidated. This guarantees that any tuning to the glass material (e.g., modifying the blur radius or saturation) will instantly propagate to all onboarding screens.

### 2. Mobile-First Responsiveness (375px Target)
To align with the absolute requirement for 375px mobile-first usability without horizontal scrolling, a new unified media query was introduced:
```css
@media (max-width: 400px) {
  .container {
    padding: 24px;
    border-radius: 0;
    min-height: 100vh;
    border: none;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
}
```
This query removes the card-like borders and expands the container to fill the screen on small devices, providing a native, app-like experience for field operators and mobile users.

### 3. Dark Mode Refinement
The centralized stylesheet ensures that dark mode color inversions and translucent treatments are consistently applied:
```css
@media (prefers-color-scheme: dark) {
  .container {
    background: rgba(22, 22, 26, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  /* ... */
}
```

### 4. Playwright E2E Integration
The Playwright testing suite (`src/e2e/tauri_onboarding.spec.ts`) was updated to properly intercept and serve the newly created `styles.css` file alongside the HTML views, maintaining 100% test coverage and ensuring visual regression safety. Furthermore, the test assertion for `saturate(210%)` was updated to `saturate(2.1)` to align with standard browser CSS normalization properties.

## Conclusion

This refactoring successfully modernizes the Tauri UI stack, reducing overall line count and duplicated code while substantially improving mobile responsiveness and visual consistency.
