import { test, expect } from './fixtures';

test.describe('Viral Share Cards Growth Loop', () => {
  test('verify social share cards flow and viral branding', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/dashboard');

    // 2. Verify Social Share Cards Growth Loop section
    const shareCardsHeading = page.locator('h2', { hasText: 'Social Share Cards' });
    await expect(shareCardsHeading).toBeVisible({ timeout: 10000 });

    // 3. Click the Generate Share Cards button
    const generateBtn = page.locator('a:has-text("Generate Share Cards")');
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // 4. Wait for navigation to /share-cards
    await page.waitForURL('**/share-cards');
    await expect(page.locator('h1', { hasText: 'Social Share Cards' })).toBeVisible();

    // 5. Verify the ⚡ Powered by OHC branding in the card preview
    const cardFooter = page.locator('span', { hasText: '⚡ Powered by OHC' });
    await expect(cardFooter).toBeVisible();
  });
});
