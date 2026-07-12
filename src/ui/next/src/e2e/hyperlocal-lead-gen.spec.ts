import { test, expect } from '../../../../e2e/fixtures';
import { adminPage } from '../../../../e2e/fixtures';

test.describe('Hyperlocal Lead Gen CUJ', () => {
  test('Carlos the Handyman sets up a weekly lead gen campaign', async ({ page }) => {
    await adminPage(page);

    // 1. Navigate to the dashboard where the Lead Gen card is located.
    // The adminPage fixture logs in and defaults to /dashboard, but ensure we are there.
    await page.goto('/dashboard');

    // 2. Locate the Lead Gen card link and click it to navigate to the new Tauri page
    const leadGenLink = page.locator('a#lead-gen-link');
    await expect(leadGenLink).toBeVisible();
    await leadGenLink.click();

    // Wait for the new page to load
    await expect(page.locator('h1', { hasText: 'Hyperlocal Lead Gen' })).toBeVisible();

    // 3. Fill in the budget and zip code.
    const budgetInput = page.getByTestId('lead-gen-budget');
    const zipInput = page.getByTestId('lead-gen-zip');

    await expect(budgetInput).toBeVisible();
    await expect(zipInput).toBeVisible();

    await budgetInput.fill('50');
    await zipInput.fill('90210');

    // 4. Submit the campaign.
    const submitBtn = page.getByTestId('lead-gen-submit');
    await submitBtn.click();

    // 5. Verify the UI updates to show the active campaign.
    const successMsg = page.locator('text=Campaign Active!');
    await expect(successMsg).toBeVisible({ timeout: 10000 });

    const detailsMsg = page.locator('text=The AI is now finding leads for 90210');
    await expect(detailsMsg).toBeVisible();

    // 6. Verify that a test lead message was sent to the inbox.
    // The LeadGenWorker simulates this by inserting into inbox_messages.
    // Wait for the simulated worker to complete the background DB writes.
    await page.waitForTimeout(2000);

    await page.goto('/inbox');

    // Look for the simulated message
    const message = page.locator('text=Hi, I saw your ad for service in the area. Can I book an appointment?');
    await expect(message.first()).toBeVisible({ timeout: 10000 });
  });
});
