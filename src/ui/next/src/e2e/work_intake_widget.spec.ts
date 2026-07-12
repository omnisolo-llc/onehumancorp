import { test, expect } from '../../../../e2e/fixtures';

test.describe('Embeddable Work-Intake Widget Growth Loop', () => {
    test('dashboard shows the embed work intake widget and generates correct HTML', async ({ page }) => {
        // Go to dashboard
        await page.goto('/dashboard');

        // Look for the "Work-Intake Widget" link in the Dashboard Growth & Virality section
        const widgetLink = page.locator('text=Work-Intake Widget');
        await expect(widgetLink).toBeVisible();
        await widgetLink.click();

        // Should now be on the Work-Intake Widget page
        const sectionHeader = page.getByRole('heading', { name: /Work-Intake Widget/ });
        await expect(sectionHeader).toBeVisible();

        // Check for the "Lead Capture Loop" badge next to the header
        await expect(page.locator('text=Lead Capture Loop').first()).toBeVisible();

        // Set the form title
        const titleInput = page.getByPlaceholder('e.g. Work Request');
        await titleInput.fill('Book an Appointment');

        // Click "Get Widget Code" button
        const getWidgetBtn = page.locator('button:has-text("Get Widget Code")');
        await expect(getWidgetBtn).toBeVisible();
        await getWidgetBtn.click();

        // Modal should appear
        const modalHeader = page.locator('h2:has-text("Embed Work-Intake Widget")');
        await expect(modalHeader).toBeVisible();

        // The textarea should contain the iframe snippet
        const textarea = page.locator('textarea').filter({ hasText: '<iframe src="https://ohc.app/api/v1/growth/work-intake/embed' });
        await expect(textarea).toBeVisible();

        // Verify the HTML snippet structure includes the custom title encoded
        const snippet = await textarea.inputValue();
        expect(snippet).toContain('Book%20an%20Appointment');
        expect(snippet).toContain('theme=light');
        expect(snippet).toContain('width="320"');
        expect(snippet).toContain('height="400"');
        expect(snippet).toContain('frameborder="0"');

        await page.waitForTimeout(500);
    });

    test('embed API endpoint returns the work intake HTML and submit works', async ({ request }) => {
        // Test GET endpoint
        const response = await request.get('/api/v1/growth/work-intake/embed?tenant=my-business&theme=light&title=TestRequest');
        expect(response.ok()).toBeTruthy();

        const html = await response.text();

        // Assert the HTML contains the correct structure and elements
        expect(html).toContain('<!DOCTYPE html>');
        expect(html).toContain('TestRequest');
        expect(html).toContain('Send Request');

        // Ensure the referral growth loop is intact in the footer
        expect(html).toContain('Powered by');
        expect(html).toContain('OHC');
        expect(html).toContain('/onboarding?ref=my-business');

        // Test POST submit endpoint
        const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=my-business', {
           data: {
             name: 'Playwright Test',
             email: 'test@example.com',
             details: 'Test details'
           },
           headers: {
             'Content-Type': 'application/x-www-form-urlencoded'
           }
        });

        expect(submitResponse.ok()).toBeTruthy();

        const submitHtml = await submitResponse.text();
        expect(submitHtml).toContain('Request Received!');
        expect(submitHtml).toContain('Thanks, Playwright Test!');

        // Confirm viral loop is still present on success screen
        expect(submitHtml).toContain('Powered by');
        expect(submitHtml).toContain('OHC');
    });
});
