package main

import (
    "io/ioutil"
    "strings"
)

func main() {
    b, err := ioutil.ReadFile("srcs/app/lib/main.dart")
    if err != nil { panic(err) }
    content := string(b)

    // add import
    if !strings.Contains(content, "import 'package:ohc_app/services/powersync_service.dart';") {
        content = strings.Replace(content, "import 'package:ohc_app/router.dart';", "import 'package:ohc_app/router.dart';\nimport 'package:ohc_app/services/powersync_service.dart';", 1)
    }

    // Convert OhcApp to ConsumerStatefulWidget
    oldWidget := `class OhcApp extends ConsumerWidget {
  const OhcApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider);`

    newWidget := `class OhcApp extends ConsumerStatefulWidget {
  const OhcApp({super.key});

  @override
  ConsumerState<OhcApp> createState() => _OhcAppState();
}

class _OhcAppState extends ConsumerState<OhcApp> {
  @override
  void initState() {
    super.initState();
    // Initialize PowerSync
    Future.microtask(() => ref.read(powerSyncProvider).init());
  }

  @override
  Widget build(BuildContext context) {
    final router = ref.watch(routerProvider);`

    if !strings.Contains(content, "_OhcAppState") {
        content = strings.Replace(content, oldWidget, newWidget, 1)
    }

    ioutil.WriteFile("srcs/app/lib/main.dart", []byte(content), 0644)
}
