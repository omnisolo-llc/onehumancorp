import { test, expect } from '@playwright/test';

test.describe('Cross Device Resume E2E', () => {
  test('Persona: Business Owner saves draft and resumes later on another device', async ({ browser }) => {
    // 1. Owner starts from a desktop browser context
    const desktopContext = await browser.newContext({
      viewport: { width: 1440, height: 900 }
    });
    const desktopPage = await desktopContext.newPage();
    const id = `cross-device-resume-${Date.now()}-${Math.random()}`;
    await desktopPage.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboarding-storage-v3');
    }, id);

    await desktopPage.goto('/login');
    // We assume the test framework has setup or we just login
    await desktopPage.getByPlaceholder(/Email/i).fill('test@example.com');
    await desktopPage.getByPlaceholder(/Password/i).fill('password123');
    await desktopPage.getByRole('button', { name: /Log In/i }).click();

    // Now on home page, click to start onboarding
    await expect(desktopPage.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });
    await desktopPage.getByRole('link', { name: /Start Onboarding/i }).click();

    // Verify it landed on the Onboarding page at chatStep 1
    await expect(desktopPage.getByText('Tell us about your business')).toBeVisible();

    // 2. Owner enters business name
    const nameInput = desktopPage.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Maya Bakery');

    // Save draft
    const saveDraftBtn1 = desktopPage.getByRole('button', { name: /Save Draft/i });
    await saveDraftBtn1.click();
    await expect(desktopPage.getByText('Draft Saved!')).toBeVisible();

    // Move to next step (chatStep 2)
    await desktopPage.getByRole('button', { name: /Next/i }).click();
    await expect(desktopPage.getByText('What do you sell?')).toBeVisible();

    // Save draft at chatStep 2
    const saveDraftBtn2 = desktopPage.getByRole('button', { name: /Save Draft/i });
    await saveDraftBtn2.click();
    await expect(desktopPage.getByText('Draft Saved!')).toBeVisible();

    // Close the desktop browser context
    await desktopContext.close();

    // 3. Owner resumes on a mobile device
    const mobileContext = await browser.newContext({
      viewport: { width: 375, height: 667 },
      userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1',
      isMobile: true,
      hasTouch: true
    });

    const mobilePage = await mobileContext.newPage();

    await mobilePage.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      // Notice we do NOT set onboarding-storage-v3, simulating a fresh browser without Zustand persistence
    }, id);

    await mobilePage.goto('/login');
    // Login again on the new mobile device
    await mobilePage.getByPlaceholder(/Email/i).fill('test@example.com');
    await mobilePage.getByPlaceholder(/Password/i).fill('password123');
    await mobilePage.getByRole('button', { name: /Log In/i }).click();

    await expect(mobilePage.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });

    // They navigate to onboarding
    await mobilePage.getByRole('link', { name: /Start Onboarding/i }).click();

    // Because it fetches from the backend on mount, it should restore the state,
    // including the business name and chatStep = 2
    await expect(mobilePage.getByText('What do you sell?')).toBeVisible({ timeout: 15000 });

    // Check if we can go back and see the previously entered business name
    await mobilePage.getByRole('button', { name: /Back/i }).click();
    await expect(mobilePage.getByText('Tell us about your business')).toBeVisible();
    await expect(mobilePage.getByPlaceholder(/e.g. Maya's Custom Cakes/i)).toHaveValue('Maya Bakery');

    await mobileContext.close();
  });
});
