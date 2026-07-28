import { test, expect } from './fixtures';

test.describe('Viral Streak Widget Generator', () => {
  test('should load the generator, update live preview, show paywall, and generate embed code', async ({ page, loginAs, unlimitedAdminUser }) => {
    // E2E UI tests usually require auth in OHC
    await loginAs(page, unlimitedAdminUser);

    // Using the internal app router URL logic
    await page.goto('/viral-streak-widget');

    // Wait for the page to load
    await expect(page.locator('h1')).toContainText('Viral Streak Generator');

    // 1. Verify Default Live Preview
    const previewContainer = page.locator('.flex-1.flex.flex-col.p-8').first();
    await expect(previewContainer).toBeVisible();
    await expect(previewContainer.locator('h3.text-2xl')).toContainText('Build your daily streak!');
    await expect(previewContainer.locator('p').first()).toContainText('Hit 7 days to unlock a mystery discount');

    // 2. Modify inputs and check preview updates
    const titleInput = page.locator('input').nth(0); // Title is the first input
    await titleInput.fill('Read 10 Pages Daily');

    const goalInput = page.locator('input[type="number"]');
    await goalInput.fill('5');

    const rewardInput = page.locator('input').nth(2); // Reward is the third input
    await rewardInput.fill('a free ebook');

    await expect(previewContainer.locator('h3.text-2xl')).toContainText('Read 10 Pages Daily');
    await expect(previewContainer.locator('p').first()).toContainText('Hit 5 days to unlock a free ebook');

    // 3. Test Soft Paywall
    const removeBrandingCheckbox = page.locator('input#removeBranding');
    await expect(removeBrandingCheckbox).toBeVisible();

    // The test user is unlimitedAdminUser so it might actually have Pro,
    // if the UI checks it and hides the "PRO" badge, it won't show the paywall.
    // So let's evaluate window to pretend we are not Pro to trigger paywall
    await page.evaluate(() => {
        window.localStorage.setItem('has_pro', 'false');
    });

    await page.reload();
    await titleInput.fill('Read 10 Pages Daily');
    await goalInput.fill('5');
    await rewardInput.fill('a free ebook');

    // Now it should show the PRO badge
    // Click it to trigger paywall
    await removeBrandingCheckbox.click();

    const paywallModal = page.locator('h2:has-text("Upgrade to Remove Branding")');
    await expect(paywallModal).toBeVisible();

    // Close the paywall
    await page.locator('button', { hasText: 'Cancel' }).click();
    await expect(paywallModal).toBeHidden();

    // 4. Test Embed Code Generation
    const getEmbedBtn = page.locator('button', { hasText: 'Get Embed Code' });
    await getEmbedBtn.click();

    const embedModal = page.locator('h2:has-text("Embed Streak Widget")');
    await expect(embedModal).toBeVisible();

    const embedTextarea = page.locator('textarea[readonly]');
    await expect(embedTextarea).toBeVisible();

    const embedCode = await embedTextarea.inputValue();
    expect(embedCode).toContain('<iframe src="https://ohc.app/api/v1/growth/viral-streak/embed?');
    expect(embedCode).toContain('title=Read%2010%20Pages%20Daily');
    expect(embedCode).toContain('goal=5');
    expect(embedCode).toContain('reward=a%20free%20ebook');
    expect(embedCode).toContain('branding=true');
  });
});
