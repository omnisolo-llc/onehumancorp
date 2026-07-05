# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Validation errors prevent launching without complete admin info
- Location: src/e2e/onboarding.spec.ts:180:7

# Error details

```
TimeoutError: locator.click: Timeout 15000ms exceeded.
Call log:
  - waiting for getByRole('button', { name: 'Continue' })

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
      - img [ref=e14]
      - paragraph [ref=e16]: Backend connection failed
    - generic [ref=e18]:
      - img [ref=e20]
      - generic [ref=e22]:
        - button "Back" [ref=e23] [cursor=pointer]:
          - img [ref=e24]
          - text: Back
        - heading "Where are you located?" [level=2] [ref=e26]
        - generic [ref=e27]:
          - paragraph [ref=e28]: This helps us set up your shipping and tax settings.
          - button "Save Draft" [ref=e29] [cursor=pointer]:
            - generic [ref=e33]: Save Draft
        - textbox "e.g. Portland, OR" [active] [ref=e36]: Local
        - button "Next" [ref=e38] [cursor=pointer]:
          - generic [ref=e41]: Next
  - button "Help" [ref=e44]:
    - img [ref=e45]
  - button "Open help chat" [ref=e49]:
    - generic [ref=e50]: ✨
    - generic [ref=e51]: Ask anything
  - button "Voice Assistant" [ref=e52]:
    - img [ref=e53]
  - alert [ref=e55]
```

# Test source

```ts
  98  |     await page.getByRole('button', { name: 'Next' }).click();
  99  |
  100 |     await page.waitForTimeout(5000); await expect(page.locator('input').nth(1)).toBeVisible({ timeout: 15000 });
  101 |     // Removed product assertion since fallback logic doesn't generate products
  102 |     await page.getByRole('button', { name: 'Continue' }).click();
  103 |
  104 |     await page.getByText('Classic').click();
  105 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Leo Tutor');
  106 |     await page.getByPlaceholder(/you@example.com/i).fill('leo@music.com');
  107 |     await page.getByPlaceholder(/••••••••/i).fill('pass1234');
  108 |
  109 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  110 |     await expect(page.getByText("You're Live!")).toBeVisible();
  111 |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  112 |     // expect(storedTenantId).not.toBeNull();
  113 |   });
  114 |
  115 |   test('Fatima the Food Cart Operator on a slower network', async ({ page }) => {
  116 |
  117 |
  118 |     await page.goto('/onboarding');
  119 |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  120 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  121 |
  122 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Fatima Halal Food');
  123 |     await page.getByRole('button', { name: 'Next' }).click();
  124 |
  125 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Halal food cart pickup orders');
  126 |     await page.getByRole('button', { name: 'Next' }).click();
  127 |
  128 |     await page.getByPlaceholder(/Portland, OR/i).fill('New York, NY');
  129 |     await page.getByRole('button', { name: 'Next' }).click();
  130 |
  131 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Professionals');
  132 |     await page.getByRole('button', { name: 'Next' }).click();
  133 |
  134 |     await page.waitForTimeout(5000); await expect(page.locator('input').nth(1)).toBeVisible({ timeout: 15000 });
  135 |     await page.getByRole('button', { name: 'Continue' }).click();
  136 |
  137 |     await page.getByText('Bold').click();
  138 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Fatima');
  139 |     await page.getByPlaceholder(/you@example.com/i).fill('fatima@foodcart.com');
  140 |     await page.getByPlaceholder(/••••••••/i).fill('halal123');
  141 |
  142 |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  143 |     await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 5000 });
  144 |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  145 |     // expect(storedTenantId).not.toBeNull();
  146 |   });
  147 |
  148 |   test('User can save a draft and restore it across sessions', async ({ page, context }) => {
  149 |
  150 |     let savedWizardState: Record<string, unknown> | undefined;
  151 |
  152 |     // 1. Start Wizard and Save Draft
  153 |     await page.goto('/onboarding');
  154 |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  155 |
  156 |     // Check for glassmorphism classes
  157 |     await expect(page.locator('#setup-screen')).toHaveClass(/.*glassmorphism.*/);
  158 |
  159 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  160 |
  161 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('My Restored Business');
  162 |     await page.getByRole('button', { name: 'Save Draft' }).click();
  163 |     await expect(page.getByText('Draft Saved!')).toBeVisible();
  164 |
  165 |     // Ensure localStorage is populated via zustand persist
  166 |     const lsStore = await page.evaluate(() => window.localStorage.getItem('onboarding-storage-v4'));
  167 |     expect(lsStore).toContain('My Restored Business');
  168 |
  169 |     // 2. Clear local storage to simulate device switch
  170 |     await page.evaluate(() => window.localStorage.clear());
  171 |
  172 |     // 3. Reload page and check restoration
  173 |     await page.reload();
  174 |
  175 |     // We should be restored to the first step of the wizard where we were, with the text filled
  176 |     await page.waitForTimeout(5000); await expect(page.getByText("What's the name of your business?")).toBeVisible({ timeout: 15000 });
  177 |     await expect(page.locator('input').first()).toHaveValue('My Restored Business', { timeout: 15000 });
  178 |   });
  179 |
  180 |   test('Validation errors prevent launching without complete admin info', async ({ page }) => {
  181 |
  182 |     await page.goto('/onboarding');
  183 |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  184 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  185 |
  186 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
  187 |     await page.getByRole('button', { name: 'Next' }).click();
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
> 198 |     await page.getByRole('button', { name: 'Continue' }).click({ timeout: 15000 });
      |                                                          ^ TimeoutError: locator.click: Timeout 15000ms exceeded.
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
  288 |     await page.waitForTimeout(5000); await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  289 |   });
  290 | });
  291 |
```