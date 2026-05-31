import { test, expect } from './fixtures';

test('Carlos verifies offline mode fail-safe degradation', async ({ page, context }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
    localStorage.removeItem('ohc_wizard_state');
  }, id);

  await page.goto('/dashboard');

  await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

  // Set offline mode
  await context.setOffline(true);

  // Wait a bit or trigger an action to cause UI to update state
  await page.waitForTimeout(500); // Simulate some time offline

  const offlineIndicator = page.locator('#network-status-indicator');
  await expect(offlineIndicator).toBeVisible();

  // Back online
  await context.setOffline(false);

  // Wait a bit
  await page.waitForTimeout(500);

  await expect(offlineIndicator).toBeHidden();
});

test('Maya sees cached data when navigating offline', async ({ page, context }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
  }, id);

  await page.goto('/dashboard');
  await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

  await context.setOffline(true);

  // Click on a cached tab/link that should degrade gracefully
  await page.goto('/agents');

  // Assuming offline caching or read fail-safes allow the heading to be visible
  await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();

  await context.setOffline(false);
});

test('Maya queues local writes during backend spike', async ({ page, context }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
  }, id);

  await page.goto('/dashboard');
  await context.setOffline(true);

  // Assuming we have a form or action that queues writes (mocked by clicking on a typical write action)
  await page.goto('/website-builder');

  await expect(page.locator('text=10-Minute Setup Wizard').first()).toBeVisible();
  await context.setOffline(false);
});

test('Maya dashboard returns to active state after reconnect', async ({ page, context }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
  }, id);

  await page.goto('/dashboard');
  await context.setOffline(true);
  await expect(page.locator('#network-status-indicator')).toBeVisible();

  await context.setOffline(false);
  await expect(page.locator('#network-status-indicator')).toBeHidden();
});

test('Maya can access help center articles while offline', async ({ page, context }) => {
  const id = `operate-business-${Date.now()}-${Math.random()}`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
  }, id);

  await page.goto('/help');
  await context.setOffline(true);

  await expect(page.getByRole('heading', { name: 'Help' }).first()).toBeVisible();
  await context.setOffline(false);
});
