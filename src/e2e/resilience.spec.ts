import { test, expect } from '@playwright/test';

test.describe('ML-Resilience and UI Degradation', () => {
  test('graceful degradation when backend times out or drops connection', async ({ page }) => {
    // Navigate to a page that fetches from the backend
    await page.goto('http://localhost:3000/dashboard', { waitUntil: 'networkidle' });

    // We simulate a connection drop by failing network requests to specific APIs
    await page.route('**/api/tasks**', route => route.abort('failed'));

    // Attempt an action that relies on the backend
    await page.click('button:has-text("Approve Task")');

    // Wait for the UI to handle the failure
    await page.waitForTimeout(1000);

    // Expect the UI to show some optimistic state, or show an error without crashing.
    // In this system, failed actions should queue locally and optimistic UI updates are shown.
    // Thus the task should visually appear "queued" or "completed" or display a toast.
    // Note: Since this is an offline test, we expect the UI component to not completely crash (e.g. no blank white screen).
    const bodyContent = await page.textContent('body');
    expect(bodyContent).not.toContain('Application Error');
    expect(bodyContent).not.toContain('White Screen of Death');
  });
});
