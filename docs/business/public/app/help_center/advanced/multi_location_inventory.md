# Multi-Location Inventory Routing

If your business has grown beyond a single garage or stockroom, you might have inventory spread across multiple locations. This could mean two different retail stores, a warehouse and a storefront, or even a third-party fulfillment center.

This guide explains how to track stock levels at different physical locations and how OHC decides which location to ship from.

## Setting Up Multiple Locations

First, you need to tell OHC about your physical buildings.

1. Go to **My Store > Settings > Locations**.
2. Click **Add Location**.
3. Enter the details:
   - **Name:** e.g., "Downtown Store", "Main Warehouse", "Pop-up Tent".
   - **Address:** Enter the full physical address. This is crucial for calculating shipping rates accurately.
   - **Fulfillment Status:** Check the box that says "Fulfill online orders from this location" if you plan to ship boxes from this building. If this is just a retail store that doesn't ship, leave it unchecked.
4. Click **Save Location**.

## Managing Stock per Location

Once you have more than one location, your inventory screens will look a bit different. Instead of a single "Stock" number, you will see stock broken down by building.

1. Go to **My Store > Inventory**.
2. When you click on a product, you will see a list of your locations.
3. You can adjust the stock level for each location independently.
   - e.g., Blue Shirt: 10 in Downtown Store, 50 in Main Warehouse.
4. The total available stock shown to the customer on your website is the sum of all locations where "Fulfill online orders" is checked. (In this example, the customer sees 60 available).

## Order Routing Rules

When a customer buys an item online, OHC needs to decide which location should put the item in a box and mail it. This is called Order Routing.

We use a simple set of rules to determine the best location:

1. **Rule 1: Can the location fulfill the entire order?**
   - If a customer orders 5 Blue Shirts, and the Downtown Store only has 3, OHC will route the order to the Main Warehouse (which has 50).
2. **Rule 2: Which location is closest to the customer?**
   - If a customer in New York buys a shirt, and you have a warehouse in New Jersey and another in California (both with enough stock), OHC will automatically route the order to the New Jersey warehouse to save you money on shipping costs and get the item to the customer faster.

## Split Shipments

Sometimes, a customer orders two different items, and they are located in different buildings.

- Example: Customer orders a Blue Shirt (only in stock at the Warehouse) and a Red Hat (only in stock at the Downtown Store).
- In this scenario, OHC creates a **Split Shipment**.
- The main order is divided into two separate fulfillment tickets.
- The Warehouse team gets a notification to ship the Blue Shirt.
- The Downtown Store team gets a notification to ship the Red Hat.
- The customer receives two tracking numbers.

*Note: You only charge the customer for shipping once at checkout. Split shipments increase your operational costs, so it is best to keep your inventory balanced when possible.*

## Transferring Inventory Between Locations

If the Downtown Store is running out of Blue Shirts, you need to move them from the Main Warehouse. You should record this transfer in OHC so your numbers stay accurate.

1. Go to **My Store > Inventory > Transfers**.
2. Click **Create Transfer**.
3. Select the **Origin Location** (Main Warehouse).
4. Select the **Destination Location** (Downtown Store).
5. Add the products and quantities you are moving (e.g., 20 Blue Shirts).
6. Click **Mark as In Transit**.
7. When the boxes physically arrive at the Downtown Store, a manager there should log in, open the transfer ticket, and click **Receive Items**. The stock levels will immediately update in both locations.
