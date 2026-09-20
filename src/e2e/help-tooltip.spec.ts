import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Help Tooltips', () => {
  test('should show tooltip on hover', async ({ page }) => {
    await adminPage(page);
    await page.goto('/dashboard.html');

    // Hover an element with data-tooltip
    const tooltipTrigger = page.locator('#generate-link-btn');
    await tooltipTrigger.waitFor({ state: 'visible' });
    await tooltipTrigger.hover();

    // Wait for the tooltip element to become visible
    const tooltip = page.locator('.omnisolo-tooltip.visible');
    await expect(tooltip).toBeVisible();

    // Check that it contains text
    await expect(tooltip).toContainText('Click here to share access with a team member.');

    // Move away to hide it
    await page.mouse.move(0, 0);
    await expect(tooltip).not.toBeVisible();
  });

  test('should show tooltip when hovering over nested element', async ({ page }) => {
    await adminPage(page);
    await page.goto('/dashboard.html');

    // Inject a deeply nested element inside something we know will trigger tooltips,
    // or just append a test structure to the body.
    await page.evaluate(() => {
        const container = document.createElement('div');
        container.setAttribute('data-tooltip', 'Nested tooltip text');
        container.id = 'test-nested-tooltip-container';

        const child1 = document.createElement('div');
        child1.id = 'test-nested-child-1';

        const child2 = document.createElement('span');
        child2.id = 'test-nested-child-2';
        child2.textContent = 'Hover me';
        child2.style.padding = '20px';
        child2.style.display = 'block';

        child1.appendChild(child2);
        container.appendChild(child1);
        document.body.appendChild(container);
    });

    const target = page.locator('#test-nested-child-2');
    await target.hover();

    const tooltip = page.locator('.omnisolo-tooltip.visible');
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toContainText('Nested tooltip text');
  });
});
