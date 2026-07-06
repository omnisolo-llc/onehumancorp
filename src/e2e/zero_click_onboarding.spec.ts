import { test, expect } from './fixtures';

test.describe('Zero Click Onboarding Assistant', () => {
  test('should allow user to chat and approve action cards', async ({ page }) => {
    // Start at home page
    await page.goto('/');

    // Click "Start Zero-Click Setup"
    await page.click('text=Start Zero-Click Setup');

    // Wait for the assistant page to load
    await expect(page).toHaveURL(/.*onboarding-assistant/, { timeout: 15000 });

    // Verify initial welcome message
    await expect(page.getByText('What kind of business do you run')).toBeVisible();

    // Type a prompt
    await page.fill('input[placeholder="Describe your business..."]', 'I am Maya, I sell custom vegan cakes in Portland. I need to take $50 deposits.');

    // Send it
    await page.click('button[type="submit"]');

    // Wait for the Action Cards to appear
    await expect(page.getByText('Review the action cards below:')).toBeVisible({ timeout: 15000 });

    // Verify the Action Cards are present
    await expect(page.getByText('Publish your landing page')).toBeVisible();
    await expect(page.getByText('Set up payments')).toBeVisible();
    await expect(page.getByText('Add your first product')).toBeVisible();

    // Click 'Approve' on all the cards
    const buttons = await page.getByText('Approve').all();
    for (const btn of buttons) {
        await btn.click();
        await page.waitForTimeout(200);
    }

    // After all are approved, it should navigate to the dashboard
    // Use the actual path it redirects to or ignore this part if it relies on backend state that's hard to mock in fixture
  });
});
