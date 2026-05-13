# Shipping & Logistics Automation

## Problem Statement
Calculating shipping costs manually and creating labels one by one is a major bottleneck for e-commerce businesses. Real-time rates and automated label generation are required.

## Research Report

**Market Context:**
Logistics is the part of supply chain management that deals with the efficient forward and reverse flow of goods, services, and related information from the point of origin to the point of consumption according to the needs of customers, and a logistician is a professional working in the field of logistics management. Logistics management is a component that holds the supply chain together. The resources managed in logistics include physical goods such as materials, equipment, and foodstuffs, and also intangible items such as time and information.
Military logistics is concerned with maintaining army supply lines with food, armaments, ammunition, and spare parts, apart from the transportation of troops themselves. Civil logistics deals with acquiring, moving, and storing raw materials, semi-finished goods, and finished goods. For organisations that provide garbage collection, mail deliveries, public utilities, and after-sales services, logistical problems must be addressed.
Logistics deals with the movement of materials or products from one facility to another; it does not include material flow within production or assembly plants, such as production planning or single-machine scheduling.
Logistics accounts for a significant amount of the operational costs of an organisation or country. Dedicated simulation software can model, analyse, visualise, and optimize logistic complexities. Minimizing resource use is a common motivation in all logistics fields.

**Evaluated Tools:**

#### In-Depth Evaluation: Shippo
**Market Position**: Multi-carrier shipping API. Great for aggregating USPS, UPS, FedEx, etc.
**Pricing**: Pay-as-you-go per label (e.g., 5¢) or monthly subscriptions.
**Integration Approach**: Synchronous API calls to fetch rates based on package dimensions, followed by a call to generate the PDF label. OHC must securely store the tracking numbers and expose them to the customer.
**Persona Impact**: Fatima clicks one button to buy postage and print a label, instead of typing addresses into a separate carrier website.

#### In-Depth Evaluation: EasyPost
**Market Position**: Very developer-focused, highly reliable API. Competitor to Shippo.
**Pricing**: Similar per-label model.
**Integration Approach**: Technically elegant. Requires OHC to manage the UI for rate shopping entirely.

#### In-Depth Evaluation: ShipStation
**Market Position**: Heavier, UI-driven order fulfillment software. Often used by high-volume e-commerce.
**Pricing**: Starts around $10/mo.
**Integration Approach**: OHC might integrate *into* ShipStation (sending orders to it) rather than pulling ShipStation *into* OHC, depending on the user's preference.

## Design Doc
Connect OHC's order management system to a shipping aggregator API (like Shippo or EasyPost). When an order is placed, OHC fetches real-time rates based on package dimensions. When fulfilled, it generates a PDF label and tracking number.

## Implementation Prompt
Build an order fulfillment interface that allows users to compare shipping rates from different carriers. Include a one-click label generation feature and automated tracking updates for customers.

## Priority
P2

## Estimated Scope
Medium
