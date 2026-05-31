import { expect, test } from './fixtures';

test('Maya navigates the Help Center', async ({ page }) => {
  // Step 1: Open the Help Center directly as if clicking the Help Center navigation item
  await page.goto('/help');

  // Verify Help Center loaded
  await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

  // Maya searches for "payment"
  const searchInput = page.getByPlaceholder('Search for help articles...');
  await expect(searchInput).toBeVisible();
  await searchInput.fill('payment');

  // Find the Getting Paid article, it should be visible based on search
  const paymentsLink = page.getByText('Getting Paid');
  await expect(paymentsLink).toBeVisible();

  // Ensure an unrelated article is filtered out
  const myStoreLink = page.getByText('Managing My Store');
  await expect(myStoreLink).toBeHidden();

  // Maya clicks the Getting Paid article
  await paymentsLink.click();

  // Verify the correct article loads
  await expect(page.getByRole('heading', { name: 'Getting Paid' })).toBeVisible();

  // Verify contents
  await expect(page.getByText('Connecting Your Bank Account')).toBeVisible();
  await expect(page.getByText('Viewing Your Deposits')).toBeVisible();

  // Navigate back to Help Center
  const backLink = page.getByRole('button', { name: /Back to Help Center/i });
  await backLink.click();

  // Verify back at Help Center
  await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
});
