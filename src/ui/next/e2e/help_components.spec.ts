import { test, expect } from '@playwright/test';

test.describe('Help Components', () => {
  // Tests using the Help Center as a host page for the floating components
  test.beforeEach(async ({ page }) => {
    // E2E overrides NEXT_PUBLIC_E2E which hides components, so we need to inject/mock if needed
    // However, the components are hidden explicitly by `if (process.env.NEXT_PUBLIC_E2E === 'true') return null;`
    // The test environment might have this set. We will navigate to a page and check for components.

    // Instead of overriding env here, we will trust the components load in regular Next.js dev server if E2E is false
    // or test the components directly if they are conditionally hidden. For now we will just verify the help page loads.
  });

  test('Help Center page loads with articles', async ({ page }) => {
    await page.goto('/help');

    // Wait for at least one article title to appear
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();
    await expect(page.locator('text=Getting Started')).toBeVisible();
  });

  test('Contextual Tooltip triggers correctly', async ({ page }) => {
    // This requires a component that uses WithTooltip on the page.
    // We can test the /pricing page which has one.
    await page.goto('/pricing');

    // Hover over the pricing tier heading to trigger the tooltip
    // In pricing/page.tsx: <WithTooltip id="pricing-tier-tooltip" defaultText="..."> <h1 ...>Pricing Plans</h1> </WithTooltip>
    const target = page.locator('h1:has-text("Pricing Plans")');
    await expect(target).toBeVisible();

    // Trigger the hover
    await target.hover();

    // Verify the tooltip text is visible
    const tooltipText = page.locator('text=Select the plan that best fits your business needs.');
    await expect(tooltipText).toBeVisible();
  });

  test('Interactive Walkthrough functions correctly on dashboard', async ({ page }) => {
    await page.goto('/dashboard?test_walkthrough=true');

    const startTourBtn = page.locator('button:has-text("Start Tour")');
    await expect(startTourBtn).toBeVisible();
    await startTourBtn.click();

    // Verify the first walkthrough step appears
    const firstStepTitle = page.getByRole('dialog').getByText('Business Analytics');
    await expect(firstStepTitle).toBeVisible();

    // Advance to the next step
    const nextBtn = page.locator('button:has-text("Next")');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    // Verify the second walkthrough step appears
    const secondStepTitle = page.getByRole('dialog').getByText('Operations Map');
    await expect(secondStepTitle).toBeVisible();

    // Finish the walkthrough
    const finishBtn = page.locator('button:has-text("Finish")');
    await expect(finishBtn).toBeVisible();
    await finishBtn.click();

    // Verify the walkthrough bubble is no longer visible
    await expect(secondStepTitle).not.toBeVisible();
  });
});
