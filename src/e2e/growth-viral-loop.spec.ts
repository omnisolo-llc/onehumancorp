import { test, expect } from './fixtures';

test.describe('Growth Viral Loop: Powered by OHC Banner', () => {
  test('Merchant can toggle the Powered By banner and it renders correctly', async ({ page }) => {
    // Ensure we start from a clean state
    await page.goto('/dashboard'); // Need to be on same origin to clear localStorage
    await page.evaluate(() => localStorage.clear());
    await page.goto('/builder');

    // Bypassing the wizard completely to avoid timeout flakiness
    await page.evaluate(() => {
        localStorage.setItem('builder-storage', JSON.stringify({
            state: {
                bio: "E2E Bio",
                businessName: "E2E Store",
                businessCategory: "E2E Category",
                vibe: "Professional",
                wizardStep: 3,
                blocks: [
                    { type: "Hero", props: { headline: "Test", copy: "Test" } },
                    { type: "PoweredBy", props: {} }
                ],
                drafts: [],
                status: "idle",
                businessGoal: "products",
                liveUrl: ""
            },
            version: 0
        }));
        // Setting it in both possible locations where the app checks
        localStorage.setItem('tenant_id', 'e2e-tenant');
    });

    // Now that state is seeded, navigate directly to builder, bypassing wizard
    await page.goto('/builder');

    // Wait for the builder interface to load deterministically without try/catch ignore blocks
    await page.waitForSelector('text=Powered by', { timeout: 15000 });

    // Wait for the toggle to be attached and check its state
    const toggle = page.locator('input[type="checkbox"]');
    await toggle.waitFor({ state: 'attached', timeout: 10000 });

    // We expect it to be checked since it's the default state in our code
    // However, depending on timing it might need a moment
    await expect(toggle).toBeChecked({ timeout: 5000 });

    // Verify the banner is visible
    const banner = page.getByText('Powered by');
    await expect(banner).toBeVisible();

    // Toggle it off (accepting the confirm dialog)
    page.on('dialog', dialog => dialog.accept());
    await toggle.uncheck();

    // Verify the banner is no longer visible
    await expect(banner).not.toBeVisible();

    // Toggle it back on
    await toggle.check();
    await expect(banner).toBeVisible();

    // Click the banner
    const bannerLink = page.getByRole('link', { name: 'One Human Corp' });
    await expect(bannerLink).toHaveAttribute('href', /ohc\.store\/join\?ref=/);

    // Wait for the click to be registered in the backend
    const [request] = await Promise.all([
      page.waitForRequest(req => req.url().includes('/api/v1/growth/powered-by-banner/click') && req.method() === 'POST'),
      bannerLink.click()
    ]);

    expect(request.url()).toContain('/api/v1/growth/powered-by-banner/click');
  });
});
