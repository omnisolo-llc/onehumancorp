import { test, expect } from './fixtures';

test.describe('Viral Milestone Celebration Widget', () => {
  test('should load the widget and generate a milestone link', async ({ page, loginAs, adminUser }) => {
    // Start at dashboard after login as required
    await loginAs(page, adminUser);

    // Navigate via UI click as required by E2E standards
    await page.locator('#milestone-widget-link').click();

    // Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Milestone Generator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Input values
    await page.locator('#milestone-input').fill('5,000 Happy Customers');
    await page.locator('#discount-input').fill('20% OFF Everything');

    // Click generate
    await generateBtn.click();

    // Verify loading state
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    // Wait for result area to appear
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Check share link generated correctly
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toBeVisible();
    await expect(shareLink).toHaveValue(/^https:\/\/[a-zA-Z0-9.-]+\/celebrate\?m=5%2C000%20Happy%20Customers&d=20%25%20OFF%20Everything&ref=viral/);
  });
});
