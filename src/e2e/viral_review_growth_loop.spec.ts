
import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// Remove currentAppSmoke that causes TypeError

test.describe('Viral Review Growth Loop', () => {
  test('submitting a 5-star review reveals the viral referral widget', async ({ page }) => {

    await page.goto('/leave-review?order=e2e-order-123');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Check if the form is visible
    try {
      await expect(page.getByRole('heading', { name: 'How was your experience?' })).toBeVisible({ timeout: 5000 });
    } catch (e) {
      console.log(await page.content());
      throw e;
    }

    await expect(page.getByText('Order #e2e-order-123')).toBeVisible();

    // Verify "Powered by OHC" branding is on the review form
    await expect(page.locator('.branding-form')).toBeVisible();

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
    await expect(page.locator('.branding-success')).toBeVisible();

    // Verify a link was generated
    const linkInput = page.locator('input[readonly]');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/^http/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
