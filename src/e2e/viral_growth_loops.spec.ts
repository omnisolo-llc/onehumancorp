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

  test('verify abandoned cart recovery loop', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for page to be mostly loaded
    await page.waitForLoadState('networkidle');

    // 2. Click the Recover Cart button
    const recoverCartBtn = page.getByRole('button', { name: 'Recover Cart' });
    await expect(recoverCartBtn).toBeVisible({ timeout: 10000 });
    await recoverCartBtn.click();

    // 3. Verify the AI Cart Recovery modal is opened
    const modalHeading = page.getByRole('heading', { name: 'AI Cart Recovery' });
    await expect(modalHeading).toBeVisible();

    // Wait for the textarea to be visible before asserting value
    const generatedText = page.locator('textarea');
    await expect(generatedText).toBeVisible({ timeout: 5000 });

    // Explicitly wait for the value to settle, bypassing simple assertions
    await page.waitForFunction(() => {
        const ta = document.querySelector('textarea');
        return ta && ta.value && ta.value.includes('We noticed you left some items in your cart');
    }, { timeout: 10000 });

    // 4. Send the campaign
    const sendCampaignBtn = page.getByRole('button', { name: 'Send Campaign' });
    await sendCampaignBtn.click();

    // Verify success message
    await expect(page.getByText('Campaign Sent Successfully!')).toBeVisible({ timeout: 5000 });

    // 5. Verify the modal can be closed
    const closeButton = page.locator('div.fixed').locator('button').first();
    await closeButton.click();

    await expect(modalHeading).not.toBeVisible();
  });
});
