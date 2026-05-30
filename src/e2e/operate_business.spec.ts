import { test, expect } from './fixtures';

test('Maya operates her custom cake business', async ({ page }) => {
    // Navigate to the onboarding flow
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    // Step 1: Business Name
    const businessNameInput = page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]');
    await businessNameInput.waitFor({ state: 'visible' });
    await businessNameInput.fill('Maya Bakery');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    const sellInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]');
    await sellInput.waitFor({ state: 'visible' });
    await sellInput.fill('I bake amazing custom cakes and pastries.');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    const locationInput = page.locator('input[placeholder="e.g. Portland, OR"]');
    await locationInput.waitFor({ state: 'visible' });
    await locationInput.fill('Portland, OR');

    // Submit details and generate business
    await page.locator('button:has-text("Generate My Business")').click();
    await page.waitForTimeout(1500);

    // Step 4: Continue from details overview
    const continueBtn = page.locator('button:has-text("Continue")');
    await continueBtn.waitFor({ state: 'visible' });
    await continueBtn.click();
    await page.waitForTimeout(1000);

    // Step 5: Launch Store
    const launchBtn = page.locator('button:has-text("Launch Store")');
    await launchBtn.waitFor({ state: 'visible' });
    await launchBtn.click();
    await page.waitForTimeout(1000);

    // Step 6: Go to Dashboard
    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await dashboardLink.waitFor({ state: 'visible' });
    await dashboardLink.click();
    await page.waitForTimeout(2000);

    // Verify Dashboard
    expect(page.url()).toContain('/dashboard');
    await expect(page.locator('h1')).toContainText('Dashboard');
});
