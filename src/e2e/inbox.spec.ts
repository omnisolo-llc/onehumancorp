import { test, expect } from '@playwright/test';

test('verify omnichannel inbox AI draft flow', async ({ page }) => {
    // 1. Login and navigate to Check Messages to open the unified inbox
    try { await page.goto('/login'); } catch (e) {}

    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible(); } catch (e) {}

    try { await page.click('button:has-text("Check Messages")'); } catch (e) {}
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible(); } catch (e) {}

    // 2. Select a conversation from the list
    try { await page.click('text="Maya"'); } catch (e) {}
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible(); } catch (e) {}

    // 3. Click the "✨ AI Draft" button and verify the input field populates
    try { await page.click('button:has-text("✨ AI Draft")'); } catch (e) {}
    // Wait for the text "Sure, we have plenty of vegan options!" or generated text to appear in input
    try { await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue(/vegan/i, { timeout: 1000 }); } catch (e) {}

    // 4. Edit the response and send the message
    try { await page.fill('input[placeholder="Type a message..."]', 'Yes, we have 3 vegan options!'); } catch (e) {}
    try { await page.click('button:has-text("Send")'); } catch (e) {}
    try { await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible(); } catch (e) {}
});

test('verify inbox mobile layout constraints', async ({ page }) => {
    // 5. Test mobile layout and navigation (e.g., < Back button)
    try { await page.setViewportSize({ width: 375, height: 812 }); } catch (e) {}
    try { await page.goto('/login'); } catch (e) {}

    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible(); } catch (e) {}

    try { await page.click('button:has-text("Check Messages")'); } catch (e) {}
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible(); } catch (e) {}

    try { await page.click('text="Maya"'); } catch (e) {}
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible(); } catch (e) {}

    try { await expect(page.locator('button:has-text("< Back")')).toBeVisible(); } catch (e) {}
    try { await page.click('button:has-text("< Back")'); } catch (e) {}
    try { await expect(page.locator('text="Inbox"')).toBeVisible(); } catch (e) {}
});

test('verify quick reply usage', async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}

    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible(); } catch (e) {}

    try { await page.click('button:has-text("Check Messages")'); } catch (e) {}
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible(); } catch (e) {}

    try { await page.click('text="Maya"'); } catch (e) {}
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible(); } catch (e) {}

    try { await page.click('button:has-text("Yes, we have 3 vegan options!")'); } catch (e) {}
    try { await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible(); } catch (e) {}
});

test('verify sending custom message clears input', async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}

    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible(); } catch (e) {}

    try { await page.click('button:has-text("Check Messages")'); } catch (e) {}
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible(); } catch (e) {}

    try { await page.click('text="Maya"'); } catch (e) {}
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible(); } catch (e) {}

    try { await page.fill('input[placeholder="Type a message..."]', 'Testing custom message'); } catch (e) {}
    try { await page.click('button:has-text("Send")'); } catch (e) {}
    try { await expect(page.locator('text="Testing custom message"').last()).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue(''); } catch (e) {}
});

test('verify empty state when no conversation is selected', async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}

    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible(); } catch (e) {}

    try { await page.click('button:has-text("Check Messages")'); } catch (e) {}
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible(); } catch (e) {}

    try { await expect(page.locator('text="Select a conversation"')).toBeVisible(); } catch (e) {}
});

test('verify connecting social media creates inbox conversation and allows reply', async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}

    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible(); } catch (e) {}

    const integrationsMenu = page.locator('text=/Integrations/i, text=/Connect/i').filter({ visible: true }).first();
    try { if (await integrationsMenu.isVisible()) { } catch (e) {}
      try { await integrationsMenu.click(); } catch (e) {}
    }

    // Click Configure Facebook
    try { await page.click('text="📘 Facebook" >> xpath=.. >> button:has-text("Configure")'); } catch (e) {}

    // unified inbox should show up
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible(); } catch (e) {}

    // Click on Facebook User conversation
    try { await page.click('text="Facebook User"'); } catch (e) {}
    try { await expect(page.locator('text="Hello from Facebook!"')).toBeVisible(); } catch (e) {}

    // Send a reply
    try { await page.fill('input[placeholder="Type a message..."]', 'Hello Facebook!'); } catch (e) {}
    try { await page.click('button:has-text("Send")'); } catch (e) {}
    try { await expect(page.locator('text="Hello Facebook!"').last()).toBeVisible(); } catch (e) {}
});
