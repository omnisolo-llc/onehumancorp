import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test viral_trial_extension', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'viral_trial_extension');
});

test.describe('Viral Trial Extension Loop', () => {
  test('should display the trial extension page and handle share', async ({ page }) => {
    // Navigate to dashboard first to find the link
    await page.goto('/dashboard');

    const extensionLink = page.locator('a[href="/trial-extension"]');
    await expect(extensionLink).toBeVisible();
    await extensionLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Interactive Trial Extension' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Want 7 Extra Days of Pro?' })).toBeVisible();

    // The share button should be present
    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await expect(shareButton).toBeVisible();
    await expect(shareButton).toBeEnabled();

    // Intercept window.open or just click and observe state change if possible in playwright.
    // Since window.open opens a new tab, we mock it or just click and expect UI state.
    // However, playwright handles new pages. Let's just click it.
    const [newPage] = await Promise.all([
      page.waitForEvent('popup'),
      shareButton.click()
    ]);

    // The new page should be the twitter intent
    expect(newPage.url()).toContain('twitter.com/intent/tweet');
    await newPage.close();

    // The UI should show verifying...
    await expect(page.getByText(/Verifying Share.../i)).toBeVisible();

    // After 2 seconds, it should show success
    await expect(page.getByRole('heading', { name: 'Trial Extended!' })).toBeVisible({ timeout: 5000 });
    await expect(page.getByText(/Your Pro trial has been successfully extended by 7 days/i)).toBeVisible();

    const dashboardBtn = page.getByRole('link', { name: 'Return to Dashboard' });
    await expect(dashboardBtn).toBeVisible();
    await dashboardBtn.click();

    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
