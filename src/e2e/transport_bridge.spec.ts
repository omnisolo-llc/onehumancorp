import { test, expect } from './fixtures';

test('universal transport bridge end-to-end lifecycle', async ({ page }) => {
  // Login via the UI
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();

  await page.fill('input[name="email"]', 'admin@ohc.inc');
  await page.fill('input[name="password"]', 'admin123');
  await page.click('button[type="submit"]');

  // Verify successful login
  await expect(page.url()).toContain('/dashboard');

  // Exercise the transport bridge (e.g. going to agents page)
  await page.click('a[href="/agents"]');
  await expect(page.url()).toContain('/agents');

  // Wait for some network or websocket activity to show the agents
  // and assert UI state that implies transport was successful
  const firstAgent = page.locator('.agent-card').first();
  await expect(firstAgent).toBeVisible({ timeout: 10000 });
});
