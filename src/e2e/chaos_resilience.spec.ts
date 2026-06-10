import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// Start base smoke validation as a baseline
import { test } from '@playwright/test';
test('smoke', async ({ page, request }) => {
  await test('smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, page, request, 'chaos_resilience_baseline');
});
});

test.describe("Chaos Engineering Validation - Backend/Host Stress Verification", () => {
  // Rather than front-end side-effects or network mocks, we interact with real
  // features that can degrade without mocks.

  test('Graceful failure via corrupted path navigation and 404 boundaries', async ({ page }) => {
    // Navigate via standard user flow
    await page.goto('/login');
    await page.fill('input[placeholder="Email or Username"]', 'Maya');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Navigate to a corrupted deep-link or unconfigured capability that simulates
    // a missing agent configuration state or data sync loss.
    const response = await page.goto('/agents/corrupted-missing-state');

    // Instead of crashing the frontend application with an unhandled exception,
    // it must gracefully return a 404 or boundary page.
    expect(response?.status()).toBe(404);
    await expect(page.locator('text=404').first()).toBeVisible();
  });

  test('CPU Resource Exhaustion Test - Maintains basic input responsiveness', async ({ page }) => {
    // Navigate to login
    await page.goto('/login');

    // Simulate high CPU/memory exhaustion by injecting an aggressive synchronous script,
    // which simulates heavy computation on the main thread locally on the thin client
    await page.addInitScript(() => {
        window.addEventListener('load', () => {
            const start = Date.now();
            while (Date.now() - start < 1500) {
               Math.sqrt(Math.random() * Math.random());
            }
        });
    });

    // Verify the UI still permits interaction and form filling after the artificial lag
    const emailInput = page.locator('input[placeholder="Email or Username"]');
    await emailInput.fill('Maya', { timeout: 10000 }); // Explicit timeout for the stressed UI

    await page.getByRole('button', { name: 'Log In' }).click();

    // Check that despite the heavy thread contention, the form submission succeeds and navigation completes
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
  });

  test('UI Graceful degradation for unavailable chaos report pages', async ({ page }) => {
    const response = await page.goto('/chaos-report');

    // Verify that attempting to access missing administrative/chaos tools gracefully returns a 404.
    // It should not sit indefinitely, spin a loader forever, or throw a 500 error.
    expect(response?.status()).toBe(404);
    await expect(page.locator('text=404').first()).toBeVisible();
  });
});
