import { test, expect, adminPage } from './fixtures';


test.describe('Walkthrough Features', () => {
  test('should display walkthrough when dashboard tour is triggered', async ({ browser }) => {
    let page = await adminPage(browser);
    // Navigate to dashboard where the walkthrough button lives
    await page.goto('/dashboard?test_walkthrough=true');

    // Wait for the button
    const walkthroughBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkthroughBtn).toBeVisible();
    await walkthroughBtn.scrollIntoViewIfNeeded();

    // Click the button to start walkthrough
    await page.evaluate(() => document.getElementById('dashboard-walkthrough-btn')?.click());

    // The walkthrough bubble should appear
    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();

    // It should have the correct title for the first step
    await expect(bubble.locator('h4')).toHaveText('Welcome');

    // Test the "Next" button moves to next step
    const nextBtn = page.locator('#wt-next');
    await page.evaluate(() => document.getElementById('wt-next')?.click());
    await expect(bubble.locator('h4')).toHaveText('Operations Map');

    // Test the "Finish" button closes the walkthrough
    await page.evaluate(() => document.getElementById('wt-next')?.click());
    await expect(bubble.locator('h4')).toHaveText('AI Savings');

    await expect(nextBtn).toHaveText('Finish');
    await page.evaluate(() => document.getElementById('wt-next')?.click());
    await expect(bubble).not.toBeVisible();
  });
});
