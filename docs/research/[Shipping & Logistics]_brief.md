# Title: Integrate Shippo for Multi-Carrier Shipping and Logistics

## Problem Statement
Boutique owners like Maya waste hours manually calculating shipping rates, standing in line at the post office, and dealing with lost tracking numbers. Negotiating rates with individual carriers is impossible for small volumes. She needs a simple tool to compare rates, buy labels, and automate tracking updates for her customers.

## Research Report
**Tool Evaluated:** Shippo
**Ease of Use:** High. It abstracts the complexity of dealing with individual carriers into one unified system.
**Key Features:** Rate calculation across 40+ global carriers (USPS, UPS, FedEx, DHL, etc.), label generation, address validation, and automated tracking notifications.
**Pricing:** Very friendly for SMBs. They offer a "Pay as you go" plan with no monthly fees, charging only a few cents per label plus the postage cost.
**Reputation:** Highly regarded in the e-commerce space, used by millions of customers and major platforms.
**Environments:** Cloud API integration.

## Design Doc
**Trigger:** User marks an order as "Ready to Ship" in the OHC dashboard.
**Action:** OHC prompts the user for package dimensions/weight, fetches rates via the Shippo API, and allows the user to purchase and print a label.
**User Experience:** Maya sees a "Fulfillment" list. She clicks "Buy Label," selects the cheapest rate shown on the screen, prints the label from her browser, and the system automatically sends a tracking text/email to the customer.

## Implementation Prompt
Integrate the Shippo API to provide shipping label generation within OHC. Create a "Fulfillment" UI where a user can input box dimensions and weight, retrieve a list of available carrier rates, and purchase a label. Ensure the UI clearly highlights the "Cheapest" and "Fastest" options. Once purchased, provide a button to download the printable PDF label and save the tracking number to the order record.

## Priority
P1

## Estimated Scope
Medium