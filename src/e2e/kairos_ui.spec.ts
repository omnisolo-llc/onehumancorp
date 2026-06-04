import { test, expect } from '@playwright/test';

test('navigation to kairos works', async ({ page }) => {
<<<<<<< HEAD
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('/dashboard');
  await page.waitForLoadState('networkidle');
=======
  await page.goto('/dashboard');
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)

  // Click on the KAIROS link
  const kairosLink = page.getByRole('link', { name: /KAIROS/i });
  await kairosLink.click();

  // Verify KAIROS dashboard loaded
<<<<<<< HEAD
  await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible({ timeout: 15000 });
});

test('shared task list loads', async ({ page }) => {
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('/kairos');
  await page.waitForLoadState('networkidle');

  await expect(page.getByRole('heading', { name: 'Shared Task List' })).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('Inventory Reorder Strategy')).toBeVisible({ timeout: 15000 });
});

test('teammate mesh nodes are visible', async ({ page }) => {
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('/kairos');
  await page.waitForLoadState('networkidle');

  await expect(page.getByRole('heading', { name: 'Teammate Mesh' })).toBeVisible({ timeout: 15000 });

  // Checking node types are present
  await expect(page.locator('#kairos-nerves').getByText('Brain', { exact: true })).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#kairos-nerves').getByText('Nerve', { exact: true })).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#kairos-nerves').getByText('Memory', { exact: true })).toBeVisible({ timeout: 15000 });
});

test('autodream memory stats', async ({ page }) => {
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('/kairos');
  await page.waitForLoadState('networkidle');

  await expect(page.getByRole('heading', { name: 'AutoDream Memory' })).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('Infinite Context')).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('842.5 MB')).toBeVisible({ timeout: 15000 });
});

test('walkthrough tooltips appear', async ({ page }) => {
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
  await page.goto('/kairos?walkthrough=true');
  await page.waitForLoadState('networkidle');
=======
  await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible();
});

test('shared task list loads', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Shared Task List' })).toBeVisible();
  await expect(page.getByText('Inventory Reorder Strategy')).toBeVisible();
});

test('teammate mesh nodes are visible', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Teammate Mesh' })).toBeVisible();

  // Checking node types are present
  await expect(page.locator('#kairos-nerves').getByText('Brain', { exact: true })).toBeVisible();
  await expect(page.locator('#kairos-nerves').getByText('Nerve', { exact: true })).toBeVisible();
  await expect(page.locator('#kairos-nerves').getByText('Memory', { exact: true })).toBeVisible();
});

test('autodream memory stats', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'AutoDream Memory' })).toBeVisible();
  await expect(page.getByText('Infinite Context')).toBeVisible();
  await expect(page.getByText('842.5 MB')).toBeVisible();
});

test('walkthrough tooltips appear', async ({ page }) => {
  await page.goto('/kairos?walkthrough=true');
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)

  // The walkthrough has a 1 second delay
  await page.waitForTimeout(1500);

  // The walkthrough should show a tooltip
<<<<<<< HEAD
  await expect(page.getByText("The Shared Task List is the 'Brain'")).toBeVisible({ timeout: 30000 });
=======
  await expect(page.getByText("The Shared Task List is the 'Brain'")).toBeVisible();
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
});
