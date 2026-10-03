import { test, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';

test.describe('Viral Post Generator', () => {
  test('should allow owner to generate post and handle paywall', async ({ page, baseURL }) => {
    if (!baseURL) throw new Error('The local browser fixture URL is required');
    const fixtureOrigin = new URL(baseURL);
    expect(['http:', 'https:']).toContain(fixtureOrigin.protocol);
    expect(['localhost', '127.0.0.1', '[::1]']).toContain(fixtureOrigin.hostname);
    const owner = await createGrowthOwner(page, baseURL);
    const planResponse = await page.request.get('/api/v1/billing/my-plan', {headers: {
      'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId,
    }});
    expect(planResponse.status()).toBe(200);
    const plan = await planResponse.json();
    expect(plan.error ?? null).toBeNull();
    expect(plan.success).not.toBe(false);
    expect(typeof plan.current_plan).toBe('string');
    expect(plan.current_plan.toLowerCase()).toBe('free');

    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Promoter Agent / Viral Post Generator link in GrowBusinessCard
    const promoterLink = page.locator('a[href="/viral-post-generator"]');
    await expect(promoterLink).toBeVisible();
    await promoterLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Social Post Template' })).toBeVisible();

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // 3. Fill in details
    const productNameInput = page.getByPlaceholder('e.g. Signature Coffee Blend');
    await productNameInput.fill('Ultimate Developer Coffee');

    const keyBenefitInput = page.getByPlaceholder('e.g. a bold start to your morning');
    await keyBenefitInput.fill('maximum focus and energy');

    // Generate the post
    const generateBtn = page.getByRole('button', { name: 'Generate Post' });
    await generateBtn.click();

    // Verify output
    await expect(page.getByText(/Ultimate Developer Coffee/)).toBeVisible();
    await expect(page.getByText(/maximum focus and energy/)).toBeVisible();
    const generatedPost = page.locator('.whitespace-pre-wrap', { hasText: /OmniSolo/ });
    await expect(generatedPost).toBeVisible();
    await expect(generatedPost).not.toContainText('.cloud.omnisolo.co');
    await expect(page.getByText(/Create a local text template from your details/)).toBeVisible();

    // 4. Try to remove branding
    const removeBrandingCheckbox = page.getByRole('checkbox', { name: /Remove "OmniSolo" branding/i });
    await removeBrandingCheckbox.click();

    // Paywall appears
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Close the paywall
    await page.getByRole('button', { name: 'Close paywall' }).click();

    // Ensure paywall is gone
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeHidden();

    // 5. Copy the post
    const copyButton = page.getByRole('button', { name: 'Copy to Clipboard' });
    await expect(copyButton).toBeVisible();

    const origin = new URL(page.url());
    expect(['http:', 'https:']).toContain(origin.protocol);
    expect(['localhost', '127.0.0.1', '[::1]']).toContain(origin.hostname);
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write'], { origin: origin.origin });
    await page.bringToFront();
    const expectedClipboard = await generatedPost.textContent();
    await copyButton.click();
    await expect(page.getByText('Copied!')).toBeVisible();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(expectedClipboard);
  });
});
