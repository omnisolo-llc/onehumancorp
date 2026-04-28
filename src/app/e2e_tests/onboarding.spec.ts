import { test, expect } from '@playwright/test';

test('Mobile-first onboarding flow supports products and bookings', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('/');

  await page.fill('input[placeholder="Email or Username"]', 'maya@bakery.com');
  await page.fill('input[placeholder="Password"]', 'password123');
  await page.click('text="Sign In"');

  await expect(page.locator('text="Business Setup"')).toBeVisible();
  await page.click('text="Next"');

  await page.fill('input[placeholder="Business Type (e.g. Baker, Handyman)"]', 'Baker');
  await page.fill('input[placeholder="Company Name"]', 'Maya Cakes');
  await page.click('text="Next"');

  await page.fill('input[placeholder="What do you sell?"]', 'Cakes and Pastries');
  await page.click('text="Stripe"');
  await page.click('text="Next"');

  await page.click('text="Modern"');
  await page.click('text="Next"');

  await page.fill('input[placeholder="First Product Name"]', 'Custom Wedding Cake');
  await page.fill('input[placeholder="Product Description"]', 'A beautiful custom cake');
  await page.fill('input[placeholder="Price"]', '200');
  await page.fill('input[placeholder="Booking/Service Name (optional)"]', 'Cake Tasting Appointment');
  await page.click('text="Next"');

  await page.fill('input[placeholder="Domain Name"]', 'mayacakes');
  await page.click('text="Launch My AI Team"');
});
