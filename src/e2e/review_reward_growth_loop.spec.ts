import { test, expect } from './fixtures';
import { E2E_ADMIN_USER } from './fixtures';

test.describe.serial('Review Reward Growth Loop', () => {
  test('should allow owner to create an embeddable review widget with viral loop', async ({ page, adminUser, loginAs }) => {
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

    // 3. Click the Review Reward Generator link
    await page.getByRole('link', { name: 'Review Reward Generator ⭐️' }).click();

    // Verify page loaded
    await expect(page.getByRole('heading', { name: 'Review & Reward Generator' })).toBeVisible();

    // 4. Test Preview rendering with default value
    const previewTitle = page.locator('#previewTitle');
    await expect(previewTitle).toHaveText('Leave a review, get 15% off!');

    // Ensure the powered by OHC link is present
    const poweredByLink = page.locator('#previewBranding');
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toHaveText('⚡ Powered by OHC');

    // 5. Test generated HTML code includes the watermark
    const codeOutput = page.locator('#codeOutput');
    let generatedHtml = await codeOutput.textContent();
    expect(generatedHtml).toContain('⚡ Powered by OHC');

    // 6. Test interaction: modifying the inputs changes the code
    await page.fill('#widgetTitle', 'Leave a 5 star review!');
    await expect(previewTitle).toHaveText('Leave a 5 star review!');

    generatedHtml = await codeOutput.textContent();
    expect(generatedHtml).toContain('Leave a 5 star review!');

    // 7. Test removing branding soft paywall
    // Ensure the user doesn't have pro
    await page.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });

    // Try to toggle "Remove branding"
    const toggleLabel = page.getByText('Remove branding PRO');
    await toggleLabel.click();

    // Verify soft paywall modal is shown
    await expect(page.getByText('Pro Feature', { exact: true })).toBeVisible();

    // Dismiss modal
    await page.getByRole('button', { name: 'Keep Branding' }).click();
    await expect(page.getByText('Pro Feature')).not.toBeVisible();

    // 8. Test removing branding successfully as a Pro user
    await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

    // The toggle should now work without modal
    await toggleLabel.click();
    await expect(page.getByText('Pro Feature')).not.toBeVisible();

    // Watermark should be gone in preview
    await expect(poweredByLink).not.toBeVisible();

    // Watermark should be gone in code output
    generatedHtml = await codeOutput.textContent();
    expect(generatedHtml).not.toContain('⚡ Powered by OHC');
  });
});
