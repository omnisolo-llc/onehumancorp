import { expect, test } from './fixtures';

/**
 * Persona: Carlos — The Freelance Handyman
 * Business: Local home repairs
 *
 * CUJ: Carlos opens the app on a spotty 3G network while on a job site.
 * The backend is experiencing high latency (simulated chaos) and intermittent
 * request drops. The mobile-first app should remain functional, fail-safe
 * gracefully, and not display white screens of death.
 */
test('Chaos Resilience CUJ: Graceful degradation under high latency and network drops', async ({ page }) => {
  // 1. Intercept network to simulate chaos (high latency / dropped packets)
  await page.route('**/api/**', async route => {
    const url = route.request().url();
    // Simulate packet drop for non-critical metrics/telemetry
    if (url.includes('/metrics') || url.includes('/telemetry')) {
      return route.abort('failed');
    }
    // Simulate high latency (>2s) as per degradation validation rules
    await new Promise(r => setTimeout(r, 2100));
    await route.continue();
  });

  // 2. Carlos opens the dashboard
  await page.goto('/dashboard');

  // 3. The UI should still load the shell and main headings without crashing
  await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('Business Snapshot').first()).toBeVisible({ timeout: 15000 });

  // 4. Carlos checks his inbox for new client requests
  await page.goto('/inbox');
  await expect(page.getByRole('heading', { name: /Customer Messages|Inbox/i }).first()).toBeVisible({ timeout: 15000 });

  // 5. Navigate to AI agents configuration
  await page.goto('/agents');
  await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible({ timeout: 15000 });

  // Verify that the UI didn't crash and core elements remain visible despite delays
  const agentOperations = page.getByRole('button', { name: /The Manager|The Ambassador/i }).first();
  await expect(agentOperations).toBeVisible({ timeout: 15000 });
});
