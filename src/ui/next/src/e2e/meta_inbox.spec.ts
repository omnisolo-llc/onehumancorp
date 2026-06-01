import { test, expect } from '@playwright/test';

test.describe('Unified Meta Inbox', () => {
    test('User connects Meta accounts and replies to a message', async ({ page }) => {
        // Step 1: Go to inbox
        await page.goto('http://localhost:3000/inbox');

        // Verify the heading exists
        await expect(page.locator('h1:has-text("Customer Inbox")')).toBeVisible();

        // Check that Facebook and Instagram messages are NOT visible initially
        // because they are disconnected by default
        await expect(page.locator('text="Facebook User"')).not.toBeVisible();
        await expect(page.locator('text="Instagram User"')).not.toBeVisible();

        // Step 2: Open Settings to connect
        await page.click('button[title="Channel Settings"]');
        await expect(page.locator('h2:has-text("Connect Platforms")')).toBeVisible();

        // Step 3: Connect Facebook
        const facebookConnectBtn = page.locator('button:has-text("Connect Facebook")');
        await expect(facebookConnectBtn).toBeVisible();
        await facebookConnectBtn.click();

        // Wait for OAuth simulation to complete
        await expect(page.locator('text="Connecting Facebook..."')).toBeVisible();
        await expect(page.locator('text="Connecting Facebook..."')).not.toBeVisible({ timeout: 5000 });

        // Verify it now says Connected (be more specific)
        const facebookRow = page.locator('.space-y-3 > div').filter({ hasText: 'Facebook' }).filter({ has: page.locator('span:has-text("Connected")') });
        await expect(facebookRow).toBeVisible();

        // Step 4: Connect Instagram
        const instagramConnectBtn = page.locator('button:has-text("Connect Instagram")');
        await expect(instagramConnectBtn).toBeVisible();
        await instagramConnectBtn.click();

        // Wait for OAuth simulation to complete
        await expect(page.locator('text="Connecting Instagram..."')).toBeVisible();
        await expect(page.locator('text="Connecting Instagram..."')).not.toBeVisible({ timeout: 5000 });

        // Verify it now says Connected
        const instagramRow = page.locator('.space-y-3 > div').filter({ hasText: 'Instagram' }).filter({ has: page.locator('span:has-text("Connected")') });
        await expect(instagramRow).toBeVisible();

        // Close Settings Modal
        // Wait a bit to ensure animations or state updates finish before clicking close
        await page.waitForTimeout(500);
        await page.locator('button').filter({ has: page.locator('svg') }).first().click();
        await expect(page.locator('h2:has-text("Connect Platforms")')).not.toBeVisible();

        // Step 5: Verify messages are now visible
        await expect(page.locator('text="Facebook User"')).toBeVisible();
        await expect(page.locator('text="Instagram User"')).toBeVisible();

        // Step 6: Reply to a message (Instagram message)
        // Find the "Edit" button for the Instagram AI Draft and click it
        // The element hierarchy in page.tsx:
        // div containing "Instagram User" and then an "Edit" button in a nested div
        const instagramMsgDiv = page.locator('#messages-list > div').filter({ hasText: 'Instagram User' });
        const editButton = instagramMsgDiv.locator('button:has-text("Edit")');
        await editButton.click();

        // Type a reply
        await page.fill('textarea[id="reply-input-edit"]', 'Thanks for your patience, it will be shipped tomorrow.');

        // Send reply
        await instagramMsgDiv.locator('button:has-text("Send")').click();

        // Step 7: Verify reply appears in feed
        await expect(page.locator('text="Thanks for your patience, it will be shipped tomorrow."')).toBeVisible();
        await expect(page.locator('span:has-text("Me")')).toBeVisible();
    });
});
