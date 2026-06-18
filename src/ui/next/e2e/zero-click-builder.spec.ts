import { test, expect } from '@playwright/test';

test.describe('Zero Click Builder Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should generate a storefront and redirect to dashboard', async ({ page }) => {
    // Navigate to the zero-click-builder page
    await page.goto('/zero-click-builder');

    // Ensure the page is loaded
    await expect(page.getByRole('heading', { name: 'Zero-Click Business Generator' })).toBeVisible();

    // Fill the prompt textarea
    const promptInput = page.getByPlaceholder(/e.g., I am a home baker/i);
    await promptInput.fill('I am a baker in Seattle who makes gluten-free cookies');

    // Click "Generate My Business" button
    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await expect(generateBtn).toBeEnabled();

    // We mock the API call in case it takes too long or uses a real LLM, but ideally we want to test E2E.
    // However, the task says "Zero mock data may appear in the UI", and "run against the real local OHC stack."
    // So we'll just click it and wait for the response.
    await generateBtn.click();

    // Wait for the storefront preview iframe or success message to appear
    await expect(page.getByRole('heading', { name: 'Your business is live!' })).toBeVisible({ timeout: 60000 });

    // Verify iframe exists
    const iframe = page.locator('iframe[title="Live Storefront Preview"]');
    await expect(iframe).toBeVisible();

    // Click "Launch My Store" button
    const launchBtn = page.getByRole('button', { name: /Launch My Store/i });

    // Clicking launch should set localStorage and redirect to /dashboard
    await launchBtn.click();

    // Wait for redirect
    await page.waitForURL('**/dashboard**');

    // Verify we are on dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);

    // Also verify local storage has been set
    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id'));
    const userId = await page.evaluate(() => localStorage.getItem('user_id'));

    expect(tenantId).toBeTruthy();
    expect(userId).toBeTruthy();

    // Verify Unified Agent Feed is present and functional
    const agentFeedHeading = page.getByRole('heading', { name: /Action Center/i }).first();
    await expect(agentFeedHeading).toBeVisible({ timeout: 10000 });
  });
});
