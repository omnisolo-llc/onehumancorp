import { test, expect } from './fixtures';
import { e2eTenantId } from './fixtures';

test.describe('Trial Extension', () => {
  test('should display the trial extension page', async ({ page }) => {
    await page.goto('/trial-extension');
    await expect(page.getByText('Interactive Trial Extension')).toBeVisible();
    await expect(page.getByText('Want 7 Extra Days of Pro?')).toBeVisible();
  });

  test('should claim trial extension successfully', async ({ page }) => {
    // Navigate to the page
    await page.goto('/trial-extension');

    // Stub window.open so the test doesn't actually open Twitter
    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    // We assume the fixture has already seeded a tenant and logged in
    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await expect(shareButton).toBeVisible();

    await shareButton.click();

    await expect(page.getByText(/Verifying Share.../i)).toBeVisible();
    await expect(page.getByText('Trial Extended!')).toBeVisible({ timeout: 10000 });
  });

  test('should fail gracefully if backend returns error', async ({ page }) => {
    await page.goto('/trial-extension');

    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    // Mock fetch to simulate a backend error for this specific test
    await page.route('/api/v1/growth/trial-extension/claim', route => {
      route.fulfill({
        status: 500,
        body: JSON.stringify({ error: 'Internal Server Error' })
      });
    });

    // Capture alert dialog
    let dialogMessage = '';
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await shareButton.click();

    // Verify alert message was shown
    await expect(async () => {
        expect(dialogMessage).toContain('Failed to claim trial extension');
    }).toPass({ timeout: 10000 });

    // The button should still be enabled (or reset) and the success message should NOT be shown
    await expect(page.getByText('Trial Extended!')).not.toBeVisible();
  });

  test('should fail gracefully on network error', async ({ page }) => {
    await page.goto('/trial-extension');

    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    await page.route('/api/v1/growth/trial-extension/claim', route => {
      route.abort('failed');
    });

    // Capture alert dialog
    let dialogMessage = '';
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await shareButton.click();

    await expect(async () => {
        expect(dialogMessage).toContain('Error claiming trial extension');
    }).toPass({ timeout: 10000 });

    await expect(page.getByText('Trial Extended!')).not.toBeVisible();
  });

  test('should have a working back to dashboard link', async ({ page }) => {
    await page.goto('/trial-extension');
    const backLink = page.getByRole('link', { name: /Back to Dashboard/i });
    await expect(backLink).toBeVisible();
    await backLink.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
