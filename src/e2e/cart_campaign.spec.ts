import { test, expect } from './fixtures';

test.describe('Cart Campaign Flow', () => {
  test('verify cart campaign generation and sending', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Click on Recover Cart
    const recoverCartBtn = page.getByRole('button', { name: 'Recover Cart' });
    await expect(recoverCartBtn).toBeVisible();
    await recoverCartBtn.click();

    // 3. Verify the Abandoned Cart Modal is opened
    const modalHeading = page.getByRole('heading', { name: 'AI Cart Recovery' });
    await expect(modalHeading).toBeVisible();

    // 4. Verify message text is populated from backend
    const textArea = page.locator('textarea');
    await expect(textArea).toBeVisible();
    await expect(textArea).toHaveValue(/We noticed you left some items in your cart totaling \$85\.00/i, { timeout: 5000 });

    // 5. Send campaign
    const sendCampaignBtn = page.getByRole('button', { name: 'Send Campaign' });
    await sendCampaignBtn.click();

    // 6. Verify campaign is sent successfully
    await expect(page.getByText('Campaign Sent Successfully!')).toBeVisible();

    // 7. Verify the modal can be closed
    const closeButton = page.locator('div.fixed').locator('button').first();
    await closeButton.click();

    await expect(modalHeading).not.toBeVisible();
  });
});
