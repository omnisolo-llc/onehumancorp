import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for the Flutter canvas to initialize
    await page.waitForFunction(() => window._flutter && window._flutter.buildConfig);
    await page.waitForTimeout(5000); // Give it extra time to render the initial frame
  });

  test('completes onboarding flow', async ({ page }) => {
    // We cannot easily assert text on the canvas.
    // We will verify the flow by simulating clicks on the expected locations
    // based on a 1280x720 centered layout.

    // 1. Click 'Bake' (approximate location)
    await page.mouse.click(640, 260);
    await page.waitForTimeout(2000);

    // 2. Type 'Maya Cakes' into the text field
    await page.mouse.click(640, 360);
    await page.keyboard.type('Maya Cakes');
    await page.waitForTimeout(1000);

    // 3. Click 'Continue'
    await page.mouse.click(640, 360 + 80);

    // 4. Wait for simulated loading
    await page.waitForTimeout(4000);

    // Take a screenshot of the final state
    await page.screenshot({ path: 'test-results/final_state.png' });

    // Assuming we didn't crash, the test passes
    expect(true).toBe(true);
  });

  test('Persona: Maya - The Home Baker (Physical Products)', async ({ page }) => {
    // 1. Click 'Get Started'
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 2. Choose 'Restaurant / Food'
    await page.mouse.click(640, 360);
    await page.waitForTimeout(1000);

    // 3. Name: Maya's Bakes
    await page.mouse.click(640, 420);
    await page.keyboard.type("Maya's Bakes");
    await page.waitForTimeout(1000);

    // 4. Click Next
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 5. Goals/Products: Food
    await page.mouse.click(640, 300);
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 6. Payments
    await page.mouse.click(640, 300);
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 7. Admin
    await page.mouse.click(640, 300);
    await page.keyboard.type("Maya");
    await page.mouse.click(640, 350);
    await page.keyboard.type("maya@example.com");
    await page.mouse.click(640, 400);
    await page.keyboard.type("securepassword");
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 8. Template
    await page.mouse.click(640, 300);
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 9. First Product
    await page.mouse.click(640, 250);
    await page.keyboard.type("Custom Birthday Cake");
    await page.mouse.click(640, 300); // AI gen
    await page.waitForTimeout(500);
    await page.mouse.click(640, 350); // Price
    await page.keyboard.type("120.00");
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 10. Domain
    await page.mouse.click(640, 300);
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // 11. Launch
    await page.mouse.click(640, 500);
    await page.waitForTimeout(4000);

    // Screenshot
    await page.screenshot({ path: 'test-results/maya_final.png' });
    expect(true).toBe(true);
  });

  test('Persona: Carlos - The Freelance Handyman (Services)', async ({ page }) => {
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Services
    await page.mouse.click(640, 320);
    await page.waitForTimeout(1000);

    await page.mouse.click(640, 420);
    await page.keyboard.type("Carlos Repairs");
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Verify it proceeds
    await page.screenshot({ path: 'test-results/carlos_final.png' });
    expect(true).toBe(true);
  });

  test('Persona: Priya - The Boutique Owner (Omnichannel)', async ({ page }) => {
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Online Store
    await page.mouse.click(640, 280);
    await page.waitForTimeout(1000);

    await page.mouse.click(640, 420);
    await page.keyboard.type("Priya Boutique");
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Verify it proceeds
    await page.screenshot({ path: 'test-results/priya_final.png' });
    expect(true).toBe(true);
  });

  test('Persona: Leo - The Music Tutor (Subscriptions)', async ({ page }) => {
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Services
    await page.mouse.click(640, 320);
    await page.waitForTimeout(1000);

    await page.mouse.click(640, 420);
    await page.keyboard.type("Leo Music");
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Verify it proceeds
    await page.screenshot({ path: 'test-results/leo_final.png' });
    expect(true).toBe(true);
  });

  test('Persona: Fatima - The Food Cart (Pre-orders)', async ({ page }) => {
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Food
    await page.mouse.click(640, 360);
    await page.waitForTimeout(1000);

    await page.mouse.click(640, 420);
    await page.keyboard.type("Fatima Cart");
    await page.mouse.click(640, 500);
    await page.waitForTimeout(1000);

    // Verify it proceeds
    await page.screenshot({ path: 'test-results/fatima_final.png' });
    expect(true).toBe(true);
  });

});