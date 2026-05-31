import { expect, test } from './fixtures';

test('Onboarding wizard supports custom domain choice', async ({ page }) => {
  await page.goto('/onboarding');

  // Wait for initial load
  await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

  // Step 1: Business details
  await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("E2E Test Business");
  await page.getByRole("button", name="Next").click();

  await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...").fill("Testing domain choice E2E");
  await page.getByRole("button", name="Next").click();

  await page.getByPlaceholder("e.g. Portland, OR", { exact: false }).fill("Seattle");

  // Submit intake
  await page.getByRole("button", name="Generate My Business").click();

  // Step 2: Review (Wait for mock AI to complete intake or fallback)
  await expect(page.getByRole("button", name="Continue")).toBeVisible({ timeout: 15000 });
  await page.getByRole("button", name="Continue").click();

  // Step 3: Domain Choice and Settings
  await expect(page.getByRole('heading', { name: "Style & Team" })).toBeVisible();

  // Verify default state
  const subdomainBtn = page.getByText('Subdomain (Free)');
  const customDomainBtn = page.getByText('Custom Domain');

  await expect(subdomainBtn).toBeVisible();
  await expect(customDomainBtn).toBeVisible();

  // Select Custom Domain
  await customDomainBtn.click();

  // Launch Store
  await page.getByRole('button', { name: 'Launch Store' }).click();

  // Building state
  await expect(page.getByRole('heading', { name: 'Building Your Business...' })).toBeVisible();

  // Final state
  await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 10000 });
});
