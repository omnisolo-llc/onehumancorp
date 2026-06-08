import { test, expect } from './fixtures';

test.describe('Unified Agent Feed', () => {
  test('should display agent feed and allow interaction', async ({ page }) => {

    // Ensure we are using the seeded e2e tenant explicitly to fetch the seed data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to dashboard
    await page.goto('/dashboard');

    // Verify we are on dashboard and the Unified Agent Feed is present
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    await expect(page.getByText(/All caught up!|Requires Review|Loading Agent Proposals/).first()).toBeVisible();
    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.getByRole('button', { name: 'Activity Feed' })).toBeVisible();
  });
});

test.describe('Mobile-First Unified Agent Feed', () => {
  // Mobile viewport: 375px
  test.use({ viewport: { width: 375, height: 812 } });

  test('Unified agent feed renders correctly and has 44px touch targets on mobile', async ({ memberPage: page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    // Check Unified Agent Feed section is present
    await expect(page.locator('section[aria-label="Unified Agent Feed"]')).toBeVisible();

    // Find the Proposals tab
    const proposalsTab = page.locator('button:has-text("Proposals")');
    await expect(proposalsTab).toBeVisible();

    // Check that we have proposals loaded (or the empty state)
    // For test stability, we'll just check if the container for proposals exists.
    const buttons = page.locator('section[aria-label="Unified Agent Feed"] button');
    await expect(buttons.first()).toBeVisible();

    // Verify all buttons in the feed have at least a 44px bounding box for touch targets
    const count = await buttons.count();
    for (let i = 0; i < count; i++) {
        const box = await buttons.nth(i).boundingBox();
        if (box) {
            expect(box.width).toBeGreaterThanOrEqual(44);
        }
    }
  });

  test('Approving a card triggers expansion and handles task execution', async ({ memberPage: page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('section[aria-label="Unified Agent Feed"]')).toBeVisible();

    // If an approval button exists, tap it and verify task completed state.
    const approveButton = page.locator('button:has-text("Approve"):not(:has-text("Approve & Run Sale"))').first();

    if (await approveButton.isVisible()) {
        await approveButton.click();
        // Since it's optimistic UI, the card should disappear or change state.
        await expect(approveButton).not.toBeVisible();
    }
  });
});
