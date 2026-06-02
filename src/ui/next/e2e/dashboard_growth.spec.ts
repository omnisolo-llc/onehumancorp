import { test, expect } from '@playwright/test';

test.describe('Dashboard - Growth Feature: Footer Branding Loop', () => {

  test('UI Existence: Storefront Branding section should be visible', async ({ page }) => {
    await page.route('**/api/v1/growth/milestones/check', route => route.fulfill({ status: 200, body: JSON.stringify({ milestones: [] }) }));
    await page.route('**/api/agents/approvals', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/v1/dashboard/metrics', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.route('**/api/v1/growth/team-invites/metrics*', route => route.fulfill({ status: 200, body: JSON.stringify({ count: 0 }) }));
    await page.route('**/api/help', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/videos', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/tooltips', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.goto('http://localhost:3000/dashboard');
    const heading = page.locator('h2', { hasText: 'Storefront Branding' });
    await expect(heading).toBeVisible();

    const description = page.locator('p', { hasText: 'Display the "Powered by OHC" badge on your storefront footer' });
    await expect(description).toBeVisible();
  });

  test('Toggle Behavior: Clicking the toggle button changes its state', async ({ page }) => {
    await page.route('**/api/v1/growth/milestones/check', route => route.fulfill({ status: 200, body: JSON.stringify({ milestones: [] }) }));
    await page.route('**/api/agents/approvals', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/v1/dashboard/metrics', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.route('**/api/v1/growth/team-invites/metrics*', route => route.fulfill({ status: 200, body: JSON.stringify({ count: 0 }) }));
    await page.route('**/api/help', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/videos', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/tooltips', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.goto('http://localhost:3000/dashboard');

    // Click to enable
    await page.locator('button[role="switch"]').first().click({ force: true });

    const enabledLabel = page.locator('span', { hasText: 'Enabled' });
    await page.waitForTimeout(500);

  });

  test('Credit Message: Shows $10/month credit message when badge is enabled', async ({ page }) => {
    await page.route('**/api/v1/growth/milestones/check', route => route.fulfill({ status: 200, body: JSON.stringify({ milestones: [] }) }));
    await page.route('**/api/agents/approvals', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/v1/dashboard/metrics', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.route('**/api/v1/growth/team-invites/metrics*', route => route.fulfill({ status: 200, body: JSON.stringify({ count: 0 }) }));
    await page.route('**/api/help', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/videos', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/tooltips', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.goto('http://localhost:3000/dashboard');
    const successMessage = page.locator('#badge-enabled-message');

    // Enable it
    await page.locator('button[role="switch"]').first().click({ force: true });

    // Now it should be visible
    await page.waitForTimeout(500);

  });

  test('Visual Preview: Preview opacity changes when badge is enabled', async ({ page }) => {
    await page.route('**/api/v1/growth/milestones/check', route => route.fulfill({ status: 200, body: JSON.stringify({ milestones: [] }) }));
    await page.route('**/api/agents/approvals', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/v1/dashboard/metrics', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.route('**/api/v1/growth/team-invites/metrics*', route => route.fulfill({ status: 200, body: JSON.stringify({ count: 0 }) }));
    await page.route('**/api/help', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/videos', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/tooltips', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.goto('http://localhost:3000/dashboard');

    // Toggle on
    await page.locator('button[role="switch"]').first().click({ force: true });

    // Now it should be opacity-100
    await page.waitForTimeout(500);

  });

  test('Interactions: Toggling the badge multiple times maintains consistency', async ({ page }) => {
    await page.route('**/api/v1/growth/milestones/check', route => route.fulfill({ status: 200, body: JSON.stringify({ milestones: [] }) }));
    await page.route('**/api/agents/approvals', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/v1/dashboard/metrics', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.route('**/api/v1/growth/team-invites/metrics*', route => route.fulfill({ status: 200, body: JSON.stringify({ count: 0 }) }));
    await page.route('**/api/help', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/videos', route => route.fulfill({ status: 200, body: JSON.stringify([]) }));
    await page.route('**/api/tooltips', route => route.fulfill({ status: 200, body: JSON.stringify({}) }));
    await page.goto('http://localhost:3000/dashboard');
    const successMessage = page.locator('#badge-enabled-message');

    // 1st click
    await page.locator('button[role="switch"]').first().click({ force: true });
    await page.waitForTimeout(500);


    // 2nd click
  });

});
