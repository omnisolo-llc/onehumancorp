import { test, expect } from './fixtures';

test.describe('Email Marketing Flow', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should navigate to login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should draft and send email broadcast', async ({ page }) => {
    await page.goto('/dashboard');
    // For Tauri components injected into or served as static pages, we might need a specific routing path depending on how Next is configured,
    // but the test environment configures /ui/ routes or serves files locally.
    // The link from the dashboard exists, so we click it.
    await page.goto('/api/ui/dashboard.html');
    await expect(page.getByRole('heading', { name: 'Welcome' }).first()).toBeVisible();

    // Click the new AI Email Broadcasts link
    await page.getByRole('link', { name: 'AI Email Broadcasts' }).click();

    // Wait for the heading to ensure the page loaded
    await expect(page.getByRole('heading', { name: 'Promoter Agent' })).toBeVisible();

    // Fill in the prompt
    await page.getByPlaceholder('e.g., Tell my customers about the new summer dress collection...').fill('Summer sale is here!');

    await page.getByRole('button', { name: 'Draft Email' }).click();

    // Because this uses a real LLM endpoint during test, the content can vary.
    // We check that the preview section becomes visible and contains non-empty text.
    const previewSubject = page.locator('#preview-subject');
    await expect(previewSubject).toBeVisible({ timeout: 30000 });

    // Check it has some content
    const text = await previewSubject.textContent();
    expect(text?.length).toBeGreaterThan(5);

    // Click Send
    await page.getByRole('button', { name: 'Send to All Customers' }).click();

    // Wait for success message
    await expect(page.locator('#success-message')).toBeVisible();
    await expect(page.locator('#success-message')).toHaveText('Broadcast sent successfully!');
  });
});

test.describe('Navigation', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('navigation', { name: 'Primary' })).toBeVisible();
    await page.getByRole('link', { name: 'Agents' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('should show welcome message on dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });
});
