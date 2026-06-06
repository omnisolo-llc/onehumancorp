import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('review_campaigns');

test.describe('Review Campaigns Growth Feature', () => {
  test('generates and sends an AI review campaign draft', async ({ page }) => {
    // 1. Go to the review campaigns page
    await page.goto('/review-campaigns');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // 2. Verify page heading
    await expect(page.getByRole('heading', { name: 'Automated Review Campaigns ⭐️' })).toBeVisible();

    // 3. Fill in the Campaign Configuration form
    await page.getByLabel('Product to Feature (Optional)').fill('Super Premium Espresso Beans');

    // 4. Generate the draft
    await page.getByRole('button', { name: 'Generate Email Draft' }).click();

    // The draft should generate text containing the product name from the backend API fallback (since real LLM is not in the e2e test environment by default unless mock is provided)
    await expect(page.getByText(/Super Premium Espresso Beans/)).toBeVisible({ timeout: 15000 });

    // 5. Send campaign flow (should trigger paywall or send logic)
    const sendButton = page.locator('button', { hasText: 'Send to Audience' });
    await sendButton.click();

    // Since Pro mode might not be active, it could trigger the Soft Paywall modal
    // Verify paywall modal appears
    await expect(page.getByRole('heading', { name: 'Unlock Automated Campaigns' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'View Plans & Upgrade' })).toBeVisible();

    // Close the modal
    await page.getByRole('button', { name: 'Maybe Later' }).click();
    await expect(page.getByRole('heading', { name: 'Unlock Automated Campaigns' })).not.toBeVisible();
  });
});
