import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Cost Transparency and Pricing Dashboard', () => {
    test('displays the My Plan section with usage metrics', async ({ adminPage }) => {
        // Navigate to the pricing page
        await adminPage.goto('/ui/pricing.html');

        // Wait for the My Plan section to become visible
        const myPlanSection = adminPage.locator('#my-plan-section');
        await expect(myPlanSection).toBeVisible({ timeout: 10000 });

        // Verify the plan name is displayed
        const planName = adminPage.locator('#current-plan-name');
        await expect(planName).not.toBeEmpty();

        // Verify AI actions used is displayed
        const aiActions = adminPage.locator('#my-plan-ai-used');
        await expect(aiActions).not.toBeEmpty();
        await expect(aiActions).toContainText('/');

        // Verify Storage used is displayed
        const storageUsed = adminPage.locator('#my-plan-storage-used');
        await expect(storageUsed).not.toBeEmpty();
        await expect(storageUsed).toContainText('/');

        // Verify Estimated Next Bill is displayed
        const nextBill = adminPage.locator('#my-plan-next-bill');
        await expect(nextBill).not.toBeEmpty();
        await expect(nextBill).toContainText('$');

        // Verify Manage Plan button exists
        const managePlanBtn = adminPage.locator('#manage-plan-btn');
        await expect(managePlanBtn).toBeVisible();
    });

    test('dynamically updates pricing buttons based on current plan', async ({ adminPage }) => {
        await adminPage.goto('/ui/pricing.html');

        // Wait for the My Plan section to become visible
        await expect(adminPage.locator('#my-plan-section')).toBeVisible({ timeout: 10000 });

        // Get the current plan name
        const currentPlan = await adminPage.locator('#current-plan-name').textContent();

        // Ensure the current plan button is disabled
        if (currentPlan && ['Free', 'Starter', 'Pro', 'Business'].includes(currentPlan)) {
            const currentPlanBtn = adminPage.locator(`#btn-${currentPlan}`);
            await expect(currentPlanBtn).toBeDisabled();
            await expect(currentPlanBtn).toHaveText('Current Plan');
        }
    });

    test('navigates to billing portal when manage plan is clicked', async ({ adminPage }) => {
        // We mock the API response for billing portal to avoid actual redirection in tests
        await adminPage.route('/api/billing/create-billing-portal-session', async route => {
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ url: 'https://mock-billing-portal.com' })
            });
        });

        await adminPage.goto('/ui/pricing.html');
        await expect(adminPage.locator('#my-plan-section')).toBeVisible({ timeout: 10000 });

        const managePlanBtn = adminPage.locator('#manage-plan-btn');
        await managePlanBtn.click();

        // Check if error message is not shown (success mock)
        const errorMsg = adminPage.locator('#error-message');
        await expect(errorMsg).toBeEmpty();
    });

    test('displays error when billing portal session fails', async ({ adminPage }) => {
        // Mock a failure response
        await adminPage.route('/api/billing/create-billing-portal-session', async route => {
            await route.fulfill({
                status: 500,
                contentType: 'application/json',
                body: JSON.stringify({ error: 'Internal Server Error' })
            });
        });

        await adminPage.goto('/ui/pricing.html');
        await expect(adminPage.locator('#my-plan-section')).toBeVisible({ timeout: 10000 });

        const managePlanBtn = adminPage.locator('#manage-plan-btn');
        await managePlanBtn.click();

        // Verify the error message is displayed
        const errorMsg = adminPage.locator('#error-message');
        await expect(errorMsg).toContainText('Failed to launch billing portal.');
    });

    test('cost dashboard displays My Plan correctly', async ({ adminPage }) => {
        await adminPage.goto('/ui/cost-dashboard.html');

        // Verify that My Plan is visible
        const myPlanWidget = adminPage.locator('#my-plan-widget');
        await expect(myPlanWidget).toBeVisible({ timeout: 10000 });

        // Verify plan name
        const planName = adminPage.locator('#my-plan-name');
        await expect(planName).not.toBeEmpty();

        // Verify AI Actions text
        const aiActions = adminPage.locator('#ai-actions-text');
        await expect(aiActions).not.toBeEmpty();
        await expect(aiActions).toContainText('/');

        // Verify Storage text
        const storageText = adminPage.locator('#storage-text');
        await expect(storageText).not.toBeEmpty();
        await expect(storageText).toContainText('/');
    });
});
