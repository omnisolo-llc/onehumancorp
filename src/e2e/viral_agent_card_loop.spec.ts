import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_agent_card_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_agent_card_loop');
});

test.describe('Viral Agent Card Growth Loop', () => {
  test('should allow creating an agent card, toggle branding, and copy link', async ({ page, context }) => {
    // Grant clipboard permissions for copying the link
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.goto('/agent-card.html');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Verify header and description
    await expect(page.getByRole('heading', { name: 'Agent Public Card' })).toBeVisible();
    await expect(page.getByText('Generate a shareable public card')).toBeVisible();

    // Fill the agent details
    await page.locator('#agent-name').fill('Sarah');
    await page.locator('#agent-role').fill('Sales Assistant');
    await page.locator('#agent-desc').fill('Hello! How can I help you today?');

    // Verify preview updates
    await expect(page.locator('#preview-name-el')).toHaveText('Sarah');
    await expect(page.locator('#preview-role-el')).toHaveText('Sales Assistant');
    await expect(page.locator('#preview-desc-el')).toHaveText('Hello! How can I help you today?');

    // Verify "Powered by OHC" branding is visible by default
    const brandingLink = page.locator('#branding-link');
    await expect(brandingLink).toBeVisible();
    await expect(brandingLink).toContainText('Powered by OHC');
    await expect(brandingLink).toHaveAttribute('href', /api\/v1\/growth\/referrals\/click/);

    // Toggle the "Remove branding" checkbox
    await page.locator('label', { hasText: 'Remove "Powered by OHC" Badge' }).click();

    // Verify the branding footer is hidden
    await expect(page.locator('#preview-footer-el')).toBeHidden();

    // Click "Copy Share Link"
    const copyBtn = page.locator('#generate-btn');
    await copyBtn.click();

    // Verify button text changes to Copied!
    await expect(copyBtn).toHaveText('Copied!');

    // Verify clipboard content
    const clipboardText = await page.evaluate("navigator.clipboard.readText()");
    expect(clipboardText).toContain('agent-profile?');
    expect(clipboardText).toContain('name=Sarah');
    expect(clipboardText).toContain('role=Sales+Assistant');
    expect(clipboardText).toContain('hideBranding=true');
  });
});
