import { test, expect, adminPage } from './fixtures';

test.describe('Walkthrough Features Setup optimization', () => {
  test('should display walkthrough when setup tour is triggered', async ({ browser }) => {
    let page = await adminPage(browser);
    // Navigate to setup where the walkthrough button might live, or trigger directly since we want to test setup walkthrough optimizations
    await page.goto('/setup.html?test_walkthrough=true');

    // Evaluate the window.startWalkthrough on setup page to test the styling we just applied
    await page.evaluate(() => {
        if (window.startWalkthrough) {
            window.startWalkthrough([
                {targetId: '#setup-screen', title: 'Welcome to Setup', content: 'Let\'s get your business setup.'},
                {targetId: '#setup-screen', title: 'Welcome to Step 2', content: 'Step 2 info.'}
            ]);
        }
    });

    // Wait for the overlay
    const overlay = page.locator('#walkthrough-overlay');
    await expect(overlay).toBeVisible();

    // The walkthrough bubble should appear
    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();

    // It should have the correct title for the first step
    await expect(bubble.locator('.ohc-walkthrough-title')).toHaveText('Welcome to Setup');

    // Test the "Next" button moves to next step
    const nextBtn = page.locator('#wt-next');
    await nextBtn.click();
    await expect(bubble.locator('.ohc-walkthrough-title')).toHaveText('Welcome to Step 2');

    // Test the "Finish" button closes the walkthrough
    await expect(nextBtn).toHaveText('Finish');
    await nextBtn.click();
    await expect(bubble).not.toBeVisible();
    await expect(overlay).not.toBeVisible();
  });
});
