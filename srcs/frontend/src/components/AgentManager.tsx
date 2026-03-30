import { useEffect, useState } from 'react';
import { fetchDashboard, fireAgent, Agent, seedDevData } from '../api';
import HireAgentModal from './HireAgentModal';

const AgentManager = () => {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isHireModalOpen, setIsHireModalOpen] = useState(false);
  const [isFiring, setIsFiring] = useState<string | null>(null);

  const loadData = async () => {
    try {
      setLoading(true);
      let data = await fetchDashboard();
      if (data.agents.length === 0 && data.meetings.length === 0) {
        await seedDevData();
        data = await fetchDashboard();
      }
      setAgents(data.agents || []);
      setError(null);
    } catch (err: any) {
      setError(err.message || 'Authentication required or API error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const handleFire = async (agentId: string) => {
    if (!window.confirm('Are you sure you want to fire this agent?')) return;
    setIsFiring(agentId);
    try {
      await fireAgent(agentId);
      await loadData();
    } catch (err: any) {
      setError(`Failed to fire agent: ${err.message}`);
    } finally {
      setIsFiring(null);
    }
  };

  return (
    <div>
      <div className="page-header">
        <div>
          <h1>HR & Operations</h1>
          <p>Manage the AI workforce hierarchy.</p>
        </div>
        <button className="btn btn-primary" onClick={() => setIsHireModalOpen(true)} disabled={loading || !!error}>
          + Hire Agent
        </button>
      </div>

      {error && <div style={{ color: 'var(--danger)', marginBottom: '16px', background: 'rgba(239, 68, 68, 0.1)', padding: '16px', borderRadius: '8px' }}>
        <strong>Access Denied: </strong>
        {error}
        <div style={{ marginTop: '8px', fontSize: '14px', color: 'var(--text-secondary)' }}>
          Please ensure you have a valid auth token to perform these actions.
        </div>
      </div>}

      {!error && (
      <div className="glass-panel" style={{ padding: '24px' }}>
        {loading ? (
          <div style={{ color: 'var(--text-secondary)' }}>Loading agents...</div>
        ) : agents.length === 0 ? (
          <div style={{ textAlign: 'center', padding: '40px', color: 'var(--text-secondary)' }}>
            No agents found. Hire your first agent to get started.
          </div>
        ) : (
          <div className="agent-list">
            {agents.map((agent) => (
              <div key={agent.id} className="glass-panel agent-card">
                <div className="agent-info">
                  <div className={`agent-avatar ${agent.status === 'RUNNING' || agent.status === 'ACTIVE' ? 'active' : ''}`}>
                    🤖
                  </div>
                  <div className="agent-details">
                    <h3>{agent.name}</h3>
                    <p>{agent.role} · ID: {agent.id.slice(-8)}</p>
                  </div>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
                  <span className={`agent-status ${agent.status === 'RUNNING' || agent.status === 'ACTIVE' ? 'active' : 'idle'}`}>
                    {agent.status}
                  </span>
                  <button
                    className="btn btn-danger"
                    onClick={() => handleFire(agent.id)}
                    disabled={isFiring === agent.id}
                  >
                    {isFiring === agent.id ? 'Firing...' : 'Fire'}
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
      )}

      <HireAgentModal
        isOpen={isHireModalOpen}
        onClose={() => setIsHireModalOpen(false)}
        onSuccess={loadData}
      />
    </div>
  );
};

export default AgentManager;