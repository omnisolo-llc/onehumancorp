import { test, expect } from '@playwright/test';

test.describe('Hybrid Landing Page CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the hybrid-landing page
    await page.goto('/hybrid-landing');
  });

  test('Local Sovereignty card rendering and sub-items', async ({ page }) => {
    const card = page.locator('text="Local Sovereignty"').locator('..');
    await expect(card).toBeVisible();
    await expect(page.getByText(/Zero Data Leakage/i)).toBeVisible();
    await expect(page.getByText(/Air-Gapped Autonomy/i)).toBeVisible();
    await expect(page.getByText(/Self-Hosted LLMs/i)).toBeVisible();
  });

  test('Cloud Convenience card rendering and sub-items', async ({ page }) => {
    const card = page.locator('text="Cloud Convenience"').locator('..');
    await expect(card).toBeVisible();
    await expect(page.getByText(/Team Collaboration/i)).toBeVisible();
    await expect(page.getByText(/Anywhere Access/i)).toBeVisible();
    await expect(page.getByText(/Fully Managed/i)).toBeVisible();
  });

  test('Download Desktop button triggers loading state', async ({ page }) => {
    // Setup dialog handler to dismiss the simulation alert
    page.on('dialog', dialog => dialog.accept());

    const downloadBtn = page.getByRole('button', { name: 'Download Desktop' });
    await expect(downloadBtn).toBeVisible();
    await downloadBtn.click();

    // It should immediately say downloading
    await expect(page.getByText('Downloading...')).toBeVisible();

    // After 1500ms (as per source), the alert pops up, then button returns to "Download Desktop"
    await expect(page.getByRole('button', { name: 'Download Desktop' })).toBeVisible({ timeout: 5000 });
  });

  test('Start Web Trial link is present and navigates to dashboard', async ({ page }) => {
    const startLink = page.getByRole('link', { name: 'Start Web Trial' });
    await expect(startLink).toBeVisible();
    await expect(startLink).toHaveAttribute('href', '/dashboard');
  });

  test('OHC Hybrid OS heading is rendered', async ({ page }) => {
    await expect(page.getByText('OHC Hybrid OS', { exact: true })).toBeVisible();
  });
});
