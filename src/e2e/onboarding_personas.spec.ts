import { test, expect } from './fixtures';

test.describe('OHC Onboarding Persona Critical User Journeys', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  const personas = [
    {
      name: 'Maya',
      businessName: "Maya's Custom Cakes",
      description: "I bake custom vegan cakes for weddings and parties in Portland",
      location: "Portland, OR",
      expectedRole: "The Promoter"
    },
    {
      name: 'Carlos',
      businessName: "Carlos Repairs",
      description: "Home plumbing and painting services in Miami",
      location: "Miami, FL",
      expectedRole: "The Manager"
    },
    {
      name: 'Priya',
      businessName: "Priya's Boutique",
      description: "A curated clothing boutique with handpicked seasonal items",
      location: "New York, NY",
      expectedRole: "The Promoter"
    },
    {
      name: 'Leo',
      businessName: "Leo Guitar Lessons",
      description: "Online and in-person guitar lessons for beginners",
      location: "Austin, TX",
      expectedRole: "The Salesperson"
    },
    {
      name: 'Fatima',
      businessName: "Fatima's Food Cart",
      description: "Authentic halal food pre-orders for local pickup",
      location: "Chicago, IL",
      expectedRole: "The Manager"
    }
  ];

  for (const persona of personas) {
    test(`CUJ: ${persona.name} Onboarding Flow`, async ({ page }) => {
      await page.goto('/onboarding');
      await expect(page.getByRole('heading', { name: "What's the name of your business?", exact: false })).toBeVisible({ timeout: 15000 });
      await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill(persona.businessName);
      await page.getByRole('button', { name: 'Next' }).click();
      await expect(page.getByRole('heading', { name: 'What do you sell?', exact: false })).toBeVisible({ timeout: 15000 });
      await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill(persona.description);
      await page.getByRole('button', { name: 'Next' }).click();
      await expect(page.getByRole('heading', { name: 'Where are you located?', exact: false })).toBeVisible({ timeout: 15000 });
      await page.getByPlaceholder('e.g. Portland, OR').fill(persona.location);
      await page.getByRole('button', { name: /Generate My Business/i }).click();
      await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
      await expect(page.getByDisplayValue(persona.businessName)).toBeVisible();
      await page.getByRole('button', { name: /Continue/i }).click();
      await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
      await page.getByText('Bold').click();
      await page.getByRole('button', { name: /Launch Store/i }).click();
      await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });
      await expect(page.getByText('my-business.ohc.store')).toBeVisible();
      await page.getByRole('link', { name: /Go to Dashboard/i }).click();
      await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
      await page.goto('/agents');
      await expect(page.getByText(persona.expectedRole)).toBeVisible({ timeout: 15000 });
    });
  }
});
