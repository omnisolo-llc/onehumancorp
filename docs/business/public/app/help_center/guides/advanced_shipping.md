# Advanced Shipping Rules

Setting up shipping can be complicated, but OneHumanCorp makes it easy. This guide helps you configure shipping zones, weight-based rates, and free shipping thresholds.

## Understanding Shipping Zones

A Shipping Zone is a geographic area where the same shipping rates apply. For example, you might have one zone for "Domestic" (your home country) and another for "International."

1. Go to **My Store > Settings > Shipping**.
2. Click **Create New Zone**.
3. Name your zone (e.g., "North America").
4. Select the countries or states that belong in this zone.
5. Click **Save Zone**.

## Setting Up Flat Rate Shipping

Flat rate shipping is the simplest method. You charge the same amount no matter what the customer buys.

1. Inside a Shipping Zone, click **Add Rate**.
2. Choose **Flat Rate**.
3. Name the rate (e.g., "Standard Shipping - 3 to 5 Days").
4. Enter the cost (e.g., \$5.00).
5. Click **Save Rate**.

## Setting Up Weight-Based Shipping

If you sell items that vary greatly in size (like jewelry vs. furniture), you should charge shipping based on the total weight of the order.

1. First, make sure every product in your store has a weight entered in its details page.
2. Inside a Shipping Zone, click **Add Rate**.
3. Choose **Weight-Based Rate**.
4. Define your tiers:
   - Example Tier 1: 0 lbs to 5 lbs = \$5.00
   - Example Tier 2: 5.1 lbs to 10 lbs = \$12.00
   - Example Tier 3: 10.1 lbs and up = \$20.00
5. Click **Save Rate**.

## Offering Free Shipping

Free shipping is a great marketing tool. You can offer free shipping automatically if a customer spends over a certain amount.

1. Inside a Shipping Zone, click **Add Rate**.
2. Choose **Price-Based Rate**.
3. Check the box for "Free Shipping."
4. Set the condition: "If order total is over \$50.00."
5. Click **Save Rate**.

Now, if a customer's cart reaches \$50.00, they will automatically see a "Free Shipping" option at checkout.

## Using the OHC Shipping Integration (Pro Plan)

If you are on the Pro Plan, you can connect directly to FedEx, UPS, or USPS to get real-time shipping quotes.

1. Go to **My Store > Settings > Integrations**.
2. Find the "Live Shipping Rates" integration and click **Connect**.
3. You will need to enter your API key provided by the shipping carrier.
4. Once connected, your checkout will automatically calculate the exact cost of shipping based on the customer's address and the weight of their items.
