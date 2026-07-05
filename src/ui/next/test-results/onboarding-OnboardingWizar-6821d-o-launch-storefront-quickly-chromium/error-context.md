# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> User can use Instant Build to launch storefront quickly
- Location: src/e2e/onboarding.spec.ts:275:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByText('You\'re Live!')
Expected: visible
Timeout: 15000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for getByText('You\'re Live!')

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
- button "Generate Storefront"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- alert
```

# Test source

```ts
  188 |
  189 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
  190 |     await page.getByRole('button', { name: 'Next' }).click();
  191 |
  192 |     await page.getByPlaceholder(/Portland, OR/i).fill('Local');
  193 |     await page.getByRole('button', { name: 'Next' }).click();
  194 |
  195 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Anyone');
  196 |     await page.getByRole('button', { name: 'Next' }).click();
  197 |
  198 |     await page.getByRole('button', { name: 'Continue' }).click({ timeout: 15000 });
  199 |
  200 |     // Do NOT fill out admin email and password initially
  201 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  202 |
  203 |     // Attempt to launch store
  204 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  205 |
  206 |     // Expect validation errors to be visible
  207 |     await expect(page.getByText(/is required/i).first()).toBeVisible();
  208 |
  209 |     // Fill in invalid email and password without number
  210 |     await page.getByPlaceholder(/you@example.com/i).fill('invalid-email');
  211 |     await page.getByPlaceholder(/••••••••/i).fill('password');
  212 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  213 |
  214 |     await expect(page.getByText('Please enter a valid email address')).toBeVisible();
  215 |     await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();
  216 |
  217 |     // Ensure it hasn't progressed to the success screen
  218 |     await expect(page.getByText("You're Live!")).toBeHidden();
  219 |   });
  220 |
  221 |   test('Submitting empty inputs displays validation errors with visual indicators', async ({ page }) => {
  222 |     await page.goto('/onboarding');
  223 |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  224 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  225 |
  226 |     // Step 1: Empty Business Name
  227 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  228 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('  ');
  229 |     await page.getByRole('button', { name: 'Next' }).click();
  230 |
  231 |     const businessNameInput = page.getByPlaceholder(/Maya's Custom Cake/i);
  232 |     await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
  233 |     await expect(businessNameInput).toHaveClass(/.*border-\[#FF3B30\].*/); // Note: We verify the visual class directly here
  234 |
  235 |     // Proceed to Step 2
  236 |     await businessNameInput.fill('Valid Business Name');
  237 |     await page.getByRole('button', { name: 'Next' }).click();
  238 |
  239 |     // Step 2: Empty What you sell
  240 |     await expect(page.getByText("What do you sell?")).toBeVisible();
  241 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('');
  242 |     await page.getByRole('button', { name: 'Next' }).click();
  243 |
  244 |     const whatYouSellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
  245 |     await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
  246 |     await expect(whatYouSellInput).toHaveClass(/.*border-\[#FF3B30\].*/);
  247 |
  248 |     // Proceed to Step 3
  249 |     await whatYouSellInput.fill('Valid products');
  250 |     await page.getByRole('button', { name: 'Next' }).click();
  251 |
  252 |     // Step 3: Empty Location
  253 |     await expect(page.getByText("Where are you located?")).toBeVisible();
  254 |     await page.getByPlaceholder(/Portland, OR/i).fill('  ');
  255 |     await page.getByRole('button', { name: 'Next' }).click();
  256 |
  257 |     const locationInput = page.getByPlaceholder(/Portland, OR/i);
  258 |     await expect(page.getByText('Please tell us your location.')).toBeVisible();
  259 |     await expect(locationInput).toHaveClass(/.*border-\[#FF3B30\].*/);
  260 |
  261 |     // Proceed to Step 4
  262 |     await locationInput.fill('Valid Location');
  263 |     await page.getByRole('button', { name: 'Next' }).click();
  264 |
  265 |     // Step 4: Empty Target Audience
  266 |     await expect(page.getByText("Who is your target audience?")).toBeVisible();
  267 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('  ');
  268 |     await page.getByRole('button', { name: 'Next' }).click();
  269 |
  270 |     const audienceInput = page.getByPlaceholder(/Local families, Tech startups/i);
  271 |     await expect(page.getByText('Please tell us your target audience.')).toBeVisible();
  272 |     await expect(audienceInput).toHaveClass(/.*border-\[#FF3B30\].*/);
  273 |   });
  274 |
  275 |   test('User can use Instant Build to launch storefront quickly', async ({ page }) => {
  276 |     await page.goto('/onboarding');
  277 |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  278 |
  279 |     await page.getByRole('button', { name: 'Instant Build' }).click();
  280 |     await expect(page.getByText("Tell us about your business")).toBeVisible();
  281 |
  282 |     const bioInput = page.getByPlaceholder(/e.g. I run a local bakery/i);
  283 |     await bioInput.fill('I am Maya, I run a local bakery making custom vegan cakes in Portland, OR.');
  284 |
  285 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
  286 |
  287 |     // Expect it to eventually reach "You're Live!" screen
> 288 |     await page.waitForTimeout(5000); await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
      |                                                                                   ^ Error: expect(locator).toBeVisible() failed
  289 |   });
  290 | });
  291 |
```