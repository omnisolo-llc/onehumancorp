import { test, expect } from './fixtures';

test.describe('KAIROS AI OS Orchestration (Phase 4)', () => {
  test('Test 1: Verify the dashboard loads and displays the Dashboard heading', async ({ page }) => {
    test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('Test 2: Navigate to /agents page and verify the Agents heading', async ({ page }) => {
    test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('Test 3: Verify the Business Snapshot section is displayed', async ({ page }) => {
    test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Business Snapshot' })).toBeVisible();
  });

  test('Test 4: Verify the Today\'s Sales section is displayed', async ({ page }) => {
    test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await expect(page.getByText('Today\'s Sales')).toBeVisible();
  });

  test('Test 5: Complete end-to-end CUJ logging in and checking sections', async ({ page }) => {
    test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business Snapshot' })).toBeVisible();
    await expect(page.getByText('Today\'s Sales')).toBeVisible();
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});
