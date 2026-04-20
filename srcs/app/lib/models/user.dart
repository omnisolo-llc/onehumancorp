export 'package:ohc_app/proto/user.domain.dart';

extension UserMetadataConvenience on UserMetadata {
  bool get isAdmin => roles.contains('admin');
}
