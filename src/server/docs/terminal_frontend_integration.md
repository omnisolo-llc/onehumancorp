# Stripe Terminal Frontend Integration Guide

## Overview
This document outlines the required steps to integrate the Zero-Config Universal Tap-to-Pay POS Engine into the OHC Flutter/PWA application. The integration will utilize the newly established backend endpoints (`/api/terminal/token` and `/api/terminal/intent`) to interface with Stripe Terminal SDKs on iOS, Android, and potentially web (though tap-to-pay is primarily mobile).

## Prerequisites
1.  **Stripe SDK:** The `stripe_terminal` package (or appropriate native wrappers for React Native/Flutter depending on the final frontend framework choice) must be installed.
2.  **Permissions:** The mobile application must request and handle Bluetooth and Location permissions, which are strictly required by the Stripe Terminal SDK for security and compliance.

## Integration Steps

### 1. Initialize the Stripe Terminal SDK
Upon application startup or when entering the POS module, initialize the SDK. This requires a `fetchToken` callback.

```dart
// Example Flutter (Dart) pseudocode
Terminal.init(fetchTokenProvider: () async {
  // Call the OHC backend to retrieve a connection token scoped to the current tenant
  final response = await http.get(Uri.parse('https://api.ohc.com/api/terminal/token'), headers: authHeaders);
  final data = jsonDecode(response.body);
  return data['token'];
});
```

### 2. Discover and Connect to a Reader
For "Tap to Pay on iPhone" or "Tap to Pay on Android," the device itself acts as the reader (Local Mobile).
1.  Start discovery for `DiscoveryMethod.localMobile`.
2.  Connect to the discovered local mobile reader.

### 3. Create a Payment Intent
When the merchant is ready to charge a customer:
1.  Calculate the total cart amount.
2.  Request a Payment Intent from the OHC backend.

```dart
// Example Flutter (Dart) pseudocode
final intentResponse = await http.post(
  Uri.parse('https://api.ohc.com/api/terminal/intent'),
  headers: authHeaders,
  body: jsonEncode({'amount_cents': 1500, 'currency': 'usd'}),
);
final intentId = jsonDecode(intentResponse.body)['intent_id'];
```

### 4. Collect Payment Method
Once the intent is created, instruct the Terminal SDK to collect the payment method. This will trigger the native OS UI (e.g., the Apple Tap to Pay sheet).

```dart
// Example Flutter (Dart) pseudocode
final paymentIntent = await Terminal.instance.retrievePaymentIntent(intentId);
final collectedIntent = await Terminal.instance.collectPaymentMethod(paymentIntent);
```

### 5. Process the Payment
After the customer taps their card or device, confirm the payment intent.

```dart
// Example Flutter (Dart) pseudocode
final processedIntent = await Terminal.instance.processPayment(collectedIntent);
if (processedIntent.status == PaymentIntentStatus.requiresCapture) {
  // Payment succeeded locally. The OHC backend webhook will handle the capture and fulfillment.
  showSuccessUI();
}
```

## AI Agent Interaction
*   **Operations Agent:** The backend webhook (`payment_intent.succeeded` or `payment_intent.requires_capture`) should trigger the Operations Agent to decrement inventory for the purchased items.
*   **Finance Agent:** The transaction will automatically appear in the Stripe ledger. The Finance Agent will reconcile this in the nightly batch via the `STRIPE_TERMINAL_TAP` payment source identifier added to the `Order` model.
