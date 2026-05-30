import { test, expect } from './fixtures';

test.describe('Onboarding Wizard - Cross Device Resilience', () => {
  test('persists state correctly across separate sessions', async ({ browser }) => {
    // Context 1: Mobile Phone (User starts onboarding)
    const context1 = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const page1 = await context1.newPage();

    await page1.goto('/onboarding');
    await page1.waitForTimeout(1000);

    // Chat Step 1
    await expect(page1.getByRole('heading', { name: "What's the name of your business?", exact: false })).toBeVisible({ timeout: 15000 });
    await page1.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Bakery");
    await page1.getByRole('button', { name: "Next" }).click();

    // Chat Step 2
    await expect(page1.getByRole('heading', { name: "What do you sell?", exact: false })).toBeVisible({ timeout: 15000 });
    await page1.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill("maya bakes cakes");
    await page1.getByRole('button', { name: "Next" }).click();

    // Wait for auto-save debounce (1000ms in frontend)
    await page1.waitForTimeout(1500);

    // Close the phone browser without finishing
    await context1.close();

    // Context 2: Desktop Browser (User resumes onboarding)
    const context2 = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const page2 = await context2.newPage();

    await page2.goto('/onboarding');
    await page2.waitForTimeout(1000);

    // The backend should restore state to Chat Step 3 since Step 1 and 2 are completed
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
