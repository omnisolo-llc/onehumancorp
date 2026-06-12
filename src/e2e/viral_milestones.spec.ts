import { test, expect } from './fixtures';

test.describe('Viral Milestones Page', () => {

  test('viral milestones: verify dynamic loading and card generation', async ({ page, loginAs, unlimitedAdminUser }) => {
    // E2E test data must be seeded into DB! Our fixture loginAs seeds the basic tenant.
    // The e2e-seed.sql injects 'first_sale' and '10th_order'
    await loginAs(page, unlimitedAdminUser);

    // Navigate to the tauri UI
    await page.goto('/milestones.html');

    // Wait for milestones to load
    await expect(page.locator('h2:has-text("Your Achievements")')).toBeVisible();

    const milestoneList = page.locator('.milestone-item');
    await expect(milestoneList.first()).toBeVisible();

    // Verify that an image is loaded for the selected milestone (first unlocked should be auto-selected)
    const milestoneImage = page.locator('#milestone-image');
    await expect(milestoneImage).toHaveAttribute('src', /api\/v1\/growth\/milestone\/card/);
  });

  test('viral milestones: verify multiple milestone titles from API', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/milestones.html');
    await expect(page.locator('h3:has-text("First Sale!")')).toBeVisible({ timeout: 15000 });
  });

  test('viral milestones: verify social share buttons and viral loop', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/milestones.html');

    const whatsappBtn = page.locator('text=Share to WhatsApp');
    await expect(whatsappBtn).toBeVisible();
    const fbBtn = page.locator('text=Share on Facebook');
    await expect(fbBtn).toBeVisible();
    const xBtn = page.locator('text=Share on X');
    await expect(xBtn).toBeVisible();

    // Check href values
    const waHref = await whatsappBtn.getAttribute('href');
    expect(waHref).toContain('wa.me/?text=');
    expect(waHref).toContain(encodeURIComponent('Powered by OHC'));

    const xHref = await xBtn.getAttribute('href');
    expect(xHref).toContain('twitter.com/intent/tweet?text=');
    expect(xHref).toContain(encodeURIComponent('Powered by OHC'));

    const fbHref = await fbBtn.getAttribute('href');
    expect(fbHref).toContain('facebook.com/sharer/sharer.php');
    expect(fbHref).toContain('quote=');
    expect(fbHref).toContain(encodeURIComponent('Powered by OHC'));
  });
});
