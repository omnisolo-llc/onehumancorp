import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Creative/ }).click();
    await page.locator('#ob-company-name').fill('Alex Art');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Services/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('Portrait Session');
    await page.locator('#ob-product-price').fill('120');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Both Online/ }).click();
    await page.locator('#ob-admin-name').fill('Alex Artist');
    await page.locator('#ob-admin-email').fill('alex@example.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Service business without physical products', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Service Business/ }).click();
    await page.locator('#ob-company-name').fill('Quick Fix Plumbing');
    await page.locator('#ob-company-desc').fill('We fix plumbing');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Services/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('Plumbing Inspection');
    await page.locator('#ob-product-price').fill('150');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Online/ }).click();
    await page.locator('#ob-admin-name').fill('John Plumber');
    await page.locator('#ob-admin-email').fill('john@quickfix.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Bold' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Food cart with physical products and in-person payments', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Restaurant \/ Food/ }).click();
    await page.locator('#ob-company-name').fill('Spicy Tacos');
    await page.locator('#ob-company-desc').fill('Best tacos');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical Products/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('3 Taco Combo');
    await page.locator('#ob-product-price').fill('12');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Both Online/ }).click();
    await page.locator('#ob-admin-name').fill('Maria Chef');
    await page.locator('#ob-admin-email').fill('maria@spicytacos.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Local business with custom domain and online payment', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Local Business/ }).click();
    await page.locator('#ob-company-name').fill('City Yoga');
    await page.locator('#ob-company-desc').fill('Yoga in the city');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Services/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('10 Class Pass');
    await page.locator('#ob-product-price').fill('100');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Online/ }).click();
    await page.locator('#ob-admin-name').fill('Yoga Instructor');
    await page.locator('#ob-admin-email').fill('instructor@cityyoga.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Bold' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Online store skipping AI descriptions', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.locator('#ob-company-name').fill('Tech Gadgets');
    await page.locator('#ob-company-desc').fill('Tech items');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('Smartphone Case');
    await page.locator('#ob-product-price').fill('25');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Online/ }).click();
    await page.locator('#ob-admin-name').fill('Tech Boss');
    await page.locator('#ob-admin-email').fill('boss@techgadgets.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Service business without physical products', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Service Business/ }).click();
    await page.locator('#ob-company-name').fill('Quick Fix Plumbing');
    await page.locator('#ob-company-desc').fill('We fix plumbing');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Services/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('Plumbing Inspection');
    await page.locator('#ob-product-price').fill('150');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Online/ }).click();
    await page.locator('#ob-admin-name').fill('John Plumber');
    await page.locator('#ob-admin-email').fill('john@quickfix.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Bold' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Food cart with physical products and in-person payments', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Restaurant \/ Food/ }).click();
    await page.locator('#ob-company-name').fill('Spicy Tacos');
    await page.locator('#ob-company-desc').fill('Best tacos');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical Products/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('3 Taco Combo');
    await page.locator('#ob-product-price').fill('12');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Both Online/ }).click();
    await page.locator('#ob-admin-name').fill('Maria Chef');
    await page.locator('#ob-admin-email').fill('maria@spicytacos.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Local business with custom domain and online payment', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Local Business/ }).click();
    await page.locator('#ob-company-name').fill('City Yoga');
    await page.locator('#ob-company-desc').fill('Yoga in the city');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Services/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('10 Class Pass');
    await page.locator('#ob-product-price').fill('100');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Online/ }).click();
    await page.locator('#ob-admin-name').fill('Yoga Instructor');
    await page.locator('#ob-admin-email').fill('instructor@cityyoga.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Bold' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });

  test('Online store skipping AI descriptions', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.locator('#ob-company-name').fill('Tech Gadgets');
    await page.locator('#ob-company-desc').fill('Tech items');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.locator('#ob-product-name').fill('Smartphone Case');
    await page.locator('#ob-product-price').fill('25');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Online/ }).click();
    await page.locator('#ob-admin-name').fill('Tech Boss');
    await page.locator('#ob-admin-email').fill('boss@techgadgets.com');
    await page.locator('#ob-admin-pwd').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });
});
