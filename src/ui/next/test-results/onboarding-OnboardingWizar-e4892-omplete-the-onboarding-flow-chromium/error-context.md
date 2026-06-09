# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> OnboardingWizard CUJ >> Maya the Baker can complete the onboarding flow
- Location: src/e2e/onboarding.spec.ts:52:7

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for getByRole('button', { name: 'Generate My Business' })

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
        - button "Back" [ref=e11]:
          - img
          - text: Back
        - heading "Where are you located?" [level=2] [ref=e13]
        - generic [ref=e14]:
          - paragraph [ref=e15]: This helps us set up your shipping and tax settings.
          - button "Save Draft" [ref=e16]:
            - generic [ref=e17]:
              - img [ref=e18]
              - generic [ref=e21]: Save Draft
        - textbox "e.g. Portland, OR" [active] [ref=e24]: Seattle, WA
        - button "Next Step" [ref=e26]:
          - generic [ref=e27]:
            - img [ref=e28]
            - generic [ref=e30]: Next Step
  - button "Help" [ref=e33]:
    - img [ref=e34]
  - button "Open help chat" [ref=e37]:
    - generic [ref=e38]: ✨
    - generic [ref=e39]: Ask anything
  - button "Open Next.js Dev Tools" [ref=e45] [cursor=pointer]:
    - img [ref=e46]
  - alert [ref=e49]
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
  9   |
  10  |     await page.route('/api/onboarding/intake', async route => {
  11  |       await route.fulfill({
  12  |         status: 200,
  13  |         contentType: 'application/json',
  14  |         body: JSON.stringify({
  15  |           business_type: "Test Business Type",
  16  |           product_name: "Test Product",
  17  |           product_price: "99.99",
  18  |           location: "Test City",
  19  |           ai_agents: ["Operations", "Marketing"]
  20  |         })
  21  |       });
  22  |     });
  23  |
  24  |     await page.route('/api/onboarding/start', async route => {
  25  |       await route.fulfill({
  26  |         status: 200,
  27  |         contentType: 'application/json',
  28  |         body: JSON.stringify({
  29  |           message: "Your business has been successfully launched.",
  30  |           tenant_id: "test-tenant-id"
  31  |         })
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
> 67  |     await page.getByRole('button', { name: 'Generate My Business' }).click();
      |                                                                      ^ Error: locator.click: Test timeout of 30000ms exceeded.
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
  132 |     await expect(page.locator('input[value="Guitar tutoring online"]')).toBeVisible();
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
```