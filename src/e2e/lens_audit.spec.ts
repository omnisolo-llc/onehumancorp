import { test, expect } from './fixtures';

test.describe('Lens Audit Gold Standard Verification', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('1. verify premium design tokens and glassmorphism', async ({ page }) => {
    // Verify primary blue accent
    const dashboardTitle = page.getByRole('heading', { name: 'Overview' });
    await expect(dashboardTitle).toBeVisible();

    // Check computed styles for glassmorphism
    const glassCard = page.locator('.card.glass').first();
    const styles = await glassCard.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return {
            backdropFilter: style.backdropFilter || style.webkitBackdropFilter,
            borderRadius: style.borderRadius
        };
    });

    expect(styles.backdropFilter).toContain('blur(30px)');
    expect(styles.borderRadius).toBe('16px');
  });

  test('2. verify grandmother test compliance (plain language labels)', async ({ page }) => {
    // Check top nav
    await expect(page.getByText('Overview').first()).toBeVisible();
    await expect(page.getByText('Assistants').first()).toBeVisible();
    await expect(page.getByText('Quick Setup').first()).toBeVisible();
    await expect(page.getByText('Connect Apps').first()).toBeVisible();

    // Technical jargon should not be visible to grandmother by default
    const bodyText = await page.innerText('body');
    expect(bodyText).not.toContain('API Server');
    expect(bodyText).not.toContain('gRPC');
    expect(bodyText).not.toContain('Redis');
  });

  test('3. verify data truth cycle (UI -> DB -> UI)', async ({ page }) => {
    // Start at dashboard
    await expect(page.locator('#snapshot-orders')).toContainText('Orders: 0');

    // Trigger mutation
    await page.getByRole('button', { name: 'Mark Order Ready' }).click();

    // Verify UI reflects "Data Truth" from backend simulated response
    await expect(page.locator('#snapshot-orders')).toContainText('Orders: 1');
    await expect(page.locator('#dashboard-total-sales')).not.toHaveText('$0.00');
  });

  test('4. verify developer mode toggle and technical screen masking', async ({ page }) => {
    await page.goto('/settings');
    const devToggle = page.locator('#dev-mode-toggle');
    await expect(devToggle).not.toBeChecked();

    // Check that technical screen is blurred/masked
    await page.goto('/diagnostics');
    const diagnosticsScreen = page.locator('#diagnostics-screen');
    const filter = await diagnosticsScreen.evaluate(el => window.getComputedStyle(el).filter);
    expect(filter).toContain('blur(4px)');

    // Enable Dev Mode
    await page.goto('/settings');
    await devToggle.check();

    // Check diagnostics again
    await page.goto('/diagnostics');
    const filterAfter = await diagnosticsScreen.evaluate(el => window.getComputedStyle(el).filter);
    expect(filterAfter).toBe('none');
  });

  test('5. verify mobile navigation active state synchronization', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    const messagesBtn = page.locator('#mobile-bottom-nav').get_by_role('button', { name: 'Messages' });
    await messagesBtn.click();

    // Check active class
    await expect(messagesBtn).toHaveClass(/active/);

    const homeBtn = page.locator('#mobile-bottom-nav').get_by_role('button', { name: 'Home' });
    await homeBtn.click();
    await expect(homeBtn).toHaveClass(/active/);
    await expect(messagesBtn).not.toHaveClass(/active/);
  });
});
