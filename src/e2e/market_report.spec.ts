import { test, expect } from './fixtures';

test.describe('Market Dynamics Report', () => {
  test('CUJ: Business owner navigates to Market Dynamics Report and views insights', async ({ page }) => {
    // 1. Navigate to the dashboard
    await page.goto('/dashboard');

    // 2. Click the Market Report link in the header navigation
    const marketReportLink = page.getByRole('link', { name: 'Market Report' });
    await expect(marketReportLink).toBeVisible();
    await marketReportLink.click();

    // 3. Verify successful navigation
    await expect(page).toHaveURL(/.*\/research\/ohc-market-report/);

    // 4. Verify main heading
    await expect(page.getByRole('heading', { name: 'OHC Market Dynamics & Competitor Deep-Dive' })).toBeVisible();

    // 5. Verify Overview tab content is displayed by default
    await expect(page.getByText('Invisible AI')).toBeVisible();

    // 6. Navigate to Competitors tab
    const mappingButton = page.getByRole('button', { name: 'Market Mapping' });
    await mappingButton.click();
    await expect(page.getByRole('heading', { name: 'Top 10 Traditional Platforms' })).toBeVisible();
    await expect(page.getByText('Shopify')).toBeVisible();

    // 7. Navigate to Gap Analysis tab
    const gapsButton = page.getByRole('button', { name: 'Gap Analysis' });
    await gapsButton.click();
    await expect(page.getByRole('heading', { name: 'Gap Matrix: Shopify vs. OHC' })).toBeVisible();
    await expect(page.getByText('Omnichannel Sync Nightmare')).toBeVisible();

    // 8. Navigate to Agentic Solutions tab
    const solutionsButton = page.getByRole('button', { name: 'Agentic Solutions' });
    await solutionsButton.click();
    await expect(page.getByRole('heading', { name: 'Agentic Solutions for Market Gaps' })).toBeVisible();
    await expect(page.getByText('Invisible Local Delivery & Inventory Mesh')).toBeVisible();

    // 9. Navigate back to dashboard using breadcrumb
    const dashboardBreadcrumb = page.getByRole('link', { name: 'Dashboard' }).first();
    await dashboardBreadcrumb.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});