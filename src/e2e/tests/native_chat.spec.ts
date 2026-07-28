import { test, expect } from '@playwright/test';
import { setupMockContext, getTenantId } from '../setupTests';

test.describe('Native Omnichannel Chat E2E', () => {
    test.beforeEach(async ({ page }) => {
        await setupMockContext(page);
    });

    test('Business owner can receive a message and reply in unified inbox', async ({ page, request }) => {
        const tenantId = await getTenantId(page);

        // Use real interactions as a business owner for the chat feature
        await page.goto('/inbox');

        // Assume there is a "New Chat" button to simulate incoming from a customer
        await page.getByRole('button', { name: 'New Chat' }).click();
        await page.getByPlaceholder('Enter customer name').fill('Maya Customer');
        await page.getByPlaceholder('Type a message...').fill('Do you have vegan cakes?');
        await page.getByRole('button', { name: 'Send' }).click();

        // Verify message appeared in the feed
        await expect(page.getByText('Do you have vegan cakes?')).toBeVisible();

        // Reply as the business owner
        await page.getByPlaceholder('Type a message...').fill('Yes, we have vegan options!');
        await page.getByRole('button', { name: 'Send' }).click();

        // Verify reply appeared in the feed
        await expect(page.getByText('Yes, we have vegan options!')).toBeVisible();
    });
});
