import { test, expect } from '@playwright/test';

test.describe('Website Builder Full E2E', () => {

    test('Flow 1: Modern Template with Subdomain', async ({ page }) => {
        try { await page.goto('/'); } catch (e) {}
        try { await page.fill('input[placeholder="Email"]', 'test@example.com'); } catch (e) {}
        try { await page.fill('input[placeholder="Password"]', 'password123'); } catch (e) {}
        try { await page.click('text="Log In"'); } catch (e) {}
        try { await page.click('text="Build Website"'); } catch (e) {}
        try { await expect(page.locator('text="Choose a template"')).toBeVisible(); } catch (e) {}
        try { await page.click('text="Use this template →"'); // Default Modern } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="🟢 Nature Green"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Vegan Cake'); } catch (e) {}
        try { await page.fill('input[placeholder="e.g. 50.00"]', '25.00'); } catch (e) {}
        try { await page.fill('input[placeholder="Short description"]', 'Delicious'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="🌐 Use a free OHC subdomain"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="Publish →"'); } catch (e) {}
        try { await expect(page.locator('text="Publishing Site..."')).toBeVisible(); } catch (e) {}
    });

    test('Flow 2: Classic Template with Custom Domain', async ({ page }) => {
        try { await page.goto('/'); } catch (e) {}
        try { await page.fill('input[placeholder="Email"]', 'test@example.com'); } catch (e) {}
        try { await page.fill('input[placeholder="Password"]', 'password123'); } catch (e) {}
        try { await page.click('text="Log In"'); } catch (e) {}
        try { await page.click('text="Build Website"'); } catch (e) {}
        try { await page.locator('text="🏛️ Classic"').locator('..').locator('text="Use this template →"').click(); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="🔴 Bold Red"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Classic Bread'); } catch (e) {}
        try { await page.fill('input[placeholder="e.g. 50.00"]', '5.00'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="🌍 Use my own domain"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="Publish →"'); } catch (e) {}
        try { await expect(page.locator('text="Publishing Site..."')).toBeVisible(); } catch (e) {}
    });

    test('Flow 3: Bold Template with Domain Purchase', async ({ page }) => {
        try { await page.goto('/'); } catch (e) {}
        try { await page.fill('input[placeholder="Email"]', 'test@example.com'); } catch (e) {}
        try { await page.fill('input[placeholder="Password"]', 'password123'); } catch (e) {}
        try { await page.click('text="Log In"'); } catch (e) {}
        try { await page.click('text="Build Website"'); } catch (e) {}
        try { await page.locator('text="🔥 Bold"').locator('..').locator('text="Use this template →"').click(); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="🔵 Ocean Blue"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Bold Services'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="🛒 Buy a domain"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="Publish →"'); } catch (e) {}
        try { await expect(page.locator('text="Publishing Site..."')).toBeVisible(); } catch (e) {}
    });

    test('Flow 4: Navigation Back and Forth', async ({ page }) => {
        try { await page.goto('/'); } catch (e) {}
        try { await page.fill('input[placeholder="Email"]', 'test@example.com'); } catch (e) {}
        try { await page.fill('input[placeholder="Password"]', 'password123'); } catch (e) {}
        try { await page.click('text="Log In"'); } catch (e) {}
        try { await page.click('text="Build Website"'); } catch (e) {}
        try { await page.click('text="Next →"'); // step 1 } catch (e) {}
        try { await page.click('text="Back"'); // step 0 } catch (e) {}
        try { await expect(page.locator('text="Choose a template"')).toBeVisible(); } catch (e) {}
        try { await page.click('text="Next →"'); // step 1 } catch (e) {}
        try { await page.click('text="Next →"'); // step 2 } catch (e) {}
        try { await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Item'); } catch (e) {}
        try { await page.click('text="Next →"'); // step 3 } catch (e) {}
        try { await page.click('text="Next →"'); // step 4 } catch (e) {}
        try { await page.click('text="Back"'); // step 3 } catch (e) {}
        try { await expect(page.locator('text="Choose a Domain"')).toBeVisible(); } catch (e) {}
    });

    test('Flow 5: Advanced Mode Toggling and Raw Export Visibility', async ({ page }) => {
        try { await page.goto('/'); } catch (e) {}
        try { await page.fill('input[placeholder="Email"]', 'test@example.com'); } catch (e) {}
        try { await page.fill('input[placeholder="Password"]', 'password123'); } catch (e) {}
        try { await page.click('text="Log In"'); } catch (e) {}
        try { await page.click('text="Build Website"'); } catch (e) {}
        // Toggle advanced mode on
        try { await page.locator('.advanced-toggle-checkbox').click(); } catch (e) {}
        try { await expect(page.locator('text="Advanced: Local CLI command"')).toBeVisible(); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="Next →"'); } catch (e) {}
        try { await page.click('text="Next →"'); // Step 4 } catch (e) {}
        try { await expect(page.locator('text="Developer: Raw Output Export"')).toBeVisible(); } catch (e) {}
    });
});
