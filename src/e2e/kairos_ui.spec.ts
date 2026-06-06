import { test, expect } from './fixtures';

test('navigation to kairos works', async ({ page }) => {
  await page.goto('/dashboard');
  await page.waitForLoadState('domcontentloaded');

  // Click on the KAIROS link
  const kairosLink = page.getByRole('link', { name: /KAIROS/i });
  await kairosLink.click();

  // Verify KAIROS dashboard loaded
  await expect(page.getByRole('heading', { name: 'Kairos' })).toBeVisible({ timeout: 15000 });
});

test('shared task list loads', async ({ page }) => {
  await page.goto('/kairos');
  await page.waitForLoadState('domcontentloaded');

  await expect(page.getByText('Shared Task List')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#kairos-brain')).toBeVisible({ timeout: 15000 });
});

test('teammate mesh nodes are visible', async ({ page }) => {
  await page.goto('/kairos');
  await page.waitForLoadState('domcontentloaded');

  await expect(page.getByText('Teammate Mesh')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#kairos-nerves')).toBeVisible({ timeout: 15000 });
});

test('autodream memory stats', async ({ page }) => {
  await page.goto('/kairos');
  await page.waitForLoadState('domcontentloaded');

  await expect(page.getByText('AutoDream Memory')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#kairos-memory')).toBeVisible({ timeout: 15000 });
});

test('walkthrough tooltips appear', async ({ page }) => {
  await page.goto('/login');
  await page.evaluate(() => window.localStorage.setItem('TEST_WALKTHROUGH', 'true'));
  await page.goto('/kairos?walkthrough=true&test_walkthrough=true');
  await page.waitForLoadState('domcontentloaded');

  // The walkthrough has a 1 second delay
  await page.waitForTimeout(1500);

  // The walkthrough should show a tooltip
  await expect(page.getByText('Shared tasks appear here when the orchestration backend returns active work.')).toBeVisible({ timeout: 30000 });
});
