import { test, expect } from './fixtures';

test.describe('Onboarding Resume Feature', () => {
  test('Persona: Business Owner generates a resume link on save draft', async ({ page }) => {
    // Navigate to the onboarding route directly
    await page.goto('/onboarding');

    // Fill out the business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.waitFor({ state: 'visible' });
    await nameInput.fill('Resume Bakery');

    // Click Save Draft
    await page.getByRole('button', { name: /Save Draft/i }).first().click();

    // Verify the resume toast appears and contains the origin url with parameters
    const resumeToast = page.getByText('Resume later on any device with this link:');
    await expect(resumeToast).toBeVisible({ timeout: 15000 });

    const linkInput = page.locator('input[readonly]').first();
    const linkValue = await linkInput.inputValue();

    expect(linkValue).toContain('/onboarding?resume_tenant=');
    expect(linkValue).toContain('&resume_user=');

    // Verify copy button exists
    const copyButton = page.getByRole('button', { name: /Copy/i }).first();
    await expect(copyButton).toBeVisible();
  });

  test('Persona: Business Owner resumes session from link successfully', async ({ page, browser }) => {
    await page.goto('/onboarding');

    // Inject mock local storage
    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'test_tenant_xyz');
      localStorage.setItem('user_id', 'test_user_xyz');
    });

    await page.goto('/onboarding');

    // Chat Step 1
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.waitFor({ state: 'visible' });
    await nameInput.fill('Restored Bakery');

    // Save draft
    await page.getByRole('button', { name: /Save Draft/i }).first().click();
    await expect(page.getByText('Draft Saved!')).toBeVisible({ timeout: 15000 });

    // Open a completely fresh context representing a new device (like a phone)
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    // Use the resume link
    await newPage.goto('/onboarding?resume_tenant=test_tenant_xyz&resume_user=test_user_xyz');

    // Wait for hydration
    await newPage.waitForTimeout(2000);

    // Verify it hydrated the values correctly into the local storage
    const restoredTenant = await newPage.evaluate(() => localStorage.getItem('tenant_id'));
    const restoredUser = await newPage.evaluate(() => localStorage.getItem('user_id'));

    expect(restoredTenant).toBe('test_tenant_xyz');
    expect(restoredUser).toBe('test_user_xyz');

    await newContext.close();
  });

  test('Persona: Business Owner copies link to clipboard', async ({ page, context }) => {
    // Grant clipboard permissions
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.goto('/onboarding');

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.waitFor({ state: 'visible' });
    await nameInput.fill('Clipboard Bakery');

    await page.getByRole('button', { name: /Save Draft/i }).first().click();

    // Click the copy button
    const copyButton = page.getByRole('button', { name: /Copy/i }).first();
    await expect(copyButton).toBeVisible({ timeout: 15000 });
    await copyButton.click();

    // Wait for the message to change to "Link copied!"
    await expect(page.getByText('Link copied!')).toBeVisible();

    // Validate clipboard content using page evaluate
    const clipboardText = await page.evaluate(() => navigator.clipboard.readText());
    expect(clipboardText).toContain('/onboarding?resume_tenant=');
  });

  test('Persona: Business Owner generates a unique tenant id automatically', async ({ page }) => {
    // Clear any existing localStorage
    await page.goto('/onboarding');
    await page.evaluate(() => localStorage.clear());
    await page.goto('/onboarding');

    // Wait for mount
    await page.waitForTimeout(2000);

    // Read the generated tenant id
    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id'));

    expect(tenantId).not.toBeNull();
    expect(tenantId).not.toBe('storefront');
    expect(tenantId?.startsWith('temp_')).toBeTruthy();
  });

  test('Persona: Cross-device link persists in URL briefly and cleans up', async ({ page }) => {
    await page.goto('/onboarding?resume_tenant=testing123&resume_user=testingabc');

    // The effect in onboarding/page.tsx calls history.replaceState to remove the query params
    await page.waitForTimeout(2000);

    const currentUrl = page.url();
    expect(currentUrl).not.toContain('resume_tenant');

    const restoredTenant = await page.evaluate(() => localStorage.getItem('tenant_id'));
    expect(restoredTenant).toBe('testing123');
  });
});
