import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_ai_lead_magnet_builder_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_ai_lead_magnet_builder');
});

test.describe('Viral AI Lead Magnet Builder Loop', () => {
  test('should display the lead magnet builder and handle soft paywall share bypass', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 1. Verify the AI Lead Magnet Builder link is visible
    const leadMagnetLink = page.locator('a#ai-lead-magnet-link');
    await expect(leadMagnetLink).toBeVisible();

    // 2. Click the link to go to the builder
    await leadMagnetLink.click();
    await page.waitForLoadState('networkidle');

    // 3. Verify the builder page loaded
    const heading = page.getByRole('heading', { name: 'AI Lead Magnet Builder' });
    await expect(heading).toBeVisible();

    // 4. Fill in custom details
    const titleInput = page.getByLabel('Offer Title');
    await titleInput.fill('Free Security Audit');

    const descInput = page.getByLabel('Description');
    await descInput.fill('Get a free security score for your app.');

    await page.waitForTimeout(500);

    // Verify preview updated
    const previewTitle = page.locator('#preview-title');
    await expect(previewTitle).toHaveText('Free Security Audit');

    // 5. Check 'Remove Branding' toggle behavior
    const removeBrandingCheckbox = page.locator('input#remove-branding');
    const brandingPreview = page.locator('#preview-branding');

    await expect(brandingPreview).toBeVisible();

    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // Toggle to remove branding
    await page.locator('.slider').click();

    // 6. Verify Paywall modal opens
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();

    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await expect(shareButton).toBeVisible();

    // 7. Click Share to bypass
    await shareButton.click();

    await expect(page.locator('#soft-paywall-status')).toContainText('Verifying Share...', { timeout: 2000 });
    await expect(page.locator('#soft-paywall-status')).toContainText('Unlocked!', { timeout: 10000 });

    await expect(paywallHeading).not.toBeVisible({ timeout: 5000 });
    await expect(removeBrandingCheckbox).toBeChecked();
    await expect(brandingPreview).not.toBeVisible();

    // 8. Generate Embed Code
    const generateBtn = page.getByRole('button', { name: 'Generate Embed Code' });
    await generateBtn.click();

    const embedHeading = page.getByRole('heading', { name: 'Embed Your Lead Magnet' });
    await expect(embedHeading).toBeVisible();

    const embedTextarea = page.locator('#embed-code');
    const codeValue = await embedTextarea.inputValue();
    expect(codeValue).toContain('<iframe');
    expect(codeValue).toContain('hide_branding=true');
    expect(codeValue).toContain('title=Free%20Security%20Audit');

    const closeEmbedBtn = page.getByRole('button', { name: 'Close' });
    await closeEmbedBtn.click();
    await expect(embedHeading).not.toBeVisible();
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

  test('should close the paywall modal when the close button is clicked', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/ai-lead-magnet-builder.html');
    await page.waitForLoadState('networkidle');

    // Click slider to trigger paywall
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
