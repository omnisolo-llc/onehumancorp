import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class Task {
  final String id;
  final String title;
  final String status;
  Task({required this.id, required this.title, required this.status});
}

final tasksProvider = StateProvider<List<Task>>((ref) => []);

class SharedTaskList extends ConsumerWidget {
  const SharedTaskList({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasks = ref.watch(tasksProvider);

    return ClipRRect(
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: BackdropFilter(
          filter: ColorFilter.matrix(const <double>[
            2.0, 0, 0, 0, 0,
            0, 2.0, 0, 0, 0,
            0, 0, 2.0, 0, 0,
            0, 0, 0, 1.0, 0,
          ]),
          child: Container(
            color: const Color.fromRGBO(255, 255, 255, 0.05),
            padding: const EdgeInsets.all(16.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Shared Task List',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    color: Colors.white,
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 16),
                Expanded(
                  child: ListView.builder(
                    itemCount: tasks.length,
                    itemBuilder: (context, index) {
                      final task = tasks[index];
                      return Container(
                        margin: const EdgeInsets.only(bottom: 8.0),
                        padding: const EdgeInsets.all(12.0),
                        decoration: BoxDecoration(
                          color: const Color.fromRGBO(255, 255, 255, 0.1),
                          borderRadius: BorderRadius.circular(8.0),
                        ),
                        child: Text(
                          '${task.title} - ${task.status}',
                          style: const TextStyle(
                            fontFamily: 'Outfit',
                            color: Colors.white70,
                          ),
                        ),
                      );
                    },
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
