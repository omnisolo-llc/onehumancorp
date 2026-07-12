import { test, expect } from './fixtures';

test.describe('Viral Founder Story Loop', () => {
  test('should display the viral founder story builder and handle code generation', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    // Click the new Viral Founder Story link
    const founderStoryBtn = page.locator('a#viral-founder-story-link');
    if (await founderStoryBtn.isVisible()) {
        await founderStoryBtn.click();
    } else {
        await page.goto('/viral-founder-story.html');
    }

    await expect(page.locator('h1')).toHaveText('Viral Founder Story Builder');

    // Wait for the iframe preview to load initially
    await expect(page.locator('#preview-frame')).toBeVisible();
    let frameSrc = await page.locator('#preview-frame').getAttribute('src');
    expect(frameSrc).toContain('founder_name=Maya');

    // Modify inputs
    await page.fill('#founder-name', 'Carlos');
    await page.fill('#story-text', 'My repair service started from a simple garage.');
    await page.fill('#reward-name', '10% off repair');

    // Trigger a change Event for standard bindings (just wait a tiny bit for UI to update frame)
    await page.waitForTimeout(500);

    // Check that preview updates in iframe source
    frameSrc = await page.locator('#preview-frame').getAttribute('src');
    expect(frameSrc).toContain('founder_name=Carlos');
    expect(frameSrc).toContain('reward=10%25%20off%20repair');

    // Check dark mode
    await page.click('#theme-dark');
    await page.waitForTimeout(100);
    frameSrc = await page.locator('#preview-frame').getAttribute('src');
    expect(frameSrc).toContain('theme=dark');

    // Open Embed Modal
    await page.click('#get-code-btn');

    const embedModal = page.locator('#embed-modal');
    await expect(embedModal).toHaveClass(/active/);

    const embedCode = await page.inputValue('#embed-code');
    expect(embedCode).toContain('founder_name=Carlos');
    expect(embedCode).toContain('reward=10%25%20off%20repair');
    expect(embedCode).toContain('theme=dark');
    expect(embedCode).toContain('hideBranding=false');

    await page.click('#close-embed-btn');
    await expect(embedModal).not.toHaveClass(/active/);
  });
});
