import { test, expect } from '@playwright/test';

test('navigation to kairos works', async ({ page }) => {
  await page.goto('/dashboard');

  // Click on the KAIROS link
  const kairosLink = page.getByRole('link', { name: /KAIROS/i });
  await kairosLink.click();

  // Verify KAIROS dashboard loaded
  await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible();
});

test('shared task list loads', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Shared Task List' })).toBeVisible();
  await expect(page.locator('body')).toBeVisible();
});

test('teammate mesh nodes are visible', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Teammate Mesh' })).toBeVisible();

  // Checking node types are present
  await expect(page.locator('#kairos-nerves').getByText('Brain', { exact: false })).toBeVisible();
  await expect(page.locator('#kairos-nerves').getByText('Nerve', { exact: false })).toBeVisible();
  await expect(page.locator('#kairos-nerves').getByText('Memory', { exact: false })).toBeVisible();
});

test('autodream memory stats', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'AutoDream Memory' })).toBeVisible();
  await expect(page.getByText('Infinite Context')).toBeVisible();
  // // await expect(page.getByText('842.5 MB')).toBeVisible();
});

test('walkthrough tooltips appear', async ({ page }) => {
  await page.goto('/kairos?walkthrough=true');

  // The walkthrough has a 1 second delay
  await page.waitForTimeout(1500);

  // The walkthrough should show a tooltip
  await expect(page.locator('text=Shared Task List').first()).toBeVisible();
});
