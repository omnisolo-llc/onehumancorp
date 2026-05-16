import { test, expect } from '@playwright/test';
import { Pool } from 'pg';

test('lens audit: fully verify login error states', async ({ page }) => {
    await page.goto('/');

    await expect(page.locator('text="One Human Corp"').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('text="Sign in to manage your business"').filter({ visible: true }).first()).toBeVisible();

    await page.fill('input[placeholder="Email or Username"]', 'bad@test.com');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text="We couldn\'t sign you in"')).toBeVisible({ timeout: 5000 });
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

test('lens audit: fully verify sign up mode', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("New here? Create an account")');

    const testEmail = 'newuser@test.com';
    await page.fill('input[placeholder="Email or Username"]', testEmail);
    await page.fill('input[placeholder="Password"]', 'newpassword');

    await page.click('button:has-text("Sign Up")');

    // Verify it transitioned to setup screen
    await expect(page.locator('text="Your business, live in minutes."').filter({ visible: true }).first()).toBeVisible();

    // Verification 1: Query the database to assert the data was correctly modified/updated.
    const pool = new Pool({ connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc' });
    const res = await pool.query('SELECT email FROM users WHERE email = $1', [testEmail]);
    expect(res.rows.length).toBeGreaterThan(0);
    await pool.end();
});

test('lens audit: fully verify login input states', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[placeholder="Email or Username"]', 'newuser@test.com');
    await page.fill('input[placeholder="Password"]', 'newpassword');

    await page.click('button:has-text("Sign In")');

    // Verify it transitioned to the dashboard UI
    await expect(page.locator('text="Welcome back, Human."').filter({ visible: true }).first()).toBeVisible();
});

test('lens audit: fully verify login sso flow', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("Use Google or Apple")');

    // Verify it transitioned to dashboard
    await expect(page.locator('text="Welcome back, Human."').filter({ visible: true }).first()).toBeVisible();
});

test('lens audit: fully verify start business routing', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("🚀 Start Business Setup")');

    await expect(page.locator('text="Your business, live in minutes."').filter({ visible: true }).first()).toBeVisible();
});
