import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_upgrade_paywall', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_upgrade_paywall');
});

test.describe('Viral SaaS Upgrade Soft Paywall Growth Loop', () => {
  test('should display the upgrade paywall widget on the dashboard', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 1. Verify the widget is visible
    const widgetHeading = page.getByRole('heading', { name: /Unlock AI Autopilot/i });
    await expect(widgetHeading).toBeVisible();

    // 2. Verify the progress text
    await expect(page.getByText('1 / 3')).toBeVisible();
    await expect(page.getByText('2 more to unlock')).toBeVisible();

    // 3. Verify the share/copy button is present
    const copyButton = page.getByRole('button', { name: /Copy Link/i });
    await expect(copyButton).toBeVisible();
    await expect(copyButton).toBeEnabled();

    // 4. Test the copy link interaction
    await copyButton.click();
    await expect(page.getByRole('button', { name: /Copied!/i })).toBeVisible();

    // Check if clipboard has correct format
    // Playwright cannot easily check the clipboard in all headless browsers without permissions setup,
    // but the success state change to "Copied!" verifies the handler ran.
  });
});
