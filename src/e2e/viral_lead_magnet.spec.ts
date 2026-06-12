import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_lead_magnet', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_lead_magnet');
});

test.describe('Viral Lead Magnet Loop', () => {
  test('should display the lead magnet widget on dashboard and copy embed code', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Navigate to dashboard
    await page.goto('/dashboard');


    // 1. Verify the Viral Lead Magnet Embed widget is visible
    const leadMagnetHeading = page.getByRole('heading', { name: /Grow Your Email List/i });
    await expect(leadMagnetHeading).toBeVisible();

    // 2. Verify the copy button exists
    const copyBtn = page.getByRole('button', { name: /Copy Embed Code/i });
    await expect(copyBtn).toBeVisible();

    // 3. Mock clipboard to prevent actual clipboard writes during test
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // 4. Click copy button
    await copyBtn.click();

    // 5. Verify the success state
    await expect(page.getByText('Embed Code Copied!')).toBeVisible();

    // 6. Verify clipboard content contains the embed iframe and the viral link
    const clipboardText = await page.evaluate('navigator.clipboard.readText()');
    expect(clipboardText).toContain('<iframe src="https://ohc.app/api/v1/growth/lead-magnet/embed?tenant=');
    expect(clipboardText).toContain('<a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=');
    expect(clipboardText).toContain('⚡ Powered by OHC</a>');
  });

  test('should render the embed API route successfully', async ({ request }) => {
    // Check that the embed route returns the expected HTML
    const response = await request.get('/api/v1/growth/lead-magnet/embed?tenant=test-tenant');
    expect(response.status()).toBe(200);

    const html = await response.text();
    expect(html).toContain('Get Our Free Guide');
    expect(html).toContain('Send Me the Guide');
    expect(html).toContain('<form id="lead-form"');
  });
});
