import { expect, test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// Retain existing smoke test
currentAppSmoke('test_services_billing');

test.describe('Billing Services CUJ', () => {

    test.beforeEach(async ({ page }) => {
        // Setup: Navigate to root and mock login (assuming local dev server)
        await page.goto('/');
        await page.waitForLoadState('networkidle');

        // Ensure user is logged in
        await page.evaluate(() => {
             localStorage.setItem('token', 'test-token');
        });
        await page.goto('/dashboard');
        await page.waitForLoadState('networkidle');
    });

    test('CUJ: Navigate to My Plan and verify the limits display', async ({ page }) => {
        // Go to My Plan page
        await page.goto('/plan');
        await page.waitForSelector('text=My Plan', { state: 'visible' });

        // Verify status snapshot components are visible
        await expect(page.locator('text=Current Plan').first()).toBeVisible();
        await expect(page.locator('text=Estimated Next Bill').first()).toBeVisible();

        // Verify limits sections exist
        await expect(page.locator('text=AI Actions Used').first()).toBeVisible();
        await expect(page.locator('text=Storage Used').first()).toBeVisible();

        // Verify quick links
        await expect(page.locator('text=View Cost Details').first()).toBeVisible();
        await expect(page.locator('text=Change Plan').first()).toBeVisible();
    });

    test('CUJ: Navigate to Pricing Page and verify the items and the Glassmorphism CSS styles', async ({ page }) => {
        await page.goto('/pricing');
        await page.waitForSelector('text=Pricing Plans', { state: 'visible' });

        // Verify tier cards exist
        await expect(page.locator('h3:has-text("Free")')).toBeVisible();
        await expect(page.locator('h3:has-text("Starter")')).toBeVisible();
        await expect(page.locator('h3:has-text("Pro")')).toBeVisible();
        await expect(page.locator('h3:has-text("Business")')).toBeVisible();

        // Verify the background filter css property for Translucent Glass standard on Free tier
        const freeCard = page.locator('div:has(> div > h3:has-text("Free"))').first();
        const styleAttr = await freeCard.getAttribute('style');
        expect(styleAttr).toContain('backdrop-filter: blur(20px) saturate(200%)');

        // Check header style
        const header = page.locator('header').first();
        const headerStyle = await header.getAttribute('style');
        expect(headerStyle).toContain('backdrop-filter: blur(20px) saturate(200%)');
    });

    test('CUJ: Navigate to Cost Dashboard and verify the cost parameters', async ({ page }) => {
        await page.goto('/cost-dashboard');
        await page.waitForSelector('text=Business Advisory Dashboard', { state: 'visible' });

        // Verify overview section
        await expect(page.locator('text=Cost Transparency').first()).toBeVisible();
        await expect(page.locator('text=Total Costs').first()).toBeVisible();

        // Verify Cost Breakdown parameters
        await expect(page.locator('text=Cost Breakdown').first()).toBeVisible();
        await expect(page.locator('text=LLM Usage').first()).toBeVisible();
        await expect(page.locator('text=Storage').first()).toBeVisible();
        await expect(page.locator('text=Payment Fees').first()).toBeVisible();

        // Check Glassmorphism styling on sections
        const breakdownSection = page.locator('section:has(h2:has-text("Cost Breakdown"))').first();
        const sectionStyle = await breakdownSection.getAttribute('style');
        expect(sectionStyle).toContain('backdrop-filter: blur(20px) saturate(200%)');
    });
});
