# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Submitting empty inputs displays validation errors with visual indicators
- Location: src/e2e/onboarding.spec.ts:244:7

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for getByRole('button', { name: 'Next Step' })
    - locator resolved to <button disabled aria-label="Next Step" class="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed">…</button>
  - attempting click action
    2 × waiting for element to be visible, enabled and stable
      - element is not stable
    - retrying click action
    - waiting 20ms
    2 × waiting for element to be visible, enabled and stable
      - element is not enabled
    - retrying click action
      - waiting 100ms
    54 × waiting for element to be visible, enabled and stable
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
            - generic [ref=e15]:
              - img [ref=e16]
              - generic [ref=e19]: Save Draft
        - textbox "e.g. Maya's Custom Cakes" [active] [ref=e22]
        - button "Next Step" [disabled] [ref=e24]:
          - generic [ref=e25]:
            - img [ref=e26]
            - generic [ref=e28]: Next Step
  - button "Help" [ref=e31]:
    - img [ref=e32]
  - button "Open help chat" [ref=e35]:
    - generic [ref=e36]: ✨
    - generic [ref=e37]: Ask anything
  - button "Open Next.js Dev Tools" [ref=e43] [cursor=pointer]:
    - img [ref=e44]
  - alert [ref=e47]
```

# Test source

```ts
  152 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  153 |
  154 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Fatima Halal Food');
  155 |     await page.getByRole('button', { name: 'Next Step' }).click();
  156 |
  157 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Halal food cart pickup orders');
  158 |     await page.getByRole('button', { name: 'Next Step' }).click();
  159 |
  160 |     await page.getByPlaceholder(/Portland, OR/i).fill('New York, NY');
  161 |     await page.getByRole('button', { name: 'Next Step' }).click();
  162 |
  163 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Locals');
  164 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  165 |
  166 |     await expect(page.locator('input[value="Halal food cart pickup orders"]')).toBeVisible();
  167 |     await page.getByRole('button', { name: 'Continue' }).click();
  168 |
  169 |     await page.getByText('Bold').click();
  170 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Fatima');
  171 |     await page.getByPlaceholder(/you@example.com/i).fill('fatima@foodcart.com');
  172 |     await page.getByPlaceholder(/••••••••/i).fill('halal123');
  173 |
  174 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  175 |     await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 5000 });
  176 |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  177 |     expect(storedTenantId).not.toBeNull();
  178 |   });
  179 |
  180 |   test('User can save a draft and restore it across sessions', async ({ page }) => {
  181 |     let savedWizardState: Record<string, unknown> | undefined;
  182 |
  183 |     // 1. Start Wizard and Save Draft
  184 |     await page.goto('/onboarding');
  185 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  186 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  187 |
  188 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('My Restored Business');
  189 |     await page.getByRole('button', { name: 'Save Draft' }).click();
  190 |     await expect(page.getByText('Draft Saved!')).toBeVisible();
  191 |
  192 |     // 2. Clear local storage to simulate device switch
  193 |     await page.evaluate(() => window.localStorage.clear());
  194 |
  195 |     // 3. Reload page and check restoration
  196 |     await page.reload();
  197 |
  198 |     // We should be restored to the first step of the wizard where we were, with the text filled
  199 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  200 |     await expect(page.locator('input[value="My Restored Business"]')).toBeVisible();
  201 |   });
  202 |
  203 |   test('Validation errors prevent launching without complete admin info', async ({ page }) => {
  204 |
  205 |     await page.goto('/onboarding');
  206 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  207 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  208 |
  209 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
  210 |     await page.getByRole('button', { name: 'Next Step' }).click();
  211 |
  212 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
  213 |     await page.getByRole('button', { name: 'Next Step' }).click();
  214 |
  215 |     await page.getByPlaceholder(/Portland, OR/i).fill('Local');
  216 |     await page.getByRole('button', { name: 'Next Step' }).click();
  217 |
  218 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Locals');
  219 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  220 |
  221 |     await page.getByRole('button', { name: 'Continue' }).click();
  222 |
  223 |     // Do NOT fill out admin email and password initially
  224 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');
  225 |
  226 |     // Attempt to launch store
  227 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  228 |
  229 |     // Expect validation errors to be visible
  230 |     await expect(page.getByText(/is required/i).first()).toBeVisible();
  231 |
  232 |     // Fill in invalid email and password without number
  233 |     await page.getByPlaceholder(/you@example.com/i).fill('invalid-email');
  234 |     await page.getByPlaceholder(/••••••••/i).fill('password');
  235 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  236 |
  237 |     await expect(page.getByText('Please enter a valid email address')).toBeVisible();
  238 |     await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();
  239 |
  240 |     // Ensure it hasn't progressed to the success screen
  241 |     await expect(page.getByText("You're Live!")).toBeHidden();
  242 |   });
  243 |
  244 |   test('Submitting empty inputs displays validation errors with visual indicators', async ({ page }) => {
  245 |     await page.goto('/onboarding');
  246 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  247 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  248 |
  249 |     // Step 1: Empty Business Name
  250 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  251 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('  ');
> 252 |     await page.getByRole('button', { name: 'Next Step' }).click();
      |                                                           ^ Error: locator.click: Test timeout of 30000ms exceeded.
  253 |
  254 |     const businessNameInput = page.getByPlaceholder(/Maya's Custom Cake/i);
  255 |     await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
  256 |     await expect(businessNameInput).toHaveClass(/border-red-500/);
  257 |
  258 |     // Proceed to Step 2
  259 |     await businessNameInput.fill('Valid Business Name');
  260 |     await page.getByRole('button', { name: 'Next Step' }).click();
  261 |
  262 |     // Step 2: Empty What you sell
  263 |     await expect(page.getByText("What do you sell?")).toBeVisible();
  264 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('');
  265 |     await page.getByRole('button', { name: 'Next Step' }).click();
  266 |
  267 |     const whatYouSellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
  268 |     await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
  269 |     await expect(whatYouSellInput).toHaveClass(/border-red-500/);
  270 |
  271 |     // Proceed to Step 3
  272 |     await whatYouSellInput.fill('Valid products');
  273 |     await page.getByRole('button', { name: 'Next Step' }).click();
  274 |
  275 |     // Step 3: Empty Location
  276 |     await expect(page.getByText("Where are you located?")).toBeVisible();
  277 |     await page.getByPlaceholder(/Portland, OR/i).fill('  ');
  278 |     await page.getByRole('button', { name: 'Next Step' }).click();
  279 |
  280 |     const locationInput = page.getByPlaceholder(/Portland, OR/i);
  281 |     await expect(page.getByText('Please tell us your location.')).toBeVisible();
  282 |     await expect(locationInput).toHaveClass(/border-red-500/);
  283 |
  284 |     // Proceed to Step 4
  285 |     await locationInput.fill('Valid location');
  286 |     await page.getByRole('button', { name: 'Next Step' }).click();
  287 |
  288 |     // Step 4: Empty Target Audience
  289 |     await expect(page.getByText("Who is your target audience?")).toBeVisible();
  290 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('  ');
  291 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  292 |
  293 |     const targetAudienceInput = page.getByPlaceholder(/Local families, Tech startups/i);
  294 |     await expect(page.getByText('Please tell us your target audience.')).toBeVisible();
  295 |     await expect(targetAudienceInput).toHaveClass(/border-red-500/);
  296 |   });
  297 |
  298 |   test('User can use Instant Build to launch storefront quickly', async ({ page }) => {
  299 |     await page.goto('/onboarding');
  300 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  301 |
  302 |     await page.getByRole('button', { name: 'Instant Build' }).click();
  303 |     await expect(page.getByText("Tell us about your business")).toBeVisible();
  304 |
  305 |     const bioInput = page.getByPlaceholder(/e.g. I run a local bakery/i);
  306 |     await bioInput.fill('I am Maya, I run a local bakery making custom vegan cakes in Portland, OR.');
  307 |
  308 |     await page.getByRole('button', { name: 'Generate Storefront' }).click();
  309 |
  310 |     // Expect it to eventually reach "You're Live!" screen
  311 |     await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  312 |   });
  313 | });
  314 |
```