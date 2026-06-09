import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test review_campaign_growth_loop', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'review_campaign_growth_loop');
});

test.describe('Automated Review Campaign Growth Loop', () => {
  test('should generate a review draft and trigger the soft paywall', async ({ page }) => {
    // 1. Navigate to the review campaigns page
    await page.goto('/review-campaigns');

    // 2. Wait for page load
    await page.waitForLoadState('networkidle');
    await expect(page.getByRole('heading', { name: 'Automated Review Campaigns ⭐️' })).toBeVisible();

    // 3. Fill in the "Product to Feature" input
    const productInput = page.getByLabel('Product to Feature (Optional)');
    await productInput.fill('Signature Coffee Blend');

    // 4. Trigger AI Generation
    const generateBtn = page.getByRole('button', { name: /Generate Email Draft/i });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // 5. Verify that the AI Generated Draft is rendered and contains the target strings
    const draftContainer = page.locator('pre');
    await expect(draftContainer).toBeVisible({ timeout: 15000 });
    const draftText = await draftContainer.textContent();
    expect(draftText).toContain('Signature Coffee Blend');
    expect(draftText).toContain('Powered by OHC');

    // 6. Click "Send to Audience"
    const sendBtn = page.getByRole('button', { name: /Send to Audience/i });
    await expect(sendBtn).toBeVisible();
    await sendBtn.click();

    // 7. Verify the soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Unlock Automated Campaigns' });
    await expect(paywallHeading).toBeVisible();
    await expect(page.getByText('Sending AI-generated review campaigns is a Pro feature.')).toBeVisible();

    // 8. Click "Maybe Later" and ensure it closes
    const maybeLaterBtn = page.getByRole('button', { name: 'Maybe Later' });
    await maybeLaterBtn.click();

    await expect(paywallHeading).toBeHidden();
  });
});
