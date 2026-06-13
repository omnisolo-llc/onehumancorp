import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('lens audit base', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'lens_audit_base');
});

test('lens audit cost dashboard checks', async ({ page, adminUser }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('/login');
  await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
  await page.fill('input[placeholder="Password"]', 'password123');
  await page.getByRole('button', { name: 'Log In' }).click();

  await page.goto('/cost-dashboard');
  await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' }).first()).toBeVisible({ timeout: 15000 });

  const llmCost = page.locator('#cost-dashboard-llm');
  await expect(llmCost).toBeVisible();

  const totalCosts = page.locator('#cost-dashboard-total');
  await expect(totalCosts).toBeVisible();
});

test('lens audit pricing checks', async ({ page, adminUser }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('/login');
  await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
  await page.fill('input[placeholder="Password"]', 'password123');
  await page.getByRole('button', { name: 'Log In' }).click();

  await page.goto('/pricing');
  await expect(page.getByText('Pricing Plans')).toBeVisible({ timeout: 15000 });
});

test('lens audit website builder checks', async ({ page, adminUser }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('/login');
  await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
  await page.fill('input[placeholder="Password"]', 'password123');
  await page.getByRole('button', { name: 'Log In' }).click();

  await page.goto('/website-builder');
  await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' }).first()).toBeVisible({ timeout: 15000 });
});

test('lens audit integrations checks', async ({ page, adminUser }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('/login');
  await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
  await page.fill('input[placeholder="Password"]', 'password123');
  await page.getByRole('button', { name: 'Log In' }).click();

  await page.goto('/integrations');
  await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible({ timeout: 15000 });
});
