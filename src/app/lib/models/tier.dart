import 'package:flutter_riverpod/flutter_riverpod.dart';

class TierStatus {
  final String tierName;
  final int productsCount;
  final int maxProducts;
  final int aiActionsUsed;
  final int aiActionsLimit;

  const TierStatus({
    this.tierName = 'Free',
    this.productsCount = 0,
    this.maxProducts = 10,
    this.aiActionsUsed = 0,
    this.aiActionsLimit = 100,
  });

  TierStatus copyWith({
    String? tierName,
    int? productsCount,
    int? maxProducts,
    int? aiActionsUsed,
    int? aiActionsLimit,
  }) {
    return TierStatus(
      tierName: tierName ?? this.tierName,
      productsCount: productsCount ?? this.productsCount,
      maxProducts: maxProducts ?? this.maxProducts,
      aiActionsUsed: aiActionsUsed ?? this.aiActionsUsed,
      aiActionsLimit: aiActionsLimit ?? this.aiActionsLimit,
    );
  }
}

class TierNotifier extends StateNotifier<TierStatus> {
  TierNotifier() : super(const TierStatus());

  void addProduct() {
    state = state.copyWith(productsCount: state.productsCount + 1);
  }

  void setProductsCount(int count) {
    state = state.copyWith(productsCount: count);
  }

  void addAiAction() {
    state = state.copyWith(aiActionsUsed: state.aiActionsUsed + 1);
  }

  void upgradeToStarter() {
    state = state.copyWith(
      tierName: 'Starter',
      maxProducts: 100,
      aiActionsLimit: 1000,
    );
  }

  void mockFreeTierLimitExceeded() {
    state = state.copyWith(
      productsCount: 10,
      maxProducts: 10,
      aiActionsUsed: 100,
      aiActionsLimit: 100,
    );
  }
}

final tierProvider = StateNotifierProvider<TierNotifier, TierStatus>((ref) {
  return TierNotifier();
});
