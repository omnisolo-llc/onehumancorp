import { test, expect } from './fixtures';

test.describe('Viral Giveaway Generator', () => {
  test('should generate a giveaway link successfully', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Navigate to the dashboard first to ensure auth context is loaded, then go to the giveaway generator
    await page.goto('/dashboard.html');
    await page.goto('/ui/giveaway/index.html');

    // Check initial state
    await expect(page.locator('h1', { hasText: 'Viral Giveaway Generator' })).toBeVisible();
    await expect(page.locator('#result-area')).toBeHidden();

    // Fill out the form
    await page.fill('#title', 'Win a Free Lifetime OHC Pro Account');
    await page.fill('#desc', 'Share this amazing link on your socials and sign up to win!');

    // Click Generate
    await page.click('#generate-btn');

    // Verify result area becomes visible
    await expect(page.locator('#result-area')).toBeVisible({ timeout: 5000 });

    // Verify input contains URL
    const urlInput = page.locator('#generated-url');
    await expect(urlInput).toBeVisible();

    const generatedUrl = await urlInput.inputValue();
    expect(generatedUrl).toContain('/giveaway/enter');
    expect(generatedUrl).toContain('title=Win+a+Free+Lifetime+OHC+Pro+Account');
    expect(generatedUrl).toContain('desc=Share+this+amazing+link+on+your+socials+and+sign+up+to+win');

    // Verify Share buttons
    await expect(page.locator('#copy-btn')).toBeVisible();
    await expect(page.locator('#share-x-btn')).toBeVisible();
  });
});
