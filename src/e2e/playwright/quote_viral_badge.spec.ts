import { test, expect } from '../fixtures';
import { createOwnerQuote } from './quote_fixture';

test.describe('Quote Viral Badge E2E', () => {
  test('customer quote shows the referral badge with the actual tenant attribution', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, {
      description: 'Referral quote service',
    });
    // The referral badge belongs to the standalone customer quote, not the
    // maintained dashboard's unrelated triage card or owner edit screen.
    await page.goto(`/ui/quote.html?id=${quoteId}&tenant=${encodeURIComponent(adminUser.organizationId)}`);
    await expect(page.locator('#line-items-container')).toContainText('Referral quote service');
    await expect(page.locator('#quote-total')).toHaveText('$50.00');
    const badge = page.getByTestId('viral-quote-badge');
    await expect(badge).toBeVisible();
    await expect(badge).toContainText('Run your business like this with AI assistant.');
    const referral = badge.getByRole('link');
    await expect(referral).toHaveAttribute('href', `/setup.html?ref=${encodeURIComponent(adminUser.organizationId)}&source=quote_viewer`);
    await referral.click();
    await page.waitForURL((url) =>
      url.pathname === '/setup.html'
      && url.searchParams.get('ref') === adminUser.organizationId
      && url.searchParams.get('source') === 'quote_viewer');
    await expect(page).toHaveTitle(/Setup/);
  });
});
