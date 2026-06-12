# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: client-intake-to-proposal.spec.ts >> Automated Client Intake to Proposal Generation Pipeline >> New lead submits a request and owner approves the AI drafted proposal
- Location: src/e2e/client-intake-to-proposal.spec.ts:4:7

# Error details

```
Error: expect(received).toBeTruthy()

Received: false
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Automated Client Intake to Proposal Generation Pipeline', () => {
  4  |   test('New lead submits a request and owner approves the AI drafted proposal', async ({ page, request }) => {
  5  |
  6  |     // Step 1: Simulate the form intake API submission
  7  |     const submitResponse = await request.post('/api/v1/work-intake/submit?tenant=tenant-1', {
  8  |       data: {
  9  |         name: 'Nora Customer',
  10 |         email: 'nora@example.com',
  11 |         details: 'I need a Plumbing Fix for my house'
  12 |       },
  13 |       headers: {
  14 |         'Content-Type': 'application/x-www-form-urlencoded'
  15 |       }
  16 |     });
  17 |
> 18 |     expect(submitResponse.ok()).toBeTruthy();
     |                                 ^ Error: expect(received).toBeTruthy()
  19 |
  20 |     // Step 2: Owner navigates to the unified dashboard and checks the feed
  21 |     await page.goto('/dashboard');
  22 |
  23 |     const proposalsTab = page.locator('button', { hasText: /Proposals/ }).first();
  24 |     await expect(proposalsTab).toBeVisible({ timeout: 15000 });
  25 |
  26 |     const quoteCard = page.getByTestId('quote-draft-card').first();
  27 |     await expect(quoteCard).toBeVisible();
  28 |
  29 |     await expect(page.getByText('Draft Quote: Plumbing Fix for Customer')).toBeVisible();
  30 |
  31 |     // Step 3: Owner taps "Approve & Send Proposal"
  32 |     const approveBtn = page.getByTestId('approve-quote-draft').first();
  33 |     await approveBtn.waitFor({ state: 'visible' });
  34 |     await approveBtn.click();
  35 |
  36 |     // Step 4: The card is removed from the feed (optimistic UI update)
  37 |     await expect(quoteCard).toHaveCount(0);
  38 |
  39 |     // Step 5: Simulate the client accepting the quote
  40 |     const acceptResponse = await request.post('/api/agents/approvals/simulate-quote-accepted', {
  41 |       headers: {
  42 |         'x-tenant-id': 'tenant-1',
  43 |         'x-user-id': 'default'
  44 |       }
  45 |     });
  46 |
  47 |     expect(acceptResponse.ok()).toBeTruthy();
  48 |
  49 |     // Step 6: Verify the "Draft Invoice" card is visible
  50 |     const invoiceCard = page.getByTestId('approve-send-invoice').first();
  51 |     await invoiceCard.waitFor({ state: 'visible', timeout: 15000 });
  52 |
  53 |     await expect(page.getByText('Client: Test Client')).toBeVisible();
  54 |     await expect(page.getByText('Total Amount: $1500.00')).toBeVisible();
  55 |
  56 |     // Step 7: Owner taps "Approve & Send Invoice"
  57 |     await invoiceCard.click();
  58 |
  59 |     // Step 8: The card is removed from the feed (optimistic UI update)
  60 |     await expect(invoiceCard).toHaveCount(0);
  61 |   });
  62 | });
  63 |
```