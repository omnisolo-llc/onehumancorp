import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_goal_tracker');

test.describe('Viral Goal Tracker Loop', () => {
  test('should allow owner to create a goal tracker widget with soft paywall flow', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');


    // 2. Find and click the Goal Tracker link
    const trackerLink = page.locator('a[href="/goal-tracker"]');
    await expect(trackerLink).toBeVisible();
    await trackerLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: /Viral Goal Tracker/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Goal Details' })).toBeVisible();

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // Make sure we simulate a user without Pro initially
    await page.route("**/api/v1/auth/me", async route => { await route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify({ tenant: { plan_tier: "free" } }) }); });
    await page.reload();
    await page.waitForTimeout(500);

    // 3. Fill out the configuration
    const titleInput = page.getByLabel('Goal Name');
    await titleInput.fill('Help us reach 100 sales!');

    const targetInput = page.getByLabel('Target Number');
    await targetInput.fill('100');

    // 4. Click generate link, which should trigger the paywall
    const generateBtn = page.getByRole('button', { name: 'Generate Tracker Widget' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Verify the soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();

    // 6. Intercept the Twitter share which extends the trial
    const shareBtn = page.getByRole('button', { name: 'Share on X to get 7 Days Free' });
    await expect(shareBtn).toBeVisible();

    await page.route("**/api/v1/growth/trial-extension/claim", async route => { await route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify({ success: true, message: "Trial claimed" }) }); });

    // Mock window.open to prevent the actual popup
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    await shareBtn.click();

    // 7. Verify soft paywall is closed
    await expect(paywallHeading).toBeHidden({ timeout: 15000 });

    // Wait for AI/Widget generation to complete
    await expect(page.getByText('Widget Ready!')).toBeVisible({ timeout: 15000 });

    // Capture the HTML
    const linkInput = page.locator('input[readonly]');
    const generatedHtml = await linkInput.inputValue();
    expect(generatedHtml).toContain('Help us reach 100 sales!');
    expect(generatedHtml).toContain('Target: 100');
    expect(generatedHtml).toContain('Powered by OHC');
  });
});
