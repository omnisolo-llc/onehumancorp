import { test, expect } from '@playwright/test';

test.describe('Onboarding to Dashboard Flow', () => {
  test('Completes wizard with instant build and verifies morning briefing', async ({ page }) => {
    await page.goto('/');

    // E2E test starting from unauthenticated state
    await page.goto('/business-setup');

    // Step 0: Welcome
    await expect(page.getByText('Your business,')).toBeVisible();
    await page.getByText('Instant Build').click();

    // Step 11: Instant Build
    await expect(page.getByText('Tell us about your business')).toBeVisible();
    await page.getByRole('textbox').first().fill('I run a local bakery called Maya\'s Cakes.');
    await page.getByText('Generate Storefront').click();

    // Verify AI generation state
    await expect(page.getByText('The Promoter is designing your storefront...')).toBeVisible();

    // Simulate waiting for generation completion (in a real scenario, this resolves after AI API finishes)
    // The test asserts that eventually it reaches the launch state.
    await expect(page.getByText('Ready to launch!')).toBeVisible({ timeout: 15000 });
    await page.getByText('Launch My Business').click();

    // Verify dashboard renders correctly without network mocking
    await expect(page.getByText('My Business')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Morning Briefing')).toBeVisible();
    await expect(page.getByText('Add your first product')).toBeVisible();
  });
});
