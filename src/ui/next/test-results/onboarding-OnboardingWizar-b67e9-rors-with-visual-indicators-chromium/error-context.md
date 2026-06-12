# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Submitting empty inputs displays validation errors with visual indicators
- Location: src/e2e/onboarding.spec.ts:210:7

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for getByRole('button', { name: 'Next' })
    - locator resolved to <button disabled class="w-full bg-[#0066FF] text-white min-h-[44px] min-w-[44px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed">…</button>
  - attempting click action
    - waiting for element to be visible, enabled and stable
    - element is not enabled
  - retrying click action
    - waiting for element to be visible, enabled and stable
    - element is not stable
  - retrying click action
    - waiting 20ms
    - waiting for element to be visible, enabled and stable
    - element is not stable
  2 × retrying click action
      - waiting 100ms
      - waiting for element to be visible, enabled and stable
      - element is not enabled
  52 × retrying click action
       - waiting 500ms
       - waiting for element to be visible, enabled and stable
       - element is not enabled
  - retrying click action
    - waiting 500ms

```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e3]:
    - generic [ref=e4]:
      - heading "Setup" [level=1] [ref=e5]
      - paragraph [ref=e6]: Your business, live in minutes.
    - generic [ref=e8]:
      - generic:
        - img
      - generic [ref=e10]:
        - heading "What's the name of your business?" [level=2] [ref=e11]
        - generic [ref=e12]:
          - paragraph [ref=e13]: Our AI will instantly generate your storefront, products, and back-office agents.
          - button "Save Draft" [ref=e14]:
            - generic [ref=e18]: Save Draft
        - textbox "e.g. Maya's Custom Cakes" [active] [ref=e21]
        - button "Next" [disabled] [ref=e23]:
          - generic [ref=e26]: Next
  - button "Help" [ref=e29]:
    - img [ref=e30]
  - button "Open help chat" [ref=e33]:
    - generic [ref=e34]: ✨
    - generic [ref=e35]: Ask anything
  - button "Voice Assistant" [ref=e36]:
    - img
  - alert [ref=e38]
```

# Test source

```ts
  118 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  119 |
  120 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Fatima Halal Food');
  121 |     await page.getByRole('button', { name: 'Next' }).click();
  122 |
  123 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Halal food cart pickup orders');
  124 |     await page.getByRole('button', { name: 'Next' }).click();
  125 |
  126 |     await page.getByPlaceholder(/Portland, OR/i).fill('New York, NY');
  127 |     await page.getByRole('button', { name: 'Next' }).click();
  128 |
  129 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Professionals');
  130 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
  131 |
  132 |     await expect(page.locator('input[value="Halal food cart pickup orders"]')).toBeVisible();
  133 |     await page.getByRole('button', { name: 'Continue' }).click();
  134 |
  135 |     await page.getByText('Bold').click();
  136 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Fatima');
  137 |     await page.getByPlaceholder(/you@example.com/i).fill('fatima@foodcart.com');
  138 |     await page.getByPlaceholder(/••••••••/i).fill('halal123');
  139 |
  140 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  141 |     await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 5000 });
  142 |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  143 |     expect(storedTenantId).not.toBeNull();
  144 |   });
  145 |
  146 |   test('User can save a draft and restore it across sessions', async ({ page }) => {
  147 |     let savedWizardState: Record<string, unknown> | undefined;
  148 |
  149 |     // 1. Start Wizard and Save Draft
  150 |     await page.goto('/onboarding');
  151 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  152 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  153 |
  154 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('My Restored Business');
  155 |     await page.getByRole('button', { name: 'Save Draft' }).click();
  156 |     await expect(page.getByText('Draft Saved!')).toBeVisible();
  157 |
  158 |     // 2. Clear local storage to simulate device switch
  159 |     await page.evaluate(() => window.localStorage.clear());
  160 |
  161 |     // 3. Reload page and check restoration
  162 |     await page.reload();
  163 |
  164 |     // We should be restored to the first step of the wizard where we were, with the text filled
  165 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  166 |     await expect(page.locator('input[value="My Restored Business"]')).toBeVisible();
  167 |   });
  168 |
  169 |   test('Validation errors prevent launching without complete admin info', async ({ page }) => {
  170 |
  171 |     await page.goto('/onboarding');
  172 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  173 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  174 |
  175 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
  176 |     await page.getByRole('button', { name: 'Next' }).click();
  177 |
  178 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
  179 |     await page.getByRole('button', { name: 'Next' }).click();
  180 |
  181 |     await page.getByPlaceholder(/Portland, OR/i).fill('Local');
  182 |     await page.getByRole('button', { name: 'Next' }).click();
  183 |
  184 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Anyone');
  185 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
  186 |
  187 |     await page.getByRole('button', { name: 'Continue' }).click();
  188 |
  189 |     // Do NOT fill out admin email and password initially
  190 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  191 |
  192 |     // Attempt to launch store
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
> 218 |     await page.getByRole('button', { name: 'Next' }).click();
      |                                                      ^ Error: locator.click: Test timeout of 30000ms exceeded.
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
  293 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
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
```