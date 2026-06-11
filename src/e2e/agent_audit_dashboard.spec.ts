import { test, expect } from '@playwright/test';

test('agent audit dashboard rendering and glassmorphism', async ({ page }) => {
  await page.viewportSize();

  await page.goto('/agent-audit-dashboard');

  await expect(page.locator('h1', { hasText: 'Agent Audit Dashboard' })).toBeVisible({ timeout: 15000 });
  await expect(page.locator('text=Cost Tracker')).toBeVisible();
  await expect(page.locator('text=Operations')).toBeVisible();
  await expect(page.locator('text=Marketing & Advertising')).toBeVisible();
  await expect(page.locator('text=Violation Feed')).toBeVisible();
  await expect(page.locator('text=Agent Health: Optimal')).toBeVisible();
  await expect(page.locator('text=Campaigns Sync: Active')).toBeVisible();

  // Verify glassmorphism style drift on dashboard panels
  const panel = page.locator('.app-panel').first();
  await expect(panel).toBeVisible();
  await expect(panel).toHaveCSS('backdrop-filter', /blur\(30px\)/);
  await expect(panel).toHaveCSS('border-radius', '16px');

  // Verify glassmorphism style drift on dashboard cards
  const card = page.locator('.app-card').first();
  await expect(card).toBeVisible();
  await expect(card).toHaveCSS('backdrop-filter', /blur\(30px\)/);
  await expect(card).toHaveCSS('border-radius', '16px');
});

