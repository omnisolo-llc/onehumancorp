# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> User can save a draft and restore it across sessions
- Location: src/e2e/onboarding.spec.ts:146:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByText('What\'s the name of your business?')
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for getByText('What\'s the name of your business?')

```

```yaml
- heading "Setup" [level=1]
- paragraph: Your business, live in minutes.
- img
- heading "10-Minute Setup Wizard" [level=2]
- paragraph: Zero tech skills needed. We do the heavy lifting. Review and add any extra details to help our AI generate the perfect store.
- button "Start My Business"
- button "Instant Build"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- alert
```

# Test source

```ts
  65  |     await expect(page.locator('input[value="Plumbing and general repairs"]')).toBeVisible();
  66  |     await page.getByRole('button', { name: 'Continue' }).click();
  67  |
  68  |     await page.getByText('Minimal').click();
  69  |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Carlos');
  70  |     await page.getByPlaceholder(/you@example.com/i).fill('carlos@example.com');
  71  |     await page.getByPlaceholder(/••••••••/i).fill('password123');
  72  |
  73  |     await page.getByRole('button', { name: 'Launch Store' }).click();
  74  |     await expect(page.getByText("You're Live!")).toBeVisible();
  75  |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  76  |     expect(storedTenantId).not.toBeNull();
  77  |   });
  78  |
  79  |   test('Leo the Music Tutor configures online bookings', async ({ page }) => {
  80  |
  81  |
  82  |     await page.goto('/onboarding');
  83  |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  84  |     await page.getByRole('button', { name: 'Start My Business' }).click();
  85  |
  86  |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Leo Guitar Lessons');
  87  |     await page.getByRole('button', { name: 'Next' }).click();
  88  |
  89  |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Guitar tutoring online');
  90  |     await page.getByRole('button', { name: 'Next' }).click();
  91  |
  92  |     await page.getByPlaceholder(/Portland, OR/i).fill('Remote');
  93  |     await page.getByRole('button', { name: 'Next' }).click();
  94  |
  95  |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Students');
  96  |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  97  |
  98  |     await expect(page.locator('input[value="Guitar tutoring online"]')).toBeVisible();
  99  |     // Removed product assertion since fallback logic doesn't generate products
  100 |     await page.getByRole('button', { name: 'Continue' }).click();
  101 |
  102 |     await page.getByText('Classic').click();
  103 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Leo Tutor');
  104 |     await page.getByPlaceholder(/you@example.com/i).fill('leo@music.com');
  105 |     await page.getByPlaceholder(/••••••••/i).fill('pass1234');
  106 |
  107 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  108 |     await expect(page.getByText("You're Live!")).toBeVisible();
  109 |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  110 |     expect(storedTenantId).not.toBeNull();
  111 |   });
  112 |
  113 |   test('Fatima the Food Cart Operator on a slower network', async ({ page }) => {
  114 |
  115 |
  116 |     await page.goto('/onboarding');
  117 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
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
  130 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
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
> 165 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
      |                                                                       ^ Error: expect(locator).toBeVisible() failed
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
  185 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
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
  257 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  258 |
  259 |     const audienceInput = page.getByPlaceholder(/Local families, Tech startups/i);
  260 |     await expect(page.getByText('Please tell us your target audience.')).toBeVisible();
  261 |     await expect(audienceInput).toHaveClass(/border-\[#FF3B30\]/);
  262 |   });
  263 |
  264 |   test('User can use Instant Build to launch storefront quickly', async ({ page }) => {
  265 |     await page.goto('/onboarding');
```