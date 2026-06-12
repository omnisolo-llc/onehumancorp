# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> Store launch correctly fails when start API is down
- Location: src/e2e/onboarding.spec.ts:323:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByText(/Failed to start onboarding/i)
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for getByText(/Failed to start onboarding/i)

```

```yaml
- heading "Setup" [level=1]
- paragraph: Your business, live in minutes.
- text: "HTTP error! status: 502"
- button "Back":
  - img
  - text: Back
- heading "Style & Team" [level=2]
- paragraph: Pick your storefront vibe. We'll automatically assign the best AI agents to manage it.
- button "Save Draft"
- text: Website Template Modern Minimal Bold Classic Web Address Free Subdomain your-name.ohc.app Custom Domain your-name.com Account Setup Admin Name
- textbox "e.g. Maya Smith": Test Admin
- text: Admin Email
- textbox "you@example.com": admin@test.com
- text: Admin Password
- textbox "••••••••": password123
- text: Auto-Configured AI Departments
- paragraph: Here are the AI departments we've configured for you.
- img
- text: Operations
- img
- text: Marketing
- img
- text: Finance
- img
- text: Legal
- img
- text: Advisory Allow AI to Auto-Respond
- checkbox "Allow AI to Auto-Respond" [checked]
- button "Launch Store"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- alert
```

# Test source

```ts
  249 |
  250 |     // Proceed to Step 4
  251 |     await locationInput.fill('Valid Location');
  252 |     await page.getByRole('button', { name: 'Next' }).click();
  253 |
  254 |     // Step 4: Empty Target Audience
  255 |     await expect(page.getByText("Who is your target audience?")).toBeVisible();
  256 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('  ');
  257 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  258 |
  259 |     const audienceInput = page.getByPlaceholder(/Local families, Tech startups/i);
  260 |     await expect(page.getByText('Please tell us your target audience.')).toBeVisible();
  261 |     await expect(audienceInput).toHaveClass(/border-\[#FF3B30\]/);
  262 |   });
  263 |
  264 |   test('User can use Instant Build to launch storefront quickly', async ({ page }) => {
  265 |     await page.goto('/onboarding');
  266 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  267 |
  268 |     await page.getByRole('button', { name: 'Instant Build' }).click();
  269 |     await expect(page.getByText("Tell us about your business")).toBeVisible();
  270 |
  271 |     const bioInput = page.getByPlaceholder(/e.g. I run a local bakery/i);
  272 |     await bioInput.fill('I am Maya, I run a local bakery making custom vegan cakes in Portland, OR.');
  273 |
  274 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  275 |
  276 |     // Expect it to eventually reach "You're Live!" screen
  277 |     await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  278 |   });
  279 | });
  280 |
  281 |   test('Instant Build handles network failures gracefully without mock data', async ({ page, context }) => {
  282 |     await page.goto('/onboarding');
  283 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  284 |     await page.getByRole('button', { name: 'Instant Build' }).click();
  285 |     await expect(page.getByText("Tell us about your business")).toBeVisible();
  286 |
  287 |     // Fill the form
  288 |     await page.getByPlaceholder(/e.g. I run a local bakery/i).fill('Failing business info');
  289 |
  290 |     // Intercept the API route to fail
  291 |     await context.route('/api/onboarding/intake', route => route.abort('failed'));
  292 |
  293 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  294 |
  295 |     // Should display a real error message, not mock data
  296 |     await expect(page.getByText(/Failed to launch. Please try again./i)).toBeVisible();
  297 |
  298 |     // Stop interception
  299 |     await context.unroute('/api/onboarding/intake');
  300 |   });
  301 |
  302 |   test('Step-by-step intake handles backend processing errors correctly', async ({ page, context }) => {
  303 |     await page.goto('/onboarding');
  304 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  305 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
  306 |     await page.getByRole('button', { name: 'Next' }).click();
  307 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
  308 |     await page.getByRole('button', { name: 'Next' }).click();
  309 |     await page.getByPlaceholder(/Portland, OR/i).fill('Local');
  310 |     await page.getByRole('button', { name: 'Next' }).click();
  311 |
  312 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Testing');
  313 |
  314 |     // Mock the backend responding with a 500 error
  315 |     await context.route('/api/onboarding/intake', route => route.fulfill({ status: 500, json: { error: 'Internal Server Error' } }));
  316 |
  317 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  318 |     await expect(page.getByText(/Internal Server Error/i)).toBeVisible();
  319 |
  320 |     await context.unroute('/api/onboarding/intake');
  321 |   });
  322 |
  323 |   test('Store launch correctly fails when start API is down', async ({ page, context }) => {
  324 |     await page.goto('/onboarding');
  325 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  326 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
  327 |     await page.getByRole('button', { name: 'Next' }).click();
  328 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
  329 |     await page.getByRole('button', { name: 'Next' }).click();
  330 |     await page.getByPlaceholder(/Portland, OR/i).fill('Local');
  331 |     await page.getByRole('button', { name: 'Next' }).click();
  332 |
  333 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Testing');
  334 |
  335 |     // Normal intake response
  336 |     await context.route('/api/onboarding/intake', route => route.fulfill({ status: 200, json: { business_name: 'Test Business', business_type: 'Test', initial_products: [], categories: [] } }));
  337 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  338 |     await expect(page.getByText('Review Details')).toBeVisible();
  339 |     await page.getByRole('button', { name: 'Continue' }).click();
  340 |
  341 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  342 |     await page.getByPlaceholder(/you@example.com/i).fill('admin@test.com');
  343 |     await page.getByPlaceholder(/••••••••/i).fill('password123');
  344 |
  345 |     // Mock the start API failing
  346 |     await context.route('/api/onboarding/start', route => route.fulfill({ status: 502 }));
  347 |
  348 |     await page.getByRole('button', { name: 'Launch Store' }).click();
> 349 |     await expect(page.getByText(/Failed to start onboarding/i)).toBeVisible();
      |                                                                 ^ Error: expect(locator).toBeVisible() failed
  350 |
  351 |     await context.unroute('/api/onboarding/start');
  352 |     await context.unroute('/api/onboarding/intake');
  353 |   });
  354 |
```