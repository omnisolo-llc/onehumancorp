# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Leo the Music Tutor configures online bookings
- Location: src/e2e/onboarding.spec.ts:113:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('input[value="Guitar tutoring online"]')
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('input[value="Guitar tutoring online"]')

```

```yaml
- heading "Setup" [level=1]
- paragraph: Your business, live in minutes.
- button "Back":
  - img
  - text: Back
- heading "Review Details" [level=2]
- paragraph: Here's what our AI figured out. Feel free to tweak these.
- button "Save Draft"
- text: Business Name
- textbox: My Business
- text: Business Type
- textbox: Test Business Type
- text: Categories (Comma separated)
- textbox: physical
- text: First Product
- textbox: First Product
- text: Price
- textbox: "10.00"
- button "Continue"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- alert
```

# Test source

```ts
  32  |       });
  33  |     });
  34  |
  35  |     let savedWizardState: Record<string, unknown> | null = null;
  36  |     await page.route('/api/onboarding/draft', async route => {
  37  |       if (route.request().method() === 'POST') {
  38  |         const body = await route.request().postDataJSON();
  39  |         savedWizardState = body.wizardState;
  40  |         await route.fulfill({ status: 200, json: {} });
  41  |       } else {
  42  |         await route.fulfill({ status: 200, json: { wizardState: savedWizardState } });
  43  |       }
  44  |     });
  45  |
  46  |     await page.route('/api/onboarding/state', async route => {
  47  |       await route.fulfill({ status: 200, json: { wizardState: null } });
  48  |     });
  49  |   });
  50  |
  51  |
  52  |   test('Maya the Baker can complete the onboarding flow', async ({ page }) => {
  53  |
  54  |
  55  |     await page.goto('/onboarding');
  56  |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  57  |     await page.getByRole('button', { name: 'Start My Business' }).click();
  58  |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  59  |
  60  |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Maya Bakery');
  61  |     await page.getByRole('button', { name: 'Next Step' }).click();
  62  |
  63  |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('I bake custom vegan cakes for weddings and parties.');
  64  |     await page.getByRole('button', { name: 'Next Step' }).click();
  65  |
  66  |     await page.getByPlaceholder(/Portland, OR/i).fill('Seattle, WA');
  67  |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  68  |
  69  |     await expect(page.locator('input[value="I bake custom vegan cakes f..."]')).toBeVisible();
  70  |     await page.getByRole('button', { name: 'Continue' }).click();
  71  |
  72  |     await page.getByText('Modern').click();
  73  |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya Smith');
  74  |     await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
  75  |     await page.getByPlaceholder(/••••••••/i).fill('mypassword123');
  76  |
  77  |     await page.getByRole('button', { name: 'Launch Store' }).click();
  78  |     await expect(page.getByText("You're Live!")).toBeVisible();
  79  |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  80  |     expect(storedTenantId).not.toBeNull();
  81  |   });
  82  |
  83  |   test('Carlos the Handyman sets up his repair business', async ({ page }) => {
  84  |
  85  |
  86  |     await page.goto('/onboarding');
  87  |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  88  |     await page.getByRole('button', { name: 'Start My Business' }).click();
  89  |
  90  |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Carlos Fixes It');
  91  |     await page.getByRole('button', { name: 'Next Step' }).click();
  92  |
  93  |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Plumbing and general repairs');
  94  |     await page.getByRole('button', { name: 'Next Step' }).click();
  95  |
  96  |     await page.getByPlaceholder(/Portland, OR/i).fill('Austin, TX');
  97  |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  98  |
  99  |     await expect(page.locator('input[value="Plumbing and general repairs"]')).toBeVisible();
  100 |     await page.getByRole('button', { name: 'Continue' }).click();
  101 |
  102 |     await page.getByText('Minimal').click();
  103 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Carlos');
  104 |     await page.getByPlaceholder(/you@example.com/i).fill('carlos@example.com');
  105 |     await page.getByPlaceholder(/••••••••/i).fill('password123');
  106 |
  107 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  108 |     await expect(page.getByText("You're Live!")).toBeVisible();
  109 |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  110 |     expect(storedTenantId).not.toBeNull();
  111 |   });
  112 |
  113 |   test('Leo the Music Tutor configures online bookings', async ({ page }) => {
  114 |
  115 |
  116 |     await page.goto('/onboarding');
  117 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  118 |     await page.getByRole('button', { name: 'Start My Business' }).click();
  119 |
  120 |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Leo Guitar Lessons');
  121 |     await page.getByRole('button', { name: 'Next Step' }).click();
  122 |
  123 |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Guitar tutoring online');
  124 |     await page.getByRole('button', { name: 'Next Step' }).click();
  125 |
  126 |     await page.getByPlaceholder(/Portland, OR/i).fill('Remote');
  127 |     await page.getByRole('button', { name: 'Next Step' }).click();
  128 |
  129 |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Students');
  130 |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  131 |
> 132 |     await expect(page.locator('input[value="Guitar tutoring online"]')).toBeVisible();
      |                                                                         ^ Error: expect(locator).toBeVisible() failed
  133 |     // Removed product assertion since fallback logic doesn't generate products
  134 |     await page.getByRole('button', { name: 'Continue' }).click();
  135 |
  136 |     await page.getByText('Classic').click();
  137 |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Leo Tutor');
  138 |     await page.getByPlaceholder(/you@example.com/i).fill('leo@music.com');
  139 |     await page.getByPlaceholder(/••••••••/i).fill('pass1234');
  140 |
  141 |     await page.getByRole('button', { name: 'Launch Store' }).click();
  142 |     await expect(page.getByText("You're Live!")).toBeVisible();
  143 |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  144 |     expect(storedTenantId).not.toBeNull();
  145 |   });
  146 |
  147 |   test('Fatima the Food Cart Operator on a slower network', async ({ page }) => {
  148 |
  149 |
  150 |     await page.goto('/onboarding');
  151 |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
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
```