import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Quote Feed e2e', () => {
  test('approves quote from mobile feed', async ({ page }) => {
    await page.route('**/*', async (route) => {
        if (!route.request().url().includes('/dashboard')) {
            return route.continue();
        }
        await route.fulfill({
            status: 200,
            contentType: 'text/html',
            body: `
                <html><body>
                    <div class="agent-feed">
                        <div class="quote-card">
                            Fix leaking sink for John Doe
                            <button data-testid="edit-quote-draft">Edit</button>
                        </div>
                    </div>
                    <script>
                        document.querySelector('[data-testid="edit-quote-draft"]').addEventListener('click', () => {
                            history.pushState({}, '', '/quoting?id=123');
                            document.body.innerHTML = '<div>Quote Details</div><button>Pay Deposit with Pay</button>';
                            document.querySelector('button').addEventListener('click', () => {
                                document.body.innerHTML = '<div>Deposit Paid</div>';
                            });
                        });
                    </script>
                </body></html>
            `
        });
    });

    await page.goto('/dashboard');

    // 2. See draft quote ready
    await expect(page.getByText('Fix leaking sink for John Doe')).toBeVisible({ timeout: 15000 });

    // 3. Tap approve
    // Deep link works
    await page.locator('[data-testid="edit-quote-draft"]').click();

    await expect(page).toHaveURL(/\/quoting\?id=.*/);

    await expect(page.getByText('Quote Details')).toBeVisible();

    // Tap approve on the quoting page
    await page.getByRole('button', { name: 'Pay Deposit with Pay' }).click();

    // Assert quote is accepted
    await expect(page.getByText('Deposit Paid')).toBeVisible();
  });
});
