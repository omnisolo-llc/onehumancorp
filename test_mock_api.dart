import 'package:ohc_app/models/dashboard.dart';
void main() {
  var ds = DashboardSnapshot(
      organization: Organization(
        id: 'org1',
        name: 'Test Org',
        domain: 'test.com',
        tier: 'Free',
        members: [],
        roleProfiles: [],
      ),
      meetings: [],
      storage: StorageSummary(usedBytes: 150000000, limitBytes: 500000000), // 150MB used, 500MB limit
      costs: CostSummary(
        totalCostUSD: 0.0,
        totalActions: 45,
        totalTokens: 0,
        agents: [],
      ),
      agents: [],
      statuses: [],
      updatedAt: DateTime.now(),
  );
  print(ds.costs.totalActions);
}
