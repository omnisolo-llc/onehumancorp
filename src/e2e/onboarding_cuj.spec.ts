import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => window.localStorage.clear());
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Bakery',
          business_name: 'Maya Bakery',
          categories: ['food'],
          initial_products: [{ name: 'Cake', price: '20' }]
        }),
      });
    });
    await page.route('/api/onboarding/state', async route => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({}),
        });
        return;
      }

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({}),
      });
    });
    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          message: 'Your business has been successfully launched.',
        }),
      });
    });
  });

  async function startOnboarding(page: import('@playwright/test').Page) {
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
  }


  // Test 1: Persona navigates from home, starts onboarding
  test('Persona: Business Owner completes initial setup successfully', async ({ page }) => {
    await startOnboarding(page);

    // 2. Owner enters business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    // 3. Owner enters what they sell
    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await sellInput.fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    // 4. Owner enters location
    const locInput = page.getByPlaceholder(/Portland, OR/i);
    await locInput.fill('NY');

    const generateBtn = page.getByRole('button', { name: /Next/i });
    await generateBtn.click();

    const audienceInput = page.getByPlaceholder(/e.g. Local families, Tech startups/i);
    await audienceInput.fill('Tech enthusiasts and developers');
    await page.getByRole('button', { name: /Next/i }).click();

    // Depending on backend speed we may need to wait for the analysis overlay to disappear
    // 5. Verify it transitions to Step 2: Review Details
    // Depending on backend speed we may need to increase timeout or just await visibility
    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });

    // 6. Owner continues to Step 3: Style & Team
    await page.getByRole('button', { name: /Continue/i }).click();
    await expect(page.getByText('Style & Team')).toBeVisible();

    // 7. Owner launches store
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya Smith');
    await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('mypassword123');
    await page.getByRole('button', { name: /Launch Store/i }).click({ force: true });

    // 8. Verify it transitions to Live Screen
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('link', { name: /Open Assistant/i })).toBeVisible();
  });

  // Test 2: Ensure validation fails on small name
  test('Persona: Business Owner fails validation on short business name', async ({ page }) => {
    await startOnboarding(page);

    // Owner enters short business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('M');
    await page.getByRole('button', { name: /Next/i }).click();

    // Expect validation failure message immediately
    await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
  });

  // Test 3: Validate missing location blocks progression
  test('Persona: Business Owner cannot progress without location', async ({ page }) => {
    await startOnboarding(page);

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await sellInput.fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    // Keep location empty
    const generateBtn = page.getByRole('button', { name: /Next/i });
    await expect(generateBtn).toBeDisabled();
  });

  // Test 4: Navigating Back works
  test('Persona: Business Owner can navigate back from sell step', async ({ page }) => {
    await startOnboarding(page);

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();
    await expect(page.getByText('What do you sell?')).toBeVisible();

    const backBtn = page.getByRole('button', { name: /Back/i });
    await backBtn.click();
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
  });

  // Test 5: Can cancel from Style & Team
  test('Persona: Business Owner can toggle Auto Respond on Style & Team step', async ({ page }) => {
    await startOnboarding(page);

    await page.getByPlaceholder(/e.g. Maya's Custom Cakes/i).fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();
    await page.getByPlaceholder(/Portland, OR/i).fill('NY');
    await page.getByRole('button', { name: /Next/i }).click();
    const audienceInput = page.getByPlaceholder(/e.g. Local families, Tech startups/i);
    await audienceInput.fill('Tech enthusiasts and developers');
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    await expect(page.getByText('Style & Team')).toBeVisible();

    // Toggle auto-respond
    const autoRespondToggle = page.getByRole('checkbox', { name: /Allow AI to Auto-Respond/i });
    await expect(autoRespondToggle).toBeChecked();
    await page.getByText('Allow AI to Auto-Respond').click();
    await expect(autoRespondToggle).not.toBeChecked();
  });
});
