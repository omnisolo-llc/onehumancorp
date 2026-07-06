import { test, expect } from '@playwright/test';

test.describe('Zero Click Builder Mobile Onboarding', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile-first constraint

  test('User can generate a store with a single prompt', async ({ page, context }) => {
    // Navigate to the zero-click-builder.html page directly (Tauri UI test)
    await page.goto('/zero-click-builder.html');

    // 1. Verify premium tokens & text
    await expect(page.getByText('Zero-Click Business Generator')).toBeVisible();

    // The single text area where the prompt is typed
    const promptInput = page.locator('#prompt');
    await expect(promptInput).toBeVisible();

    // 2. Type natural language prompt
    await promptInput.fill('I am a home baker in Austin selling custom vegan cakes and cupcakes.');

    // Also fill optional image url
    const imageUrlInput = page.locator('#image-url');
    await expect(imageUrlInput).toBeVisible();
    await imageUrlInput.fill('https://example.com/cake.jpg');

    // 3. Find "Generate Store" button
    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // Testing end-to-end flow with real backend

    // 4. Tap Generate Button
    await generateBtn.click();

    // 5. Verify visually engaging loading state
    await expect(page.getByText('Analyzing your business...')).toBeVisible();

    // 6. Verify completion & transition to live preview (it redirects to success.html)
    // Wait for the success page to load and display "You're Live!"
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });

    // Verify auth/redirect handoff button from success.html
    const dashboardBtn = page.getByRole('button', { name: /Go to Dashboard/i });
    if (await dashboardBtn.isVisible()) {
        await dashboardBtn.click();
        await expect(page).toHaveURL(/.*dashboard.*/);
    }
  });
});
