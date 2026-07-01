# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: work_intake_widget.spec.ts >> Embeddable Work-Intake Widget Growth Loop >> embed API endpoint returns the work intake HTML and submit works
- Location: src/e2e/work_intake_widget.spec.ts:48:9

# Error details

```
Error: expect(received).toContain(expected) // indexOf

Expected substring: "Thanks, Playwright Test!"
Received string:    "·
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
      <meta charset=\"UTF-8\">
      <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
      <title>Request Submitted</title>
      <link href=\"https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap\" rel=\"stylesheet\">
      <style>
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 16px; background: transparent; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .card {
            background-color: #ffffff;
            border: 1px solid #e5e7eb;
            border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            max-width: 24rem;
            margin: 0 auto;
        }
        .content { padding: 40px 20px; text-align: center; }
        .icon {
            font-size: 4rem;
            margin-bottom: 16px;
        }
        .title {
            color: #111827;
            font-size: 1.5rem;
            font-weight: 700;
            margin-bottom: 8px;
        }
        .desc {
            color: #4b5563;
            font-size: 1rem;
            margin-bottom: 24px;
            line-height: 1.5;
        }
        .footer {
            padding-top: 16px;
            margin-top: 16px;
            border-top: 1px solid #f3f4f6;
            color: #6b7280;
            font-size: 0.75rem;
            text-align: center;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
        }
        .footer a {
            font-weight: 700;
            color: #3b82f6;
            text-decoration: none;
            transition: color 0.15s ease;
        }
        .footer a:hover { color: #2563eb; text-decoration: underline; }
      </style>
    </head>
    <body>
      <div class=\"card\">
        <div class=\"content\">
            <div class=\"icon\">✅</div>
            <h2 class=\"title font-outfit\">Request Received!</h2>
            <p class=\"desc\">Thanks, null! We've received your request and will be in touch shortly.</p>
        </div>
        <div style=\"padding: 0 20px 20px;\">
             <!-- Viral Growth Loop Footer -->
             <div class=\"footer\">
                <span>⚡ Powered by</span>
                <a href=\"/api/v1/growth/referrals/click?target=/onboarding&ref=my-business\" target=\"_blank\" rel=\"noopener noreferrer\">OHC</a>
             </div>
        </div>
      </div>
    </body>
    </html>
    "
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Embeddable Work-Intake Widget Growth Loop', () => {
  4  |     test('dashboard shows the embed work intake widget and generates correct HTML', async ({ page }) => {
  5  |         // Go to dashboard
  6  |         await page.goto('/dashboard');
  7  |
  8  |         // Look for the "Work-Intake Widget" link in the Dashboard Growth & Virality section
  9  |         const widgetLink = page.locator('text=Work-Intake Widget');
  10 |         await expect(widgetLink).toBeVisible();
  11 |         await widgetLink.click();
  12 |
  13 |         // Should now be on the Work-Intake Widget page
  14 |         const sectionHeader = page.getByRole('heading', { name: /Work-Intake Widget/ });
  15 |         await expect(sectionHeader).toBeVisible();
  16 |
  17 |         // Check for the "Lead Capture Loop" badge next to the header
  18 |         await expect(page.locator('text=Lead Capture Loop').first()).toBeVisible();
  19 |
  20 |         // Set the form title
  21 |         const titleInput = page.getByPlaceholder('e.g. Work Request');
  22 |         await titleInput.fill('Book an Appointment');
  23 |
  24 |         // Click "Get Widget Code" button
  25 |         const getWidgetBtn = page.locator('button:has-text("Get Widget Code")');
  26 |         await expect(getWidgetBtn).toBeVisible();
  27 |         await getWidgetBtn.click();
  28 |
  29 |         // Modal should appear
  30 |         const modalHeader = page.locator('h2:has-text("Embed Work-Intake Widget")');
  31 |         await expect(modalHeader).toBeVisible();
  32 |
  33 |         // The textarea should contain the iframe snippet
  34 |         const textarea = page.locator('textarea').filter({ hasText: '<iframe src="https://ohc.app/api/v1/growth/work-intake/embed' });
  35 |         await expect(textarea).toBeVisible();
  36 |
  37 |         // Verify the HTML snippet structure includes the custom title encoded
  38 |         const snippet = await textarea.inputValue();
  39 |         expect(snippet).toContain('Book%20an%20Appointment');
  40 |         expect(snippet).toContain('theme=light');
  41 |         expect(snippet).toContain('width="320"');
  42 |         expect(snippet).toContain('height="400"');
  43 |         expect(snippet).toContain('frameborder="0"');
  44 |
  45 |         await page.waitForTimeout(500);
  46 |     });
  47 |
  48 |     test('embed API endpoint returns the work intake HTML and submit works', async ({ request }) => {
  49 |         // Test GET endpoint
  50 |         const response = await request.get('/api/v1/growth/work-intake/embed?tenant=my-business&theme=light&title=TestRequest');
  51 |         expect(response.ok()).toBeTruthy();
  52 |
  53 |         const html = await response.text();
  54 |
  55 |         // Assert the HTML contains the correct structure and elements
  56 |         expect(html).toContain('<!DOCTYPE html>');
  57 |         expect(html).toContain('TestRequest');
  58 |         expect(html).toContain('Send Request');
  59 |
  60 |         // Ensure the referral growth loop is intact in the footer
  61 |         expect(html).toContain('Powered by');
  62 |         expect(html).toContain('OHC');
  63 |         expect(html).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=my-business');
  64 |
  65 |         // Test POST submit endpoint
  66 |         const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=my-business', {
  67 |            data: {
  68 |              name: 'Playwright Test',
  69 |              email: 'test@example.com',
  70 |              details: 'Test details'
  71 |            },
  72 |            headers: {
  73 |              'Content-Type': 'application/x-www-form-urlencoded'
  74 |            }
  75 |         });
  76 |
  77 |         expect(submitResponse.ok()).toBeTruthy();
  78 |
  79 |         const submitHtml = await submitResponse.text();
  80 |         expect(submitHtml).toContain('Request Received!');
> 81 |         expect(submitHtml).toContain('Thanks, Playwright Test!');
     |                            ^ Error: expect(received).toContain(expected) // indexOf
  82 |
  83 |         // Confirm viral loop is still present on success screen
  84 |         expect(submitHtml).toContain('Powered by OHC');
  85 |     });
  86 | });
  87 |
```