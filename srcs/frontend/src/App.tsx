import React, { useEffect, useState } from 'react';
import './App.css';
import { fetchDashboard, DashboardResponse, hireAgent, fireAgent } from './api';

export default function App() {
  const [dashboard, setDashboard] = useState<DashboardResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadDashboard();
  }, []);

  const loadDashboard = async () => {
    try {
      const data = await fetchDashboard();
      setDashboard(data);
      setError(null);
    } catch (err: any) {
      setError(err.message || 'Failed to load dashboard');
    }
  };

  const handleHire = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);
    const name = formData.get('name') as string;
    const role = formData.get('role') as string;
    if (!name || !role) return;

    try {
      await hireAgent(name, role);
      await loadDashboard(); // refresh
      (e.target as HTMLFormElement).reset();
    } catch (err: any) {
      setError(err.message || 'Failed to hire agent');
    }
  };

  const handleFire = async (agentId: string) => {
    try {
      await fireAgent(agentId);
      await loadDashboard(); // refresh
    } catch (err: any) {
      setError(err.message || 'Failed to fire agent');
    }
  };

  return (
    <div className="dashboard-container">
      <div className="header">
        <h1>One Human Corp Control Panel</h1>
        {dashboard && <span>Organization: {dashboard.name}</span>}
      </div>

      {error && <div className="error-message">{error}</div>}

      {!dashboard && !error && <div className="loading">Loading dashboard...</div>}

      {dashboard && (
        <>
          <div className="panel">
            <div className="panel-header">Dynamic Scaling Wizard</div>
            <form className="wizard-form" onSubmit={handleHire}>
              <div className="form-group">
                <label htmlFor="name">Agent Name</label>
                <input type="text" id="name" name="name" placeholder="e.g. New SWE" required />
              </div>
              <div className="form-group">
                <label htmlFor="role">Agent Role</label>
                <select id="role" name="role" required>
                  <option value="software_engineer">Software Engineer</option>
                  <option value="product_manager">Product Manager</option>
                  <option value="qa_tester">QA Tester</option>
                  <option value="security_engineer">Security Engineer</option>
                  <option value="marketing_manager">Marketing Manager</option>
                  <option value="designer">UI/UX Designer</option>
                  <option value="sales_representative">Sales Representative</option>
                </select>
              </div>
              <button type="submit" className="primary">Hire Agent</button>
            </form>
          </div>

          <div className="panel">
            <div className="panel-header">Active Agents ({dashboard.agents?.length || 0})</div>
            <div className="agent-grid">
              {dashboard.agents && dashboard.agents.map((agent) => (
                <div key={agent.id} className="agent-card">
                  <div className="agent-info">
                    <h3>{agent.name}</h3>
                    <p>{agent.role}</p>
                    <span className="status-badge">{agent.status || 'Active'}</span>
                  </div>
                  <button className="danger" onClick={() => handleFire(agent.id)}>Fire</button>
                </div>
              ))}
              {(!dashboard.agents || dashboard.agents.length === 0) && (
                <p style={{ color: 'var(--text-secondary)' }}>No active agents.</p>
              )}
            </div>
          </div>
        </>
      )}
    </div>
  );
}
