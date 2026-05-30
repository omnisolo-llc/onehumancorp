import { expect, test } from './fixtures';

test.describe('Public Profile Link-in-Bio', () => {
  test('should display Powered by OHC footer with correct tenant referral code', async ({ page }) => {
    await page.goto('/login');
    await page.goto('/dashboard');
    await page.goto('/public-profile/demo-tenant');
    await expect(page.getByRole('heading', { name: 'OHC Demo Business' })).toBeVisible();

    const poweredByLink = page.locator('a[href^="ohc://join?ref="]');
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toContainText('Powered by');
    await expect(poweredByLink).toContainText('OHC');

    const href = await poweredByLink.getAttribute('href');
    expect(href).toMatch(/^ohc:\/\/join\?ref=demo-tenant-profile$/);
  });

  test('should display core business navigation links', async ({ page }) => {
    await page.goto('/login');
    await page.goto('/dashboard');
    await page.goto('/public-profile/demo-tenant');

    const storeLink = page.getByRole('link', { name: 'Shop Our Store' });
    const bookingLink = page.getByRole('link', { name: 'Book an Appointment' });
    const contactLink = page.getByRole('link', { name: 'Contact Us' });

    await expect(storeLink).toBeVisible();
    await expect(storeLink).toHaveAttribute('href', '/storefront-builder');

    await expect(bookingLink).toBeVisible();
    await expect(bookingLink).toHaveAttribute('href', '/booking');

    await expect(contactLink).toBeVisible();
  });

  test('should display business bio and correct profile initials', async ({ page }) => {
    await page.goto('/login');
    await page.goto('/dashboard');
    await page.goto('/public-profile/demo-tenant');

    const bioText = page.getByText('Providing exceptional services and products.');
    await expect(bioText).toBeVisible();

    const initials = page.getByText('OH');
    await expect(initials).toBeVisible();
  });

  test('should display social media icon links', async ({ page }) => {
    await page.goto('/login');
    await page.goto('/dashboard');
    await page.goto('/public-profile/demo-tenant');

    // Check that we have at least 3 social icons rendered
    const socialLinks = page.locator('main > div:last-child > a');
    await expect(socialLinks).toHaveCount(3);

    // Verify visibility of elements that look like icons
    await expect(socialLinks.first()).toBeVisible();
  });

  test('should pass visual responsiveness for mobile containers', async ({ page }) => {
    await page.goto('/login');
    await page.goto('/dashboard');
    await page.goto('/public-profile/demo-tenant');

    const mainContainer = page.locator('main');
    // Ensure max-w class is present which handles mobile bounding
    await expect(mainContainer).toHaveClass(/max-w-\[414px\]/);
  });
});
