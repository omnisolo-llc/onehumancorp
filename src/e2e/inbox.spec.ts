import { test, expect } from '@playwright/test';

test('verify omnichannel inbox AI draft flow', async ({ page }) => {
    // 1. Login and navigate to Check Messages to open the unified inbox
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    // 2. Select a conversation from the list
    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    // 3. Click the "✨ AI Draft" button and verify the input field populates
    await page.click('button:has-text("✨ AI Draft")');
    // Wait for the text "Sure, we have plenty of vegan options!" or generated text to appear in input
    await expect(page.locator('input[placeholder="Type an iMessage-style reply..."]')).toHaveValue(/vegan/i, { timeout: 10000 });

    // 4. Edit the response and send the message
    await page.fill('input[placeholder="Type an iMessage-style reply..."]', 'Yes, we have 3 vegan options!');
    await page.click('button:has-text("Send")');
    await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible();
});

test('verify inbox mobile layout constraints', async ({ page }) => {
    // 5. Test mobile layout and navigation (e.g., < Back button)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    await expect(page.locator('button:has-text("< Back")')).toBeVisible();
    await page.click('button:has-text("< Back")');
    await expect(page.locator('text="Inbox"')).toBeVisible();
});

test('verify quick reply usage', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    await page.click('button:has-text("Yes, we have 3 vegan options!")');
    await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible();
});

test('verify sending custom message clears input', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    await page.fill('input[placeholder="Type an iMessage-style reply..."]', 'Testing custom message');
    await page.click('button:has-text("Send")');
    await expect(page.locator('text="Testing custom message"').last()).toBeVisible();
    await expect(page.locator('input[placeholder="Type an iMessage-style reply..."]')).toHaveValue('');
});

test('verify empty state when no conversation is selected', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("Check Messages")');
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    await expect(page.locator('text="Select a conversation"')).toBeVisible();
});

test('verify connecting social media creates inbox conversation and allows reply', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    const integrationsMenu = page.locator('text=/Integrations/i, text=/Connect/i').filter({ visible: true }).first();
    if (await integrationsMenu.isVisible()) {
      await integrationsMenu.click();
    }

    // Click Connect Instagram/Facebook
    await page.click('text="📘 Social Integrations" >> xpath=.. >> button:has-text("Connect Instagram/Facebook")');

    // unified inbox should show up
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    // Click on Facebook User conversation
    await page.click('text="Facebook User"');
    await expect(page.locator('text="Hello from Facebook!"')).toBeVisible();

    // Send a reply
    await page.fill('input[placeholder="Type an iMessage-style reply..."]', 'Hello Facebook!');
    await page.click('button:has-text("Send")');
    await expect(page.locator('text="Hello Facebook!"').last()).toBeVisible();
});

test('verify connecting Instagram and AI draft', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    const integrationsMenu = page.locator('text=/Integrations/i, text=/Connect/i').filter({ visible: true }).first();
    if (await integrationsMenu.isVisible()) {
      await integrationsMenu.click();
    }

    // Click Connect Instagram/Facebook
    await page.click('text="📘 Social Integrations" >> xpath=.. >> button:has-text("Connect Instagram/Facebook")');

    // unified inbox should show up
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    // Click on Instagram User conversation
    await page.click('text="Instagram User"');
    await expect(page.locator('text="Is this available?"')).toBeVisible();

    // AI Draft
    await page.click('text="Instagram User" >> xpath=.. >> button:has-text("✨ AI Draft")');
    await expect(page.locator('input[placeholder="Type an iMessage-style reply..."]')).toHaveValue(/available/i, { timeout: 10000 });
});

test('verify TikTok user and AI draft', async ({ page }) => {
    await page.goto('/login');

    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    const integrationsMenu = page.locator('text=/Integrations/i, text=/Connect/i').filter({ visible: true }).first();
    if (await integrationsMenu.isVisible()) {
      await integrationsMenu.click();
    }

    await page.click('text="📘 Social Integrations" >> xpath=.. >> button:has-text("Connect Instagram/Facebook")');
    await expect(page.locator('text="Unified Social Inbox"')).toBeVisible();

    await page.click('text="TikTok User"');
    await expect(page.locator('text="Love your videos! Do you ship internationally?"')).toBeVisible();

    await page.click('text="TikTok User" >> xpath=.. >> button:has-text("✨ AI Draft")');
    await expect(page.locator('input[placeholder="Type an iMessage-style reply..."]')).toHaveValue(/worldwide/i, { timeout: 10000 });
});
