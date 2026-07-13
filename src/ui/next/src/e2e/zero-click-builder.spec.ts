import { test, expect } from '../../../../e2e/fixtures';

test.describe('Zero Click Builder Mobile Onboarding', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile-first constraint

  test('User can generate a store with a single prompt', async ({ page, context }) => {
    // Navigate to the zero-click-builder page
    await page.goto('/zero-click-builder');

    // 1. Verify premium tokens & text
    await expect(page.getByText('Zero-Click Business Generator')).toBeVisible();

    // The single text area where the prompt is typed
    const promptInput = page.locator('#prompt');
    await expect(promptInput).toBeVisible();

    // 2. Type natural language prompt
    await promptInput.fill('I am a home baker in Austin selling custom vegan cakes and cupcakes.');

    // 3. Find "Generate Store" button
    const generateBtn = page.getByRole('button', { name: /Generate Store/i });
    await expect(generateBtn).toBeEnabled();

    // Testing end-to-end flow with real backend

    // 4. Tap Generate Button
    await generateBtn.click();

    // 5. Verify visually engaging loading state
    await expect(page.getByText('Analyzing your business...')).toBeVisible();

    // 6. Verify completion & transition to live preview
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 15000 });

    // Check if iframe for preview rendered
    await expect(page.locator('iframe[title="Live Storefront Preview"]')).toBeVisible();

    // Verify auth/redirect handoff button
    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });
    await expect(launchBtn).toBeVisible();

    // Test the button navigates to dashboard
    await launchBtn.click();
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
