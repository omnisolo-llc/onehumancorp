# Flutter Stripe Terminal Integration Outline

This document outlines the high-level architecture and necessary steps to integrate Stripe Terminal (Tap to Pay) into the OHC Flutter app, fulfilling the final acceptance criteria for the In-Person Tap-to-Pay Integration.

## 1. Add Dependencies

Update `pubspec.yaml` to include a compatible Flutter Stripe Terminal package. Since official Stripe Terminal support in Flutter relies on community or enterprise wrappers, we typically use packages like `stripe_terminal` or `flutter_stripe` (if Tap to Pay is supported).

```yaml
dependencies:
  flutter:
    sdk: flutter
  # Verify latest compatible version for Tap to Pay
  stripe_terminal: ^1.0.0
  http: ^1.1.0
```

## 2. Initialize the SDK

During the app initialization phase (e.g., in `main.dart` or an auth-guarded initialization sequence), initialize the Stripe Terminal SDK. This requires providing a function to fetch the connection token from our OHC backend.

```dart
import 'package:stripe_terminal/stripe_terminal.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

Future<String> fetchConnectionToken() async {
  // Use the OHC HTTP client which automatically handles Auth (SPIFFE/JWT headers)
  final response = await http.post(
    Uri.parse('https://api.onehumancorp.com/api/v1/pos/terminal/connection_token'),
    headers: {
      'Authorization': 'Bearer $authToken',
      'x-tenant-id': tenantId,
    },
  );

  if (response.statusCode == 200) {
    final data = jsonDecode(response.body);
    return data['token'];
  } else {
    throw Exception('Failed to fetch connection token');
  }
}

void initializeTerminal() {
  StripeTerminal.getInstance().init(
    fetchToken: fetchConnectionToken,
  );
}
```

## 3. Discover and Connect to a Reader

For Tap to Pay (Local Mobile Reader), we must discover the local device as a reader and connect to it.

```dart
Future<void> connectTapToPay() async {
  try {
    final terminal = StripeTerminal.getInstance();

    // Discover local mobile readers (Tap to Pay on iPhone/Android)
    final readers = await terminal.discoverReaders(
      DiscoveryConfiguration(
        discoveryMethod: DiscoveryMethod.localMobile,
        simulated: false, // Set to true for local testing without physical cards
      ),
    );

    if (readers.isNotEmpty) {
      // Connect to the first discovered local mobile reader
      await terminal.connectLocalMobileReader(readers.first);
      print('Connected to Tap to Pay!');
    }
  } catch (e) {
    print('Error connecting to reader: $e');
  }
}
```

## 4. Process a Payment

When the owner initiates a checkout, request a Payment Intent from the OHC Backend, collect the payment method via NFC, and process the payment.

```dart
Future<void> processCheckout(int amountCents, String currency) async {
  try {
    final terminal = StripeTerminal.getInstance();

    // 1. Request Payment Intent from OHC Backend
    final response = await http.post(
      Uri.parse('https://api.onehumancorp.com/api/v1/pos/terminal/create_intent'),
      headers: {
        'Authorization': 'Bearer $authToken',
        'x-tenant-id': tenantId,
        'Content-Type': 'application/json',
      },
      body: jsonEncode({
        'amount_cents': amountCents,
        'currency': currency,
      }),
    );

    if (response.statusCode != 200) {
      throw Exception('Failed to create payment intent');
    }

    final data = jsonDecode(response.body);
    final clientSecret = data['client_secret'];

    // 2. Retrieve the PaymentIntent using the Stripe Terminal SDK
    final paymentIntent = await terminal.retrievePaymentIntent(clientSecret);

    // 3. Collect Payment Method (This triggers the native Tap to Pay UI)
    final paymentIntentWithMethod = await terminal.collectPaymentMethod(paymentIntent);

    // 4. Process the Payment
    final processedIntent = await terminal.processPayment(paymentIntentWithMethod);

    if (processedIntent.status == PaymentIntentStatus.requiresCapture ||
        processedIntent.status == PaymentIntentStatus.succeeded) {
      // Payment successful!
      // Update UI to show Success State (Green checkmark)
      // The backend Webhook will receive the success event and update the Ledger
      print('Payment successful!');
    }
  } catch (e) {
    print('Payment failed: $e');
    // Update UI to show Error State and offer Retry/Fallback
  }
}
```

## 5. UI/UX Considerations

- **Permissions**: Ensure necessary iOS/Android permissions for NFC and location are requested and declared in `Info.plist` and `AndroidManifest.xml`.
- **Background**: Ensure the app gracefully handles backgrounding during the Tap to Pay flow.
- **Feedback**: Provide immediate visual feedback when the intent is created and when the SDK is waiting for a card tap.