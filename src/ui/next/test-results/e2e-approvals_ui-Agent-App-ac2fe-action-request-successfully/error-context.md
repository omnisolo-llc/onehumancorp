# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: e2e/approvals_ui.spec.ts >> Agent Approvals Workflow E2E >> should approve an action request successfully
- Location: e2e/approvals_ui.spec.ts:61:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('button:has-text("The Promoter")').locator('text=1 item awaiting approval')
Expected: visible
Timeout: 10000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 10000ms
  - waiting for locator('button:has-text("The Promoter")').locator('text=1 item awaiting approval')

```

```yaml
- heading "Your Team" [level=1]
- paragraph: Invisible specialized AI teams
- button "M The Manager Active and running":
  - text: M
  - heading "The Manager" [level=3]
  - paragraph: Active and running
  - img
- button "P The Promoter Active and running":
  - text: P
  - heading "The Promoter" [level=3]
  - paragraph: Active and running
  - img
- button "S The Salesperson Active and running":
  - text: S
  - heading "The Salesperson" [level=3]
  - paragraph: Active and running
  - img
- button "A The Ambassador Active and running":
  - text: A
  - heading "The Ambassador" [level=3]
  - paragraph: Active and running
  - img
- button "A The Accountant Active and running":
  - text: A
  - heading "The Accountant" [level=3]
  - paragraph: Active and running
  - img
- button "P The Protector Active and running":
  - text: P
  - heading "The Protector" [level=3]
  - paragraph: Active and running
  - img
- button "A The Advisor Active and running":
  - text: A
  - heading "The Advisor" [level=3]
  - paragraph: Active and running
  - img
- button "Help":
  - img
- alert
```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('Agent Approvals Workflow E2E', () => {
  4   |   test.beforeEach(async ({ page }) => {
  5   |     // Navigate to the Team page directly (assuming unauthenticated access is allowed for e2e tests or handled by mock)
  6   |     await page.goto('http://localhost:3000/team');
  7   |   });
  8   |
  9   |   test('should display the Team page with all departments', async ({ page }) => {
  10  |     await expect(page.locator('text=Your Team')).toBeVisible({ timeout: 10000 });
  11  |     await expect(page.locator('text=Invisible specialized AI teams')).toBeVisible();
  12  |
  13  |     // Verify all 7 departments are listed
  14  |     await expect(page.locator('text=The Manager')).toBeVisible({ timeout: 10000 });
  15  |     await expect(page.locator('text=The Promoter')).toBeVisible();
  16  |     await expect(page.locator('text=The Salesperson')).toBeVisible();
  17  |     await expect(page.locator('text=The Ambassador')).toBeVisible();
  18  |     await expect(page.locator('text=The Accountant')).toBeVisible();
  19  |     await expect(page.locator('text=The Protector')).toBeVisible();
  20  |     await expect(page.locator('text=The Advisor')).toBeVisible();
  21  |   });
  22  |
  23  |   test('should navigate to a department inbox and view pending requests', async ({ page }) => {
  24  |     // We'll mock the API response for pending approvals
  25  |     await page.route('/api/agents/approvals', async route => {
  26  |       const json = {
  27  |         pending_approvals: [
  28  |           {
  29  |             id: 'req-123',
  30  |             tenant_id: 'org1',
  31  |             department: 'Marketing',
  32  |             description: 'Drafted social media post for new product launch.',
  33  |             status: 'Pending',
  34  |             action_risk: 'High',
  35  |             feature_type: 'social_calendar'
  36  |           }
  37  |         ],
  38  |         next_cursor: null
  39  |       };
  40  |       await route.fulfill({ json });
  41  |     });
  42  |
  43  |     await page.goto('http://localhost:3000/team');
  44  |
  45  |     // Wait for the approvals to load and the badge to appear
  46  |     const promoterCard = page.locator('button:has-text("The Promoter")');
  47  |     await expect(promoterCard.locator('text=1 item awaiting approval')).toBeVisible({ timeout: 10000 });
  48  |
  49  |     // Click on the department
  50  |     await promoterCard.click();
  51  |
  52  |     // Verify inbox UI
  53  |     await expect(page.locator('text=Approval Inbox')).toBeVisible();
  54  |     await expect(page.locator('text=Review drafted actions for The Promoter.')).toBeVisible();
  55  |
  56  |     // Verify the specific approval request is visible
  57  |     await expect(page.locator('text=Drafted social media post for new product launch.')).toBeVisible();
  58  |     await expect(page.locator('text=7-Day Social Calendar Generated')).toBeVisible(); // specific feature UI
  59  |   });
  60  |
  61  |   test('should approve an action request successfully', async ({ page }) => {
  62  |     await page.route('/api/agents/approvals', async route => {
  63  |       const json = {
  64  |         pending_approvals: [
  65  |           {
  66  |             id: 'req-123',
  67  |             tenant_id: 'org1',
  68  |             department: 'Marketing',
  69  |             description: 'Drafted social media post',
  70  |             status: 'Pending',
  71  |             action_risk: 'High'
  72  |           }
  73  |         ],
  74  |         next_cursor: null
  75  |       };
  76  |       await route.fulfill({ json });
  77  |     });
  78  |
  79  |     // Mock the POST approval endpoint
  80  |     let approveCalled = false;
  81  |     await page.route('/api/agents/approvals/req-123', async route => {
  82  |       if (route.request().method() === 'POST') {
  83  |         const postData = route.request().postDataJSON();
  84  |         if (postData && postData.approved === true) {
  85  |           approveCalled = true;
  86  |           await route.fulfill({ json: { success: true } });
  87  |           return;
  88  |         }
  89  |       }
  90  |       await route.fallback();
  91  |     });
  92  |
  93  |     await page.goto('http://localhost:3000/team');
  94  |
  95  |     const promoterCard = page.locator('button:has-text("The Promoter")');
> 96  |     await expect(promoterCard.locator('text=1 item awaiting approval')).toBeVisible({ timeout: 10000 });
      |                                                                         ^ Error: expect(locator).toBeVisible() failed
  97  |     await promoterCard.click();
  98  |
  99  |     // Click Approve
  100 |     await page.locator('button:has-text("Approve")').click({ timeout: 10000 });
  101 |
  102 |     // The request should disappear
  103 |     await expect(page.locator('text=Drafted social media post')).not.toBeVisible();
  104 |     expect(approveCalled).toBeTruthy();
  105 |   });
  106 |
  107 |   test('should reject an action request successfully', async ({ page }) => {
  108 |     await page.route('/api/agents/approvals', async route => {
  109 |       const json = {
  110 |         pending_approvals: [
  111 |           {
  112 |             id: 'req-123',
  113 |             tenant_id: 'org1',
  114 |             department: 'Marketing',
  115 |             description: 'Drafted social media post',
  116 |             status: 'Pending',
  117 |             action_risk: 'High'
  118 |           }
  119 |         ],
  120 |         next_cursor: null
  121 |       };
  122 |       await route.fulfill({ json });
  123 |     });
  124 |
  125 |     // Mock the POST rejection endpoint
  126 |     let rejectCalled = false;
  127 |     await page.route('/api/agents/approvals/req-123', async route => {
  128 |       if (route.request().method() === 'POST') {
  129 |         const postData = route.request().postDataJSON();
  130 |         if (postData && postData.approved === false) {
  131 |           rejectCalled = true;
  132 |           await route.fulfill({ json: { success: true } });
  133 |           return;
  134 |         }
  135 |       }
  136 |       await route.fallback();
  137 |     });
  138 |
  139 |     await page.goto('http://localhost:3000/team');
  140 |
  141 |     const promoterCard = page.locator('button:has-text("The Promoter")');
  142 |     await expect(promoterCard.locator('text=1 item awaiting approval')).toBeVisible({ timeout: 10000 });
  143 |     await promoterCard.click();
  144 |
  145 |     // Click Reject
  146 |     await page.locator('button:has-text("Reject / Edit")').click({ timeout: 10000 });
  147 |
  148 |     // The request should disappear
  149 |     await expect(page.locator('text=Drafted social media post')).not.toBeVisible();
  150 |     expect(rejectCalled).toBeTruthy();
  151 |   });
  152 |
  153 |   test('should navigate back to the main team page from the inbox', async ({ page }) => {
  154 |      await page.route('/api/agents/approvals', async route => {
  155 |       const json = {
  156 |         pending_approvals: [],
  157 |         next_cursor: null
  158 |       };
  159 |       await route.fulfill({ json });
  160 |     });
  161 |
  162 |     await page.goto('http://localhost:3000/team');
  163 |     await page.locator('button:has-text("The Promoter")').click();
  164 |
  165 |     // Check we are in inbox
  166 |     await expect(page.locator('text=Approval Inbox')).toBeVisible();
  167 |
  168 |     // Click back button (the SVG button)
  169 |     await page.locator('button:has(svg)').first().click();
  170 |
  171 |     // Check we are back on Team page
  172 |     await expect(page.locator('text=Your Team')).toBeVisible({ timeout: 10000 });
  173 |   });
  174 | });
  175 |
```