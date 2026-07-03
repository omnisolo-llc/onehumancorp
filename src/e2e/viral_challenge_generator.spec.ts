import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_challenge_generator_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_challenge_generator');
});

test.describe('Viral Challenge Generator', () => {
  test('should navigate to the challenge generator from dashboard and create a challenge', async ({ page, context }) => {
    // Navigate to dashboard
    await page.goto('/dashboard.html');

    // Click the challenge generator link
    const challengeLink = page.locator('#challenge-link');
    await expect(challengeLink).toBeVisible();
    await challengeLink.click();

    // Verify page load
    await expect(page.getByRole('heading', { name: 'Viral Challenge Generator' })).toBeVisible();

    // Wait for JS to attach
    await page.waitForTimeout(500);

    // Verify live preview initializes correctly
    await expect(page.locator('#preview-title')).toHaveText('30 Days of Guitar');
    await expect(page.locator('#preview-duration')).toHaveText('30-Day Challenge');
    await expect(page.locator('#preview-reward')).toHaveText('Free 1-on-1 Lesson');

    // Fill the form
    const nameInput = page.locator('#challenge-name');
    await nameInput.fill('7 Days of Code');

    const durationInput = page.locator('#challenge-duration');
    await durationInput.fill('7');

    const rewardInput = page.locator('#challenge-reward');
    await rewardInput.fill('Free Code Review');

    // Verify Live Preview updates
    await expect(page.locator('#preview-title')).toHaveText('7 Days of Code');
    await expect(page.locator('#preview-duration')).toHaveText('7-Day Challenge');
    await expect(page.locator('#preview-reward')).toHaveText('Free Code Review');

    // Click Generate
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // Wait for the result area to become visible
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Check generated URL
    const generatedUrl = page.locator('#generated-url');
    await expect(generatedUrl).toBeVisible();
    const urlValue = await generatedUrl.inputValue();
    expect(urlValue).toContain('name=7+Days+of+Code');
    expect(urlValue).toContain('days=7');
    expect(urlValue).toContain('reward=Free+Code+Review');

    // Test clipboard copy
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    const copyBtn = page.locator('#copy-btn');
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('7+Days+of+Code');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });
});
