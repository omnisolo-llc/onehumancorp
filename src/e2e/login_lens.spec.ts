import { test, expect } from '@playwright/test';

test('lens audit: fully verify login error states', async ({ page }) => {
    await page.goto('/');

    await expect(page.locator('text="One Human Corp"').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('text="Sign in to manage your business"').filter({ visible: true }).first()).toBeVisible();

    // Try to login with empty credentials
    await page.click('button:has-text("Sign In")');

    // Wait for the simulated network delay/error
    await page.waitForTimeout(1500);

    // Verify error is shown
    await expect(page.locator('text="We couldn\'t sign you in"')).toBeVisible({ timeout: 5000 });

    // Switch to sign up mode
    await page.click('button:has-text("New here? Create an account")');
    await expect(page.locator('text="Create an account to start your business"').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('button:has-text("Sign Up")').filter({ visible: true }).first()).toBeVisible();

    // Verify SSO button
    await expect(page.locator('button:has-text("Use Google or Apple")').filter({ visible: true }).first()).toBeVisible();

    // Verify start business
    await expect(page.locator('button:has-text("🚀 Start Business Setup")').filter({ visible: true }).first()).toBeVisible();
});

test('lens audit: fully verify login mode toggling', async ({ page }) => {
    await page.goto('/');

    await expect(page.locator('text="Sign in to manage your business"').filter({ visible: true }).first()).toBeVisible();
    await page.click('button:has-text("New here? Create an account")');
    await expect(page.locator('text="Create an account to start your business"').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('button:has-text("Sign Up")').filter({ visible: true }).first()).toBeVisible();

    await page.click('button:has-text("Have an account? Sign In")');
    await expect(page.locator('text="Sign in to manage your business"').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('button:has-text("Sign In")').filter({ visible: true }).first()).toBeVisible();
});

test('lens audit: fully verify login input states', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[placeholder="Email or Username"]', 'hello@test.com');
    await page.fill('input[placeholder="Password"]', 'secretpassword');

    // Submit
    await page.click('button:has-text("Sign In")');
    await expect(page.locator('button:has-text("Signing in...")')).toBeVisible();
});

test('lens audit: fully verify sign up mode', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("New here? Create an account")');

    await page.fill('input[placeholder="Email or Username"]', 'newuser@test.com');
    await page.fill('input[placeholder="Password"]', 'newpassword');

    // Submit
    await page.click('button:has-text("Sign Up")');
    await expect(page.locator('button:has-text("Creating account...")')).toBeVisible();
});

test('lens audit: fully verify login sso flow', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("Use Google or Apple")');

    // Button state changes
    await expect(page.locator('button:has-text("Connecting...")')).toBeVisible();
});

test('lens audit: fully verify start business routing', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("🚀 Start Business Setup")');

    // Wait for the setup wizard modal/screen to open by checking for its contents
    await expect(page.locator('text="Your business, live in minutes."').filter({ visible: true }).first()).toBeVisible({ timeout: 5000 });
});
