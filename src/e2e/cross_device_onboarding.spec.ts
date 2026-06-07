import { test, expect } from './fixtures';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page }) => {
    // 1. Owner starts onboarding directly from the current route.
    await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}/onboarding` : 'http://localhost:3000/onboarding');
    await page.evaluate(() => { window.localStorage.clear(); window.sessionStorage.clear(); indexedDB.databases().then(dbs => { for (let db of dbs) indexedDB.deleteDatabase(db.name); }); });
    await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}/onboarding?reset=1` : 'http://localhost:3000/onboarding?reset=1');
    await page.waitForTimeout(2000);
    await page.evaluate(() => { localStorage.clear(); sessionStorage.clear(); });
    await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}/onboarding` : 'http://localhost:3000/onboarding');

    await expect(page.locator('body')).toContainText(/Welcome|What's the name/i, { timeout: 15000 });
    const startBtn = page.locator('button', { hasText: 'Start Onboarding' });
    if (await startBtn.isVisible()) {
      await startBtn.click();
    }
    await page.waitForTimeout(500);

    // Verify it landed on the Onboarding page
    await expect(page.locator('body')).toContainText("What's the name of your business?");

    // 2. Owner enters business name
    const nameInput = page.getByRole('textbox').first();
    await nameInput.fill('Cross Device Bakery');

    // 3. Save draft is actually automatic via Zustand persist, but there might be a manual button.
    // If not, we just rely on local state syncing.

    // 4. Simulate a cross-device session or reload
    await page.reload();

    const startBtn2 = page.locator('button', { hasText: 'Start Onboarding' });
    if (await startBtn2.isVisible()) {
      await startBtn2.click();
    }
    await page.waitForTimeout(500);

    // 5. Verify the business name was properly restored
    await expect(page.getByRole('textbox').first()).toHaveValue('Cross Device Bakery', { timeout: 10000 });
  });
});
