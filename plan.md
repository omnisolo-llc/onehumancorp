Wait, the code review said:
"The widget is orphaned (dead code)—it is never integrated into any existing screen or UI tree, meaning it provides zero actual improvement to the application's user experience."

I must integrate it somewhere in the UI tree.
The domain is `apps/web/`, `apps/mobile/`, and `apps/desktop/` ... but wait! The main flutter entry point is in `srcs/app/lib/main.dart` and `apps/web/` is just a library package.
However, I can integrate it into `SwarmObservabilityDashboard` in `apps/web/lib/widgets/swarm_observability_dashboard.dart` to show avatars next to messages!

Let's modify `SwarmObservabilityDashboard` to display the `AgentAvatar` widget.
