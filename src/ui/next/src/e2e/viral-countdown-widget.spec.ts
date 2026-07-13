import { test, expect } from '../../../../e2e/fixtures';

test.describe('Viral Countdown Widget', () => {
  test('allows configuring the widget and previews it correctly', async ({ page }) => {
    // Navigate to dashboard then to viral countdown widget
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Viral Countdown Widget' }).click();

    // Verify we are on the widget page
    await expect(page).toHaveURL(/\/viral-countdown-widget/);
    await expect(page.getByRole('heading', { name: 'Viral Countdown Widget', exact: true })).toBeVisible();

    // Set custom event name
    const customEventName = 'Super Awesome Summer Sale';
    const eventNameInput = page.getByLabel('Event Name');
    await eventNameInput.fill(customEventName);

    // Set custom theme to dark
    const themeSelect = page.getByLabel('Theme');
    await themeSelect.selectOption('dark');

    // Verify the iframe reflects changes
    const previewIframe = page.locator('iframe[title="Preview"]');
    await expect(previewIframe).toBeVisible();

    // Verify the embed code reflects the custom config
    const preCode = page.locator('pre');
    const codeText = await preCode.textContent();
    expect(codeText).toContain(encodeURIComponent(customEventName));
    expect(codeText).toContain('theme=dark');
    expect(codeText).toContain('branding=true');

    // Remove branding should trigger paywall if not pro (adminPage doesn't have pro by default)
    const removeBrandingCheckbox = page.getByLabel(/Remove "Powered by OHC" Badge/);
    await removeBrandingCheckbox.check();

    // Ensure soft paywall appears
    await expect(page.getByText('Upgrade to Remove Branding')).toBeVisible();
    await page.getByRole('button', { name: 'Close paywall' }).click();

    // Simulate copying
    const copyButton = page.getByRole('button', { name: 'Copy Embed Code' });
    await copyButton.click();

    // Verify button text changes
    await expect(page.getByRole('button', { name: 'Copied to Clipboard!' })).toBeVisible();
  });

  test('embed API returns correct HTML based on query params', async ({ request }) => {
    const customEventName = 'E2E Target Event';
    const response = await request.get(`/api/v1/growth/viral-countdown-widget/embed?event=${encodeURIComponent(customEventName)}&theme=dark`);

    expect(response.ok()).toBeTruthy();
    expect(response.headers()['content-type']).toContain('text/html');

    const text = await response.text();
    expect(text).toContain(customEventName);
    expect(text).toContain('id="countdown"');

    // Should have dark mode color and branding
    expect(text).toContain('#111827');
    expect(text).toContain('Powered by OHC');
  });
});
