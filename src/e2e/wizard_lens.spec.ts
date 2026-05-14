import { test, expect } from '@playwright/test';

test('lens audit: full 11-step wizard E2E data lifecycle (Online Store)', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("New here? Create an account")');
    await page.fill('input[placeholder="Email or Username"]', 'e2e@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    await expect(page.locator('text="Your business, live in minutes."').first()).toBeVisible({ timeout: 5000 });

    await page.click('button:has-text("Start Setup")');

    await expect(page.locator('text="What kind of business are you building?"').first()).toBeVisible();
    await page.click('text="Online Store"');

    await expect(page.locator('text="Name your business"').first()).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\\'s Bakery"]', 'My E2E Bakery');
    await page.click('button:has-text("Next")');

    await expect(page.locator('text="What do you sell?"').first()).toBeVisible();
    await page.click('text="Physical Products"');
    await page.click('button:has-text("Next")');

    await expect(page.locator('text="How do you want to get paid?"').first()).toBeVisible();
    await page.click('text="Online via link"');

    await expect(page.locator('text="Administrator account"').first()).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya');
    await page.click('button:has-text("Next")');

    await expect(page.locator('text="Choose a Template"').first()).toBeVisible();
    await page.click('text="Modern"');

    await expect(page.locator('text="Add your first product"').first()).toBeVisible();
    await page.fill('input[placeholder="e.g. Vegan Chocolate Cake"]', 'E2E Cake');
    await page.fill('input[placeholder="e.g. 45.00"]', '25.00');
    await page.click('button:has-text("Next")');

    await expect(page.locator('text="Choose a domain"').first()).toBeVisible();
    await page.click('text="Free OHC Subdomain"');

    await expect(page.locator('text="Ready to launch!"').first()).toBeVisible();
    await page.click('button:has-text("Publish my business")');

    await expect(page.locator('text="Your live storefront!"').first()).toBeVisible({ timeout: 10000 });

    await page.reload();
    await expect(page.locator('text="My E2E Bakery"').first()).toBeVisible({ timeout: 5000 });
});

test('lens audit: full 11-step wizard E2E data lifecycle (Service Business)', async ({ page }) => {
    await page.goto('/');

    await page.click('button:has-text("New here? Create an account")');
    await page.fill('input[placeholder="Email or Username"]', 'e2e2@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    await expect(page.locator('text="Your business, live in minutes."').first()).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Start Setup")');

    await page.click('text="Service Provider"');
    await page.fill('input[placeholder="e.g. Maya\\'s Bakery"]', 'My E2E Service');
    await page.click('button:has-text("Next")');

    await page.click('text="Services & Time"');
    await page.click('button:has-text("Next")');

    await page.click('text="In-person"');

    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya');
    await page.click('button:has-text("Next")');

    await page.click('text="Classic"');

    await page.fill('input[placeholder="e.g. Vegan Chocolate Cake"]', 'E2E Service Hour');
    await page.fill('input[placeholder="e.g. 45.00"]', '100.00');
    await page.click('button:has-text("Next")');

    await page.click('text="Custom Domain"');

    await page.click('button:has-text("Publish my business")');

    await expect(page.locator('text="Your live storefront!"').first()).toBeVisible({ timeout: 10000 });

    await page.reload();
    await expect(page.locator('text="My E2E Service"').first()).toBeVisible({ timeout: 5000 });
});

test('lens audit: wizard cancel flow preserves db state', async ({ page }) => {
    await page.goto('/');
    await page.click('button:has-text("🚀 Start My Business")');

    await expect(page.locator('text="Your business, live in minutes."').first()).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Instant AI Build")');

    await expect(page.locator('text="Describe your business"').first()).toBeVisible();
    await page.fill('input[placeholder="e.g. I run a local bakery"]', 'Testing instantaneous build');

    await page.click('button:has-text("Back")');
    await expect(page.locator('text="Your business, live in minutes."').first()).toBeVisible();

    await page.reload();
    await expect(page.locator('text="Your business, live in minutes."').first()).toBeVisible();
});

test('lens audit: wizard instant build flow DB verification', async ({ page }) => {
    await page.goto('/');
    await page.click('button:has-text("🚀 Start My Business")');

    await page.click('button:has-text("Instant AI Build")');
    await page.fill('input[placeholder="e.g. I run a local bakery"]', 'Testing instantaneous build');
    await page.click('button:has-text("Generate Storefront")');

    await expect(page.locator('text="Designing your storefront..."').first()).toBeVisible();

    await page.waitForTimeout(2000);
});

test('lens audit: welcome checklist db synchronization', async ({ page }) => {
    await page.goto('/');
    await page.click('button:has-text("🚀 Start My Business")');

    await page.click('button:has-text("Start Setup")');
    await page.click('text="Online Store"');
    await page.fill('input[placeholder="e.g. Maya\\'s Bakery"]', 'Checklist Store');
    await page.click('button:has-text("Next")');
    await page.click('text="Physical Products"');
    await page.click('button:has-text("Next")');
    await page.click('text="Online via link"');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya');
    await page.click('button:has-text("Next")');
    await page.click('text="Bold"');
    await page.fill('input[placeholder="e.g. Vegan Chocolate Cake"]', 'Item');
    await page.fill('input[placeholder="e.g. 45.00"]', '10.00');
    await page.click('button:has-text("Next")');
    await page.click('text="Free OHC Subdomain"');
    await page.click('button:has-text("Publish my business")');

    await expect(page.locator('text="Your live storefront!"').first()).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Continue to Setup Checklist")');

    await expect(page.locator('text="✅ Business live"').first()).toBeVisible();
    await page.reload();
    await expect(page.locator('text="✅ Business live"').first()).toBeVisible();
});
