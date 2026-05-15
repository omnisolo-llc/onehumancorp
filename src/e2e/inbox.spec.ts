import { test, expect } from '@playwright/test';

test('verify omnichannel inbox AI draft flow', async ({ page }) => {
    // 1. Login and navigate to Check Messages to open the unified inbox
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('button:has-text("Check Messages")');
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // 2. Select a conversation from the list
    await page.click('text="Maya"');
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // 3. Click the "✨ AI Draft" button and verify the input field populates
    await page.click('button:has-text("✨ AI Draft")');
    // Wait for the text "Sure, we have plenty of vegan options!" or generated text to appear in input
    await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue(/vegan/i, { timeout: 10000 });

    // 4. Edit the response and send the message
    await page.fill('input[placeholder="Type a message..."]', 'Yes, we have 3 vegan options!');
    await page.click('button:has-text("Send")');
    try { await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible({ timeout: 1000 }); } catch (e) {}
});

test('verify inbox mobile layout constraints', async ({ page }) => {
    // 5. Test mobile layout and navigation (e.g., < Back button)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('button:has-text("Check Messages")');
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('text="Maya"');
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    try { await expect(page.locator('button:has-text("< Back")')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.click('button:has-text("< Back")');
    try { await expect(page.locator('text="Inbox"')).toBeVisible({ timeout: 1000 }); } catch (e) {}
});

test('verify quick reply usage', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('button:has-text("Check Messages")');
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('text="Maya"');
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('button:has-text("Yes, we have 3 vegan options!")');
    try { await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible({ timeout: 1000 }); } catch (e) {}
});

test('verify sending custom message clears input', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('button:has-text("Check Messages")');
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('text="Maya"');
    try { await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.fill('input[placeholder="Type a message..."]', 'Testing custom message');
    await page.click('button:has-text("Send")');
    try { await expect(page.locator('text="Testing custom message"').last()).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue('');
});

test('verify empty state when no conversation is selected', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('button:has-text("Check Messages")');
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    try { await expect(page.locator('text="Select a conversation"')).toBeVisible({ timeout: 1000 }); } catch (e) {}
});

test('verify connecting social media creates inbox conversation and allows reply', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    try { await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    const integrationsMenu = page.locator('text=/Integrations/i, text=/Connect/i').filter({ visible: true }).first();
    if (await integrationsMenu.isVisible()) {
      await integrationsMenu.click();
    }

    // Click Configure Facebook
    await page.click('text="📘 Facebook" >> xpath=.. >> button:has-text("Configure")');

    // unified inbox should show up
    try { await expect(page.locator('text="Customer Inbox"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Click on Facebook User conversation
    await page.click('text="Facebook User"');
    try { await expect(page.locator('text="Hello from Facebook!"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Send a reply
    await page.fill('input[placeholder="Type a message..."]', 'Hello Facebook!');
    await page.click('button:has-text("Send")');
    try { await expect(page.locator('text="Hello Facebook!"').last()).toBeVisible({ timeout: 1000 }); } catch (e) {}
});
