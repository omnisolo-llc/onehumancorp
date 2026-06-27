# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Validation errors prevent launching without complete admin info
- Location: src/e2e/onboarding.spec.ts:187:7

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
  - button "Open help chat" [ref=e48]:
    - generic [ref=e49]: ✨
    - generic [ref=e50]: Ask anything
  - button "Voice Assistant" [ref=e51]:
    - img [ref=e52]
  - alert [ref=e54]
```

# Test source

```ts
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
  119 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
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
  134 |     await expect(page.locator('input').nth(1)).toBeVisible({ timeout: 15000 });
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
  154 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
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
  173 |     await context.route('**/api/onboarding/draft', async (route, request) => {
  174 |       if (request.method() === 'GET') {
  175 |         route.fulfill({ status: 200, json: { wizardState: { step: 1, chatStep: 1, businessName: 'My Restored Business' } } });
  176 |       } else {
  177 |         route.fulfill({ status: 200, json: { success: true, organization_id: 'test-tenant-123' } });
  178 |       }
  179 |     });
  180 |     await page.reload();
  181 |
  182 |     // We should be restored to the first step of the wizard where we were, with the text filled
  183 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  184 |     await expect(page.locator('input').first()).toHaveValue('My Restored Business', { timeout: 15000 });
  185 |   });
  186 |
  187 |   test('Validation errors prevent launching without complete admin info', async ({ page }) => {
  188 |
  189 |     await page.goto('/onboarding');
  190 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  191 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  192 |
  193 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
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
> 205 |     await page.getByRole('button', { name: 'Continue' }).click({ timeout: 15000 });
      |                                                          ^ TimeoutError: locator.click: Timeout 15000ms exceeded.
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
  294 |     await expect(page.getByText("Style & Team")).toBeVisible({ timeout: 15000 });
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