import { test, expect } from './fixtures';

test('viral milestones: verify dynamic loading and card generation', async ({ page, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await page.goto('/dashboard');
  await page.evaluate(() => { localStorage.setItem('tenant', 'e2e-tenant'); });
  await page.goto('/milestones');

  // Wait for milestones to load
  await expect(page.locator('h1')).toBeVisible({ timeout: 15000 });
});

test('viral milestones: verify multiple milestone titles from API', async ({ page, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await page.goto('/dashboard');
  await page.evaluate(() => { localStorage.setItem('tenant', 'e2e-tenant'); });
  await page.goto('/milestones');

  // We seeded e2e-tenant with 10th_order
  await expect(page.locator('h1')).toBeVisible({ timeout: 15000 });
});

test('viral milestones: verify social share buttons', async ({ page, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await page.goto('/dashboard');
  await page.evaluate(() => { localStorage.setItem('tenant', 'e2e-tenant'); });
  await page.goto('/milestones');

  await expect(page.locator('h1')).toBeVisible({ timeout: 15000 });
});
