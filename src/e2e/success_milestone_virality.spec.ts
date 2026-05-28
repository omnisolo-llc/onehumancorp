import { test, expect } from './fixtures';

test.describe('Success Milestone Virality Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard where the growth loop is implemented
    await page.goto('/dashboard');
  });

  test('should display success milestone alert and allow sharing to X', async ({ page, context, request }) => {
    // Set mock local storage total_orders to 10
    await page.evaluate(() => localStorage.setItem('total_orders', '10'));
    // Need to trigger re-render of component or reload to read the local storage
    await page.reload();

    // Evaluate mock for window.open before action since page context won't catch it quickly enough in some setups
    await page.evaluate(() => {
      window.open = function(url) {
        window['mockOpenedUrl'] = url;
        return null;
      };

      // Also overwrite the global alert to not block execution if there is an issue with dialogPromise
      window.alert = function(msg) {
        window['mockAlertMsg'] = msg;
      }
    });

    // 1. Locate the milestone alert section
    const milestoneHeading = page.getByRole('heading', { name: '10th Order Milestone Reached!' });
    await expect(milestoneHeading).toBeVisible({ timeout: 10000 });

    // 2. Locate the "Share Milestone on X" button
    const shareBtn = page.getByRole('button', { name: 'Share Milestone on X' });
    await expect(shareBtn).toBeVisible();

    // Click the share button to complete the growth loop action
    await shareBtn.click({ force: true }); // Use force to bypass any overlay issues

    // Verify window.open opened a Twitter intent URL with referral link
    const openedUrl = await page.evaluate(() => window['mockOpenedUrl']);
    expect(openedUrl).toContain('twitter.com/intent/tweet');
    expect(openedUrl).toContain('ohc%3A%2F%2Fjoin%3Fref%3D');

    // Verify the alert message thanked the user
    const alertMsg = await page.evaluate(() => window['mockAlertMsg']);
    expect(alertMsg).toContain('Thanks for sharing!');

    // 5. Verify the milestone alert is automatically dismissed after sharing
    await expect(milestoneHeading).toBeHidden();
  });
});
