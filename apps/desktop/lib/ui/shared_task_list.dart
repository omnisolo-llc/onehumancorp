import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;

final taskListProvider = FutureProvider<List<String>>((ref) async {
  try {
    final response = await http.get(Uri.parse('http://localhost:8080/api/tasks/queue'));
    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      return List<String>.from(data['tasks']);
    }
  } catch (_) {}

  // Graceful fallback for disconnected states (Thin Client offline, or Standalone booting)
  return ["Offline Task 1", "Offline Task 2"];
});

class SharedTaskList extends ConsumerWidget {
  const SharedTaskList({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksAsync = ref.watch(taskListProvider);

    return Scaffold(
      backgroundColor: Colors.transparent,
      body: Center(
        child: ClipRRect(
          borderRadius: BorderRadius.circular(12),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
            child: Container(
              color: const Color.fromRGBO(255, 255, 255, 0.05),
              child: tasksAsync.when(
                data: (tasks) => ListView.builder(
                  itemCount: tasks.length,
                  itemBuilder: (context, index) {
                    return ListTile(
                      title: Text(
                        tasks[index],
                        style: const TextStyle(
                          fontFamily: 'Outfit',
                          color: Colors.white,
                        ),
                      ),
                      subtitle: const Text(
                        'Status: Pending',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          color: Colors.white70,
                        ),
                      ),
                    );
                  },
                ),
                loading: () => const Center(child: CircularProgressIndicator(color: Colors.white)),
                error: (err, stack) => Center(
                  child: Text(
                    'Error loading tasks',
                    style: const TextStyle(fontFamily: 'Inter', color: Colors.redAccent),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
