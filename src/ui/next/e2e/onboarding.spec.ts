import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByText(/Tell us about your business/i)).toBeVisible();

    // Fill in the description
    const descriptionInput = page.locator('textarea[placeholder*="vegan cakes"]');
    await descriptionInput.fill('I am a freelance handyman in Miami');

    // Intercept API calls
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: { initial_products: [{ name: 'Custom Cake', price: '25.00' }] }
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      json: { message: "Your business has been successfully launched." }
    }));

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // 2. Wait for Review Details Step
    await expect(page.getByText(/Review Details/i)).toBeVisible({ timeout: 5000 });

    // Continue to next step
    await page.getByRole('button', { name: /Continue/i }).click();

    // 3. Wait for Style & Team Step
    await expect(page.getByText(/Style & Team/i)).toBeVisible({ timeout: 5000 });

    // Select Template and Launch
    await page.getByText('Classic').click();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // 4. Loading screen
    await expect(page.getByText(/Building Your Business/i)).toBeVisible({ timeout: 5000 });

    // 5. Live Screen
    await expect(page.getByText(/You're Live!/i)).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(/Your business has been successfully launched./i)).toBeVisible();
    await expect(page.getByText(/my-business.ohc.store/i)).toBeVisible();

    const dashboardLink = page.getByRole('link', { name: /Go to Dashboard/i });
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');

    await dashboardLink.click();
    await page.waitForURL('**/dashboard');

    await expect(page.getByText(/Morning Briefing/i)).toBeVisible();
    await expect(page.getByRole('link', { name: /Add your first product/i })).toBeVisible();
  });
});
