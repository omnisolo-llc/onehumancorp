# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> User can use Instant Build to launch storefront quickly
- Location: src/e2e/onboarding.spec.ts:282:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByText('Style & Team')
Expected: visible
Timeout: 15000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for getByText('Style & Team')

```

```yaml
- heading "Setup" [level=1]
- paragraph: Your business, live in minutes.
- button "Skip setup"
- img
- paragraph: Failed to fetch
- button "Back":
  - img
  - text: Back
- heading "Tell us about your business" [level=2]
- paragraph: Our AI will handle the rest in 30 seconds.
- textbox "e.g. I run a local bakery that sells custom vegan cakes...": I am Maya, I run a local bakery making custom vegan cakes in Portland, OR.
- textbox "Image URL (Optional)"
- button "Next"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- alert
```

# Test source

```ts
  194 |     await page.getByRole('button', { name: 'Next' }).click();
  195 |
  196 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
  197 |     await page.getByRole('button', { name: 'Next' }).click();
  198 |
  199 |     await page.getByPlaceholder(/Portland, OR/i).fill('Local');
  200 |     await page.getByRole('button', { name: 'Next' }).click();
  201 |
  202 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Anyone');
  203 |     await page.getByRole('button', { name: 'Next' }).click();
  204 |
  205 |     await page.getByRole('button', { name: 'Continue' }).click({ timeout: 15000 });
  206 |
  207 |     // Do NOT fill out admin email and password initially
  208 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  209 |
  210 |     // Attempt to launch store
  211 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  212 |
  213 |     // Expect validation errors to be visible
  214 |     await expect(page.getByText(/is required/i).first()).toBeVisible();
  215 |
  216 |     // Fill in invalid email and password without number
  217 |     await page.getByPlaceholder(/you@example.com/i).fill('invalid-email');
  218 |     await page.getByPlaceholder(/••••••••/i).fill('password');
  219 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  220 |
  221 |     await expect(page.getByText('Please enter a valid email address')).toBeVisible();
  222 |     await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();
  223 |
  224 |     // Ensure it hasn't progressed to the success screen
  225 |     await expect(page.getByText("You're Live!")).toBeHidden();
  226 |   });
  227 |
  228 |   test('Submitting empty inputs displays validation errors with visual indicators', async ({ page }) => {
  229 |     await page.goto('/onboarding');
  230 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  231 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  232 |
  233 |     // Step 1: Empty Business Name
  234 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  235 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('  ');
  236 |     await page.getByRole('button', { name: 'Next' }).click();
  237 |
  238 |     const businessNameInput = page.getByPlaceholder(/Maya's Custom Cake/i);
  239 |     await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
  240 |     await expect(businessNameInput).toHaveClass(/border-\[#FF3B30\]/); // Note: We verify the visual class directly here
  241 |
  242 |     // Proceed to Step 2
  243 |     await businessNameInput.fill('Valid Business Name');
  244 |     await page.getByRole('button', { name: 'Next' }).click();
  245 |
  246 |     // Step 2: Empty What you sell
  247 |     await expect(page.getByText("What do you sell?")).toBeVisible();
  248 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('');
  249 |     await page.getByRole('button', { name: 'Next' }).click();
  250 |
  251 |     const whatYouSellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
  252 |     await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
  253 |     await expect(whatYouSellInput).toHaveClass(/border-\[#FF3B30\]/);
  254 |
  255 |     // Proceed to Step 3
  256 |     await whatYouSellInput.fill('Valid products');
  257 |     await page.getByRole('button', { name: 'Next' }).click();
  258 |
  259 |     // Step 3: Empty Location
  260 |     await expect(page.getByText("Where are you located?")).toBeVisible();
  261 |     await page.getByPlaceholder(/Portland, OR/i).fill('  ');
  262 |     await page.getByRole('button', { name: 'Next' }).click();
  263 |
  264 |     const locationInput = page.getByPlaceholder(/Portland, OR/i);
  265 |     await expect(page.getByText('Please tell us your location.')).toBeVisible();
  266 |     await expect(locationInput).toHaveClass(/border-\[#FF3B30\]/);
  267 |
  268 |     // Proceed to Step 4
  269 |     await locationInput.fill('Valid Location');
  270 |     await page.getByRole('button', { name: 'Next' }).click();
  271 |
  272 |     // Step 4: Empty Target Audience
  273 |     await expect(page.getByText("Who is your target audience?")).toBeVisible();
  274 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('  ');
  275 |     await page.getByRole('button', { name: 'Next' }).click();
  276 |
  277 |     const audienceInput = page.getByPlaceholder(/Local families, Tech startups/i);
  278 |     await expect(page.getByText('Please tell us your target audience.')).toBeVisible();
  279 |     await expect(audienceInput).toHaveClass(/border-\[#FF3B30\]/);
  280 |   });
  281 |
  282 |   test('User can use Instant Build to launch storefront quickly', async ({ page }) => {
  283 |     await page.goto('/onboarding');
  284 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  285 |
  286 |     await page.getByRole('button', { name: 'Instant Build' }).click();
  287 |     await expect(page.getByText("Tell us about your business")).toBeVisible();
  288 |
  289 |     const bioInput = page.getByPlaceholder(/e.g. I run a local bakery/i);
  290 |     await bioInput.fill('I am Maya, I run a local bakery making custom vegan cakes in Portland, OR.');
  291 |
  292 |     await page.getByRole('button', { name: 'Next' }).click();
  293 |
> 294 |     await expect(page.getByText("Style & Team")).toBeVisible({ timeout: 15000 });
      |                                                  ^ Error: expect(locator).toBeVisible() failed
  295 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  296 |     await page.getByPlaceholder(/you@example.com/i).fill('admin@test.com');
  297 |     await page.getByPlaceholder(/••••••••/i).fill('password123');
  298 |
  299 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  300 |
  301 |     // Expect it to eventually reach "You're Live!" screen
  302 |     await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  303 |   });
  304 | });
  305 |
```