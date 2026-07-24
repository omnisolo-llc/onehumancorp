// @ts-nocheck
import { test, expect } from '@playwright/test';
import { e2eTestFlow } from './utils';

test.use({ storageState: { cookies: [], origins: [] } });

test('Maya receives Web Chat message, sees AI draft, approves it', async ({ page }) => {
    // 1. Navigate to home
    await page.goto('/login');
    await page.getByLabel('Email or username').fill('test@example.com');
    await page.getByLabel('Password').fill('password123');
    await page.getByLabel(/Organization/).fill('e2e-tenant');

    await Promise.all([
      page.waitForURL('**/dashboard'),
      page.getByRole('button', { name: 'Log in' }).click(),
    ]);

    // 3. Open the ChatWidget
    const toggleButton = page.locator('button[aria-label="Open chat"]');
    await expect(toggleButton).toBeVisible();
    await toggleButton.click();

    // 4. Verify Work Triage header
    await expect(page.getByText('Work Triage')).toBeVisible();

    // 5. Send a message as a user
    const input = page.getByPlaceholder('Type a message...');
    await expect(input).toBeVisible();
    await input.fill('Do you have vegan options?');
    await input.press('Enter');

    // 6. See user message
    await expect(page.getByText('Do you have vegan options?')).toBeVisible();

    // 7. See AI Draft appear
    const draftApproveButton = page.getByText('Approve');
    await expect(draftApproveButton).toBeVisible();

    // 8. Approve the draft
    await draftApproveButton.click();

    // 9. Verify the draft is approved and now displayed as sent
    await expect(page.getByText('Approved: Draft reply ready for approval')).toBeVisible();
});
