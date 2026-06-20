import { test, expect } from '@playwright/test';

test.describe('Client Intake and Proposal Draft E2E', () => {
  test('Submitting intake form generates proposal draft and can be approved in feed', async ({ page }) => {
    // Navigate to feed and dismiss existing items to have a clean slate
    await page.goto('/feed');
    // Wait for the feed to load
    await page.waitForLoadState('networkidle');

    // Simulate Client Intake webhook payload hitting the Next.js API route
    const formData = new URLSearchParams();
    formData.append('name', 'Nora Agency');
    formData.append('email', 'nora@example.com');
    formData.append('details', 'Need a custom logo design and branding package');

    const res = await page.request.post('/api/agents/client-intake', {
      data: formData.toString(),
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      }
    });

    expect(res.ok()).toBeTruthy();
    const data = await res.json();
    expect(data.success).toBe(true);

    // Refresh feed to see the newly drafted proposal
    await page.reload();
    await page.waitForLoadState('networkidle');

    // Check that the new Proposal Draft card appears
    await expect(page.locator('text=Proposal Draft Ready')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Nora Agency')).toBeVisible();

    // Verify Quick Action "Quick Edit Draft" and "Add Rush Fee" are visible
    await expect(page.locator('text=Quick Edit Draft')).toBeVisible();
    await expect(page.locator('text=+ Add Rush Fee')).toBeVisible();

    // Approve the proposal
    const approveBtn = page.locator('button:has-text("Approve & Send")').first();
    await approveBtn.click();

    // The item should disappear from the feed after approval
    await expect(page.locator('text=Nora Agency')).toBeHidden({ timeout: 10000 });
  });
});
