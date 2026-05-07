import { test, expect } from '@playwright/test';

test('verify omnichannel inbox AI draft flow', async ({ page }) => {
    // 1. Login and navigate to Check Messages to open the unified inbox
    await page.goto('/');

    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text="Welcome"')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    // 2. Select a conversation from the list
    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    // 3. Click the "✨ AI Draft" button and verify the input field populates
    await page.click('button:has-text("✨ AI Draft")');
    // Wait for the mock text "Sure, we have plenty of vegan options!" or generated text to appear in input
    await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue(/vegan/i, { timeout: 10000 });

    // 4. Edit the response and send the message
    await page.fill('input[placeholder="Type a message..."]', 'Yes, we have 3 vegan options!');
    await page.click('button:has-text("Send")');
    await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible();
});

test('verify inbox mobile layout constraints', async ({ page }) => {
    // 5. Test mobile layout and navigation (e.g., < Back button)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/');

    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text="Welcome"')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    await expect(page.locator('button:has-text("< Back")')).toBeVisible();
    await page.click('button:has-text("< Back")');
    await expect(page.locator('text="Inbox"')).toBeVisible();
});

test('verify quick reply usage', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text="Welcome"')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    await page.click('button:has-text("Yes, we have 3 vegan options!")');
    await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible();
});

test('verify sending custom message clears input', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text="Welcome"')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    await page.fill('input[placeholder="Type a message..."]', 'Testing custom message');
    await page.click('button:has-text("Send")');
    await expect(page.locator('text="Testing custom message"').last()).toBeVisible();
    await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue('');
});

test('verify empty state when no conversation is selected', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text="Welcome"')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    await expect(page.locator('text="Select a conversation"')).toBeVisible();
});
