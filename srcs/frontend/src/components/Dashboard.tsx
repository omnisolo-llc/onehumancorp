import { useEffect, useState } from 'react';
import { fetchDashboard, DashboardSnapshot, seedDevData } from '../api';

const Dashboard = () => {
  const [data, setData] = useState<DashboardSnapshot | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadData = async () => {
    try {
      setLoading(true);
      const snapshot = await fetchDashboard();
      if (snapshot.agents.length === 0 && snapshot.meetings.length === 0) {
        await seedDevData();
        const reSnapshot = await fetchDashboard();
        setData(reSnapshot);
      } else {
        setData(snapshot);
      }
    } catch (err: any) {
      setError(err.message || 'Authentication required or API error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  if (loading) return <div className="p-8 text-center text-gray-400">Loading dashboard data...</div>;
  if (error) return (
    <div className="p-8 text-center" style={{ color: 'var(--danger)' }}>
      <h2>Access Denied</h2>
      <p>{error}</p>
      <p style={{ color: 'var(--text-secondary)', fontSize: '14px', marginTop: '8px' }}>
        Please login or inject a valid auth token via local storage to view the dashboard.
      </p>
    </div>
  );
  if (!data) return null;

  const activeAgents = data.agents.filter(a => a.status === 'RUNNING' || a.status === 'ACTIVE').length;

  return (
    <div>
      <div className="page-header">
        <div>
          <h1>Dashboard</h1>
          <p>Real-time overview of the One Human Corp organization.</p>
        </div>
      </div>

      <div className="dashboard-grid">
        <div className="glass-panel stat-card">
          <div className="stat-title">Active Agents</div>
          <div className="stat-value text-indigo-400" style={{ color: '#818cf8' }}>{activeAgents}</div>
        </div>
        <div className="glass-panel stat-card">
          <div className="stat-title">Open Meetings</div>
          <div className="stat-value" style={{ color: '#2dd4bf' }}>{data.meetings.length}</div>
        </div>
        <div className="glass-panel stat-card">
          <div className="stat-title">Total Org Members</div>
          <div className="stat-value" style={{ color: '#c084fc' }}>{data.agents.length + 1}</div>
        </div>
        <div className="glass-panel stat-card">
          <div className="stat-title">Dashboard Updates</div>
          <div className="stat-value" style={{ color: '#fb923c' }}>{data.statuses.length}</div>
        </div>
      </div>

      <div className="glass-panel p-6">
        <h2 style={{ fontSize: '18px', marginTop: 0, marginBottom: '24px' }}>Organization Context</h2>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <div><strong>ID:</strong> {data.organization?.id}</div>
          <div><strong>Name:</strong> {data.organization?.name}</div>
          <div><strong>Domain:</strong> {data.organization?.domain}</div>
          <div><strong>Last Updated:</strong> {new Date(data.updatedAt).toLocaleString()}</div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;