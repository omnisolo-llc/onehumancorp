import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Newsletter Generator', () => {
  test('dashboard links to Viral Newsletter Generator, which generates an embed with a viral footer', async ({ page, adminUser, loginAs }) => {
    // 1. Log in
    await loginAs(page, adminUser);

    // 2. Navigate to dashboard
    await page.goto('/dashboard.html');
    let content = await page.content();
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/tauri_out/dashboard.html');
        content = await page.content();
    }
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/ui/dashboard.html');
        content = await page.content();
    }
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/dashboard');
    }

    // 3. Click the Viral Newsletter Generator link
    await page.getByRole('link', { name: 'Viral Newsletter Generator ✉️' }).click();

    // Verify we are on the generator page
    await expect(page.getByRole('heading', { name: 'Viral Newsletter Generator ✉️' })).toBeVisible();

    // 4. Modify some config values
    const titleInput = page.locator('#widgetTitle');
    await titleInput.fill('Join Our Awesome Newsletter');

    const descInput = page.locator('#widgetDesc');
    await descInput.fill('Get the best deals every week.');

    const btnInput = page.locator('#buttonText');
    await btnInput.fill('Sign Me Up!');

    // 5. Verify the live preview updates
    await expect(page.locator('#previewTitle')).toHaveText('Join Our Awesome Newsletter');
    await expect(page.locator('#previewDesc')).toHaveText('Get the best deals every week.');
    await expect(page.locator('#previewBtn')).toHaveText('Sign Me Up!');
    await expect(page.locator('#previewBranding')).toHaveText('⚡ Powered by OHC');

    // 6. Verify the embed code contains the viral link and correct text
    const codeOutput = page.locator('#codeOutput');
    const embedHtml = await codeOutput.textContent();
    expect(embedHtml).toContain('Join Our Awesome Newsletter');
    expect(embedHtml).toContain('Get the best deals every week.');
    expect(embedHtml).toContain('Sign Me Up!');
    expect(embedHtml).toContain('⚡ Powered by OHC');

    // 7. Test removing branding soft paywall
    // Ensure the user doesn't have pro
    await page.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });

    // Try to toggle "Remove branding"
    const toggleInput = page.locator('#brandingToggle');
    // We click the slider wrapper, or check the input
    // The label "Remove branding PRO" is what we can click on
    await page.getByText('Remove branding PRO').click();

    // Verify soft paywall modal is shown
    const modal = page.locator('#proModal');
    await expect(modal).toHaveCSS('display', 'flex');
    await expect(page.locator('#proModal').getByText('Pro Feature')).toBeVisible();

    // Dismiss modal
    await page.getByRole('button', { name: 'Keep Branding' }).click();
    await expect(modal).not.toHaveCSS('display', 'flex');

    // 8. Test removing branding successfully as a Pro user
    await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

    // The toggle should now work without modal
    await page.getByText('Remove branding PRO').click();
    await expect(modal).not.toHaveCSS('display', 'flex');

    // Watermark should be gone in preview
    await expect(page.locator('#previewBranding')).not.toBeVisible();

    // Watermark should be gone in code output
    const newEmbedHtml = await codeOutput.textContent();
    expect(newEmbedHtml).not.toContain('⚡ Powered by OHC');
  });
});
