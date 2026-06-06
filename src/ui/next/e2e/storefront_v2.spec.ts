import { test, expect, devices } from '@playwright/test';

test.use({
  ...devices['iPhone 13'],
});

test('Maya the baker journey storefront v2', async ({ page }) => {
  // Mock API responses
  await page.route('/api/v1/builder/generate', async route => {
    await route.fulfill({
      json: {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Maya Cakes', copy: 'Best cakes in town' } },
            { block_type: 'ProductGridBlock', content: { items: [{ name: 'Chocolate Cake', price: '$20' }] } }
          ]
        }]
      }
    });
  });

  await page.route('/api/v1/builder/publish_draft', async route => {
    await route.fulfill({
      json: { id: 'site-123', domain: 'mayacakes' }
    });
  });

  await page.goto('/builder');

  // Screen 1: Onboarding
  await expect(page.getByText('What are you building today?')).toBeVisible();
  await page.click('text=Selling Products');

  // Screen 1.5: Wizard (Idle state)
  await page.getByPlaceholder('e.g. Acme Corp').fill('Maya Cakes');
  await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Bakery');
  await page.click('text=Next: Choose Vibe');

  await page.click('text=Friendly');
  await page.click('text=Next: Details');

  await page.getByPlaceholder('e.g. I run a mobile dog grooming service in Portland').fill('I bake custom cakes for weddings and parties.');
  await page.click('id=generate-btn');

  // Screen 2: Generating
  await expect(page.getByText('AI Architect')).toBeVisible();

  // Screen 2.5: Selection
  await expect(page.getByText('Pick your draft')).toBeVisible();
  await page.click('text=Customize Selected Draft');

  // Screen 3: Mobile Editor
  await expect(page.getByText('Mobile Editor')).toBeVisible();
  await expect(page.getByText('Maya Cakes')).toBeVisible();

  // Test Action Sheet
  await page.click('text=Maya Cakes');
  await expect(page.getByText('Edit Hero Block')).toBeVisible();
  await page.click('text=Save Changes');

  // Screen 4: Launch
  await page.click('id=launch-btn');

  // Success Screen
  await expect(page.getByText("You're Live!")).toBeVisible();
  await expect(page.getByText('https://mayacakes.ohc.store')).toBeVisible();
});
