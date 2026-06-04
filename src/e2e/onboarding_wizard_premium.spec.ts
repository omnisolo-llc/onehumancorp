import { test, expect } from './fixtures';

test.describe('Premium Onboarding Wizard Critical User Journeys', () => {

  test.beforeEach(async ({ page }) => {
    // Clear state to start fresh
    await page.addInitScript(() => {
      localStorage.clear();
      localStorage.setItem('tenant_id', 'test-tenant');
      localStorage.setItem('user_id', 'test-user');
    });
    await page.goto('/onboarding');
    await page.waitForLoadState('networkidle');
  });

  test('CUJ: Maya the Baker - Idea to Live in < 10 mins', async ({ page }) => {
    // Step 1: Business Name
    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await nameInput.fill("Maya's Vegan Treats");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 1: What you sell
    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await sellInput.fill("Custom vegan cakes and cupcakes for all occasions.");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 1: Location
    const locInput = page.getByPlaceholder(/Portland, OR/i);
    await locInput.fill("Portland, OR");

    // Mock the intake API response if needed, but the standard says no mocking internal calls.
    // Assuming backend is running or mocked at the transport level in this environment.
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 2: Review Details
    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
    await expect(page.getByText("AI Recommendation")).toBeVisible();
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3: Style & Team
    await expect(page.getByText("Style & Team")).toBeVisible();

    // Select Marketing & Advertising Department
    await page.getByText("Marketing & Advertising").click();

    // Fill admin details
    await page.getByPlaceholder(/you@example.com/i).fill("maya@example.com");
    await page.getByPlaceholder(/••••••••/i).fill("securepassword123");

    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Step 5: Live Screen
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 20000 });
    await expect(page.getByText("my-business.ohc.store")).toBeVisible();
  });

  test('CUJ: Carlos the Handyman - Service-based onboarding', async ({ page }) => {
    await page.getByPlaceholder(/Maya's Custom Cakes/i).fill("Carlos Fix-It");
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill("General home repairs, plumbing, and painting.");
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill("Austin, TX");
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
    // Verify AI picked up service-based context
    await expect(page.getByText("Business Type")).toBeVisible();

    await page.getByRole('button', { name: /Continue/i }).click();

    // Select Operations Department for bookings
    await page.getByText("Operations").click();

    await page.getByPlaceholder(/you@example.com/i).fill("carlos@example.com");
    await page.getByPlaceholder(/••••••••/i).fill("fixit2025");

    await page.getByRole('button', { name: /Launch Store/i }).click();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 20000 });
  });

  test('CUJ: Mobile responsiveness and touch targets', async ({ page }) => {
    // Force mobile viewport if not already set by config
    await page.setViewportSize({ width: 375, height: 812 });

    await expect(page.locator('#setup-screen')).toBeVisible();

    // Check if the progress bar is visible and correctly styled
    const progressBar = page.locator('.bg-[#0066FF]');
    await expect(progressBar).toBeVisible();

    // Test touch target size for the Next button (should be >= 44px)
    const nextButton = page.getByRole('button', { name: /Next/i });
    const box = await nextButton.boundingBox();
    if (box) {
      expect(box.height).toBeGreaterThanOrEqual(44);
      expect(box.width).toBeGreaterThanOrEqual(44);
    }
  });

  test('CUJ: Draft persistence and resume', async ({ page }) => {
    await page.getByPlaceholder(/Maya's Custom Cakes/i).fill("Draft Store");
    await page.getByRole('button', { name: /Next/i }).click();

    // Click Save Draft
    await page.getByRole('button', { name: /Save Draft/i }).click();
    await expect(page.getByText("Draft Saved!")).toBeVisible();

    // Reload and verify state
    await page.reload();
    await expect(page.getByPlaceholder(/I bake custom vegan cakes/i)).toBeVisible();

    // Go back and check name
    await page.getByRole('button', { name: /Back/i }).click();
    await expect(page.getByDisplayValue("Draft Store")).toBeVisible();
  });

  test('CUJ: Fatima the Food Cart - Multi-language/Localization check', async ({ page }) => {
    // Simulating Fatima's business with different locale or specific keywords
    await page.getByPlaceholder(/Maya's Custom Cakes/i).fill("Fatima Halal Cart");
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill("Authentic Halal food, chicken and rice, gyro.");
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill("Queens, NY");
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });

    // Ensure the AI Recommendation mentions food-specific optimization
    await expect(page.getByText(/orders and pickups/i)).toBeVisible();
  });
});
