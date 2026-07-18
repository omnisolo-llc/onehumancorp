'use client';

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

interface DepartmentTierUsage {
  current_plan: string;
  period: string;
  departments: {
    id: string;
    department_type: string;
    agent_id: string;
    actions_used: number;
    action_limit: number | null;
    usage_percent: number | null;
    soft_limit_reached: boolean;
  }[];
}

interface CostDashboardResponse {
  department_tier_usage?: DepartmentTierUsage;
}

export default function AiUsagePaywallPage() {
  const router = useRouter();
  const [data, setData] = useState<CostDashboardResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [refLink, setRefLink] = useState('default');

  useEffect(() => {
    async function fetchData() {
      try {
        const headers: Record<string, string> = {};
        if (typeof window !== 'undefined') {
          const tenantId = localStorage.getItem('ohc_active_tenant_id');
          if (tenantId) headers['x-ohc-tenant-id'] = tenantId;
        }

        const costRes = await fetch('/api/v1/billing/cost-dashboard', { headers });
        if (costRes.ok) {
          const costData = await costRes.json();
          setData(costData);
        } else {
          console.error("Failed to fetch cost data:", costRes.status);
        }
      } catch (err) {
        console.error("Failed to fetch cost data:", err);
      } finally {
        setLoading(false);
      }
    }
    fetchData();

    if (typeof window !== 'undefined') {
      setRefLink(localStorage.getItem('ohc_active_tenant_id') || 'default');
    }
  }, []);

  const handleShareOnX = () => {
    router.push('/growth-loop?ref=twitter');
  };

  const handleUpgrade = () => {
    router.push('/pricing');
  };

  if (loading) {
    return (
      <AppShell title="AI Usage" subtitle="Monitor AI actions across active departments.">
        <div className="app-panel glassmorphism border border-white/40 dark:border-white/10">
          <div className="app-empty">Loading AI usage...</div>
        </div>
      </AppShell>
    );
  }

  const usage = data?.department_tier_usage;
  const currentPlan = usage?.current_plan || 'Free';
  const departments = usage?.departments || [];

  // Calculate total actions
  const totalUsed = departments.reduce((acc, d) => acc + d.actions_used, 0);
  const totalLimit = departments.reduce((acc, d) => acc + (d.action_limit || 0), 0);
  const overallPercent = totalLimit > 0 ? Math.min(100, Math.round((totalUsed / totalLimit) * 100)) : 0;

  const isLimitReached = departments.some(d => d.soft_limit_reached) || (totalLimit > 0 && totalUsed >= totalLimit);

  return (
    <AppShell title="AI Usage" subtitle="Monitor AI actions across active departments.">
      <div className="app-grid">
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 shadow-sm">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title">Plan Usage</div>
              <div className="app-list-subtitle">Monitor AI actions across active departments.</div>
            </div>
            <span className={`app-badge ${isLimitReached ? 'bad' : 'good'}`}>{currentPlan} Plan</span>
          </div>
          <div className="app-panel-body">
            <div className="app-grid two">
              <div className="app-card p-4 glassmorphism border border-white/40 dark:border-white/10">
                <div className="app-metric-label">Actions Used</div>
                <div className="app-metric-value">
                  {totalUsed}
                  <span className="ml-2 text-base font-semibold text-gray-500">/ {totalLimit > 0 ? totalLimit : 'Unlimited'}</span>
                </div>
                <div className="app-metric-note">Current billing period</div>
              </div>

              <div className="app-card p-4 glassmorphism border border-white/40 dark:border-white/10">
                <div className="flex items-center justify-between gap-3">
                  <div>
                    <div className="app-metric-label">Capacity</div>
                    <div className={`mt-2 text-xl font-bold ${isLimitReached ? 'text-red-700' : 'text-gray-900 dark:text-white'}`}>{overallPercent}%</div>
                  </div>
                  <div className="w-2/3">
                    <div className="h-2 w-full overflow-hidden rounded-md bg-gray-100">
                      <div
                        className={`h-full rounded-md transition-all duration-700 ${isLimitReached ? 'bg-[#FF3B30]' : 'bg-[#0f766e]'}`}
                        style={{ width: `${overallPercent}%` }}
                      />
                    </div>
                  </div>
                </div>
                <div className="app-metric-note">Limits apply only to capped departments.</div>
              </div>
            </div>
          </div>
        </section>

        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 shadow-sm">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title">Department Breakdown</div>
              <div className="app-list-subtitle">Usage by department and assigned agent.</div>
            </div>
          </div>
          <div className="app-list">
            {departments.length === 0 ? (
              <div className="app-empty">No department usage recorded for this period.</div>
            ) : departments.map((dept) => (
              <div key={dept.id} className="app-list-item border-b border-gray-100/50">
                <div>
                  <div className="app-list-title capitalize">{dept.department_type}</div>
                  <div className="app-list-subtitle">{dept.agent_id.replace(/_/g, ' ')}</div>
                </div>
                <div className="text-right">
                  <div className="app-list-title">{dept.actions_used} / {dept.action_limit || 'Unlimited'}</div>
                  {dept.soft_limit_reached && <div className="app-list-subtitle text-red-600">Limit reached</div>}
                </div>
              </div>
            ))}
          </div>
        </section>

        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 shadow-sm">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title">Plan Actions</div>
              <div className="app-list-subtitle">Upgrade capacity or share your referral link.</div>
            </div>
          </div>
          <div className="app-panel-body flex flex-col gap-3 sm:flex-row">
            <button onClick={handleUpgrade} className="app-button primary bg-[#0f766e] hover:bg-[#0d645d] border-none text-white" type="button">
              Upgrade to Pro
            </button>
            <button onClick={handleShareOnX} className="app-button" type="button">
              Share to get 10 free tasks
            </button>
            <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${refLink}`} target="_blank" rel="noopener noreferrer" className="app-button">
              Referral Link
            </a>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
