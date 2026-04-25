import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/help/walkthrough_overlay.dart';

class WalkthroughRegistry {
  static final storeSetupWalkthrough = WalkthroughState(
    steps: [
      WalkthroughStep(
        targetKey: GlobalKey(), // In a real scenario, this key would be attached to a specific UI element
        title: 'Welcome to OneHumanCorp!',
        content: 'This is your dashboard. From here, you can manage your entire business.',
      ),
      WalkthroughStep(
        targetKey: GlobalKey(),
        title: 'Configure Settings',
        content: 'First, lets go to Settings to add your business details.',
      ),
    ],
  );

  static final paymentSetupWalkthrough = WalkthroughState(
    steps: [
      WalkthroughStep(
        targetKey: GlobalKey(),
        title: 'Accept Payments',
        content: 'To start getting paid, connect your bank account via Stripe.',
      ),
    ],
  );

  static final activateAgentWalkthrough = WalkthroughState(
    steps: [
      WalkthroughStep(
        targetKey: GlobalKey(),
        title: 'Hire an AI Agent',
        content: 'Click here to hire a new AI worker for your team.',
      ),
      WalkthroughStep(
        targetKey: GlobalKey(),
        title: 'Configure the Agent',
        content: 'Give your agent a name and customize its instructions.',
      ),
    ],
  );
}
