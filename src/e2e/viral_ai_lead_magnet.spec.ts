import { createGrowthOwner } from './growth_owner';
import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_ai_lead_magnet_builder_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_ai_lead_magnet_builder');
});

test.describe('Viral AI Lead Magnet Builder Loop', () => {
  test('links to the builder and keeps free branding while trial verification is unavailable', async ({ page, baseURL }) => {
    const owner = await createGrowthOwner(page, baseURL);
    const trialClaims: string[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && new URL(request.url()).pathname === '/api/v1/growth/trial-extension/claim') trialClaims.push(request.url());
    });
    await page.goto('/dashboard.html');
    const leadMagnetLink = page.locator('a#ai-lead-magnet-link');
    await expect(leadMagnetLink).toBeVisible();
    await leadMagnetLink.click();
    await expect(page.getByRole('heading', { name: 'AI Lead Magnet Builder' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Generate Embed Code', exact: true })).toBeEnabled();
    await page.getByLabel('Offer Title').fill('Free Security Audit');
    await page.getByLabel('Description').fill('Get a free security score for your app.');
    await expect(page.locator('#preview-title')).toHaveText('Free Security Audit');
    await page.locator('.slider').click();
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro', exact: true });
    await expect(paywallHeading).toBeVisible();
    await expect(page.getByText('Trial unlock is unavailable because this flow cannot verify sharing or a trial entitlement.', { exact: false })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Open X share draft', exact: true })).toBeVisible();
    // A share draft is not a provider receipt. This CI journey does not open a
    // social provider, override window.open, or request an entitlement grant.
    await expect(page.locator('#remove-branding')).not.toBeChecked();
    await expect(page.locator('#preview-branding')).toBeVisible();
    expect(trialClaims).toEqual([]);
    await page.locator('#close-paywall').click();

    await page.getByRole('button', { name: 'Generate Embed Code', exact: true }).click();
    const embedHeading = page.getByRole('heading', { name: 'Embed Your Lead Magnet', exact: true });
    await expect(embedHeading).toBeVisible();
    const snippet = await page.locator('#embed-code').inputValue();
    const src = await page.evaluate(value => new DOMParser().parseFromString(value, 'text/html').querySelector('iframe')?.getAttribute('src'), snippet);
    expect(src).toBeTruthy();
    const url = new URL(src!);
    expect(url.origin).toBe(new URL(page.url()).origin);
    expect(url.pathname).toBe('/api/v1/growth/lead-magnet/embed');
    expect(url.searchParams.get('tenant')).toBe(owner.tenantId);
    expect(url.searchParams.get('title')).toBe('Free Security Audit');
    expect(url.searchParams.get('hideBranding')).toBe('false');
    await page.getByRole('button', { name: 'Close', exact: true }).click();
    await expect(embedHeading).not.toBeVisible();
    await page.goto(url.href);
    await expect(page.getByText('Free Security Audit', { exact: true })).toBeVisible();
    await expect(page.getByRole('link', { name: /Powered by OmniSolo/ })).toBeVisible();
    expect(trialClaims).toEqual([]);
  });

  test('should verify preview updates in real time', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/ai-lead-magnet-builder.html');
    await page.waitForLoadState('networkidle');

    const inputLabel = page.getByLabel('Input Field Placeholder');
    await inputLabel.fill('https://newurl.com');

    const btnText = page.getByLabel('Button Text');
    await btnText.fill('Start Now');

    await page.waitForTimeout(500);

    const previewInputLabel = page.locator('#preview-input-label');
    await expect(previewInputLabel).toHaveAttribute('placeholder', 'https://newurl.com');

    const previewBtnText = page.locator('#preview-btn-text');
    await expect(previewBtnText).toHaveText('Start Now');
  });

  test('should close the paywall modal when the close button is clicked', async ({ page, baseURL }) => {
    await createGrowthOwner(page, baseURL);
    await page.goto('/ai-lead-magnet-builder.html');
    await page.waitForLoadState('networkidle');

    // Wait for the real free-owner plan before exercising the gate.
    await expect(page.getByRole('button', { name: 'Generate Embed Code', exact: true })).toBeEnabled();
    await page.locator('.slider').click();

    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();

    // Click close button
    const closeBtn = page.locator('#close-paywall');
    await closeBtn.click();

    await expect(paywallHeading).not.toBeVisible();

    // Checkbox should be unchecked
    const removeBrandingCheckbox = page.locator('input#remove-branding');
    await expect(removeBrandingCheckbox).not.toBeChecked();
  });

  test('should navigate back to dashboard when back button is clicked', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/ai-lead-magnet-builder.html');
    await page.waitForLoadState('networkidle');

    const backBtn = page.getByRole('button', { name: 'Back to Dashboard' });
    await backBtn.click();

    await page.waitForURL('**/dashboard.html');
    await expect(page).toHaveURL(/.*dashboard\.html/);
  });
});
