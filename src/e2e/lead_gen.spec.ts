import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Hyperlocal Lead Generation Agent', () => {
  test('should allow a business owner to start a lead generation campaign and see inbox conversion', async ({ page }) => {
    // 1. Owner opens the mobile app and navigates to the Marketing/Dashboard
    await page.goto('/dashboard');

    // Look for the new Lead Gen card
    const leadGenCard = page.getByRole('link', { name: /Want more local jobs this week\?/i });
    await expect(leadGenCard).toBeVisible();
    await leadGenCard.click();

    // 2. Owner inputs a weekly budget and service radius
    await expect(page).toHaveURL(/.*lead-gen/);
    await expect(page.getByRole('heading', { name: 'Start Finding Jobs' })).toBeVisible();

    const budgetInput = page.getByLabel('Weekly Budget ($)');
    await budgetInput.fill('50');

    const zipCodeInput = page.getByLabel('Target Zip Code / Radius');
    await zipCodeInput.fill('90210');

    // 3. The platform initiates the LeadGenCampaign via the backend AI job queue
    const startButton = page.getByRole('button', { name: 'Start Finding Jobs' });

    // Use promise.all to wait for navigation while clicking
    const [response] = await Promise.all([
      page.waitForResponse(res => res.url().includes('/api/v1/growth/campaign/start-lead-gen') && res.status() === 200),
      startButton.click(),
    ]);

    // Should redirect back to dashboard with a query parameter
    await expect(page).toHaveURL(/.*dashboard\?lead_gen_started=1/);

    // 4. Ideally, we wait for the background job to finish and check the inbox/orders
    // Navigate to inbox to verify
    await page.goto('/inbox');

    // Check for the "New booking received from local lead generation campaign!" message
    const message = page.getByText('New booking received from local lead generation campaign!');
    // Since background jobs can take a few seconds, wait up to 10s for the message to appear.
    // In e2e fixtures the db operations occur quickly though.
    await expect(message).toBeVisible({ timeout: 10000 });
  });
});
