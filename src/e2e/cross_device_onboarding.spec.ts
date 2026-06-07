import { test, expect } from './fixtures';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page, browser }) => {
    // 1. Owner starts onboarding directly from the current route.
    await page.goto('/onboarding');

    // Inject fixed IDs to ensure it matches
    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });
    await page.goto('/onboarding');

    // Sometimes the welcome screen shows first, we need to click Start Onboarding to get to the business details
    const startButton = page.getByRole('link', { name: 'Start Onboarding' });
    if (await startButton.isVisible()) {
      await startButton.click({ force: true });
    } else {
        const altStartButton = page.getByRole('button', { name: 'Start Onboarding' });
        if (await altStartButton.isVisible()) {
            await altStartButton.click({ force: true });
        }
    }

    // Verify it landed on the Onboarding page
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 2. Owner enters business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Cross Device Bakery');

    // 3. Click Save Draft
    const saveDraftBtn = page.getByRole('button', { name: /Save Draft/i }).first();
    await saveDraftBtn.click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    // Wait a brief moment for the backend save to complete before moving contexts
    await page.waitForTimeout(500);

    // 4. Simulate a cross-device session with a new browser context
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    // We need to inject the same local storage user ID so the backend knows it's the same user
    await newPage.goto('/dashboard'); // Navigate to the same domain first
    await newPage.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });

    await newPage.goto('/onboarding'); // Re-navigate so it picks up the correct user id

    // If there's a start button, click it
    const newStartButton = newPage.getByRole('button', { name: 'Start Onboarding' });
    if (await newStartButton.isVisible()) {
      await newStartButton.click({ force: true });
    }

    // 5. Verify the business name was properly restored
    await expect(newPage.getByPlaceholder(/e.g. Maya's Custom Cakes/i)).toHaveValue('Cross Device Bakery', { timeout: 10000 });

    await newContext.close();
  });
});
