import { test, expect } from '@playwright/test';

test.describe('API Documentation', () => {

  test.beforeEach(async ({ page }) => {
    // Mock the backend API responses required for the help center to load correctly
    await page.route('**/api/api-docs-spec', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
            "openapi": "3.0.0",
            "info": {
                "title": "OHC Advanced API Reference",
                "version": "1.0.0",
                "description": "API Reference for advanced users integrating with OneHumanCorp.",
            },
            "servers": [
                {
                    "url": "http://localhost:8080",
                }
            ],
            "paths": {
                "/api/orgs/register": {
                    "post": {
                        "summary": "Register a new organization"
                    }
                }
            }
        })
      });
    });
  });

  test('should load the Swagger UI on the api-docs page', async ({ page }) => {
    // Navigate to the API Docs page
    await page.goto('/api/ui/api-docs.html');

    // Check for the "Advanced" warning banner
    await expect(page.getByText('This section is for developers directly integrating')).toBeVisible({ timeout: 15000 });

    // Check for the Swagger UI container and some basic swagger elements
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });

    // Check if the title from our spec is rendered
    await expect(page.getByText('OHC Advanced API Reference')).toBeVisible();

    // Check if at least one of our API paths is rendered
    await expect(page.getByText('/api/orgs/register')).toBeVisible();
  });
});
