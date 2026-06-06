import { test, expect } from './fixtures';

test.describe('Autonomous Hyperlocal Lead Generation', () => {
  test('Service owner can activate local lead gen and receive simulated booking in inbox', async ({ page, memberPage }) => {
    // Both admin and member should be able to activate lead gen. Let's use the default admin.

    // 1. Navigate to the dashboard
    await page.goto('/dashboard');

    // 2. Locate the LeadGenCard and verify it's visible
    await expect(page.locator('text=Want more local jobs this week?')).toBeVisible();

    // 3. Enter a budget and zip code
    const budgetInput = page.locator('input[placeholder="e.g. 50"]');
    await expect(budgetInput).toBeVisible();
    await budgetInput.fill('50');

    const zipInput = page.locator('input[placeholder="e.g. 90210"]');
    await expect(zipInput).toBeVisible();
    await zipInput.fill('90210');

    // 4. Submit the form
    const startButton = page.locator('button:has-text("Start Finding Jobs")');
    await expect(startButton).toBeEnabled();
    await startButton.click();

    // 5. Verify the success state
    await expect(page.locator('text=Campaign Active!')).toBeVisible({ timeout: 5000 });

    // Wait for the backend worker to simulate the job and insert the message
    await page.waitForTimeout(6000);

    // 6. Navigate to the inbox and verify the simulated lead booking
    await page.goto('/inbox');
    await expect(page.locator('text=New Booking: Sink Repair. $50 deposit paid.')).toBeVisible({ timeout: 10000 });
  });
});
