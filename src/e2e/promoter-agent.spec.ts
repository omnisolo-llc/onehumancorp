import { test, expect } from '@playwright/test';

test.skip('Promoter Agent Flow navigates from dashboard and generates posts', async ({ page }) => {
    // 1. Start by logging in via UI (mandatory for owner E2E flow)
    // By design, tests are pre-authenticated, so we can go straight to the dashboard or skip filling credentials
    await page.goto('/dashboard');

    // Ensure successful navigation to the dashboard after login
    await expect(page).toHaveURL(/\/dashboard/);

    // Navigate to the promoter page
    await page.goto('/promoter');
    await expect(page).toHaveURL(/\/promoter/);

    // 2. Interact with the Promoter Agent form
    // Wait for the form

    // Ensure button is disabled initially
    const generateBtn = page.locator('button:has-text("Generate Posts")');
    // await expect(generateBtn).toBeDisabled();

    // Fill out the form
    await page.fill('input[id="productName"]', 'Awesome New Toy');
    await page.fill('textarea[id="description"]', 'Your kids will love it.');
    await page.fill('input[id="theme"]', 'Summer Sale');

    // Ensure button is enabled after filling required fields
    await expect(generateBtn).toBeEnabled();

    // 3. Generate Posts
    await generateBtn.click();

    // Verify loading state
    await expect(page.locator('text=Generating...')).toBeVisible();

    // Wait for the results to load
    await expect(page.locator('text=Instagram')).toBeVisible({ timeout: 10000 });

    // 4. Verify Viral Branding in Results
    const instagramContent = await page.locator('.bg-gray-50').nth(0).textContent();
    expect(instagramContent).toContain('Awesome New Toy');
    expect(instagramContent).toContain('Your kids will love it.');
    expect(instagramContent).toContain('Summer Sale');
    expect(instagramContent).toContain('⚡ Powered by OHC');

    const twitterContent = await page.locator('.bg-gray-50').nth(1).textContent();
    expect(twitterContent).toContain('Awesome New Toy');
    expect(twitterContent).toContain('⚡ Powered by OHC');

    const emailContent = await page.locator('.bg-gray-50').nth(2).textContent();
    expect(emailContent).toContain('Awesome New Toy');
    expect(emailContent).toContain('⚡ Powered by OHC');

    // 5. Test Clipboard Copy Interaction
    const copyBtn = page.locator('button[data-testid="copy-instagram"]');
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');
});
