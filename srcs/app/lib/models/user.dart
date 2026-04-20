import 'package:fixnum/fixnum.dart';
import 'package:ohc_app/proto/user.pb.dart' as pb;

/// UserMetadata domain model for RBAC and identity.
/// This matches the Protobuf definition used by the shared backend.
class UserMetadata {
  final String id;
  final String username;
  final String email;
  final List<String> roles;
  final bool active;
  final DateTime createdAt;

  const UserMetadata({
    required this.id,
    required this.username,
    required this.email,
    required this.roles,
    required this.active,
    required this.createdAt,
  });

  factory UserMetadata.fromProto(pb.UserMetadata proto) {
    return UserMetadata(
      id: proto.id,
      username: proto.username,
      email: proto.email,
      roles: proto.roles,
      active: proto.active,
      createdAt: DateTime.fromMillisecondsSinceEpoch(
        proto.createdAt.seconds.toInt() * 1000 +
            (proto.createdAt.nanos / 1000000).toInt(),
      ),
    );
  }

  pb.UserMetadata toProto() {
    return pb.UserMetadata(
      id: id,
      username: username,
      email: email,
      roles: List<String>.from(roles),
      active: active,
      createdAt: pb.Timestamp(
        seconds: Int64(createdAt.millisecondsSinceEpoch ~/ 1000),
        nanos: (createdAt.millisecondsSinceEpoch % 1000) * 1000000,
      ),
    );
  }

  factory UserMetadata.fromJson(Map<String, dynamic> json) {
    return UserMetadata(
      id: json['id'] as String? ?? '',
      username: json['username'] as String? ?? '',
      email: json['email'] as String? ?? '',
      roles: (json['roles'] as List<dynamic>?)?.cast<String>() ?? [],
      active: json['active'] as bool? ?? true,
      createdAt:
          json['created_at'] != null
              ? DateTime.parse(json['created_at'] as String)
              : DateTime.now(),
    );
  }

  Map<String, dynamic> toJson() => {
    'id': id,
    'username': username,
    'email': email,
    'roles': roles,
    'active': active,
    'created_at': createdAt.toIso8601String(),
  };

  bool get isAdmin => roles.contains('admin');
}

/// User domain model representing the full user entity.
class User {
  final UserMetadata metadata;
  final String? passwordHash;

  const User({required this.metadata, this.passwordHash});

  factory User.fromProto(pb.User proto) {
    return User(
      metadata: UserMetadata.fromProto(proto.metadata),
      passwordHash: proto.passwordHash,
    );
  }

  pb.User toProto() {
    return pb.User(
      metadata: metadata.toProto(),
      passwordHash: passwordHash ?? '',
    );
  }

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      metadata: UserMetadata.fromJson(
        json['metadata'] as Map<String, dynamic>? ?? json,
      ),
      passwordHash: json['password_hash'] as String?,
    );
  }

  Map<String, dynamic> toJson() => {
    'metadata': metadata.toJson(),
    if (passwordHash != null) 'password_hash': passwordHash,
  };
}
