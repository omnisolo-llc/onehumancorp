import { test, expect } from './fixtures';

test.describe('Onboarding Wizard - Cross Device Resilience', () => {
  test('persists state correctly across separate sessions', async ({ browser }) => {
    const context1 = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const page1 = await context1.newPage();

    // Step 1 - Fill partial information
    await page1.goto('/onboarding');
    await page1.waitForTimeout(1000);

    // Chat Step 1
    await expect(page1.getByRole('heading', { name: "What's the name of your business?", exact: false })).toBeVisible({ timeout: 15000 });
    await page1.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Bakery");
    await page1.getByRole('button', { name: "Next" }).click();

    // Wait for debounce sync
    await page1.waitForTimeout(1500);

    // Now start a second session (simulating desktop or another device)
    const context2 = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const page2 = await context2.newPage();

    await page2.goto('/onboarding');
    await page2.waitForTimeout(1500);

    // Wait for the state to load
    await expect(page2.getByRole('heading', { name: "What do you sell?", exact: false })).toBeVisible({ timeout: 15000 });

    await page2.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill("maya bakes cakes");
    await page2.getByRole('button', { name: "Next" }).click();

    await expect(page2.getByRole('heading', { name: "Where are you located?", exact: false })).toBeVisible({ timeout: 15000 });
    await page2.getByPlaceholder("e.g. Portland, OR").fill("Seattle, WA");
    await page2.getByRole('button', { name: "Generate My Business" }).click();

    // Review
    await expect(page2.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page2.getByRole('button', { name: /Continue/i }).click();

    // Style
    await expect(page2.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page2.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page2.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });

    await context1.close();
    await context2.close();
  });
});
