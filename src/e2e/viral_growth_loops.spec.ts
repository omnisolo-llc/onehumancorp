import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Growth Loops', () => {
  test('verify automated AI review request growth loop flow in CustomerSuccess department', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Verify Action Required is present and the specific automated review request is loaded from DB
    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 10000 });

    const reviewRequestApproval = page.locator('div.p-5').filter({ hasText: "3 customers haven't reviewed their orders. Request reviews?" });
    await expect(reviewRequestApproval).toBeVisible();

    // 3. Approve the review request
    const approveBtn = reviewRequestApproval.getByRole('button', { name: 'Approve' });
    await approveBtn.click();

    // 4. Verify the item is removed from the Action Required list
    await expect(reviewRequestApproval).not.toBeVisible();

    // 5. Verify the AI Review Request modal is opened
    const modalHeading = page.getByRole('heading', { name: 'AI Review Request' });
    await expect(modalHeading).toBeVisible();

    // Verify drafting state appears briefly, then the message text is populated
    const textArea = page.locator('textarea');
    await expect(textArea).toBeVisible();
    await expect(textArea).toHaveValue(/We noticed you recently received your Signature Coffee Blend/i, { timeout: 5000 });

    // 6. Verify the modal can be closed
    const closeButton = page.locator('div.fixed').locator('button').first();
    await closeButton.click();

    await expect(modalHeading).not.toBeVisible();
  });
});
