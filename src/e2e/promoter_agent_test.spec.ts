import { test, expect } from './fixtures';
import { aiJudgeScore } from './ai-judge';

test.describe('The Promoter Agent', () => {
  test('generates marketing copy for a new product and allows 1-tap scheduling', async ({ page, loginAs, unlimitedAdminUser }) => {
    // 1. Log in as unlimited admin user
    await loginAs(page, unlimitedAdminUser);

    // 2. Navigate to Dashboard and locate the Promoter Card
    await page.goto('/dashboard.html');
    const promoterCard = page.locator('#promoter-card');
    await expect(promoterCard).toBeVisible();
    await expect(promoterCard).toContainText('The Promoter Agent');

    // 3. Click the link to go to the Promoter app
    await page.locator('#promoter-btn').click();

    // Verify we are on the promoter page
    await expect(page).toHaveURL(/.*promoter\.html.*/);

    // 4. Input product details
    const testProductName = 'Artisan Vegan Truffles';
    await page.locator('#product-name').fill(testProductName);
    await page.locator('#product-desc').fill('Handcrafted, dairy-free chocolate truffles made with organic cocoa.');

    // 5. Click generate
    await page.locator('#generate-btn').click();

    // 6. Wait for variants to appear
    const resultsSection = page.locator('#results-section');
    await expect(resultsSection).toBeVisible({ timeout: 15000 });

    // 7. Verify we got multiple variants back
    const variantCards = page.locator('.variant-card');
    await expect(variantCards).toHaveCount(3);

    // Verify content includes product name
    const firstVariantText = await variantCards.nth(0).locator('.variant-content').innerText();
    expect(firstVariantText).toContain('Truffles');

    // 8. Test the 1-tap "Approve & Send" button
    const approveBtn = variantCards.nth(0).locator('.action-btn');
    await expect(approveBtn).toHaveText('Approve & Send');
    await approveBtn.click();
    await expect(approveBtn).toHaveText('Scheduled!');
    await expect(approveBtn).toBeDisabled();

    // 9. AI Judge score verification (optional, checking if it really sounds like marketing)
    // Here we'll just check if the generated text is somewhat coherent for marketing
    const score = await aiJudgeScore(firstVariantText, 'Is this a coherent social media marketing post for chocolate truffles?');
    expect(score).toBeGreaterThan(7);
  });
});
