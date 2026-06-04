import { test, expect } from './fixtures';

test.describe('Progressive AI Interview Onboarding CUJs', () => {

  test('Maya the Home Baker can complete the onboarding flow', async ({ page }) => {
    // Navigate to the onboarding page
    await page.goto('/onboarding');

    // --- Step 1: Chat/Intake ---
    // Wait for the first prompt
    await expect(page.getByText('Tell us about your business')).toBeVisible({ timeout: 10000 });

    // Fill in the business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill in the description
    const sellInput = page.getByPlaceholder(/e.g. I bake custom vegan cakes/i);
    await expect(sellInput).toBeVisible();
    await sellInput.fill('I bake custom vegan cakes for weddings and parties. We offer local pickup and custom deposits.');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill in the location
    const locInput = page.getByPlaceholder(/e.g. Portland, OR/i);
    await expect(locInput).toBeVisible();
    await locInput.fill('Seattle, WA');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // --- Step 2: Review Details ---
    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 }); // Wait for real LLM API call

    // Maya decides to proceed
    await page.getByRole('button', { name: 'Continue' }).click();

    // --- Step 3: Style & Team ---
    await expect(page.getByText('Style & Team')).toBeVisible();

    // Select website template
    await page.getByText('Modern').click();

    // Check auto respond toggle
    const toggle = page.getByRole('checkbox');
    await expect(toggle).toBeChecked();

    // Account Setup
    await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('mypassword123');

    // Launch store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // --- Step 4 & 5: Loading and Success ---
    // Wait for Step 4 loading text temporarily
    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 5000 });

    // Wait for the success screen (Step 5)
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 30000 });

    // Verify links to dashboard and storefront are present
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Preview Storefront' })).toBeVisible();
  });

  test('Carlos the Freelance Handyman can complete the onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText('Tell us about your business')).toBeVisible({ timeout: 10000 });

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Carlos Handyman Services');
    await page.getByRole('button', { name: 'Next' }).click();

    const sellInput = page.getByPlaceholder(/e.g. I bake custom vegan cakes/i);
    await expect(sellInput).toBeVisible();
    await sellInput.fill('I do general home repairs, plumbing fixes, and painting. Customers can book a date and time slot.');
    await page.getByRole('button', { name: 'Next' }).click();

    const locInput = page.getByPlaceholder(/e.g. Portland, OR/i);
    await expect(locInput).toBeVisible();
    await locInput.fill('Austin, TX');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.getByText('Modern').click();

    await page.getByPlaceholder(/you@example.com/i).fill('carlos@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('carlospassword123');
    await page.getByRole('button', { name: 'Launch Store' }).click();

    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 30000 });

    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
  });

  test('Priya the Boutique Owner can complete the onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText('Tell us about your business')).toBeVisible({ timeout: 10000 });

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Priya Boutique');
    await page.getByRole('button', { name: 'Next' }).click();

    const sellInput = page.getByPlaceholder(/e.g. I bake custom vegan cakes/i);
    await expect(sellInput).toBeVisible();
    await sellInput.fill('I sell women clothing both in-store and online. I need inventory tracking and daily analytics.');
    await page.getByRole('button', { name: 'Next' }).click();

    const locInput = page.getByPlaceholder(/e.g. Portland, OR/i);
    await expect(locInput).toBeVisible();
    await locInput.fill('New York, NY');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.getByText('Modern').click();

    await page.getByPlaceholder(/you@example.com/i).fill('priya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('priyapassword123');
    await page.getByRole('button', { name: 'Launch Store' }).click();

    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 30000 });

    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
  });

  test('Leo the Music Tutor can complete the onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText('Tell us about your business')).toBeVisible({ timeout: 10000 });

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Leo Guitar Lessons');
    await page.getByRole('button', { name: 'Next' }).click();

    const sellInput = page.getByPlaceholder(/e.g. I bake custom vegan cakes/i);
    await expect(sellInput).toBeVisible();
    await sellInput.fill('I teach guitar online and in person. I need a booking calendar and a simple link-in-bio page.');
    await page.getByRole('button', { name: 'Next' }).click();

    const locInput = page.getByPlaceholder(/e.g. Portland, OR/i);
    await expect(locInput).toBeVisible();
    await locInput.fill('Los Angeles, CA');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.getByText('Modern').click();

    await page.getByPlaceholder(/you@example.com/i).fill('leo@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('leopassword123');
    await page.getByRole('button', { name: 'Launch Store' }).click();

    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 30000 });

    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
  });

  test('Fatima the Food Cart Operator can complete the onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText('Tell us about your business')).toBeVisible({ timeout: 10000 });

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Fatima Halal Food');
    await page.getByRole('button', { name: 'Next' }).click();

    const sellInput = page.getByPlaceholder(/e.g. I bake custom vegan cakes/i);
    await expect(sellInput).toBeVisible();
    await sellInput.fill('I run a halal food cart and need pre-order pickup flow for my customers.');
    await page.getByRole('button', { name: 'Next' }).click();

    const locInput = page.getByPlaceholder(/e.g. Portland, OR/i);
    await expect(locInput).toBeVisible();
    await locInput.fill('Chicago, IL');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.getByText('Modern').click();

    await page.getByPlaceholder(/you@example.com/i).fill('fatima@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('fatimapassword123');
    await page.getByRole('button', { name: 'Launch Store' }).click();

    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 30000 });

    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
  });
});
