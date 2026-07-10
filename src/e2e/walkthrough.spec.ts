import { test, expect, adminPage } from './fixtures';


test.describe('Walkthrough Features', () => {
  test('should display walkthrough when dashboard tour is triggered', async ({ browser }) => {
    let page = await adminPage(browser);
    // Navigate to dashboard where the walkthrough button lives
    await page.goto('/dashboard?test_walkthrough=true');

    // Wait for the button
    const walkthroughBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkthroughBtn).toBeVisible();

    // Click the button to start walkthrough
    await walkthroughBtn.click();

    // The walkthrough bubble should appear
    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();

    // It should have the correct title for the first step
    await expect(bubble.locator('h4')).toHaveText('Business Analytics');

    // Test the "Next" button moves to next step
    const nextBtn = page.locator('#wt-next');
    await nextBtn.click();
    await expect(bubble.locator('h4')).toHaveText('Operations Map');

    // Test the "Finish" button closes the walkthrough
    await expect(nextBtn).toHaveText('Finish');
    await nextBtn.click();
    await expect(bubble).not.toBeVisible();
  });
});
