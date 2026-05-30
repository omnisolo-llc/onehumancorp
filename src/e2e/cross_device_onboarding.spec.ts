import { test, expect } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('Onboarding Wizard - Cross Device Resilience', () => {
  test('persists state correctly across separate sessions', async ({ browser }) => {
    const tenantId = `tenant-${uuidv4()}`;
    const userId = `user-${uuidv4()}`;

    // Simulate Device 1: Mobile Phone
    const context1 = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const page1 = await context1.newPage();

    // Inject auth state
    await page1.goto('/');
    await page1.evaluate(({ t, u }) => {
      localStorage.setItem('tenant_id', t);
      localStorage.setItem('user_id', u);
    }, { t: tenantId, u: userId });

    await page1.goto('/onboarding');
    await page1.waitForTimeout(1000); // wait for initial load and possible redirect

    // Chat Step 1
    await expect(page1.getByRole('heading', { name: "What's the name of your business?", exact: false })).toBeVisible({ timeout: 15000 });
    await page1.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Bakery");
    await page1.getByRole('button', { name: "Next" }).click();

    // Chat Step 2
    await expect(page1.getByRole('heading', { name: "What do you sell?", exact: false })).toBeVisible({ timeout: 15000 });
    await page1.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill("maya bakes cakes");

    // Wait for the debounced save to trigger
    await page1.waitForTimeout(2000);

    await context1.close();

    // Simulate Device 2: Desktop
    const context2 = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const page2 = await context2.newPage();

    // Inject same auth state
    await page2.goto('/');
    await page2.evaluate(({ t, u }) => {
      localStorage.setItem('tenant_id', t);
      localStorage.setItem('user_id', u);
    }, { t: tenantId, u: userId });

    await page2.goto('/onboarding');
    await page2.waitForTimeout(1000);

    // Should resume on Chat Step 2 with data pre-filled
    await expect(page2.getByRole('heading', { name: "What do you sell?", exact: false })).toBeVisible({ timeout: 15000 });
    await expect(page2.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...")).toHaveValue("maya bakes cakes");

    // Complete Chat Step 2
    await page2.getByRole('button', { name: "Next" }).click();

    // Chat Step 3
    await expect(page2.getByRole('heading', { name: "Where are you located?", exact: false })).toBeVisible({ timeout: 15000 });
    await page2.getByPlaceholder("e.g. Portland, OR").fill("Seattle, WA");
    await page2.getByRole('button', { name: "Generate My Business" }).click();

    // Step 2: Review
    await expect(page2.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page2.getByRole('button', { name: /Continue/i }).click();

    // Step 3: Style
    await expect(page2.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page2.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page2.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });

    await context2.close();
  });
});
