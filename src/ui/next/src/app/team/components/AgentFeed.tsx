import React, { useState, useEffect } from 'react';

export type AgentActionRequest = {
  id: string;
  tenant_id: string;
  action_type: string;
  status: string;
  confidence_score: number;
  department: string;
  description: string;
  product_id: string;
  suggested_quantity: number;
  suggested_price_cents: number;
  created_at_unix: number;
  updated_at_unix: number;
};

export default function AgentFeed() {
  const [actions, setActions] = useState<AgentActionRequest[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchActions = async () => {
    try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      const response = await fetch('/api/agents/agent-actions', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });
      if (response.ok) {
        const data = await response.json();
        setActions(data.action_requests || []);
      }
    } catch (error) {
      console.error("Failed to fetch agent actions", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchActions();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      setActions(prev => prev.map(a => a.id === id ? { ...a, status: 'Approved' } : a));
      const response = await fetch(`/api/agents/agent-actions/${id}/approve`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        }
      });
      if (!response.ok) fetchActions();
    } catch (error) {
      console.error("Failed to approve", error);
      fetchActions();
    }
  };

  const handleDismiss = async (id: string) => {
     try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      setActions(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/agent-actions/${id}/dismiss`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        }
      });
      if (!response.ok) fetchActions();
    } catch (error) {
      console.error("Failed to dismiss", error);
      fetchActions();
    }
  };

  if (loading) {
    return <div className="text-center py-4 text-sm text-gray-500 font-medium">Loading Agent Feed...</div>;
  }

  if (actions.length === 0) {
    return (
      <div className="text-center py-8">
        <p className="text-gray-500 text-sm">No agent suggestions right now.</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {actions.filter(a => a.status === 'Pending').map(action => (
        <div key={action.id} className="bg-white/60 backdrop-blur-md border border-gray-200 rounded-xl p-4 shadow-sm relative overflow-hidden" data-testid="agent-action-card">
          <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-400 to-indigo-500"></div>
          <div className="flex items-center gap-2 mb-2">
            <span className="text-xs font-bold px-2 py-0.5 bg-orange-100 text-orange-700 rounded-full uppercase tracking-wide">Needs Approval</span>
          </div>

          <p className="text-sm font-semibold text-gray-900 mb-1">{action.department}</p>
          <p className="text-xs text-gray-600 mb-4">{action.description}</p>

          <div className="flex gap-2">
            <button
              onClick={() => handleApprove(action.id)}
              className="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-xs font-semibold py-2 px-3 rounded-lg transition-colors min-h-[44px]"
              data-testid="approve-action-btn"
            >
              Approve
            </button>
            <button
              type="button"
              onClick={() => handleDismiss(action.id)}
              className="bg-gray-100 hover:bg-gray-200 text-gray-700 text-xs font-medium py-2 px-3 rounded-lg transition-colors min-h-[44px]"
              data-testid="dismiss-action-btn"
            >
              Dismiss
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}
