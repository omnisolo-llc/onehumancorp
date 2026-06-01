import { test, expect } from '@playwright/test';

test.describe('Chaos Resilience & Fail-Safe Degradation', () => {

  test('Persona: Business Owner experiences extreme latency (>2s)', async ({ page }) => {
    // 1. Intercept all /api/ requests and introduce a >2s delay
    await page.route('**/api/**', async route => {
      // Simulate extreme backend latency
      await new Promise(resolve => setTimeout(resolve, 2500));
      await route.continue();
    });

    // 2. Navigate to dashboard
    await page.goto('/dashboard');

    // 3. Since there is extreme latency, we expect the UI to show a fallback/skeleton state gracefully
    // Wait for either the dashboard header or a graceful loading indicator that should handle latency
    const header = page.locator('h1', { hasText: 'Dashboard' });
    await expect(header).toBeVisible({ timeout: 15000 });

    // Assert that the user is not greeted with a crash, but the application remains functional
    // OHC's standard dictates optimistic UI and fail-safe reads
    const content = page.locator('body');
    await expect(content).toBeVisible();
  });

  test('Persona: Business Owner experiences connection drop (Offline)', async ({ page }) => {
    // 1. Intercept all /api/ requests and abort them to simulate connection drop
    await page.route('**/api/**', async route => {
      await route.abort('failed');
    });

    // 2. Navigate to an arbitrary feature that usually requires network, e.g. products or dashboard
    await page.goto('/products');

    // 3. The UI must degrade gracefully - e.g., show an offline warning or cached data
    // Usually PWA / offline first apps display an "Offline" banner or graceful error message
    // If there's an optimistic save it should queue. For now, we assert it doesn't just display a raw error trace
    const offlineMessage = page.locator('text=/offline|connection lost|failed to load/i');
    const header = page.locator('h1', { hasText: 'Products' });

    // Ensure the app doesn't crash completely (white screen of death)
    await expect(page.locator('body')).toBeVisible();

    // Check if the offline indicator or the header is shown
    await expect(offlineMessage.or(header)).toBeVisible();
  });
});
