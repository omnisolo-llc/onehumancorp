import { test, expect } from '@playwright/test';

test.describe('Universal Autonomous Staff Management & Local Coordination Mesh', () => {
    test.beforeEach(async ({ page }) => {
        // Authenticate as Jun (Location Manager / Owner)
        await page.goto('/login');
        await page.fill('input[name="email"]', 'manager@test.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');
        await expect(page).toHaveURL('/staff');
    });

    test('Staff Task Feed - AI Task Prioritization & Translucent Glassmorphism UI', async ({ page }) => {
        await page.goto('/staff');

        // Check for Translucent Glassmorphism material classes
        const mainCard = page.locator('.bg-white\\/65'); // Test for rgba(255, 255, 255, 0.65) equivalent or generic translucent layer
        await expect(mainCard.first()).toBeVisible();

        // 1. Check AI Task generation
        const taskCards = page.locator('[data-testid="staff-task-card"]');
        await expect(taskCards).toHaveCount(0); // Assuming starts empty

        // Emulate Operations Agent creating a task via API directly for testing (or UI interaction if supported)
        await page.request.post('/api/staff/tasks', {
            data: {
                description: 'Prepare 15 Falafels',
                priority: 'high',
                staff_id: 'staff_1'
            }
        });

        await page.reload();

        // Verify task appears in feed
        await expect(taskCards.first()).toContainText('Prepare 15 Falafels');
        await expect(taskCards.first()).toContainText('high');
    });

    test('Escalation & Low Supply Intent', async ({ page }) => {
        await page.goto('/staff');

        // Tap "Report Issue / Escalation"
        await page.click('[data-testid="escalate-issue-btn"]');
        await page.fill('[data-testid="escalation-input"]', 'Low on Cups');
        await page.click('[data-testid="submit-escalation-btn"]');

        // Verify optimistic UI update
        const alertCard = page.locator('[data-testid="escalation-card"]');
        await expect(alertCard.first()).toContainText('Low on Cups');
        await expect(alertCard.first()).toContainText('pending');
    });

    test('Offline-First Task Completion', async ({ page }) => {
        await page.request.post('/api/staff/tasks', {
            data: {
                description: 'Restock napkin dispensers',
                priority: 'normal',
                staff_id: 'staff_1'
            }
        });

        await page.goto('/staff');
        const taskCards = page.locator('[data-testid="staff-task-card"]');
        await expect(taskCards.locator('text="Restock napkin dispensers"')).toBeVisible();

        // Go offline
        await page.context().setOffline(true);

        // Mark task complete
        const completeBtn = taskCards.locator('text="Restock napkin dispensers"').locator('..').locator('[data-testid="mark-complete-btn"]');
        await completeBtn.click();

        // Verify UI updates optimistically to completed state even while offline
        await expect(taskCards.locator('text="Restock napkin dispensers"').locator('..')).toContainText('completed');

        // Go back online and verify sync (would ideally check network requests or wait for sync indicator)
        await page.context().setOffline(false);
        await page.reload();
        await expect(taskCards.locator('text="Restock napkin dispensers"').locator('..')).toContainText('completed');
    });

    test('Manager View - End of Shift Summary', async ({ page }) => {
        await page.goto('/staff');

        // Click to view Shift Summaries tab/view
        await page.click('[data-testid="view-summaries-tab"]');

        const summaryCards = page.locator('[data-testid="shift-summary-card"]');

        // Assuming the DB is seeded or the Operations Agent ran a CRON job during tests
        await expect(summaryCards.first()).toBeVisible();
        await expect(summaryCards.first()).toContainText('Shift Performance');
    });
});
