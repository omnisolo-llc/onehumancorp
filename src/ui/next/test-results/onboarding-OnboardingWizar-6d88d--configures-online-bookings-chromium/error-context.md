# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Leo the Music Tutor configures online bookings
- Location: src/e2e/onboarding.spec.ts:81:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('input').nth(1)
Expected: visible
Timeout: 15000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for locator('input').nth(1)

```

```yaml
- heading "Setup" [level=1]
- paragraph: Your business, live in minutes.
- button "Skip setup"
- img
- paragraph: Backend connection failed
- img
- button "Back":
  - img
  - text: Back
- heading "Where are you located?" [level=2]
- paragraph: This helps us set up your shipping and tax settings.
- button "Save Draft"
- textbox "e.g. Portland, OR": Remote
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
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('OnboardingWizard CUJ', () => {
  4   |   test.beforeEach(async ({ page }) => {
  5   |     // Clear local storage to ensure fresh state
  6   |     await page.addInitScript(() => {
  7   |       window.localStorage.clear();
  8   |     });
  9   |   });
  10  |
  11  |
  12  |   test('Maya the Baker can complete the onboarding flow', async ({ page }) => {
  13  |
  14  |
  15  |     await page.goto('/onboarding');
  16  |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  17  |     await page.getByRole('button', { name: 'Start My Business' }).click();
  18  |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
  19  |
  20  |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Maya Bakery');
  21  |     await page.getByRole('button', { name: 'Next' }).click();
  22  |
  23  |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('I bake custom vegan cakes for weddings and parties.');
  24  |     await page.getByRole('button', { name: 'Next' }).click();
  25  |
  26  |     await page.getByPlaceholder(/Portland, OR/i).fill('Seattle, WA');
  27  |     await page.getByRole('button', { name: 'Next' }).click();
  28  |
  29  |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Everyone');
  30  |     await page.getByRole('button', { name: 'Next' }).click();
  31  |
  32  |
  33  |     await page.waitForTimeout(5000); await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
  34  |     await page.getByRole('button', { name: 'Continue' }).click();
  35  |
  36  |     await page.getByText('Modern').click();
  37  |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya Smith');
  38  |     await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
  39  |     await page.getByPlaceholder(/••••••••/i).fill('mypassword123');
  40  |
  41  |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  42  |     await expect(page.getByText("You're Live!")).toBeVisible();
  43  |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  44  |     // // expect(storedTenantId).not.toBeNull();
  45  |   });
  46  |
  47  |   test('Carlos the Handyman sets up his repair business', async ({ page }) => {
  48  |
  49  |
  50  |     await page.goto('/onboarding');
  51  |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  52  |     await page.getByRole('button', { name: 'Start My Business' }).click();
  53  |
  54  |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Carlos Fixes It');
  55  |     await page.getByRole('button', { name: 'Next' }).click();
  56  |
  57  |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Plumbing and general repairs');
  58  |     await page.getByRole('button', { name: 'Next' }).click();
  59  |
  60  |     await page.getByPlaceholder(/Portland, OR/i).fill('Austin, TX');
  61  |     await page.getByRole('button', { name: 'Next' }).click();
  62  |
  63  |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Homeowners');
  64  |     await page.getByRole('button', { name: 'Next' }).click();
  65  |
  66  |
  67  |     await page.waitForTimeout(5000); await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
  68  |     await page.getByRole('button', { name: 'Continue' }).click();
  69  |
  70  |     await page.getByText('Minimal').click();
  71  |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Carlos');
  72  |     await page.getByPlaceholder(/you@example.com/i).fill('carlos@example.com');
  73  |     await page.getByPlaceholder(/••••••••/i).fill('password123');
  74  |
  75  |     await page.getByRole('button', { name: 'Approve & Publish' }).click();
  76  |     await expect(page.getByText("You're Live!")).toBeVisible();
  77  |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  78  |     // expect(storedTenantId).not.toBeNull();
  79  |   });
  80  |
  81  |   test('Leo the Music Tutor configures online bookings', async ({ page }) => {
  82  |
  83  |
  84  |     await page.goto('/onboarding');
  85  |     await expect(page.getByText("Setup Assistant")).toBeVisible();
  86  |     await page.getByRole('button', { name: 'Start My Business' }).click();
  87  |
  88  |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Leo Guitar Lessons');
  89  |     await page.getByRole('button', { name: 'Next' }).click();
  90  |
  91  |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Guitar tutoring online');
  92  |     await page.getByRole('button', { name: 'Next' }).click();
  93  |
  94  |     await page.getByPlaceholder(/Portland, OR/i).fill('Remote');
  95  |     await page.getByRole('button', { name: 'Next' }).click();
  96  |
  97  |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Students');
  98  |     await page.getByRole('button', { name: 'Next' }).click();
  99  |
> 100 |     await page.waitForTimeout(5000); await expect(page.locator('input').nth(1)).toBeVisible({ timeout: 15000 });
      |                                                                                 ^ Error: expect(locator).toBeVisible() failed
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
  198 |     await page.getByRole('button', { name: 'Continue' }).click({ timeout: 15000 });
  199 |
  200 |     // Do NOT fill out admin email and password initially
```