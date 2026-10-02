import { test, expect } from '../../../../e2e/onboarding_fixtures';
import { completeReactZeroClickReview } from '../../../../e2e/react_zero_click_review';

test.describe('Onboarding flows', () => {
  test('Zero-Click Onboarding flow interactive steps', async ({ page, onboardingOwner }, testInfo) => {
    // 1. Start at the zero-click onboarding page
    await page.goto('/onboarding/zero-click');
    await expect(page).toHaveTitle(/OmniSolo/);

    // 2. Verify initial rendering and text
    await expect(page.locator('text=Tell us about your business')).toBeVisible();
    await expect(page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.')).toBeVisible();

    // 3. Fill in the input field
    const input = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await input.fill('I sell custom sneakers in New York.');

    // 4. Submit the form
    const submitBtn = page.getByRole('button', { name: 'Send message', exact: true });
    await expect(submitBtn).toBeEnabled();
    const pending = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/onboarding/chat' && response.request().method() === 'POST').then(async response => ({ status: response.status(), body: await response.json() }));
    const [result] = await Promise.all([pending, submitBtn.click()]);
    const mode = await completeReactZeroClickReview(page, onboardingOwner, result);
    testInfo.annotations.push({ type: 'setup-mode', description: mode });
    await expect(page.getByRole('button', { name: /Go to dashboard/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Share on X/i })).toBeVisible();
  });
});
