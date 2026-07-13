import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_give_get_widget_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_give_get_widget_smoke');
});

test.describe('Viral Give-Get Widget', () => {
    test('generator page renders correctly and updates preview', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);

        // Wait for dashboard to load then click link
        await page.goto('/dashboard');
        const link = page.locator('a[id="give-get-widget-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Viral Give-Get Generator' })).toBeVisible();

        // Check default values
        await expect(page.locator('#give-reward')).toHaveValue('20% Off');
        await expect(page.locator('#get-reward')).toHaveValue('$10 Credit');

        // Check preview
        await expect(page.locator('#give-display')).toHaveText('20% Off');
        await expect(page.locator('#get-display')).toHaveText('$10 Credit');

        // Change values
        await page.locator('#give-reward').fill('50% Off');
        await page.locator('#get-reward').fill('$20 Cash');

        // Assert update in preview
        await expect(page.locator('#give-display')).toHaveText('50% Off');
        await expect(page.locator('#get-display')).toHaveText('$20 Cash');
    });

    test('generates a referral link', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);

        await page.goto('/viral-give-get-widget');

        // Wait for main elements
        await expect(page.locator('h1', { hasText: 'Viral Give-Get Generator' })).toBeVisible();
        const generateBtn = page.locator('#generate-btn');
        await expect(generateBtn).toBeVisible();

        // Click generate
        await generateBtn.click();

        // Verify button goes to generating state
        await expect(generateBtn).toBeDisabled();
        await expect(generateBtn).toHaveText('Generating...');

        // Wait for the result area to show
        const resultArea = page.locator('#result-area');
        await expect(resultArea).toBeVisible({ timeout: 5000 });

        // Verify button restored
        await expect(generateBtn).not.toBeDisabled();
        await expect(generateBtn).toHaveText('Generate Referral Link');

        // Check share link generated correctly
        const shareLink = page.locator('#share-link');
        await expect(shareLink).toHaveValue(/\/give-get\/join\?ref=.+/);
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        const link = page.locator('a[id="give-get-widget-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('input[type="checkbox"]');
        await removeBrandingCheckbox.check();

        // Soft paywall should appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Pro' });
        await expect(paywallModal).toBeVisible();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'true');
        });
        await page.reload();

        const link = page.locator('a[id="give-get-widget-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('input[type="checkbox"]');
        await removeBrandingCheckbox.check();

        // Soft paywall should not appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Pro' });
        await expect(paywallModal).not.toBeVisible();

        const generateBtn = page.locator('#generate-btn');
        await expect(generateBtn).toBeVisible();
        await generateBtn.click();

        // Wait for the result area to show
        const resultArea = page.locator('#result-area');
        await expect(resultArea).toBeVisible({ timeout: 5000 });

        // The generated link should NOT include the branding parameter as true
        const shareLink = page.locator('#share-link');
        await expect(shareLink).toHaveValue(/branding=false/);
    });
});
