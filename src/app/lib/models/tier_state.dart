import 'package:flutter/foundation.dart';

enum SaasTier { free, starter, pro, business }

class TierState extends ChangeNotifier {
  SaasTier currentTier = SaasTier.free;
  int aiActionsUsed = 0;
  int productsCount = 0;

  void upgradeToStarter() {
    currentTier = SaasTier.starter;
    notifyListeners();
  }
}
