import { test, expect } from './fixtures';

test.describe('Viral Gift Cards Loop', () => {
  test('should generate a gift card with a referral loop link', async ({ page, request }) => {
    // Navigate to the Gift Cards page
    await page.goto('/gift-cards');

    // Wait for the page to load
    await expect(page.getByRole('heading', { name: 'Gift Card Generator 🎁' })).toBeVisible({ timeout: 15000 });

    // Set the Gift Card Value to 150
    const valueInput = page.locator('input[type="number"]');
    await valueInput.fill('150');

    // Click on Generate Gift Card
    const generateBtn = page.getByRole('button', { name: 'Generate Gift Card' });
    await generateBtn.click();

    // The share modal should be visible
    await expect(page.getByText('Share Your Gift Card')).toBeVisible();
    await expect(page.locator('input[aria-label="Gift Card Link"]')).toHaveValue(/https?:\/\/[^\/]+\/gift-card\?amount=150&ref=.*$/);

    // Verify the "Powered by OHC" footer link
    const poweredByLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(poweredByLink).toBeVisible();

    // Verify the href link is correct
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=');
    expect(href).toContain('source=gift_card');
  });
});
