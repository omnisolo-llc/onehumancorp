class ApprovalRequest {
  final String id;
  final String agentId;
  final String action;
  final String reason;
  final double estimatedCostUsd;
  final String riskLevel;
  final String status;
  final DateTime createdAt;
  final DateTime? decidedAt;
  final String? decidedBy;

  ApprovalRequest({
    required this.id,
    required this.agentId,
    required this.action,
    required this.reason,
    required this.estimatedCostUsd,
    required this.riskLevel,
    required this.status,
    required this.createdAt,
    this.decidedAt,
    this.decidedBy,
  });

  factory ApprovalRequest.fromJson(Map<String, dynamic> json) {
    return ApprovalRequest(
      id: json['id'] as String? ?? '',
      agentId: json['agentId'] as String? ?? '',
      action: json['action'] as String? ?? '',
      reason: json['reason'] as String? ?? '',
      estimatedCostUsd: (json['estimatedCostUsd'] ?? 0.0).toDouble(),
      riskLevel: json['riskLevel'] as String? ?? '',
      status: json['status'] as String? ?? '',
      createdAt: json['createdAt'] != null ? DateTime.parse(json['createdAt'] as String) : DateTime.now(),
      decidedAt: json['decidedAt'] != null ? DateTime.parse(json['decidedAt'] as String) : null,
      decidedBy: json['decidedBy'] as String?,
    );
  }
}
