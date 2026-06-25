import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Viral Loyalty Widget Growth Feature', () => {
  test('should display loyalty card, generate program link, and be mobile responsive', async ({ page }) => {
    // Mobile viewport constraint as per requirements
    await page.setViewportSize({ width: 375, height: 667 });

    // The operation CUJ MUST start from the home page after user login via the UI.
    await adminPage(page);

    // We wait for dashboard to load
    await expect(page.locator('h1')).toContainText('Dashboard');

    // Click on the Viral Loyalty Engine link directly without conditional logic
    await page.locator('#loyalty-link').click();

    // 1. Verify Initial State
    await expect(page.locator('h1')).toContainText('Viral Loyalty Widget Generator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Check stamp grid existence
    const stampGrid = page.locator('#stamp-grid');
    await expect(stampGrid).toBeVisible();

    const stamps = page.locator('.stamp:not(.free)');
    await expect(stamps).toHaveCount(4);

    // Initial result area is hidden
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeHidden();

    // 2. Perform Generation Action
    await generateBtn.click();

    // 3. Verify Final State
    // Wait for the result area to appear based on actual logic (no fake delays)
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // The button should be re-enabled
    await expect(generateBtn).toBeEnabled();
    await expect(generateBtn).toContainText('Generate Loyalty Program');

    // Verify the generated share link reflects a real reference ID
    const shareLinkInput = page.locator('#share-link');
    await expect(shareLinkInput).toBeVisible();

    const value = await shareLinkInput.inputValue();
    expect(value).toContain('/loyalty/join?ref=');
    expect(value).not.toContain('undefined'); // ensure we actually got an ID

    // Verify that stamps show filled state (coffee emojis)
    const filledStamps = page.locator('.stamp.filled');
    await expect(filledStamps).toHaveCount(4);
    await expect(filledStamps.first()).toContainText('☕');

    // Test Copy Button Interaction
    const copyBtn = page.locator('#copy-btn');
    await copyBtn.click();
    await expect(copyBtn).toContainText('Copied!');
  });
});
