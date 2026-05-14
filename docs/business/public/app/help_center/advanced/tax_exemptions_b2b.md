# B2B Tax Exemptions & Reseller Logic

Selling Business-to-Business (B2B) often requires different tax rules than selling Business-to-Consumer (B2C). This guide explains how to properly configure your store to handle tax-exempt purchases, reseller certificates, and bulk pricing tiers.

## Enabling B2B Mode

Before you can offer tax-exempt purchasing, you must enable B2B mode on your store.

1. Go to **My Store > Settings > B2B Features**.
2. Click the toggle to **Enable B2B Wholesale Mode**.
3. Once enabled, a new section will appear in your Customer profiles allowing you to tag specific accounts as "Wholesale" or "Tax Exempt."

## Collecting Reseller Certificates

You cannot simply stop charging tax because a customer asks you to. You must collect and store their official state-issued Reseller Certificate (sometimes called a Sales Tax Exemption Certificate).

1. In the B2B Settings page, enable the **Require Certificate Upload** option.
2. When a customer registers for a wholesale account on your website, they will be prompted to upload a PDF or image of their certificate.
3. You will receive an email notification to review the document.
4. Go to **Customers > Pending Approvals**. Review the document to ensure the name matches the business and the expiration date is valid.
5. If everything looks correct, click **Approve Exemption**.

## Configuring Tax Rules for Approved Customers

Once a customer is approved as tax-exempt, OHC handles the checkout logic automatically.

1. When the approved customer logs in, their profile is internally flagged with `tax_exempt: true`.
2. As they add items to their cart, the cart total will display the standard price.
3. Upon reaching the checkout screen, the OHC tax engine checks the customer flag.
4. The sales tax line item will explicitly display as **$0.00 (Exempt)**.
5. The final invoice and email receipt will include the text: "Tax Exempt Purchase - Certificate on File."

## Bulk Pricing and Quantity Discounts

B2B customers usually expect a discount for buying in large quantities. You can set this up directly on the product page.

1. Go to **My Store > Products** and edit a product.
2. Scroll down to the **Wholesale Pricing** section (this only appears if B2B mode is enabled).
3. Click **Add Quantity Break**.
4. Set the rules:
   - Buy 10-49: 15% off
   - Buy 50-99: 25% off
   - Buy 100+: 40% off
5. Click **Save**.

When an approved wholesale customer logs in, they will see a special table on the product page showing these bulk discounts. The discount is applied automatically when they add the required quantity to their cart.

## Managing Expiration Dates

Tax exemption certificates do not last forever. Most states require you to collect a new certificate every 1 to 3 years.

1. When you approve a certificate (as described above), enter the **Expiration Date** listed on the document.
2. Thirty days before the certificate expires, OHC will automatically send a reminder email to the customer asking them to upload a new one.
3. If the certificate expires and is not replaced, OHC will automatically remove the `tax_exempt` flag from their account, and they will be charged standard sales tax on their next purchase until a new certificate is approved.
