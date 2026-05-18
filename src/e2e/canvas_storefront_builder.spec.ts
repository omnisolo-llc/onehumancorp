import { test, expect } from './fixtures';

test.describe('Canvas Storefront Builder Full E2E', () => {

    test.use({ viewport: { width: 375, height: 667 } });

    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
        await page.click('text="Edit Website"');
    });

    test('Flow 1: Navigate and Verify Default Blocks', async ({ page }) => {
        await expect(page.locator('text="My Awesome Store"')).toBeVisible();
        await expect(page.locator('text="Featured Products (4 items)"')).toBeVisible();
        await expect(page.locator('text="Our Services"')).toBeVisible();
        await expect(page.locator('text="Best service ever! - Happy Customer"')).toBeVisible();
    });

    test('Flow 2: Open Bottom Sheet via Block Click', async ({ page }) => {
        // Click the Hero block to edit
        await page.click('text="My Awesome Store"');

        // Bottom sheet should open
        await expect(page.locator('#sheet-title', { hasText: 'Edit Hero' })).toBeVisible();
        await expect(page.locator('input#edit-title')).toHaveValue('My Awesome Store');
    });

    test('Flow 3: Edit Block and Optimistic Update', async ({ page }) => {
        // Click Hero block
        await page.click('text="My Awesome Store"');

        // Edit title in bottom sheet
        await page.fill('input#edit-title', 'Maya Bakery');
        await page.click('text="Save"');

        // Bottom sheet should close and preview should update optimistically
        await expect(page.locator('text="Maya Bakery"')).toBeVisible();
        await expect(page.locator('text="My Awesome Store"')).not.toBeVisible();
    });

    test('Flow 4: Toggle Rearrange Mode', async ({ page }) => {
        // Toggle Rearrange
        await page.click('button#toggle-rearrange-btn');

        // Drag hints should be visible
        await expect(page.locator('text="↕ Drag to reorder"').first()).toBeVisible();
        await expect(page.locator('button:has-text("↑")').first()).toBeVisible();

        // Check if bottom sheet is blocked in rearrange mode
        await page.locator('.builder-block').first().click();
        await expect(page.locator('#block-editor-sheet')).not.toHaveClass(/open/);
    });

    test('Flow 5: Publish Changes and Confetti', async ({ page }) => {
        // Click Publish Changes FAB
        await page.click('button.fab:has-text("Publish Changes")');

        // Domain setup sheet should open
        await expect(page.locator('h2:has-text("Publish Site")')).toBeVisible();

        // Select free subdomain
        await page.click('button:has-text("Free OHC Subdomain")');
        await page.fill('input#free-domain-input', 'mayabakery');

        // Publish
        await page.click('button:has-text("Publish")');

        // Should route back to Dashboard after confetti finishes (timeout handles the delay)
        await expect(page.locator('text="Welcome back, Human."')).toBeVisible({ timeout: 5000 });
    });
});
