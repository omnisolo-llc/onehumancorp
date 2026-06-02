import { test, expect } from '@playwright/test';

test.describe('Elena Persona - Complete Business Operation CUJ', () => {
  test.use({
    viewport: { width: 375, height: 812 }, // iPhone 12 Pro style
    userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_8 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Mobile/15E148 Safari/604.1',
  });

  test('Elena launches her candle shop and manages her dashboard', async ({ page }) => {
    // 1. Visit Onboarding Home
    await page.goto('http://localhost:3000/onboarding');
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // 2. Persona Intake - Business Name
    const nameInput = page.getByPlaceholder("e.g. Maya's Custom Cakes");
    await nameInput.fill("Elena's Ethos");
    await nameInput.press('Enter');

    // 3. Persona Intake - What you sell
    const sellInput = page.getByPlaceholder(/e.g. I bake/);
    await sellInput.fill("Handmade artisanal candles inspired by the Arizona desert.");
    await page.getByRole('button', { name: 'Next' }).click();

    // 4. Persona Intake - Location
    const locationInput = page.getByPlaceholder(/e.g. Portland/);
    await locationInput.fill("Sedona, AZ");
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // 5. Review Step
    await expect(page.getByText('Review Details')).toBeVisible();
    await expect(page.locator('input[value="Elena\'s Ethos"]')).toBeVisible();
    await expect(page.locator('input[value="Artisanal Candle Shop"]')).toBeVisible();
    await page.getByRole('button', { name: 'Continue' }).click();

    // 6. Style & Team Step
    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.getByText('Modern').click();
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // 7. Loading & Success
    await expect(page.getByText("You're Live!", { timeout: 15000 })).toBeVisible();
    await expect(page.getByText("Elena's Ethos is now live")).toBeVisible();

    // 8. Go to Dashboard
    await page.getByRole('link', { name: 'Go to Dashboard' }).click();
    await expect(page.url()).toContain('/dashboard');

    // 9. Verify Dashboard State (Elena's branding)
    await expect(page.getByText("Elena's Ethos")).toBeVisible();

    // Task 1: Check Business Results (Dashboard Analytics)
    await expect(page.getByText('Business Analytics')).toBeVisible();
    await expect(page.getByText('Total Sales')).toBeVisible();

    // Task 2: Add a sellable item (Sedona Sunset Candle)
    await page.getByRole('button', { name: '+ Add Item' }).click();
    await page.getByPlaceholder('e.g. Custom Cake').fill('Sedona Sunset Candle');
    await page.getByPlaceholder('0.00').fill('29.99');
    await page.getByRole('button', { name: 'Save Product' }).click();
    // Verify increment (mock state)
    await expect(page.getByText('11 / 10 Products Used')).toBeVisible();

    // Task 3: Check Inbox for customer inquiries
    await page.getByRole('link', { name: 'Inbox' }).click();
    await expect(page.url()).toContain('/inbox');
    await expect(page.getByText('Customer Messages')).toBeVisible();
    await page.goBack();

    // Task 4: AI Team Oversight (Team Activity)
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    await expect(page.getByText('Team Activity')).toBeVisible();
    // Swarm Agent should be visible in the feed
    await expect(page.locator('#agent-activity-feed')).toBeVisible();

    // Task 5: Business Advisory (Weekly Insights)
    await expect(page.getByText('AI Business Advisory')).toBeVisible();
    await expect(page.getByText('Great job! You sold 20 more lunches')).toBeVisible();

    // Responsive Audit Check
    const header = page.locator('header');
    const headerBox = await header.boundingBox();
    expect(headerBox?.width).toBeLessThanOrEqual(375);

    // Verify horizontal nav scrollability
    const nav = page.locator('nav');
    const hasScroll = await nav.evaluate(el => el.scrollWidth > el.clientWidth);
    expect(hasScroll).toBe(true);

    await page.screenshot({ path: 'screenshots/elena_cuj_final_dashboard_mobile.png' });
  });
});
