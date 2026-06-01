# Maya the Home Baker CUJ

**Persona:** Maya (28, non-technical home baker)
**Business Concept:** Custom cake shop run from home.
**Operating Plan:** Sell cakes via a beautiful storefront, accept online orders.

## CUJ Workflow: storefront setup to order completion

1. **Login & Onboarding**:
   - Action: Maya logs into the OHC platform.
   - Expected State: Maya is redirected to the dashboard.

2. **Business Configuration**:
   - Action: Maya enters business details: "Maya's Custom Cakes".
   - Expected State: Business name is updated and visible in settings.

3. **Product Addition**:
   - Action: Maya adds a new product: "Custom Vegan Chocolate Cake", Price: "$50.00".
   - Expected State: Product appears in the product list.

4. **Storefront Verification**:
   - Action: Maya navigates to her public storefront URL.
   - Expected State: Storefront shows "Maya's Custom Cakes" and the "Custom Vegan Chocolate Cake".

5. **Customer Order Placement**:
   - Action: Customer (simulated) clicks "Order" on the cake and completes checkout.
   - Expected State: Order is created in the system.

6. **Order Fulfillment/Verification**:
   - Action: Maya refreshes her dashboard orders section.
   - Expected State: New order for "Custom Vegan Chocolate Cake" is visible with "PAID" status.

## Validation Points
- UI State: Dashboard shows the new business name and product.
- Storefront: Public page renders correctly with the product.
- Persisted Data: Database contains the order linked to Maya's tenant.
- Final Proof: Maya sees the order in her dashboard.
