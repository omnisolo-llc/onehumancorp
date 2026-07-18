# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> User can use Instant Build to launch storefront quickly
- Location: src/e2e/onboarding.spec.ts:317:7

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for getByRole('button', { name: 'Next' })

```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e3]:
    - generic [ref=e5]:
      - generic [ref=e7]:
        - heading "Setup" [level=1] [ref=e8]
        - paragraph [ref=e9]: Your business, live in minutes.
      - button "Skip setup" [ref=e10] [cursor=pointer]
    - generic [ref=e13]:
      - button "Back" [ref=e14] [cursor=pointer]:
        - img [ref=e15]
        - text: Back
      - heading "Tell us about your business" [level=2] [ref=e17]
      - paragraph [ref=e18]: Our AI will handle the rest in 30 seconds.
      - generic [ref=e19]:
        - textbox "e.g. I run a local bakery that sells custom vegan cakes..." [active] [ref=e20]: I am Maya, I run a local bakery making custom vegan cakes in Portland, OR.
        - textbox "Image URL (Optional)" [ref=e21]
        - button "Generate Storefront" [ref=e23] [cursor=pointer]
  - button "Help" [ref=e26]:
    - img [ref=e27]
  - button "Open help chat" [ref=e31]:
    - generic [ref=e32]: ✨
    - generic [ref=e33]: Ask anything
  - button "Voice Assistant" [ref=e34]:
    - img [ref=e35]
  - alert [ref=e37]
```

# Test source

```ts
  227 |
  228 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
  229 |     await page.getByRole('button', { name: 'Next' }).click();
  230 |
  231 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
  232 |     await page.getByRole('button', { name: 'Next' }).click();
  233 |
  234 |     await page.getByPlaceholder(/Portland, OR/i).fill('Local');
  235 |     await page.getByRole('button', { name: 'Next' }).click();
  236 |
  237 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Anyone');
  238 |     await page.getByRole('button', { name: 'Next' }).click();
  239 |
  240 |     await page.getByRole('button', { name: 'Continue' }).click({ timeout: 15000 });
  241 |
  242 |     // Do NOT fill out admin email and password initially
  243 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  244 |
  245 |     // Attempt to launch store
  246 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  247 |
  248 |     // Expect validation errors to be visible
  249 |     await expect(page.getByText(/is required/i).first()).toBeVisible();
  250 |
  251 |     // Fill in invalid email and password without number
  252 |     await page.getByPlaceholder(/you@example.com/i).fill('invalid-email');
  253 |     await page.getByPlaceholder(/••••••••/i).fill('password');
  254 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  255 |
  256 |     await expect(page.getByText('Please enter a valid email address')).toBeVisible();
  257 |     await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();
  258 |
  259 |     // Ensure it hasn't progressed to the success screen
  260 |     await expect(page.getByText("You're Live!")).toBeHidden();
  261 |   });
  262 |
  263 |   test('Submitting empty inputs displays validation errors with visual indicators', async ({ page }) => {
  264 |     await page.goto('/onboarding');
  265 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  266 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  267 |
  268 |     // Step 1: Empty Business Name
  269 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  270 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('  ');
  271 |     await page.getByRole('button', { name: 'Next' }).click();
  272 |
  273 |     const businessNameInput = page.getByPlaceholder(/Maya's Custom Cake/i);
  274 |     await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
  275 |     await expect(businessNameInput).toHaveClass(/border-\[#FF3B30\]/); // Note: We verify the visual class directly here
  276 |
  277 |     // Proceed to Step 2
  278 |     await businessNameInput.fill('Valid Business Name');
  279 |     await page.getByRole('button', { name: 'Next' }).click();
  280 |
  281 |     // Step 2: Empty What you sell
  282 |     await expect(page.getByText("What do you sell?")).toBeVisible();
  283 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('');
  284 |     await page.getByRole('button', { name: 'Next' }).click();
  285 |
  286 |     const whatYouSellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
  287 |     await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
  288 |     await expect(whatYouSellInput).toHaveClass(/border-\[#FF3B30\]/);
  289 |
  290 |     // Proceed to Step 3
  291 |     await whatYouSellInput.fill('Valid products');
  292 |     await page.getByRole('button', { name: 'Next' }).click();
  293 |
  294 |     // Step 3: Empty Location
  295 |     await expect(page.getByText("Where are you located?")).toBeVisible();
  296 |     await page.getByPlaceholder(/Portland, OR/i).fill('  ');
  297 |     await page.getByRole('button', { name: 'Next' }).click();
  298 |
  299 |     const locationInput = page.getByPlaceholder(/Portland, OR/i);
  300 |     await expect(page.getByText('Please tell us your location.')).toBeVisible();
  301 |     await expect(locationInput).toHaveClass(/border-\[#FF3B30\]/);
  302 |
  303 |     // Proceed to Step 4
  304 |     await locationInput.fill('Valid Location');
  305 |     await page.getByRole('button', { name: 'Next' }).click();
  306 |
  307 |     // Step 4: Empty Target Audience
  308 |     await expect(page.getByText("Who is your target audience?")).toBeVisible();
  309 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('  ');
  310 |     await page.getByRole('button', { name: 'Next' }).click();
  311 |
  312 |     const audienceInput = page.getByPlaceholder(/Local families, Tech startups/i);
  313 |     await expect(page.getByText('Please tell us your target audience.')).toBeVisible();
  314 |     await expect(audienceInput).toHaveClass(/border-\[#FF3B30\]/);
  315 |   });
  316 |
  317 |   test('User can use Instant Build to launch storefront quickly', async ({ page }) => {
  318 |     await page.goto('/onboarding');
  319 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  320 |
  321 |     await page.getByRole('button', { name: 'Instant Build' }).click();
  322 |     await expect(page.getByText("Tell us about your business")).toBeVisible();
  323 |
  324 |     const bioInput = page.getByPlaceholder(/e.g. I run a local bakery/i);
  325 |     await bioInput.fill('I am Maya, I run a local bakery making custom vegan cakes in Portland, OR.');
  326 |
> 327 |     await page.getByRole('button', { name: 'Next' }).click();
      |                                                      ^ Error: locator.click: Test timeout of 30000ms exceeded.
  328 |
  329 |     await page.waitForSelector("text=Style & Team", { state: "visible", timeout: 30000 });
  330 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  331 |     await page.getByPlaceholder(/you@example.com/i).fill('admin@test.com');
  332 |     await page.getByPlaceholder(/••••••••/i).fill('password123');
  333 |
  334 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  335 |
  336 |     // Expect it to eventually reach "You're Live!" screen
  337 |     await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  338 |   });
  339 | });
  340 |
```