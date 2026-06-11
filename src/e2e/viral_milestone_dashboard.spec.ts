import { test, expect } from './fixtures';


test.describe('Dashboard Milestone Card UI', () => {
  test('should display 10th order milestone alert and card on Dashboard Page', async ({ page, loginAs, unlimitedAdminUser }) => {

    // Seed 10th order milestone for the tenant
    const tenantId = 'e2e-tenant'; // Assuming the user logs in as e2e-tenant
    try {
      await pool.query(
        `INSERT INTO business_milestones (id, tenant_id, milestone_type, reached_at)
         VALUES ($1, $2, $3, NOW()) ON CONFLICT (id) DO NOTHING`,
        ['ms_test_10th_order', tenantId, '10th_order']
      );
    } catch (e) {
    }

    // Login
    await loginAs(page, unlimitedAdminUser);

    // Navigate to the dashboard where the widget is embedded
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Verify milestone banner exists
    await expect(page.getByRole('heading', { name: /10th Order! Share your success/i })).toBeVisible();

    // Verify WhatsApp share button exists
    const whatsappLink = page.getByRole('link', { name: /Share to WhatsApp/i });
    await expect(whatsappLink).toBeVisible();

    const href = await whatsappLink.getAttribute('href');
    expect(href).toContain('wa.me');
    expect(href).toContain('10th%20order');

    // Verify the milestone card image is loaded
    const milestoneImage = page.locator('img[alt="10th Order Milestone"]');
    await expect(milestoneImage).toBeVisible();
    await expect(milestoneImage).toHaveAttribute('src', /milestone_id=10th_order/);
  });
});
