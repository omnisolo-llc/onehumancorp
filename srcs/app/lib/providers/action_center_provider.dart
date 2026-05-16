import 'package:flutter_riverpod/flutter_riverpod.dart';

class ActionItem {
  final String id;
  final String title;
  final String description;
  final String type; // e.g., 'quote', 'reply'
  final String? relatedData; // optional JSON or data

  ActionItem({
    required this.id,
    required this.title,
    required this.description,
    required this.type,
    this.relatedData,
  });
}

class ActionCenterState {
  final List<ActionItem> actions;

  ActionCenterState({
    this.actions = const [],
  });

  ActionCenterState copyWith({
    List<ActionItem>? actions,
  }) {
    return ActionCenterState(
      actions: actions ?? this.actions,
    );
  }
}

class ActionCenterNotifier extends StateNotifier<ActionCenterState> {
  ActionCenterNotifier()
      : super(ActionCenterState(actions: [
          ActionItem(
            id: '1',
            title: 'Customer Reply Drafted',
            description: 'AI drafted a reply to Maya Bakes regarding vegan cakes.',
            type: 'reply',
          ),
          ActionItem(
            id: '2',
            title: 'Quote Generated',
            description: 'AI created a quote for the plumbing service request from Carlos.',
            type: 'quote',
          ),
        ]));

  void approveAction(String id) {
    state = state.copyWith(
      actions: state.actions.where((action) => action.id != id).toList(),
    );
  }

  void rejectAction(String id) {
    state = state.copyWith(
      actions: state.actions.where((action) => action.id != id).toList(),
    );
  }
}

final actionCenterProvider = StateNotifierProvider<ActionCenterNotifier, ActionCenterState>((ref) {
  return ActionCenterNotifier();
});