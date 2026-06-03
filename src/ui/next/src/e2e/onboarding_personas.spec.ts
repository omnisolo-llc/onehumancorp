import { test, expect } from '@playwright/test';

test.describe('Onboarding Critical User Journeys (Personas)', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  async function completeConversationalIntake(page, name, sell, social, location, mockData) {
    await page.goto('http://localhost:3000/onboarding');

    // Name
    await page.fill('input[placeholder*="Maya\'s Custom Cakes"]', name);
    await page.click('button:has-text("Continue")');

    // What you sell
    await page.fill('textarea[placeholder*="I bake custom vegan cakes"]', sell);
    await page.click('button:has-text("Continue")');

    // Social Link
    if (social) {
      await page.fill('input[placeholder*="https://instagram.com"]', social);
      await page.click('button:has-text("Continue")');
    } else {
      await page.click('button:has-text("Skip this step")');
    }

    // Location
    await page.fill('input[placeholder*="Portland, OR"]', location);

    // Mock the intake API
    await page.route('**/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(mockData)
      });
    });

    await page.click('button:has-text("Generate My Business")');
  }

  test('Persona: Maya - Home Baker - Physical & Food flow', async ({ page }) => {
    await completeConversationalIntake(page, "Maya's Cakes", "I bake custom vegan cakes.", "https://instagram.com/maya", "Seattle, WA", {
      business_name: "Maya's Cakes",
      business_type: "Home Bakery",
      categories: ["physical", "food"],
      initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }]
    });

    await expect(page.getByText('Review Details')).toBeVisible();
    await expect(page.locator('button:has-text("Physical")')).toHaveClass(/bg-\[#0066FF\]/);
    await expect(page.locator('button:has-text("Food")')).toHaveClass(/bg-\[#0066FF\]/);

    await page.click('button:has-text("Continue")');

    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.fill('input[type="email"]', "maya@example.com");
    await page.click('text=The Manager');
    await page.click('text=The Promoter');

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
    });

    await page.click('button:has-text("Launch Store")');
    await expect(page.getByText("You're officially a Business Owner!")).toBeVisible();
    await expect(page.getByText("maya's-cakes.ohc.store")).toBeVisible();
  });

  test('Persona: Carlos - Handyman - Services flow', async ({ page }) => {
    await completeConversationalIntake(page, "Carlos Repairs", "Home improvements and repairs.", null, "Austin, TX", {
      business_name: "Carlos Repairs",
      business_type: "Handyman",
      categories: ["services"],
      initial_products: [{ name: "Repair Quote", price: "0.00" }]
    });

    await expect(page.getByText('Review Details')).toBeVisible();
    await expect(page.locator('button:has-text("Services")')).toHaveClass(/bg-\[#0066FF\]/);

    await page.click('button:has-text("Continue")');
    await page.fill('input[type="email"]', "carlos@example.com");
    await page.click('text=The Manager');

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
    });

    await page.click('button:has-text("Launch Store")');
    await expect(page.getByText("You're officially a Business Owner!")).toBeVisible();
    await expect(page.getByText("carlos-repairs.ohc.store")).toBeVisible();
  });

  test('Persona: Priya - Boutique Owner - Physical products', async ({ page }) => {
    await completeConversationalIntake(page, "Priya's Boutique", "Unique fashion boutique.", "https://instagram.com/priya", "New York, NY", {
      business_name: "Priya's Boutique",
      business_type: "Boutique",
      categories: ["physical"],
      initial_products: [{ name: "Designer Dress", price: "89.00" }]
    });

    await expect(page.getByText('Review Details')).toBeVisible();
    await page.click('button:has-text("Continue")');
    await page.fill('input[type="email"]', "priya@example.com");

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
    });

    await page.click('button:has-text("Launch Store")');
    await expect(page.getByText("You're officially a Business Owner!")).toBeVisible();
    await expect(page.getByText("priya's-boutique.ohc.store")).toBeVisible();
  });

  test('Persona: Leo - Music Tutor - Digital & Subscriptions', async ({ page }) => {
    await completeConversationalIntake(page, "Leo's Guitar", "Guitar lessons.", null, "Remote", {
      business_name: "Leo's Guitar",
      business_type: "Tutor",
      categories: ["digital", "subscriptions"],
      initial_products: [{ name: "Lesson Pack", price: "150.00" }]
    });

    await expect(page.getByText('Review Details')).toBeVisible();
    await expect(page.locator('button:has-text("Digital")')).toHaveClass(/bg-\[#0066FF\]/);
    await expect(page.locator('button:has-text("Subscriptions")')).toHaveClass(/bg-\[#0066FF\]/);

    await page.click('button:has-text("Continue")');
    await page.fill('input[type="email"]', "leo@example.com");

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
    });

    await page.click('button:has-text("Launch Store")');
    await expect(page.getByText("You're officially a Business Owner!")).toBeVisible();
    await expect(page.getByText("leo's-guitar.ohc.store")).toBeVisible();
  });

  test('Persona: Fatima - Food Cart - Food flow', async ({ page }) => {
    await completeConversationalIntake(page, "Fatima's Halal", "Authentic halal food.", null, "Queens, NY", {
      business_name: "Fatima's Halal",
      business_type: "Food Cart",
      categories: ["food"],
      initial_products: [{ name: "Gyro over Rice", price: "8.50" }]
    });

    await expect(page.getByText('Review Details')).toBeVisible();
    await expect(page.locator('button:has-text("Food")')).toHaveClass(/bg-\[#0066FF\]/);

    await page.click('button:has-text("Continue")');
    await page.fill('input[type="email"]', "fatima@example.com");

    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true }) });
    });

    await page.click('button:has-text("Launch Store")');
    await expect(page.getByText("You're officially a Business Owner!")).toBeVisible();
    await expect(page.getByText("fatima's-halal.ohc.store")).toBeVisible();
  });
});
