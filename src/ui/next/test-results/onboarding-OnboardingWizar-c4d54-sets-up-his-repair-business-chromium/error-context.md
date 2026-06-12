# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Carlos the Handyman sets up his repair business
- Location: src/e2e/onboarding.spec.ts:46:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('input[value="Plumbing and general repairs"]')
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('input[value="Plumbing and general repairs"]')

```

```yaml
- heading "Setup" [level=1]
- paragraph: Your business, live in minutes.
- text: Backend connection failed
- img
- button "Back":
  - img
  - text: Back
- heading "Where are you located?" [level=2]
- paragraph: This helps us set up your shipping and tax settings.
- button "Save Draft"
- textbox "e.g. Portland, OR": Austin, TX
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
  16  |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
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
  30  |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  31  |
  32  |     await expect(page.locator('input[value="I bake custom vegan cakes f..."]')).toBeVisible();
  33  |     await page.getByRole('button', { name: 'Continue' }).click();
  34  |
  35  |     await page.getByText('Modern').click();
  36  |     await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya Smith');
  37  |     await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
  38  |     await page.getByPlaceholder(/••••••••/i).fill('mypassword123');
  39  |
  40  |     await page.getByRole('button', { name: 'Launch Store' }).click();
  41  |     await expect(page.getByText("You're Live!")).toBeVisible();
  42  |     const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
  43  |     expect(storedTenantId).not.toBeNull();
  44  |   });
  45  |
  46  |   test('Carlos the Handyman sets up his repair business', async ({ page }) => {
  47  |
  48  |
  49  |     await page.goto('/onboarding');
  50  |     await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
  51  |     await page.getByRole('button', { name: 'Start My Business' }).click();
  52  |
  53  |     await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Carlos Fixes It');
  54  |     await page.getByRole('button', { name: 'Next' }).click();
  55  |
  56  |     await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Plumbing and general repairs');
  57  |     await page.getByRole('button', { name: 'Next' }).click();
  58  |
  59  |     await page.getByPlaceholder(/Portland, OR/i).fill('Austin, TX');
  60  |     await page.getByRole('button', { name: 'Next' }).click();
  61  |
  62  |     await page.getByPlaceholder(/Local families, Tech startups/i).fill('Homeowners');
  63  |     await page.getByRole('button', { name: 'Generate My Business' }).click();
  64  |
> 65  |     await expect(page.locator('input[value="Plumbing and general repairs"]')).toBeVisible();
      |                                                                               ^ Error: expect(locator).toBeVisible() failed
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
  165 |     await expect(page.getByText("What's the name of your business?")).toBeVisible();
```