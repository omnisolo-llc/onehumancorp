import { test, expect } from './onboarding_fixtures';
import { completeReactZeroClickReview } from './react_zero_click_review';

test.describe('Zero Click Builder Viral Growth Loop', () => {
  test('should allow an owner to generate a store from a single prompt and see viral share option', async ({ page, onboardingOwner }, testInfo) => {
    // Navigate to the new growth feature
    // The fixture authenticates a fresh test-owned tenant.


    await page.goto('/onboarding/zero-click');
    await page.locator('input[placeholder*="baker"]').click();


    // Verify mobile-first layout
    await page.setViewportSize({ width: 375, height: 812 });

    // Verify title
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Verify "OmniSolo" branding is present (viral loop)
    await expect(page.locator('#dashboard-footer-viral-link')).toBeVisible();

    // The generate button should be disabled initially
    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeDisabled();

    // Fill in the prompt
    await page.fill('#instant-bio', 'I am a local coffee roaster in Seattle needing a storefront.');

    // The button should now be enabled
    await expect(generateBtn).toBeEnabled();

    const pending = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/onboarding/chat' && response.request().method() === 'POST').then(async response => ({ status: response.status(), body: await response.json() }));
    const [result] = await Promise.all([pending, generateBtn.click()]);
    const mode = await completeReactZeroClickReview(page, onboardingOwner, result);
    testInfo.annotations.push({ type: 'setup-mode', description: mode });
    await expect(page.getByRole('button', { name: /Share on X/i })).toBeVisible();
    await page.getByRole('button', { name: /Go to dashboard/i }).click();
    // Check navigation to dashboard
    await expect(page).toHaveURL(/.*dashboard/, { timeout: 15000 });
  });
});