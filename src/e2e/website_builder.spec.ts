import { test, expect } from '@playwright/test';

test.describe('Website Builder Full E2E', () => {

    test('Flow 1: Modern Template with Subdomain', async ({ page }) => {
        await page.goto('/');
        await page.fill('input[placeholder="Email"]', 'test@example.com');
        await page.fill('input[placeholder="Password"]', 'password123');
        await page.click('text="Log In"');
        await page.click('text="Build Website"');
        await expect(page.locator('text="Choose a template"')).toBeVisible();
        await page.click('text="Use this template →"'); // Default Modern
        await page.click('text="Next →"');
        await page.click('text="🟢 Nature Green"');
        await page.click('text="Next →"');
        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Vegan Cake');
        await page.fill('input[placeholder="e.g. 50.00"]', '25.00');
        await page.fill('input[placeholder="Short description"]', 'Delicious');
        await page.click('text="Next →"');
        await page.click('text="🌐 Use a free OHC subdomain"');
        await page.click('text="Next →"');
        await page.click('text="Publish →"');
        await expect(page.locator('text="Publishing Site..."')).toBeVisible();
    });

    test('Flow 2: Classic Template with Custom Domain', async ({ page }) => {
        await page.goto('/');
        await page.fill('input[placeholder="Email"]', 'test@example.com');
        await page.fill('input[placeholder="Password"]', 'password123');
        await page.click('text="Log In"');
        await page.click('text="Build Website"');
        await page.locator('text="🏛️ Classic"').locator('..').locator('text="Use this template →"').click();
        await page.click('text="Next →"');
        await page.click('text="🔴 Bold Red"');
        await page.click('text="Next →"');
        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Classic Bread');
        await page.fill('input[placeholder="e.g. 50.00"]', '5.00');
        await page.click('text="Next →"');
        await page.click('text="🌍 Use my own domain"');
        await page.click('text="Next →"');
        await page.click('text="Publish →"');
        await expect(page.locator('text="Publishing Site..."')).toBeVisible();
    });

    test('Flow 3: Bold Template with Domain Purchase', async ({ page }) => {
        await page.goto('/');
        await page.fill('input[placeholder="Email"]', 'test@example.com');
        await page.fill('input[placeholder="Password"]', 'password123');
        await page.click('text="Log In"');
        await page.click('text="Build Website"');
        await page.locator('text="🔥 Bold"').locator('..').locator('text="Use this template →"').click();
        await page.click('text="Next →"');
        await page.click('text="🔵 Ocean Blue"');
        await page.click('text="Next →"');
        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Bold Services');
        await page.click('text="Next →"');
        await page.click('text="🛒 Buy a domain"');
        await page.click('text="Next →"');
        await page.click('text="Publish →"');
        await expect(page.locator('text="Publishing Site..."')).toBeVisible();
    });

    test('Flow 4: Navigation Back and Forth', async ({ page }) => {
        await page.goto('/');
        await page.fill('input[placeholder="Email"]', 'test@example.com');
        await page.fill('input[placeholder="Password"]', 'password123');
        await page.click('text="Log In"');
        await page.click('text="Build Website"');
        await page.click('text="Next →"'); // step 1
        await page.click('text="Back"'); // step 0
        await expect(page.locator('text="Choose a template"')).toBeVisible();
        await page.click('text="Next →"'); // step 1
        await page.click('text="Next →"'); // step 2
        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Item');
        await page.click('text="Next →"'); // step 3
        await page.click('text="Next →"'); // step 4
        await page.click('text="Back"'); // step 3
        await expect(page.locator('text="Choose a Domain"')).toBeVisible();
    });

    test('Flow 5: Advanced Mode Toggling and Raw Export Visibility', async ({ page }) => {
        await page.goto('/');
        await page.fill('input[placeholder="Email"]', 'test@example.com');
        await page.fill('input[placeholder="Password"]', 'password123');
        await page.click('text="Log In"');
        await page.click('text="Build Website"');
        // Toggle advanced mode on
        await page.locator('.advanced-toggle-checkbox').click();
        await expect(page.locator('text="Advanced: Local CLI command"')).toBeVisible();
        await page.click('text="Next →"');
        await page.click('text="Next →"');
        await page.click('text="Next →"');
        await page.click('text="Next →"'); // Step 4
        await expect(page.locator('text="Developer: Raw Output Export"')).toBeVisible();
    });
});
