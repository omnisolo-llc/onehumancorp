import { test, expect } from './fixtures';

test.describe('Business Setup Wizard - Instant Build', () => {
  test.beforeEach(async ({ page }) => {
    const id = `business-setup-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
    // Use mobile viewport context since this is mobile-first
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('completes the instant build flow', async ({ page }) => {
    // Re-enable routing for this test
    (page.context() as any).route = (page.context() as any).__proto__.route;
    (page as any).route = (page as any).__proto__.route;

    await page.route('**/api/v1/builder/generate', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          theme: 'Modern',
          products: [],
          shipping_settings: { zone: 'Local', rate: '0.00' },
          tax_settings: { rate: '0.0%' },
          site_draft: {
            domain: null,
            pages: [
              {
                path: '/',
                title: 'Home',
                blocks: [
                  {
                    block_type: 'HeroBlock',
                    content: { headline: 'Test Bakery', subtitle: 'Fresh baked goods' },
                    sort_order: 0
                  }
                ],
                seo_metadata: {}
              }
            ]
          }
        })
      });
    });

    await page.route('**/api/v1/builder/publish_draft', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          domain: 'myshop'
        })
      });
    });
    // Navigate to Instant Build
    await expect(page.getByRole('button', { name: /Instant Build/ })).toBeVisible();
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Verify we are on the text area input
    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible();

    // Fill in the description
    await page.getByPlaceholder('e.g. I run a local bakery').fill('I run a local bakery');

    // Generate the storefront
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    // Wait for the generating screen to appear
    await expect(page.getByText('Agents are building your store...')).toBeVisible();

    // Wait for the success screen
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 15000 });
  });
});
