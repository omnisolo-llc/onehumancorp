import { test, expect } from './fixtures';

test.describe('Onboarding Flow - Maya the Baker', () => {
  test.setTimeout(120000);

  test('completes full onboarding journey', async ({ page }) => {
    // Clear out previous state robustly using Playwright's addInitScript to intercept localstorage immediately
    const id = `maya-${Date.now()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboarding-storage-v3');
    }, id);

    await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}/business-setup` : 'http://localhost:3000/business-setup');

    // Make sure we are on the onboarding page
    await expect(page.locator('a', { hasText: 'Start Business Setup' })).toBeVisible({ timeout: 15000 });
    await page.locator('a', { hasText: 'Start Business Setup' }).click();

    // Start Onboarding (if shown, depending on state init)
    const startBtn = page.getByRole('button', { name: /Start Onboarding/i });
    if (await startBtn.isVisible()) {
      await startBtn.click();
    }

    // Step 1: Business Details
    await expect(page.locator('body')).toContainText(/Welcome|What's the name|Review Details|Style & Team/i, { timeout: 15000 });

    const nameInput = page.getByRole('textbox').first();
    if (await nameInput.isVisible()) {
      await nameInput.fill('Maya Custom Cakes');
      const continueBtn1 = page.getByRole('button', { name: /Next/i });
      if (await continueBtn1.isVisible()) {
          await continueBtn1.click();
      } else {
          await nameInput.press('Enter');
      }
    }

    // Explicitly wait between the sequential steps
    const descriptionTextarea = page.getByRole('textbox').first();
    if (await descriptionTextarea.isVisible()) {
      await descriptionTextarea.fill('Custom vegan cakes for all occasions.');
      const continueBtn1 = page.getByRole('button', { name: /Next/i });
      if (await continueBtn1.isVisible()) {
          await continueBtn1.click();
      } else {
          await descriptionTextarea.press('Enter');
      }
    }

    // Explicitly wait between the sequential steps
    const locationInput = page.getByRole('textbox').first();
    if (await locationInput.isVisible()) {
      await locationInput.fill('Brooklyn, NY');
      const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
      if (await generateBtn.isVisible()) {
          await expect(generateBtn).toBeEnabled({ timeout: 5000 });
          await generateBtn.click();
      } else {
          await locationInput.press('Enter');
      }
    }

    // Sometimes there is a generic generate button
    const generateBtn2 = page.getByRole('button', { name: /Generate My Business/i });
    if (await generateBtn2.isVisible()) {
      await expect(generateBtn2).toBeEnabled({ timeout: 5000 });
      await generateBtn2.click();
    }

    // Step 2: What do you sell? (Review Details)
    // IMPORTANT: Wait for the backend API to finish! It's generating the business.
    // If this fails, the local dev backend might be missing or network interception might be blocking.
    if (await page.locator('h2', { hasText: 'Style & Team' }).isVisible()) { /* already jumped */ } else {
      await expect(page.locator('body')).toContainText(/Review Details|Style & Team/i, { timeout: 60000 });
    }

    const businessTypeInputs = page.locator('input[type="text"]');
    if (await businessTypeInputs.nth(1).isVisible()) {
      await businessTypeInputs.nth(1).fill('Retail & Commerce');
    }

    // Categories (Click a couple)
    const categoriesInput = page.locator('input').nth(2);
    if (await categoriesInput.isVisible()) {
      await categoriesInput.fill('Physical Products');
    }

    // First Product
    const inputs = page.locator('input[type="text"]');
    if (await inputs.nth(3).isVisible()) {
      await inputs.nth(3).fill('Vegan Celebration Cake');
      await inputs.nth(4).fill('45');
    }

    const continueBtn2 = page.locator('button', { hasText: 'Continue' });
    if (await continueBtn2.isVisible()) {
      await continueBtn2.click();
    }

    // Step 3: Style & Team
    await expect(page.locator('h2', { hasText: 'Style & Team' })).toBeVisible({ timeout: 15000 });

    // Select Template
    await page.locator('div', { hasText: 'Modern' }).first().click();

    // Domain Choice
    await page.locator('div', { hasText: 'Free Subdomain' }).first().click();

    // Admin Details
    const adminInputs = page.locator('input');
    await adminInputs.nth(0).fill('Maya Smith');
    await adminInputs.nth(1).fill('maya@example.com');
    await adminInputs.nth(2).fill('securepassword123');

    // Select Agents
    await page.locator('div', { hasText: 'Sales Agent' }).first().click();
    await page.locator('div', { hasText: 'Support Agent' }).first().click();

    // Auto Respond is checked by default

    // Launch Store
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Step 4: Loading (Building Your Business...)
    await expect(page.locator('h2', { hasText: 'Building Your Business...' })).toBeVisible({ timeout: 15000 });

    // Step 5: You're Live! (wait for transition)

    await expect(page.locator('h2', { hasText: "You're Live!" })).toBeVisible({ timeout: 60000 });

    // Verify Shareable Link exists
    await expect(page.locator('text=my-business.ohc.store')).toBeVisible({ timeout: 15000 });

    // Verify Go to Dashboard button
    await expect(page.locator('a:has-text("Go to Dashboard")')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('a:has-text("Preview Storefront")')).toBeVisible({ timeout: 15000 });
  });
});
