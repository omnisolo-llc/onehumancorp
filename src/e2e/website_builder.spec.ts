import { test, expect } from './fixtures';

import { test, expect } from './fixtures';

test.describe('Website Builder E2E', () => {

test.describe('Website Builder E2E', () => {
  test('should display onboarding and navigate to step 1', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByText('What are you building today?')).toBeVisible();
    await page.getByText('Selling Products').click();
    await expect(page.getByText("Let's build your store")).toBeVisible();
    await expect(page.getByPlaceholder('e.g. Acme Corp')).toBeVisible();
  });

  test('should complete wizard and generate draft', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();
    await page.getByPlaceholder('e.g. Acme Corp').fill('Maya Cakes');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Bakery');
    await page.getByText('Next: Choose Vibe').click();
    await page.getByText('Friendly').click();
    await page.getByText('Next: Details').click();
    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('I bake amazing custom cakes.');

    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Maya Cakes', copy: 'Delicious custom cakes.' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();
    await expect(page.getByText('AI Architect')).toBeVisible();
    await expect(page.getByText('Designing your custom storefront...')).toBeVisible();
    await expect(page.getByText('Pick your draft')).toBeVisible();
  });

  test('should enter mobile editor and show blocks', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();
    await page.getByPlaceholder('e.g. Acme Corp').fill('Maya Cakes');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Bakery');
    await page.getByText('Next: Choose Vibe').click();
    await page.getByText('Friendly').click();
    await page.getByText('Next: Details').click();
    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('I bake amazing custom cakes.');

    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Maya Cakes' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();
    await expect(page.getByText('Pick your draft')).toBeVisible();
    await page.getByText('Draft 1').click();
    await page.getByText('Customize Selected Draft').click();
    await expect(page.getByText('Mobile Editor')).toBeVisible();
    await expect(page.getByText('Maya Cakes')).toBeVisible();
  });

  test('should open action sheet to edit a block', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();
    await page.getByPlaceholder('e.g. Acme Corp').fill('Test Store');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Retail');
    await page.getByText('Next: Choose Vibe').click();
    await page.getByText('Friendly').click();
    await page.getByText('Next: Details').click();
    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('Test description');

    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Initial Headline' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();
    await expect(page.getByText('Pick your draft')).toBeVisible();
    await page.getByText('Draft 1').click();
    await page.getByText('Customize Selected Draft').click();
    await expect(page.getByText('Mobile Editor')).toBeVisible();
    await page.getByText('Initial Headline').click();
    await expect(page.getByText('Edit Hero Block')).toBeVisible();
    const input = page.locator('input[type="text"]').first();
    await input.fill('Updated Headline');
    await page.getByText('Save Changes').click();
    await expect(page.getByText('Updated Headline')).toBeVisible();
  });

  test('should launch storefront', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();
    await page.getByPlaceholder('e.g. Acme Corp').fill('Launch Store');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Tech');
    await page.getByText('Next: Choose Vibe').click();
    await page.getByText('Professional').click();
    await page.getByText('Next: Details').click();
    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('Launching soon');

    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Launch Headline' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();
    await expect(page.getByText('Pick your draft')).toBeVisible();
    await page.getByText('Customize Selected Draft').click();

    await page.route('**/builder/publish_draft', async route => {
      const json = { domain: 'launch-store.ohc.page' };
      await route.fulfill({ json });
    });

    await expect(page.getByText('Mobile Editor')).toBeVisible();
    await page.getByText('1-Tap Launch').click();
    await expect(page.getByText('You are Live! 🎉')).toBeVisible();
    await expect(page.getByText('launch-store.ohc.page')).toBeVisible();
  });
});

  test('should display onboarding and navigate to step 1', async ({ page }) => {
    await page.goto('/website-builder');

    // Check onboarding screen text
    await expect(page.getByText('What are you building today?')).toBeVisible();

    // Click on a business type
    await page.getByText('Selling Products').click();

    // Should proceed to wizard step 1
    await expect(page.getByText("Let's build your store")).toBeVisible();
    await expect(page.getByPlaceholder('e.g. Acme Corp')).toBeVisible();
  });

  test('should complete wizard and generate draft', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();

    // Step 1
    await page.getByPlaceholder('e.g. Acme Corp').fill('Maya Cakes');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Bakery');
    await page.getByText('Next: Choose Vibe').click();

    // Step 2
    await page.getByText('Friendly').click();
    await page.getByText('Next: Details').click();

    // Step 3
    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('I bake amazing custom cakes.');

    // Intercept API call
    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Maya Cakes', copy: 'Delicious custom cakes.' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();

    // Generating screen
    await expect(page.getByText('AI Architect')).toBeVisible();
    await expect(page.getByText('Designing your custom storefront...')).toBeVisible();

    // Draft Selection screen
    await expect(page.getByText('Pick your draft')).toBeVisible();
  });

  test('should enter mobile editor and show blocks', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();

    await page.getByPlaceholder('e.g. Acme Corp').fill('Maya Cakes');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Bakery');
    await page.getByText('Next: Choose Vibe').click();

    await page.getByText('Friendly').click();
    await page.getByText('Next: Details').click();

    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('I bake amazing custom cakes.');

    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Maya Cakes' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();

    // Wait for Draft Selection
    await expect(page.getByText('Pick your draft')).toBeVisible();

    // Pick draft 1
    await page.getByText('Draft 1').click();
    await page.getByText('Customize Selected Draft').click();

    // Mobile Editor
    await expect(page.getByText('Mobile Editor')).toBeVisible();
    await expect(page.getByText('Maya Cakes')).toBeVisible();
  });

  test('should open action sheet to edit a block', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();

    await page.getByPlaceholder('e.g. Acme Corp').fill('Test Store');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Retail');
    await page.getByText('Next: Choose Vibe').click();

    await page.getByText('Friendly').click();
    await page.getByText('Next: Details').click();

    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('Test description');

    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Initial Headline' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();

    // Wait for Draft Selection
    await expect(page.getByText('Pick your draft')).toBeVisible();
    await page.getByText('Draft 1').click();
    await page.getByText('Customize Selected Draft').click();

    // Wait for Editor
    await expect(page.getByText('Mobile Editor')).toBeVisible();

    // Click the block to edit
    await page.getByText('Initial Headline').click();

    // Action Sheet
    await expect(page.getByText('Edit Hero Block')).toBeVisible();

    // Edit the text
    const input = page.locator('input[type="text"]').first();
    await input.fill('Updated Headline');

    await page.getByText('Save Changes').click();

    // Verify block was updated
    await expect(page.getByText('Updated Headline')).toBeVisible();
  });

  test('should launch storefront', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByText('Selling Products').click();

    await page.getByPlaceholder('e.g. Acme Corp').fill('Launch Store');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Tech');
    await page.getByText('Next: Choose Vibe').click();

    await page.getByText('Professional').click();
    await page.getByText('Next: Details').click();

    await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming service/i).fill('Launching soon');

    await page.route('**/builder/generate', async route => {
      const json = {
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Launch Headline' } }
          ]
        }]
      };
      await route.fulfill({ json });
    });

    await page.getByText('Build Store').click();

    // Wait for Draft Selection
    await expect(page.getByText('Pick your draft')).toBeVisible();
    await page.getByText('Customize Selected Draft').click();

    // Intercept publish call
    await page.route('**/builder/publish_draft', async route => {
      const json = { domain: 'launch-store.ohc.page' };
      await route.fulfill({ json });
    });

    // Editor
    await expect(page.getByText('Mobile Editor')).toBeVisible();

    // Click 1-Tap Launch
    await page.getByText('1-Tap Launch').click();

    // Wait for Live screen
    await expect(page.getByText('You are Live! 🎉')).toBeVisible();
    await expect(page.getByText('launch-store.ohc.page')).toBeVisible();
  });
});
