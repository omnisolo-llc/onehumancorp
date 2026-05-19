import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: Agent Activity Feed', () => {

    test.beforeEach(async ({ page }) => {
        // Go to the local test server
        await page.goto('http://127.0.0.1:3000');

        // Emulate login
        await page.fill('#username', 'testuser');
        await page.fill('#password', 'password');
        await page.click('button:has-text("Sign In")');

        // Verify we are on dashboard
        await expect(page.locator('#dashboard-screen')).toBeVisible();
    });

    test('should verify the Agent Activity Feed is visible on the dashboard', async ({ page }) => {
        // Find the "Agent Activity Feed" heading
        const heading = page.locator('h2:has-text("Agent Activity Feed")');
        await expect(heading).toBeVisible();

        // Check if the container is present
        const feedContainer = page.locator('.activity-feed');
        await expect(feedContainer).toBeVisible();

        // Assert we have feed items inside
        const items = feedContainer.locator('.feed-item');
        await expect(items).toHaveCount(3);
    });

    test('should tap "Approve & Send" on The Ambassador and see alert and opacity change', async ({ page }) => {
        // Mock the window.alert dialog
        let alertMessage = '';
        page.on('dialog', dialog => {
            alertMessage = dialog.message();
            dialog.accept();
        });

        const ambassadorItem = page.locator('.feed-item').filter({ hasText: 'The Ambassador' });
        await expect(ambassadorItem).toBeVisible();

        const approveBtn = ambassadorItem.locator('button:has-text("Approve & Send")');
        await approveBtn.click();

        expect(alertMessage).toBe('Message sent successfully!');

        // Assert opacity changed (to indicate success)
        await expect(ambassadorItem).toHaveCSS('opacity', '0.5');
    });

    test('should tap "Edit" on The Ambassador and be redirected to the inbox screen', async ({ page }) => {
        const ambassadorItem = page.locator('.feed-item').filter({ hasText: 'The Ambassador' });
        await expect(ambassadorItem).toBeVisible();

        const editBtn = ambassadorItem.locator('button:has-text("Edit")');
        await editBtn.click();

        // Check if inbox screen becomes visible
        await expect(page.locator('#inbox-screen')).toBeVisible();
        await expect(page.locator('#dashboard-screen')).not.toBeVisible();
    });

    test('should verify The Promoter feed item text and approve action', async ({ page }) => {
        let alertMessage = '';
        page.on('dialog', dialog => {
            alertMessage = dialog.message();
            dialog.accept();
        });

        const promoterItem = page.locator('.feed-item').filter({ hasText: 'The Promoter' });
        await expect(promoterItem).toBeVisible();
        await expect(promoterItem).toContainText('Generated a social post about your new summer collection.');

        const approveBtn = promoterItem.locator('button:has-text("Approve")');
        await approveBtn.click();

        expect(alertMessage).toBe('Post scheduled successfully!');
    });

    test('should verify The Advisor feed item and dismiss action', async ({ page }) => {
        let alertMessage = '';
        page.on('dialog', dialog => {
            alertMessage = dialog.message();
            dialog.accept();
        });

        const advisorItem = page.locator('.feed-item').filter({ hasText: 'The Advisor' });
        await expect(advisorItem).toBeVisible();
        await expect(advisorItem).toContainText("Noticed high traffic on the 'Custom Orders' page");

        const dismissBtn = advisorItem.locator('button:has-text("Dismiss")');
        await dismissBtn.click();

        expect(alertMessage).toBe('Dismissed recommendation.');
    });

});
