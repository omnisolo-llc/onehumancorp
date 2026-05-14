import { test, expect } from '@playwright/test';

test.describe('Website Builder Full E2E', () => {

    test('Flow 1: Modern Template with Subdomain', async ({ page }) => {
        await page.goto('/');
        await page.locator('#login-screen input[placeholder="Email or Username"]').fill('test@example.com');
        await page.locator('#login-screen input[placeholder="Password"]').fill('password123');
        await page.locator('#login-screen button:has-text("Log In")').click();
        await page.waitForTimeout(600);
        await page.locator('#dashboard-screen button:has-text("Build Website")').click();

        await expect(page.locator('#builder-step-0:visible')).toBeVisible();
        await page.locator('#builder-step-0 button:has-text("Use this template →")').nth(0).click();
        await page.locator('#builder-step-0 button:has-text("Next →")').click();

        await page.locator('#builder-step-1 button:has-text("🟢 Nature Green")').click();
        await page.locator('#builder-step-1 button:has-text("Next →")').click();

        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Vegan Cake');
        await page.fill('input[placeholder="e.g. 50.00"]', '25.00');

        await page.locator('button:has-text("Generate Description using AI")').click({force: true});
        await expect(page.locator('#ai-copy-result')).toBeVisible({timeout: 10000});
        // We expect it NOT to say 'Failed to fetch AI copy.'
        await expect(page.locator('#ai-copy-result')).not.toContainText('Failed');

        await page.locator('#builder-step-2 button:has-text("Next →")').click();

        await page.locator('#builder-step-3 button:has-text("Use a free OHC subdomain")').click();
        await page.locator('#builder-step-3 button:has-text("Next →")').click();

        await page.locator('#builder-step-4 button:has-text("Publish →")').click();
        await expect(page.locator('text="Publishing Site... Done! SEO generated."')).toBeVisible({timeout: 10000});
    });

    test('Flow 2: Classic Template with Custom Domain', async ({ page }) => {
        await page.goto('/');
        await page.locator('#login-screen input[placeholder="Email or Username"]').fill('test@example.com');
        await page.locator('#login-screen input[placeholder="Password"]').fill('password123');
        await page.locator('#login-screen button:has-text("Log In")').click();
        await page.waitForTimeout(600);
        await page.locator('#dashboard-screen button:has-text("Build Website")').click();

        await page.locator('#builder-step-0 button:has-text("Use this template →")').nth(1).click();
        await page.locator('#builder-step-0 button:has-text("Next →")').click();

        await page.locator('#builder-step-1 button:has-text("🔴 Bold Red")').click();
        await page.locator('#builder-step-1 button:has-text("Next →")').click();

        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Classic Bread');
        await page.fill('input[placeholder="e.g. 50.00"]', '5.00');
        await page.locator('#builder-step-2 button:has-text("Next →")').click();

        await page.locator('#builder-step-3 button:has-text("Use my own domain")').click();
        await page.locator('#builder-step-3 button:has-text("Next →")').click();

        await page.locator('#builder-step-4 button:has-text("Publish →")').click();
        await expect(page.locator('text="Publishing Site... Done! SEO generated."')).toBeVisible({timeout: 10000});
    });

    test('Flow 3: Bold Template with Domain Purchase', async ({ page }) => {
        await page.goto('/');
        await page.locator('#login-screen input[placeholder="Email or Username"]').fill('test@example.com');
        await page.locator('#login-screen input[placeholder="Password"]').fill('password123');
        await page.locator('#login-screen button:has-text("Log In")').click();
        await page.waitForTimeout(600);
        await page.locator('#dashboard-screen button:has-text("Build Website")').click();

        await page.locator('#builder-step-0 button:has-text("Use this template →")').nth(2).click();
        await page.locator('#builder-step-0 button:has-text("Next →")').click();

        await page.locator('#builder-step-1 button:has-text("🔵 Ocean Blue")').click();
        await page.locator('#builder-step-1 button:has-text("Next →")').click();

        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Bold Services');
        await page.locator('#builder-step-2 button:has-text("Next →")').click();

        await page.locator('#builder-step-3 button:has-text("Buy a domain")').click();
        await page.locator('#builder-step-3 button:has-text("Next →")').click();

        await page.locator('#builder-step-4 button:has-text("Publish →")').click();
        await expect(page.locator('text="Publishing Site... Done! SEO generated."')).toBeVisible({timeout: 10000});
    });

    test('Flow 4: Navigation Back and Forth', async ({ page }) => {
        await page.goto('/');
        await page.locator('#login-screen input[placeholder="Email or Username"]').fill('test@example.com');
        await page.locator('#login-screen input[placeholder="Password"]').fill('password123');
        await page.locator('#login-screen button:has-text("Log In")').click();
        await page.waitForTimeout(600);
        await page.locator('#dashboard-screen button:has-text("Build Website")').click();

        await page.locator('#builder-step-0 button:has-text("Next →")').click(); // step 1
        await page.locator('#builder-step-1 button:has-text("Back")').click(); // step 0
        await expect(page.locator('#builder-step-0:visible')).toBeVisible();
        await page.locator('#builder-step-0 button:has-text("Next →")').click(); // step 1
        await page.locator('#builder-step-1 button:has-text("Next →")').click(); // step 2
        await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Item');
        await page.locator('#builder-step-2 button:has-text("Next →")').click(); // step 3
        await page.locator('#builder-step-3 button:has-text("Next →")').click(); // step 4
        await page.locator('#builder-step-4 button:has-text("Back")').click(); // step 3
        await expect(page.locator('#builder-step-3:visible')).toBeVisible();
    });

    test('Flow 5: Advanced Mode Toggling and Raw Export Visibility', async ({ page }) => {
        await page.goto('/');
        await page.locator('#login-screen input[placeholder="Email or Username"]').fill('test@example.com');
        await page.locator('#login-screen input[placeholder="Password"]').fill('password123');
        await page.locator('#login-screen button:has-text("Log In")').click();
        await page.waitForTimeout(600);
        await page.locator('#dashboard-screen button:has-text("Build Website")').click();

        // Toggle advanced mode on
        await page.locator('.advanced-toggle-checkbox').click();
        await expect(page.locator('#advanced-output:visible')).toBeVisible();
        await page.locator('#builder-step-0 button:has-text("Next →")').click();
        await page.locator('#builder-step-1 button:has-text("Next →")').click();
        await page.locator('#builder-step-2 button:has-text("Next →")').click();
        await page.locator('#builder-step-3 button:has-text("Next →")').click(); // Step 4
        await expect(page.locator('#advanced-output:visible')).toBeVisible();
    });
});
