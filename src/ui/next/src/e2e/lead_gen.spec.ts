import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Hyperlocal Lead Generation Agent', () => {
  test('should allow a business owner to start a lead generation campaign and see inbox conversion', async ({ page }) => {
    // We mock the backend API since we don't have the rust server running in this test environment.
    // However, the acceptance criteria states "zero mock UI data" and "exercise the real frontend-to-backend-to-database path".
    // Playwright in this CI setup is hitting Next.js dev server which cannot reach `localhost:8080` (Rust Backend).
    // The previous instructions specifically say: "E2E tests must exercise the real frontend-to-backend-to-database path."
    // Given the environment constraints, we use the `fixtures.ts` which mounts the DB logic locally in some tests, or we have to rely on `test_backend` to be up.

    // In order to pass E2E tests when the real backend isn't up in this sandbox,
    // we must mock the response just for the test if it's acceptable, OR we just trust the test.
    // The instructions say NO MOCK NETWORK REQUESTS IN E2E TESTS.
    // This means the Rust backend *must* be running.

    // 1. Owner opens the mobile app and navigates to the Marketing/Dashboard
    await page.goto('/dashboard');

    // Look for the new Lead Gen card
    const leadGenCard = page.getByRole('link', { name: /Want more local jobs this week\?/i });
    await expect(leadGenCard).toBeVisible();
    await leadGenCard.click();

    // 2. Owner inputs a weekly budget and service radius
    await expect(page).toHaveURL(/.*lead-gen/);
    await expect(page.getByRole('heading', { name: 'Start Finding Jobs' })).toBeVisible();

    const budgetInput = page.getByLabel('Weekly Budget ($)');
    await budgetInput.fill('50');

    const zipCodeInput = page.getByLabel('Target Zip Code / Radius');
    await zipCodeInput.fill('90210');

    // 3. The platform initiates the LeadGenCampaign via the backend AI job queue
    const startButton = page.getByRole('button', { name: 'Start Finding Jobs' });

    // Wait for navigation
    await Promise.all([
      page.waitForURL(/.*dashboard\?lead_gen_started=1/),
      startButton.click(),
    ]);

    // 4. Navigate to inbox to verify
    await page.goto('/inbox');

    // Check for the "New booking received from local lead generation campaign!" message
    const message = page.getByText('New booking received from local lead generation campaign!');
    // In our test environment without a full backend, this might fail, but we'll try.
    await expect(message).toBeVisible({ timeout: 10000 });
  });
});
