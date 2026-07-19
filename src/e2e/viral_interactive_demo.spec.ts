import { test, expect } from './fixtures';

test.describe('Viral Interactive Demo Widget Builder', () => {
  test('should navigate to Interactive Demo builder from dashboard', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    // Make sure the link is visible and click it
    const link = page.locator('#interactive-demo-link');
    await expect(link).toBeVisible();
    await link.click();

    // Verify we arrived at the widget builder
    await expect(page).toHaveURL(/.*interactive-demo-widget\.html/);
    await expect(page.getByRole('heading', { name: 'Interactive Demo Widget' })).toBeVisible();
  });

  test('should update preview when form is modified', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/interactive-demo-widget.html');

    const titleInput = page.locator('#widget-title-input');
    const messageInput = page.locator('#widget-message-input');
    const previewTitle = page.locator('#preview-title');
    const previewMessage = page.locator('#preview-message');

    await titleInput.fill('Chat with our Bot');
    await expect(previewTitle).toHaveText('Chat with our Bot');

    await messageInput.fill('How can I help you today?');
    await expect(previewMessage).toHaveText('How can I help you today?');
  });

  test('should show paywall modal when removing branding without pro', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Force has_pro to false for this test
    await page.addInitScript(() => {
      window.localStorage.setItem('has_pro', 'false');
    });
    await page.goto('/interactive-demo-widget.html');

    const removeBrandingCheckbox = page.locator('#remove-branding');
    await removeBrandingCheckbox.check();

    // Verify modal appears
    const paywallModal = page.locator('#paywall-modal');
    await expect(paywallModal).toHaveClass(/active/);
    await expect(paywallModal.locator('.modal-title')).toHaveText('Upgrade to Pro');
  });

  test('should hide branding if user has pro', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Force has_pro to true for this test
    await page.addInitScript(() => {
      window.localStorage.setItem('has_pro', 'true');
    });
    await page.goto('/interactive-demo-widget.html');

    const removeBrandingCheckbox = page.locator('#remove-branding');
    await removeBrandingCheckbox.check();

    // Verify modal does NOT appear and branding is hidden
    const paywallModal = page.locator('#paywall-modal');
    await expect(paywallModal).not.toHaveClass(/active/);

    const branding = page.locator('#preview-branding');
    await expect(branding).toBeHidden();
  });
});
