import re

with open("srcs/app/lib/screens/kairos_dashboard.dart", "r") as f:
    content = f.read()

old_state = """class _MeshEventLogState extends ConsumerState<_MeshEventLog> {
  @override
  Widget build(BuildContext context) {
    final centrifuge = ref.watch(centrifugeServiceProvider);

    if (centrifuge == null) {
      return const Center(child: Text('Connecting to Teammate Mesh...'));
    }

    return StreamBuilder<CentrifugeMessage>(
      stream: centrifuge.subscribe('mesh:tasks'),
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          final msg = snapshot.data!;
          if (!widget.liveMessages.any((m) => m.id == msg.id)) {
            widget.liveMessages.insert(0, msg);
            widget.listKey.currentState?.insertItem(0, duration: const Duration(milliseconds: 500));
          }
        }

        return AnimatedList(
          key: widget.listKey,
          initialItemCount: widget.liveMessages.length,
          itemBuilder: (context, index, animation) {
            final msg = widget.liveMessages[index];
            return SlideTransition(
              position: animation.drive(Tween(begin: const Offset(-1, 0), end: Offset.zero).chain(CurveTween(curve: Curves.easeOutQuart))),
              child: FadeTransition(
                opacity: animation,
                child: Padding(
                  padding: const EdgeInsets.only(bottom: 12.0),
                  child: _GlassMessageCard(message: msg),
                ),
              ),
            );
          },
        );
      },
    );
  }
}"""

new_state = """import 'dart:async';

class _MeshEventLogState extends ConsumerState<_MeshEventLog> {
  StreamSubscription<CentrifugeMessage>? _subscription;

  @override
  void initState() {
    super.initState();
    _subscribeToMesh();
  }

  void _subscribeToMesh() {
    final centrifuge = ref.read(centrifugeServiceProvider);
    if (centrifuge != null) {
      _subscription = centrifuge.subscribe('mesh:tasks').listen((msg) {
        if (!widget.liveMessages.any((m) => m.id == msg.id)) {
          widget.liveMessages.insert(0, msg);
          widget.listKey.currentState?.insertItem(0, duration: const Duration(milliseconds: 500));
        }
      });
    }
  }

  @override
  void dispose() {
    _subscription?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final centrifuge = ref.watch(centrifugeServiceProvider);

    if (centrifuge == null) {
      return const Center(child: Text('Connecting to Teammate Mesh...'));
    }

    // Ensure subscription exists if centrifuge becomes available after init
    if (_subscription == null) {
      _subscribeToMesh();
    }

    return AnimatedList(
      key: widget.listKey,
      initialItemCount: widget.liveMessages.length,
      itemBuilder: (context, index, animation) {
        final msg = widget.liveMessages[index];
        return SlideTransition(
          position: animation.drive(Tween(begin: const Offset(-1, 0), end: Offset.zero).chain(CurveTween(curve: Curves.easeOutQuart))),
          child: FadeTransition(
            opacity: animation,
            child: Padding(
              padding: const EdgeInsets.only(bottom: 12.0),
              child: _GlassMessageCard(message: msg),
            ),
          ),
        );
      },
    );
  }
}"""

# Actually we need to make sure the import is at the top
content = content.replace(old_state, new_state.replace("import 'dart:async';\n\n", ""))

if "import 'dart:async';" not in content:
    content = "import 'dart:async';\n" + content

with open("srcs/app/lib/screens/kairos_dashboard.dart", "w") as f:
    f.write(content)
