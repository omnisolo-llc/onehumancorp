import { test, expect } from './fixtures';

test('funding engine mock dashboard UI check', async ({ page }) => {
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});

test('Funding Engine UI full flow', async ({ page }) => {
  await page.goto('/funding');

  // Verify shell loaded
  await expect(page.getByRole('heading', { name: 'Funding Engine' })).toBeVisible();

  // Wait for mock data
  await page.waitForTimeout(1500);

  // Verify notification banner
  await expect(page.getByText('New Funding Opportunity Found!')).toBeVisible();

  // Verify Opportunity Card
  await expect(page.getByText('Downtown Revitalization Grant')).toBeVisible();

  // Open Review Proposal
  await page.getByRole('button', { name: 'Review Proposal' }).click();

  // Verify Modal
  await expect(page.getByRole('heading', { name: 'Proposal Review' })).toBeVisible();

  // Submit Application
  // We mock window.alert in playwright using page.on('dialog')
  page.on('dialog', dialog => dialog.accept());
  await page.getByRole('button', { name: 'Submit Application' }).click();

  // Verify status changed to Submitted
  await expect(page.getByText('SUBMITTED')).toBeVisible();
});
