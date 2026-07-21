import { test, expect } from '@playwright/test';

test.describe('Vendor Credit Hub E2E Workflows', () => {
  test('Owner should be able to manage trade credit, negotiate terms, run sweeps and factor invoices', async ({ request, page }) => {
    // 1. Authenticate / Login to get the authorization token.
    const loginResponse = await request.post('/api/v1/auth/login', {
      data: {
        username: 'test@example.com',
        password: 'password123',
        organization_id: 'e2e-tenant'
      }
    });
    expect(loginResponse.status()).toBe(200);
    const loginData = await loginResponse.json();
    const token = loginData.token;
    expect(token).toBeDefined();

    const headers = {
      'Authorization': `Bearer ${token}`,
      'x-tenant-id': 'e2e-tenant',
      'Content-Type': 'application/json'
    };

    // 2. Query underwriting credit capacity & dynamic score
    const capacityResponse = await request.get('/api/v1/ui/credit/capacity', { headers });
    expect(capacityResponse.status()).toBe(200);
    const capacityData = await capacityResponse.json();
    expect(capacityData.tenant_id).toBe('e2e-tenant');
    expect(capacityData.approved_limit_usd).toBeGreaterThan(0);
    expect(capacityData.dynamic_score).toBeGreaterThanOrEqual(70);

    // 3. Fetch vendor relations and trigger AI-led term negotiation
    const vendorsResponse = await request.get('/api/v1/ui/credit/vendors', { headers });
    expect(vendorsResponse.status()).toBe(200);
    const vendorsList = await vendorsResponse.json();
    expect(vendorsList.length).toBeGreaterThan(0);
    const targetVendor = vendorsList[0];

    const negotiateResponse = await request.post('/api/v1/ui/credit/negotiate', {
      headers,
      data: { vendor_relation_id: targetVendor.id }
    });
    expect(negotiateResponse.status()).toBe(200);
    const negotiateData = await negotiateResponse.json();
    expect(negotiateData.term_status).toBe('NEGOTIATING');
    expect(negotiateData.current_terms).toBe('NET_30');

    // 4. Simulate sales ledger sweep for a supplier invoice and check balance accrual
    const supplierInvoiceId = 'inv-e2e-sweep-123';
    const sweepResponse = await request.post('/api/v1/ui/credit/sweep', {
      headers,
      data: {
        supplier_invoice_id: supplierInvoiceId,
        sales_amount: 1000.0 // 10% sweep = $100
      }
    });
    expect(sweepResponse.status()).toBe(200);
    const sweepData = await sweepResponse.json();
    expect(sweepData.supplier_invoice_id).toBe(supplierInvoiceId);
    expect(sweepData.accumulated_sweep_usd).toBe(100.0);

    // Run a second sweep with large sales to hit maximum limit
    const sweepResponse2 = await request.post('/api/v1/ui/credit/sweep', {
      headers,
      data: {
        supplier_invoice_id: supplierInvoiceId,
        sales_amount: 3000.0
      }
    });
    expect(sweepResponse2.status()).toBe(200);
    const sweepData2 = await sweepResponse2.json();
    expect(sweepData2.accumulated_sweep_usd).toBe(220.0); // 100 + 120 (max_sweep_usd) = 220

    // 5. Submit client invoice for factoring and assert payout balances
    const clientInvoiceId = 'client-inv-e2e-factoring-456';
    const factoringResponse = await request.post('/api/v1/ui/credit/factor', {
      headers,
      data: {
        client_invoice_id: clientInvoiceId,
        invoice_amount: 10000.0
      }
    });
    expect(factoringResponse.status()).toBe(200);
    const factoringData = await factoringResponse.json();
    expect(factoringData.client_invoice_id).toBe(clientInvoiceId);
    expect(factoringData.invoice_amount).toBe(10000.0);
    expect(factoringData.advanced_amount_usd).toBe(8330.0); // 10000 * 0.85 * 0.98 = 8330.0
    expect(factoringData.factoring_status).toBe('DISBURSED');

    // 6. Navigate to premium credit-hub page with injected local storage token
    await page.goto('/');
    const tokenVal = token;
    await page.evaluate((tok) => {
        localStorage.setItem('token', tok);
        localStorage.setItem('tenant_id', 'e2e-tenant');
        localStorage.setItem('tenant', 'e2e-tenant');
        localStorage.setItem('has_onboarded', 'true');
    }, tokenVal);

    // Navigate to the credit-hub page
    await page.goto('/credit-hub');

    // Check if the Credit Capacity pulse card renders properly
    const creditCard = page.locator('#credit-capacity-card');
    await expect(creditCard).toBeVisible();
    await expect(creditCard).toContainText('Capacity');

    // Check Tab switching
    const tabFactoring = page.locator('#tab-factoring');
    await expect(tabFactoring).toBeVisible();
    await tabFactoring.click();

    // Verify factoring form is visible
    const factoringTabContent = page.locator('#factoring-tab');
    await expect(factoringTabContent).toBeVisible();

    // Fill in factoring inputs and submit
    await page.fill('#client-invoice-input', 'client-inv-e2e-factoring-456');
    await page.fill('#invoice-amount-input', '10000');
    await page.click('#factor-btn');

    // Verify factoring result card is successfully rendered on UI
    const factoringResultCard = page.locator('#factoring-result');
    await expect(factoringResultCard).toBeVisible();
    await expect(factoringResultCard).toContainText('DISBURSED');
  });
});
