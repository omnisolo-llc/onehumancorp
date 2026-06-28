import { test, expect } from '@playwright/test';

// OHC Core Directive: Run the Real CUJ.
// Do not mock API responses. Test against the real local service stack.
test.describe('Instant Setup CUJ', () => {
  // Test runs on Desktop screen size to verify the main layout
  test.use({ viewport: { width: 1440, height: 900 } });

  test('Persona: Maya (Home Baker) completes the Zero-Click Instant Onboarding', async ({ page }) => {


    await page.goto('/onboarding');


    // Verify Initial Screen
    await expect(page).toHaveTitle(/OneHumanCorp Setup/);
    const heading = page.locator('h2');
    await expect(heading).toHaveText('Instant Work Setup');

    // 1. Enter Email
    const emailInput = page.locator('input[placeholder="name@example.com"]');
    await expect(emailInput).toBeVisible();
    await emailInput.fill('maya.baker@example.com');

    // 2. Select Persona
    const select = page.locator('select');
    await expect(select).toBeVisible();
    await select.selectOption({ label: 'Freelancer / Solo Professional' }); // closest to Home Baker in the demo list

    // 3. Enter Context (Business details)
    const contextTextarea = page.locator('textarea[placeholder="What do you do? e.g. I run a home bakery selling custom cakes..."]');
    await expect(contextTextarea).toBeVisible();
    await contextTextarea.fill('I am Maya, a home baker selling custom vegan cakes and cupcakes via Instagram DMs. I need to manage orders, deposits, and delivery schedules.');

    // 4. Submit
    const generateBtn = page.locator('button', { hasText: 'Build My Work Context' });
    await expect(generateBtn).toBeVisible();
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Verify loading texts (animation progress)
    // const btnText = await generateBtn.innerText();
    // expect(btnText).toContain('Analyzing request...');

    // Check if the text changes to the next one
    // await expect(generateBtn).toContainText('Designing storefront...', { timeout: 4000 });

    await expect(page).toHaveURL(/.*success.html/, { timeout: 60000 });
  });
});
