import { test, expect } from './fixtures';

test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display dashboard with nav', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show dashboard link in nav', async ({ page }) => {
    const dashLink = page.locator('nav a:has-text("Dashboard")');
    await expect(dashLink).toBeVisible();
  });

  test('should show agents link in nav', async ({ page }) => {
    const agentsLink = page.locator('nav a:has-text("Agents")');
    await expect(agentsLink).toBeVisible();
  });

  test('should show setup link in nav', async ({ page }) => {
    const setupLink = page.locator('nav a:has-text("Setup")');
    await expect(setupLink).toBeVisible();
  });

  test('should display welcome message', async ({ page }) => {
    await expect(page.locator('text=Welcome back')).toBeVisible();
  });

  test('should display agents working message', async ({ page }) => {
    await expect(page.locator('text=Your agents are working on your behalf')).toBeVisible();
  });
});

test.describe('Login Page', () => {
  test('should display login form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });
});

test.describe('Agents Page', () => {
  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });
});

test.describe('Business Setup Page', () => {
  test('should display setup page', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.getByRole('heading', { name: 'OneHuman' })).toBeVisible();
  });

  test('should show setup wizard text', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
  });
});

test.describe('Dashboard', () => {
  test('should have working nav links', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });
});
test.describe('Documentation Features', () => {
  test('Help Center page contains required topics', async ({ page }) => {
    await page.goto('/help');

    // Check main topics
    await expect(page.getByRole('heading', { name: 'Getting Started' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'My Store' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Payments' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'AI Agents' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Marketing' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Account & Billing' })).toBeVisible();

    // Check a topic page works
    await page.getByRole('heading', { name: 'Payments' }).click();
    await expect(page.locator('h1').filter({ hasText: 'Payments' })).toBeVisible();
    await expect(page.locator('text=Getting paid is the best part of running a business.')).toBeVisible();
  });

  test('AI Help Chat is accessible and functional', async ({ page }) => {
    await page.goto('/dashboard');

    // Open Help Widget to check if chat tab works
    // In E2E, HelpChat component is disabled, so we rely on the internal chat tab in the Help Widget instead of floating chat.
    const helpWidgetButton = page.locator('button', { hasText: '?' }).first();
    await helpWidgetButton.click({ timeout: 5000, force: true });

    // Go to Chat tab
    // Look for the exact tab based on the component's structure in help.tsx where it renders { id: "chat", label: "Ask AI" }
    // We use a broader approach for robust finding of the tab
    await page.evaluate(() => {
        Array.from(document.querySelectorAll('button')).find(el => el.textContent === 'Ask AI')?.click();
    });

    // Send a message
    const input = page.getByPlaceholder('Ask anything...');
    await input.fill('How do I setup my store?');
    await input.press('Enter');

    // Verify response
    await expect(page.locator('text=I am your AI Help Agent!')).toBeVisible();
    await expect(page.locator('a:has-text("Read the full article →")')).toBeVisible();
  });

  test('Help Widget contains Walkthroughs and Tours', async ({ page }) => {
    await page.goto('/dashboard');

    // Open Help Widget
    const helpWidgetButton = page.locator('button', { hasText: '?' }).first();
    await helpWidgetButton.click({ timeout: 5000, force: true });

    // Check tabs are visible
    await expect(page.getByRole('button', { name: 'Tours' })).toBeVisible();

    // Use evaluate for stability in the E2E environment
    await page.evaluate(() => {
        Array.from(document.querySelectorAll('button')).find(el => el.textContent === 'Tours')?.click();
    });

    // Check specific walkthroughs
    await expect(page.locator('text=Tour: Set up your store')).toBeVisible();
    await expect(page.locator('text=Tour: Accept your first payment')).toBeVisible();
    await expect(page.locator('text=Tour: Activate your AI Support Agent')).toBeVisible();
  });
});
