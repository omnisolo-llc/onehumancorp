import 'package:flutter_riverpod/flutter_riverpod.dart';

class ReferralInvite {
  final String email;
  final String status;
  ReferralInvite({required this.email, required this.status});
}

class ReferralState {
  final String userId;
  final String inviteLink;
  final List<ReferralInvite> invites;

  ReferralState({
    required this.userId,
    required this.inviteLink,
    this.invites = const [],
  });

  ReferralState copyWith({
    String? userId,
    String? inviteLink,
    List<ReferralInvite>? invites,
  }) {
    return ReferralState(
      userId: userId ?? this.userId,
      inviteLink: inviteLink ?? this.inviteLink,
      invites: invites ?? this.invites,
    );
  }
}

class ReferralNotifier extends StateNotifier<ReferralState> {
  ReferralNotifier() : super(ReferralState(
    userId: 'user123',
    inviteLink: 'ohc://join?ref=user123',
    invites: [
      ReferralInvite(email: 'friend@example.com', status: 'ACCEPTED'),
      ReferralInvite(email: 'another@example.com', status: 'PENDING'),
    ],
  ));

  // In a real implementation, this would interact with ApiService
  // to fetch the user's actual generated link and track invites from the backend.
  // For the frontend requirement, this establishes the state management.
  void addInvite(String email) {
    state = state.copyWith(
      invites: [
        ...state.invites,
        ReferralInvite(email: email, status: 'PENDING'),
      ],
    );
  }
}

final referralProvider = StateNotifierProvider<ReferralNotifier, ReferralState>((ref) {
  return ReferralNotifier();
});
