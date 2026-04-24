import { test, expect } from '@playwright/test';

test('Dashboard and Swarm Memory screens display correct observability widgets', async ({ page }) => {
  // 1. Login
  await page.goto('/');
  await page.fill('input[name="username"]', 'admin');
  await page.fill('input[name="password"]', 'admin');
  await page.click('button:has-text("Login")');

  // 2. Navigate to Dashboard and check widgets
  await page.goto('/#/dashboard');

  // Verify Swarm Observability Dashboard
  await expect(page.locator('text=Teammate Mesh Live Feed')).toBeVisible();

  // Verify new Swarm Velocity Widget
  await expect(page.locator('text=Swarm Velocity')).toBeVisible();
  await expect(page.locator('text=Task Completion Rate')).toBeVisible();
  await expect(page.locator('text=Average Latency')).toBeVisible();

  // 3. Navigate to Swarm Memory Mesh and check visualizer
  await page.goto('/#/swarm-memory');

  // Verify the new Parallax visualizer is present (via text check)
  await expect(page.locator('text=AutoDream Consolidation')).toBeVisible();
  await expect(page.locator('text=pgvector dimension: 1536')).toBeVisible();
});
