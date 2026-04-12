import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';
import 'dart:ui';
import 'package:intl/intl.dart';

class TeamInvitesDashboardScreen extends ConsumerStatefulWidget {
  const TeamInvitesDashboardScreen({super.key});

  @override
  ConsumerState<TeamInvitesDashboardScreen> createState() =>
      _TeamInvitesDashboardScreenState();
}

class _TeamInvitesDashboardScreenState extends ConsumerState<TeamInvitesDashboardScreen> {
  late Future<List<Map<String, dynamic>>> _invitesFuture;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _invitesFuture = ref.read(apiServiceProvider)!.listTeamInvites();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Team Invites Dashboard'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _refresh,
          ),
        ],
      ),
      body: Stack(
        children: [
          Positioned.fill(
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
              child: Container(
                color: Colors.white.withOpacity(0.03),
              ),
            ),
          ),
          FutureBuilder<List<Map<String, dynamic>>>(
            future: _invitesFuture,
            builder: (context, snapshot) {
              if (snapshot.connectionState == ConnectionState.waiting) {
                return const Center(child: CircularProgressIndicator());
              } else if (snapshot.hasError) {
                return Center(child: Text('Error: ${snapshot.error}'));
              } else if (!snapshot.hasData || snapshot.data!.isEmpty) {
                return const Center(child: Text('No team invites found.'));
              }

              final invites = snapshot.data!;
              return ListView.builder(
                itemCount: invites.length,
                itemBuilder: (context, index) {
                  final invite = invites[index];
                  final createdAt = DateTime.parse(invite['createdAt']).toLocal();
                  return Card(
                    margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                    color: Colors.transparent,
                    elevation: 0,
                    shape: RoundedRectangleBorder(
                      side: BorderSide(color: Colors.white.withOpacity(0.1), width: 1),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: Material(
                      type: MaterialType.transparency,
                      child: ListTile(
                        title: Text('Inviter: ${invite['inviterId']} -> Invitee: ${invite['inviteeId']}'),
                        subtitle: Text('Status: ${invite['status']} \nSent: ${DateFormat.yMd().add_jm().format(createdAt)}'),
                      ),
                    ),
                  );
                },
              );
            },
          ),
        ],
      ),
    );
  }
}
