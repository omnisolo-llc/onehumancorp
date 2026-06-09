import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_ai_savings_widget');

test.describe('Viral AI Time Savings Widget Growth Loop', () => {
  test('should display the widget on dashboard and handle the trial extension loop', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 1. Verify the widget is visible
    const widgetHeading = page.getByRole('heading', { name: /You saved .* hours this week/i });
    await expect(widgetHeading).toBeVisible();

    // 2. Verify the share button is present
    const shareButton = page.getByRole('button', { name: /Share to get 7 Days Pro/i });
    await expect(shareButton).toBeVisible();
    await expect(shareButton).toBeEnabled();

    // 3. Mock window.open to prevent opening a new tab
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // 4. Click the share button to trigger the API call
    await shareButton.click();

    // 5. Verify the loading state
    await expect(page.getByText(/Verifying Share.../i)).toBeVisible();

    // 6. Verify the success state
    const successHeading = page.getByRole('heading', { name: 'Trial Extended!' });
    await expect(successHeading).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(/Your Pro trial has been successfully extended by 7 days/i)).toBeVisible();
  });
});
