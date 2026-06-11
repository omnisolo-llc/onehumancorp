# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: videos.spec.ts >> In-App Video Tutorials >> renders videos tab, fetches videos, and opens/closes the modal player
- Location: src/ui/next/src/e2e/videos.spec.ts:4:9

# Error details

```
Error: expect(locator).not.toBeVisible() failed

Locator:  locator('div.fixed.z-\\[100\\]')
Expected: not visible
Received: visible
Timeout:  5000ms

Call log:
  - Expect "not toBeVisible" with timeout 5000ms
  - waiting for locator('div.fixed.z-\\[100\\]')
    14 × locator resolved to <div class="jsx-5b22abac5bde631 fixed bottom-6 left-1/2 -translate-x-1/2 z-[100] flex flex-col items-center gap-4 w-full max-w-[375px] px-4 pointer-events-none">…</div>
       - unexpected value "visible"

```

```yaml
- button "Voice Assistant":
  - img
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('In-App Video Tutorials', () => {
  4  |     test('renders videos tab, fetches videos, and opens/closes the modal player', async ({ page }) => {
  5  |         // Go to a page where HelpWidget is available (layout.tsx ensures it's on pages like dashboard)
  6  |         await page.goto('/dashboard'); // Use the dashboard or any public page where layout applies
  7  |
  8  |         // The help widget should be present.
  9  |         const helpButton = page.locator('#help-widget-container button').first();
  10 |         await expect(helpButton).toBeVisible();
  11 |
  12 |         // Click the help widget floating button to open the menu
  13 |         await page.goto("/help");
  14 |
  15 |
  16 |         // Wait for the videos to be fetched and rendered
  17 |         // The API returns 10 videos. We'll wait for at least one to show up.
  18 |         // The videos are rendered with titles, like 'How to set up your first store easily'
  19 |         const firstVideoTitle = page.locator('h3', { hasText: 'How to set up your first store easily' });
  20 |         await expect(firstVideoTitle).toBeVisible();
  21 |
  22 |         // Verify some other videos are present
  23 |         await expect(page.locator('h3', { hasText: 'Accept your first payment' })).toBeVisible();
  24 |
  25 |         // Click on the first video to open the modal player
  26 |         // The video container is a div parent of the title
  27 |         const videoContainer = firstVideoTitle.locator('..').locator('..'); // go up to the container
  28 |         await videoContainer.click();
  29 |
  30 |         // Verify the modal player opens
  31 |         const modalContainer = page.locator('div.fixed.z-\\[100\\]');
  32 |         await expect(modalContainer.first()).toBeVisible();
  33 |
  34 |         // Verify the modal has the correct mobile constraints (max-w-[375px])
  35 |         await expect(modalContainer.locator('div.max-w-\\[375px\\]')).toBeVisible();
  36 |
  37 |         // Verify the video title is shown in the modal header
  38 |         await expect(modalContainer.locator('h3', { hasText: 'How to set up your first store easily' })).toBeVisible();
  39 |
  40 |         // Click the close button
  41 |         const closeButton = modalContainer.locator('button[aria-label="Close video"]');
  42 |         await closeButton.first().dispatchEvent("click");
  43 |
  44 |         // Verify the modal player closes
> 45 |         await expect(modalContainer).not.toBeVisible();
     |                                          ^ Error: expect(locator).not.toBeVisible() failed
  46 |     });
  47 | });
  48 |
```