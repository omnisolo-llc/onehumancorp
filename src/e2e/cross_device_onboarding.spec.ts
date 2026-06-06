import { test, expect } from './fixtures';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page }) => {
    // Generate a unique tenant/user ID for this test run so drafts don't collide
    const uniqueId = \`test-user-\${Date.now()}\`;

    // 1. Owner starts onboarding directly from the current route.
    await page.goto('/onboarding');
    await page.evaluate((id) => {
      localStorage.setItem('tenant_id', id);
      localStorage.setItem('user_id', id);
    }, uniqueId);

    // Refresh to apply localStorage changes
    await page.reload();

    await expect(page.getByText(/Welcome|Tell us about your business/)).toBeVisible({ timeout: 15000 });
    const startButton = page.getByRole('link', { name: 'Start Onboarding' });
    if (await startButton.isVisible()) {
      await startButton.click({ force: true });
    }

    // Verify it landed on the Onboarding page
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // 2. Owner enters business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Cross Device Bakery');

    // 3. Click Save Draft
    const saveDraftBtn = page.getByRole('button', { name: /Save Draft/i }).first();
    await saveDraftBtn.click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    // 4. Simulate a cross-device session or reload
    // We remove local storage state that holds the form data
    await page.evaluate(() => window.localStorage.removeItem('onboarding-storage-v3'));

    // In local dev, the draft is stored in the Next.js API route's global memory,
    // which is enough to prove the E2E contract works through the real app network layers.
    await page.reload();

    // Wait for the app to load state from the API
    await expect(page.getByText('Tell us about your business')).toBeVisible({ timeout: 15000 });

    // 5. Verify the business name was properly restored
    await expect(page.getByPlaceholder(/e.g. Maya's Custom Cakes/i)).toHaveValue('Cross Device Bakery', { timeout: 10000 });
  });
});
