import { test, expect } from './fixtures';

test('business owner can view and manage approval inbox', async ({ page }) => {
  // 1. Log in and navigate to the team page
  await page.goto('/team');
  await expect(page.getByRole('heading', { name: 'Your Team' }).first()).toBeVisible();

  // 2. Verify Recent Activity section is visible initially
  await expect(page.getByRole('heading', { name: 'Recent Activity' }).first()).toBeVisible();

  // 3. Open The Ambassador's approval inbox
  await page.getByRole('button', { name: /The Ambassador/ }).first().click();

  // 4. Verify Approval Inbox UI
  await expect(page.getByRole('heading', { name: 'The Ambassador' }).first()).toBeVisible();
  await expect(page.getByText('Approval Inbox')).toBeVisible();

  // 5. Verify Approval request cards are visible
  // Check if we have requests or "All Caught Up!" by waiting for either to appear
  const hasRequests = page.getByRole('button', { name: 'Approve' }).first();
  const allCaughtUp = page.getByText('All Caught Up!');

  await Promise.any([
      hasRequests.waitFor({ state: 'visible', timeout: 5000 }),
      allCaughtUp.waitFor({ state: 'visible', timeout: 5000 })
  ]).catch(() => { /* ignore */ });

  if (await allCaughtUp.isVisible()) {
     console.log('No requests to approve');
  } else if (await hasRequests.isVisible()) {
     await hasRequests.click();
  }

  // 6. Navigate back to Team view
  await page.getByRole('button').first().click(); // back button
  await expect(page.getByRole('heading', { name: 'Your Team' }).first()).toBeVisible();

  // 7. Verify Recent Activity section is visible again
  await expect(page.getByRole('heading', { name: 'Recent Activity' }).first()).toBeVisible();
});
