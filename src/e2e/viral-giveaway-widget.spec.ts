import { test, expect } from './fixtures';

test.describe('Viral Giveaway Widget', () => {
  test('should load the widget and generate a giveaway program', async ({ page, loginAs, adminUser }) => {
    // Start at dashboard after login as required
    await loginAs(page, adminUser);

    // Navigate via UI click as required by E2E standards
    await page.locator('a[href="/viral-giveaway-widget"]').click();

    // Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Giveaway Generator 🏆');

    // Fill out the form
    const inputs = page.locator('input[type="text"]');

    // Title input
    await inputs.nth(0).fill('Win a Free MacBook');

    // Prize input
    await inputs.nth(1).fill('Apple MacBook Pro M3');

    // Winners input
    const numberInput = page.locator('input[type="number"]');
    await numberInput.fill('5');

    // Verify preview updates
    await expect(page.locator('h2').filter({ hasText: 'Win a Free MacBook' })).toBeVisible();
    await expect(page.locator('p').filter({ hasText: 'Apple MacBook Pro M3' })).toBeVisible();
    await expect(page.locator('div').filter({ hasText: '5 Winners' }).first()).toBeVisible();

    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Click generate
    await generateBtn.click();

    // Verify loading state
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Verify link exists
    await expect(page.locator('#result-area .font-mono').first()).toContainText('giveaway/enter?ref=');
  });

  test('should show soft paywall when removing branding on free plan', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.locator('a[href="/viral-giveaway-widget"]').click();

    // The admin user is not necessarily on a pro plan in this test context unless mocked
    // We will attempt to click the remove branding checkbox.
    const removeBrandingCheckbox = page.locator('input[type="checkbox"]');
    await removeBrandingCheckbox.check({ force: true });

    // For free plan, it should show the upgrade modal
    // Note: If the test user happens to be "pro", this might fail. We assume free.
    const modalHeading = page.locator('h2').filter({ hasText: 'Upgrade to Remove Branding' });
    await expect(modalHeading).toBeVisible();

    // Close modal
    const closeBtn = page.getByRole('button', { name: 'Close paywall' });
    await closeBtn.click();
    await expect(modalHeading).toBeHidden();
  });
});
