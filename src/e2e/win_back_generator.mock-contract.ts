import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('win_back_generator', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'win_back_generator');
});

test.describe('Customer Win-Back Generator', () => {
  test('should generate and schedule a win-back campaign', async ({ page }) => {
    // 1. Navigate to the dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 2. Click the Customer Win-Back link
    const winBackLink = page.getByRole('link', { name: /Customer Win-Back/i });
    await expect(winBackLink).toBeVisible();
    await winBackLink.click();

    // 3. Verify we are on the generator page
    await expect(page).toHaveTitle('Customer Win-Back Generator');
    await expect(page.getByRole('heading', { name: 'Customer Win-Back' })).toBeVisible();

    // 4. Fill out the form
    await page.locator('#inactive-days').selectOption('60');
    await page.locator('#offer').fill('30% off your next order');
    await page.locator('#tone').selectOption('professional');

    // 5. Intercept the AI generation API
    await page.route('/api/v1/growth/campaign/generate-win-back', async (route) => {
      const request = route.request();
      expect(request.method()).toBe('POST');
      const postData = request.postDataJSON();

      expect(postData.offer).toBe('30% off your next order');

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          subject: 'We miss you! Here is 30% off',
          body: 'Hi there,\n\nWe noticed you haven\'t been around lately. Enjoy 30% off your next order with code WINBACK30.\n\nBest,\nThe Team'
        }),
      });
    });

    // 6. Click generate and wait for preview
    const generateBtn = page.getByRole('button', { name: 'Draft Campaign with AI' });
    await generateBtn.click();

    // Verify button changes state during processing
    await expect(page.getByRole('button', { name: 'Drafting...' })).toBeVisible();

    const draftPreview = page.locator('#draft-preview');
    await expect(draftPreview).toBeVisible({ timeout: 10000 });
    await expect(draftPreview).toContainText('We miss you! Here is 30% off');

    // 7. Intercept the campaign scheduling API
    await page.route('/api/v1/growth/campaign/send-cart', async (route) => {
      const request = route.request();
      expect(request.method()).toBe('POST');
      const postData = request.postDataJSON();

      expect(postData.type).toBe('win_back');

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, message: "Email scheduled to be sent successfully" })
      });
    });

    // 8. Click schedule campaign
    const scheduleBtn = page.getByRole('button', { name: 'Schedule Campaign' });
    await scheduleBtn.click();

    // 9. Verify success state
    const statusMsg = page.locator('#status-msg');
    await expect(statusMsg).toBeVisible({ timeout: 10000 });
    await expect(statusMsg).toContainText('✅ Campaign scheduled successfully!');
  });
});
