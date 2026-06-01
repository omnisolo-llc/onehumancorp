import { test, expect } from '@playwright/test';

test.describe('Documentation Features', () => {
  test('should navigate and interact with help center features', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('http://127.0.0.1:3005/dashboard');

    // Check if we are on dashboard or setup
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 10000 });

    // 2. Test Help Chat widget visibility
    const helpChatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(helpChatButton).toBeVisible();

    // Open chat
    await helpChatButton.click();

    // Ensure chat window appears
    await expect(page.getByRole('heading', { name: 'Help Agent' })).toBeVisible();

    // Type in a question
    await page.fill('input[placeholder="Ask me anything..."]', 'How do I add a product?');
    await page.locator('button[aria-label="Send message"]').click();

    // Expect agent response
    await expect(page.getByText('I am your AI Help Agent! I specialize in answering questions')).toBeVisible({ timeout: 10000 });

    // 3. Navigate to Help Center via global nav or direct URL
    await page.goto('http://127.0.0.1:3005/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible({ timeout: 10000 });

    // 4. Test Search Functionality in Help Center
    await page.fill('input[placeholder="Search for help articles..."]', 'getting paid');
    await expect(page.getByText('Getting Paid').first()).toBeVisible();

    // Click on the article
    await page.getByText('Getting Paid').first().click();

    // 5. Verify Help Article Content
    await expect(page.getByRole('heading', { name: 'Getting Paid' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Getting paid is the most exciting part!')).toBeVisible();

    // 6. Navigate to Changelog
    await page.goto('http://127.0.0.1:3005/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();

    // 7. Navigate to API Documentation
    await page.goto('http://127.0.0.1:3005/api-docs');
    await expect(page.getByText('Advanced:')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('This section is for developers directly integrating with our APIs.')).toBeVisible();
  });
});
