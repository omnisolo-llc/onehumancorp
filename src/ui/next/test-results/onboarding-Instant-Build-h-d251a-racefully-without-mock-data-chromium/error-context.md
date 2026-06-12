# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> Instant Build handles network failures gracefully without mock data
- Location: src/e2e/onboarding.spec.ts:281:7

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for getByRole('button', { name: 'Generate Storefront' })

```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e3]:
    - generic [ref=e4]:
      - heading "Setup" [level=1] [ref=e5]
      - paragraph [ref=e6]: Your business, live in minutes.
    - generic [ref=e8]:
      - button "Back" [ref=e9]:
        - img
        - text: Back
      - heading "Tell us about your business" [level=2] [ref=e11]
      - paragraph [ref=e13]: Our AI will handle the rest in 30 seconds.
      - textbox "e.g. I run a local bakery that sells custom vegan cakes..." [active] [ref=e15]: Failing business info
      - button "Generate My Business" [ref=e17]:
        - generic [ref=e20]: Generate My Business
  - button "Help" [ref=e23]:
    - img [ref=e24]
  - button "Open help chat" [ref=e27]:
    - generic [ref=e28]: ✨
    - generic [ref=e29]: Ask anything
  - button "Voice Assistant" [ref=e30]:
    - img
  - alert [ref=e32]
```

# Test source

```ts
  193 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  194 |
  195 |     // Expect validation errors to be visible
  196 |     await expect(page.getByText(/is required/i).first()).toBeVisible();
  197 |
  198 |     // Fill in invalid email and password without number
  199 |     await page.getByPlaceholder(/you@example.com/i).fill('invalid-email');
  200 |     await page.getByPlaceholder(/••••••••/i).fill('password');
  201 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  202 |
  203 |     await expect(page.getByText('Please enter a valid email address')).toBeVisible();
  204 |     await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();
  205 |
  206 |     // Ensure it hasn't progressed to the success screen
  207 |     await expect(page.getByText("You're Live!")).toBeHidden();
  208 |   });
  209 |
  210 |   test('Submitting empty inputs displays validation errors with visual indicators', async ({ page }) => {
  211 |     await page.goto('/onboarding');
  212 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  213 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  214 |
  215 |     // Step 1: Empty Business Name
  216 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  217 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('  ');
  218 |     await page.getByRole('button', { name: 'Next' }).click();
  219 |
  220 |     const businessNameInput = page.getByPlaceholder(/Maya's Custom Cake/i);
  221 |     await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
  222 |     await expect(businessNameInput).toHaveClass(/border-\[#FF3B30\]/);
  223 |
  224 |     // Proceed to Step 2
  225 |     await businessNameInput.fill('Valid Business Name');
  226 |     await page.getByRole('button', { name: 'Next' }).click();
  227 |
  228 |     // Step 2: Empty What you sell
  229 |     await expect(page.getByText("What do you sell?")).toBeVisible();
  230 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('');
  231 |     await page.getByRole('button', { name: 'Next' }).click();
  232 |
  233 |     const whatYouSellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
  234 |     await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
  235 |     await expect(whatYouSellInput).toHaveClass(/border-\[#FF3B30\]/);
  236 |
  237 |     // Proceed to Step 3
  238 |     await whatYouSellInput.fill('Valid products');
  239 |     await page.getByRole('button', { name: 'Next' }).click();
  240 |
  241 |     // Step 3: Empty Location
  242 |     await expect(page.getByText("Where are you located?")).toBeVisible();
  243 |     await page.getByPlaceholder(/Portland, OR/i).fill('  ');
  244 |     await page.getByRole('button', { name: 'Next' }).click();
  245 |
  246 |     const locationInput = page.getByPlaceholder(/Portland, OR/i);
  247 |     await expect(page.getByText('Please tell us your location.')).toBeVisible();
  248 |     await expect(locationInput).toHaveClass(/border-\[#FF3B30\]/);
  249 |
  250 |     // Proceed to Step 4
  251 |     await locationInput.fill('Valid Location');
  252 |     await page.getByRole('button', { name: 'Next' }).click();
  253 |
  254 |     // Step 4: Empty Target Audience
  255 |     await expect(page.getByText("Who is your target audience?")).toBeVisible();
  256 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('  ');
  257 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
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
  274 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
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
> 293 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
      |                                                                     ^ Error: locator.click: Test timeout of 30000ms exceeded.
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
  317 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
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
  337 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
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
  349 |     await expect(page.getByText(/Failed to start onboarding/i)).toBeVisible();
  350 |
  351 |     await context.unroute('/api/onboarding/start');
  352 |     await context.unroute('/api/onboarding/intake');
  353 |   });
  354 |
```