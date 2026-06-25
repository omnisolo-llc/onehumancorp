import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_review_growth_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_review_growth_loop');
});

test.describe('Viral Review Growth Loop', () => {
  test('submitting a 5-star review reveals the viral referral widget', async ({ page }) => {
    // Navigate to the leave review page
    await page.goto('/leave-review?order=e2e-order-123');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Check if the form is visible
    await expect(page.getByRole('heading', { name: 'How was your experience?' })).toBeVisible();
    await expect(page.getByText('Order #e2e-order-123')).toBeVisible();

    // Verify "Powered by OHC" branding is on the review form
    await expect(page.getByText('⚡ Powered by OHC')).toBeVisible();

    // Click the 5th star
    const stars = page.locator('button:has(span:has-text("★"))');
    await expect(stars).toHaveCount(5);
    await stars.nth(4).click();

    // Optional: write a review
    await page.getByLabel('Tell us more (optional)').fill('Absolutely loved it!');

    // Submit the review
    await page.getByRole('button', { name: 'Submit Review' }).click();

    // Wait for the success screen
    await expect(page.getByRole('heading', { name: 'Thank you for your review!' })).toBeVisible();

    // Verify the viral widget is visible
    await expect(page.getByRole('heading', { name: 'Get 15% Off Your Next Order' })).toBeVisible();

    // Verify the "Powered by OHC" branding in the viral widget
    await expect(page.getByText('⚡ Powered by OHC')).toBeVisible();

    // Verify a link was generated
    const linkInput = page.locator('input[readonly]');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/^http/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});

  test('submitting a low review reveals generic feedback without referral link', async ({ page }) => {
    await page.goto('/leave-review.html');
    await page.waitForLoadState('networkidle');
    const stars = page.locator('button:has(span:has-text("★"))');
    await expect(stars).toHaveCount(5);
    await stars.nth(2).click();
    await page.getByRole('button', { name: 'Submit Review' }).click();
    await expect(page.getByRole('heading', { name: 'Thank you for your review!' })).toBeVisible();
    await expect(page.getByText('We appreciate your feedback and will use it to improve our service.')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Get 15% Off Your Next Order' })).not.toBeVisible();
  });

  test('review form submits even if optional comment is not filled', async ({ page }) => {
    await page.goto('/leave-review.html');
    await page.waitForLoadState('networkidle');
    const stars = page.locator('button:has(span:has-text("★"))');
    await expect(stars).toHaveCount(5);
    await stars.nth(4).click();
    await page.getByRole('button', { name: 'Submit Review' }).click();
    await expect(page.getByRole('heading', { name: 'Thank you for your review!' })).toBeVisible();
  });

  test('review form fails validation and disables submit if 0 stars', async ({ page }) => {
    await page.goto('/leave-review.html');
    await page.waitForLoadState('networkidle');
    const submitBtn = page.getByRole('button', { name: 'Submit Review' });
    await expect(submitBtn).toBeDisabled();
  });

  test('return to store button works from success page', async ({ page }) => {
    await page.goto('/leave-review.html');
    await page.waitForLoadState('networkidle');
    const stars = page.locator('button:has(span:has-text("★"))');
    await stars.nth(4).click();
    await page.getByRole('button', { name: 'Submit Review' }).click();
    await expect(page.getByRole('heading', { name: 'Thank you for your review!' })).toBeVisible();
    const returnBtn = page.getByRole('button', { name: 'Return to Store' });
    await expect(returnBtn).toBeVisible();
  });
