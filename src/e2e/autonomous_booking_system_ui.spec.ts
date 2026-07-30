
test.describe('Autonomous Booking System UI', () => {

    // 1. Visit booking page

    // 2. Fill the form
    await page.fill('input[type="text"]', 'Jane Doe');
    await page.fill('input[type="email"]', 'jane@example.com');
    await page.fill('textarea', 'I need a drum lesson.');

    // 3. Date Selection triggers slot loading
    const dateQuery = new Date().toISOString().split('T')[0];
    await page.fill('input[type="date"]', dateQuery);

    // Wait for the mock slots to load (9:00 AM, 11:00 AM, etc.)
    await page.waitForSelector('button:has-text("09:00 AM")');
    await page.click('button:has-text("09:00 AM")');

    // 4. Submit
    // Route mock to avoid actual backend errors if not fully seeded
                stripe_url: 'https://checkout.stripe.com/pay/mock_session',
                status: 'pending_payment'

    await page.click('button:has-text("Confirm Booking")');

    // 5. Verify deposit step
    await expect(page.getByTestId('booking-checkout-container')).toBeVisible();
    await expect(page.getByTestId('pay-deposit-btn')).toHaveAttribute('href', /checkout\.stripe\.com/);

    // 1. Visit admin bookings dashboard

    // Route mocks
        if (route.request().method() === 'GET') {

        if (route.request().method() === 'GET') {

    await page.reload();

    // 2. Check rendered content
    await expect(page.getByText('Studio A')).toBeVisible();

    // 3. Create Resource
    const newResNameInput = page.locator('input[type="text"]').first();
    await newResNameInput.fill('New Tutor Leo');

    // 4. Create Availability Block
    // Wait for the select to be populated
    await page.selectOption('select', 'res-1');
    const timeInputs = page.locator('input[type="datetime-local"]');
    await timeInputs.nth(0).fill('2025-02-01T09:00');
    await timeInputs.nth(1).fill('2025-02-01T17:00');
