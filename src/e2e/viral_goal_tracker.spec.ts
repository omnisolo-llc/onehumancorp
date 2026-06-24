import { test, expect } from './fixtures';

test.describe('Viral Goal Tracker Loop', () => {
  test('should display the viral goal tracker builder and handle code generation', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Navigate to the dashboard or go directly to the builder
    // We go directly here to the newly created page.
    await page.waitForURL('**/dashboard');
    const goalTrackerBtn = page.locator('a[href="viral-goal-tracker.html"]');
    if (await goalTrackerBtn.isVisible()) {
        await goalTrackerBtn.click();
    } else {
        await page.goto('/viral-goal-tracker.html');
    }

    await expect(page.locator('h1')).toHaveText('Viral Goal Tracker Builder');

    // Check initial preview state
    await expect(page.locator('#preview-title')).toHaveText('Unlock: Free T-Shirt & 20% Off');

    // Modify target and reward
    await page.fill('#goal-target', '25');
    await page.fill('#reward-name', 'Early Access VIP');

    // Check that preview updates
    await expect(page.locator('#preview-title')).toHaveText('Unlock: Early Access VIP');
    await expect(page.locator('#preview-target')).toHaveText('25 target');

    // Open Embed Modal
    await page.click('#get-code-btn');

    const embedModal = page.locator('#embed-modal');
    await expect(embedModal).toHaveClass(/active/);

    const embedCode = await page.inputValue('#embed-code');
    expect(embedCode).toContain('target=25');
    expect(embedCode).toContain('reward=Early%20Access%20VIP');
    expect(embedCode).toContain('theme=light');

    await page.click('#close-embed-btn');
    await expect(embedModal).not.toHaveClass(/active/);
  });
});
