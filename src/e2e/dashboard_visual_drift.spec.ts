import { test, expect } from './fixtures';

test.describe('Dashboard Visual Drift & Layout Audit', () => {
    test.use({ viewport: { width: 1440, height: 900 } });

    test('1. Verify no hardcoded mock Stripe data banner exists', async ({ page, seedDatabase }) => {
        await seedDatabase();
        await page.goto('/dashboard');
        await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

        // Assert Stripe Setup is removed to ensure no mock data
        await expect(page.locator('text=Connect Stripe to accept payments')).toHaveCount(0);
        await expect(page.locator('button:has-text("Complete Stripe Setup")')).toHaveCount(0);
    });

    test('2. Verify Team Activity is rendered independently of approvals', async ({ page, seedDatabase }) => {
        await seedDatabase();
        await page.goto('/dashboard');

        // Ensure the Team Activity header is always present, meaning it's not nested inside conditional blocks that hide it
        await expect(page.getByRole('heading', { name: 'Team Activity' })).toBeVisible({ timeout: 15000 });
    });

    test('3. Verify Swarm Online indicator is visible with correct aesthetics', async ({ page, seedDatabase }) => {
        await seedDatabase();
        await page.goto('/dashboard');

        const swarmOnline = page.locator('text=Swarm Online');
        await expect(swarmOnline).toBeVisible({ timeout: 15000 });

        // Verify it contains the pulse indicator element
        const pulseIndicator = page.locator('.animate-pulse').first();
        await expect(pulseIndicator).toBeVisible();
    });

    test('4. Verify Advanced Settings toggle is present in Action Required section (when approvals exist)', async ({ page, seedDatabase }) => {
        await seedDatabase();
        await page.goto('/dashboard');

        // Verify approvals section is visible (e2e db should have approvals)
        await expect(page.getByRole('heading', { name: 'Action Required' })).toBeVisible({ timeout: 15000 });

        // Verify advanced settings text and toggle exist next to Action Required
        await expect(page.locator('text=Advanced Settings')).toBeVisible();
        await expect(page.locator('button').filter({ has: page.locator('.translate-x-0, .translate-x-4') }).first()).toBeVisible();
    });

    test('5. Verify Action Required layout structure with Approval/Reject buttons', async ({ page, seedDatabase }) => {
        await seedDatabase();
        await page.goto('/dashboard');

        await expect(page.getByRole('heading', { name: 'Action Required' })).toBeVisible({ timeout: 15000 });

        // Ensure there are Reject and Approve buttons
        await expect(page.locator('button:has-text("Reject")').first()).toBeVisible();
        await expect(page.locator('button:has-text("Approve")').first()).toBeVisible();
    });
});
