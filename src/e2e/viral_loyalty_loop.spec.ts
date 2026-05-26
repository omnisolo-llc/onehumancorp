import { test, expect } from './fixtures';

test.describe('Customer Loyalty Anniversary Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard where the growth loop is implemented
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
  });

  test('should display and process the customer loyalty anniversary alert', async ({ page }) => {
    // 1. Locate the "Customer Loyalty Alert" card
    const loyaltyAlert = page.locator('section').filter({ hasText: 'Customer Loyalty Alert' });
    await expect(loyaltyAlert).toBeVisible();
    await expect(loyaltyAlert.getByText('Alex M. made their first purchase exactly 1 year ago today.')).toBeVisible();

    // 2. Click "Draft Anniversary Email" to trigger the modal
    const draftButton = loyaltyAlert.getByRole('button', { name: 'Draft Anniversary Email' });
    await draftButton.click();

    // 3. Verify the Anniversary Celebration modal appears
    const modalHeading = page.getByRole('heading', { name: 'Anniversary Celebration' });
    await expect(modalHeading).toBeVisible();

    // 4. Verify drafting state appears briefly, then the message text is populated
    const textArea = page.locator('textarea');
    await expect(textArea).toBeVisible();
    await expect(textArea).toHaveValue(/We can't believe it's already been 1 year\(s\) since your first order/i, { timeout: 5000 });
    await expect(textArea).toHaveValue(/ANNIVERSARY20/i);
    await expect(textArea).toHaveValue(/⚡ Powered by OHC/i);

    // 5. Send the email
    const sendButton = page.getByRole('button', { name: 'Send Email' });
    await sendButton.click();

    // 6. Verify the success state
    const successHeading = page.getByRole('heading', { name: 'Email Sent!' });
    await expect(successHeading).toBeVisible({ timeout: 5000 });

    // 7. Verify the modal eventually closes automatically (it has a 2500ms timeout)
    await expect(successHeading).toBeHidden({ timeout: 5000 });
    await expect(modalHeading).toBeHidden();
  });
});
