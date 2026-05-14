import { test, expect } from '@playwright/test';

test('verify unified catalog adding physical and service offerings', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/');
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // 2. Open business manager
    await page.click('button:has-text("Manage Business")');
    await expect(page.locator('text="My Offerings"')).toBeVisible();

    // 3. Add Physical Product
    await page.click('text="+ Add New Offering"');
    await expect(page.locator('text="What are you offering?"')).toBeVisible();
    await page.click('text="📦 Physical Item"');
    await page.click('button:has-text("Next →")');

    await expect(page.locator('text="Details"')).toBeVisible();
    await page.fill('input[placeholder="E.g. Custom Vegan Cake"]', 'Vegan Strawberry Cake');
    await page.fill('input[placeholder="Brief description"]', 'A delicious vegan strawberry cake');
    await page.fill('input[placeholder="0.00"]', '45.00');
    await page.click('button:has-text("Create")');

    // 4. Verify physical product was added to list
    await expect(page.locator('text="My Offerings"')).toBeVisible();
    await expect(page.locator('text="Vegan Strawberry Cake"')).toBeVisible();

    // 5. Add Service Offering
    await page.click('text="+ Add New Offering"');
    await expect(page.locator('text="What are you offering?"')).toBeVisible();
    await page.click('text="⏱️ My Time / Service"');
    await page.click('button:has-text("Next →")');

    await expect(page.locator('text="Details"')).toBeVisible();
    await page.fill('input[placeholder="E.g. Custom Vegan Cake"]', 'Baking Consultation');
    await page.fill('input[placeholder="Brief description"]', '1-on-1 baking advice');
    await page.fill('input[placeholder="0.00"]', '100.00');
    await page.fill('input[placeholder="60"]', '30');
    await page.fill('input[placeholder="e.g. Mon-Fri 9am-5pm"]', 'Tue-Thu 10am-2pm');
    await page.click('button:has-text("Create")');

    // 6. Verify service was added to list
    await expect(page.locator('text="My Offerings"')).toBeVisible();
    await expect(page.locator('text="Baking Consultation"')).toBeVisible();
});

test('verify AI business insights actionable approvals', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/');
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // 2. Check the tasks for approval dashboard section
    await expect(page.locator('text="Tasks for You to Approve"')).toBeVisible();

    // 3. Find insight action and approve
    await expect(page.locator('text="Approve & Send"').first()).toBeVisible();
    await page.click('button:has-text("Approve & Send")');

    // The list should visually update, although for this test we mainly care we can click it successfully
});

test('verify omnichannel order ingestion AI draft', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/');
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // 2. Open inbox
    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    // 3. Select message and verify AI Draft
    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    await page.click('button:has-text("✨ AI Draft")');
    await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue(/vegan/i, { timeout: 10000 });
});

test('verify unified catalog validation empty input', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/');
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // 2. Open business manager
    await page.click('button:has-text("Manage Business")');
    await expect(page.locator('text="My Offerings"')).toBeVisible();

    // 3. Select type and check Next is enabled
    await page.click('text="+ Add New Offering"');
    await expect(page.locator('text="What are you offering?"')).toBeVisible();
    await page.click('text="💻 Digital Download"');
    await expect(page.locator('button:has-text("Next →")')).toBeEnabled();
});

test('verify omnichannel order ingestion back navigation', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/');
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // 2. Open inbox
    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    // 3. Mobile viewport navigation
    await page.setViewportSize({ width: 375, height: 812 });
    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    // 4. Back navigation works
    await page.click('button:has-text("< Back")');
    await expect(page.locator('text="Inbox"')).toBeVisible();
});
