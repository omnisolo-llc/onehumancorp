"use client";

import React, { useState, useEffect } from 'react';
import DepartmentCard from './components/DepartmentCard';
import ApprovalInbox from './components/ApprovalInbox';

export type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
};

const DEPARTMENTS = [
  { id: 'Operations', name: 'The Manager' },
  { id: 'Marketing', name: 'The Promoter' },
  { id: 'Sales', name: 'The Salesperson' },
  { id: 'CustomerSuccess', name: 'The Ambassador' },
  { id: 'Finance', name: 'The Accountant' },
  { id: 'Legal', name: 'The Protector' },
  { id: 'BusinessAdvisory', name: 'The Advisor' },
];

export default function TeamPage() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [selectedDepartment, setSelectedDepartment] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [swarmActivity, setSwarmActivity] = useState<any[]>([]);

  const fetchApprovals = async () => {
    try {
      const response = await fetch('/api/agents/approvals');
      if (response.ok) {
        const data = await response.json();
        setApprovals(data.pending_approvals || []);
      }
    } catch (error) {
      console.error("Failed to fetch approvals", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchApprovals();

    // Connect to Teammate Mesh WebSocket for real-time swarm activity
    const connectSwarmMesh = () => {
        try {
            const ws = new WebSocket(`ws://${window.location.host}/api/v1/mesh/ws?topic=system`);

            ws.onmessage = (event) => {
                try {
                    const payload = JSON.parse(event.data);
                    setSwarmActivity(prev => [{
                        id: Math.random().toString(),
                        agent: payload.agent_id || "Swarm Agent",
                        action: payload.action || "Working on task...",
                        time: new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit', second:'2-digit'})
                    }, ...prev].slice(0, 5)); // Keep last 5
                } catch(e) {
                   // Ignore parsing errors
                }
            };

            return ws;
        } catch(e) {
            console.error("Mesh websocket failed", e);
            return null;
        }
    };

    const ws = connectSwarmMesh();

    return () => {
        if (ws) ws.close();
    };
  }, []);

  const handleApprove = async (id: string) => {
    try {
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved: true })
      });
      if (!response.ok) fetchApprovals();
    } catch (error) {
      console.error("Failed to approve", error);
      fetchApprovals();
    }
  };

  const handleReject = async (id: string) => {
     try {
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved: false })
      });
      if (!response.ok) fetchApprovals();
    } catch (error) {
      console.error("Failed to reject", error);
      fetchApprovals();
    }
  };

  if (selectedDepartment) {
    const deptInfo = DEPARTMENTS.find(d => d.id === selectedDepartment);
    const deptApprovals = approvals.filter(a => a.department === selectedDepartment);

    return (
      <ApprovalInbox
        departmentId={selectedDepartment}
        departmentName={deptInfo?.name || selectedDepartment}
        approvals={deptApprovals}
        onBack={() => setSelectedDepartment(null)}
        onApprove={handleApprove}
        onReject={handleReject}
      />
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/60 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Your Team</h1>
          <p className="text-gray-500 text-sm mt-1">Invisible specialized AI teams</p>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 hide-scrollbar">
          {loading ? (
             <div className="flex justify-center py-10">
               <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
             </div>
          ) : (
            DEPARTMENTS.map(dept => {
              const pendingCount = approvals.filter(a => a.department === dept.id).length;
              return (
                <DepartmentCard
                  key={dept.id}
                  name={dept.name}
                  pendingCount={pendingCount}
                  onClick={() => setSelectedDepartment(dept.id)}
                />
              );
            })
          )}

          {/* Swarm Observability / Team Activity Panel */}
          <section className="mt-8">
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Team Activity</h2>
                <div className="flex items-center gap-2 px-3 py-1 bg-green-50 rounded-full border border-green-100">
                    <div className="w-2 h-2 rounded-full animate-pulse" style={{ backgroundColor: '#34C759' }}></div>
                    <span className="text-xs font-medium" style={{ color: '#34C759' }}>Swarm Online</span>
                </div>
            </div>

            <div className="shadow-sm overflow-hidden" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                {swarmActivity.length === 0 ? (
                    <div className="p-8 text-center">
                        <div className="inline-block w-8 h-8 rounded-full border-2 border-gray-200 border-t-blue-500 animate-spin mb-3"></div>
                        <p className="text-sm" style={{ color: '#86868B' }}>Waiting for team activity...</p>
                    </div>
                ) : (
                    <div className="flex flex-col">
                        {swarmActivity.map((activity, index) => (
                            <div key={activity.id} className="flex items-center justify-between p-4 border-b last:border-b-0 transition-all duration-500 ease-in-out hover:bg-white/40" style={{ borderBottomColor: 'rgba(0,0,0,0.05)' }}>
                                <div className="flex items-center gap-4">
                                    <div className="w-10 h-10 rounded-full flex items-center justify-center text-xl shadow-sm" style={{ background: '#ffffff', border: '1px solid rgba(0,0,0,0.05)' }}>
                                        🤖
                                    </div>
                                    <div>
                                        <p className="text-sm font-semibold" style={{ color: '#1D1D1F' }}>{activity.agent}</p>
                                        <p className="text-sm" style={{ color: '#86868B' }}>{activity.action}</p>
                                    </div>
                                </div>
                                <div className="flex flex-col items-end gap-1">
                                    <span className="text-xs font-medium" style={{ color: '#86868B' }}>{activity.time}</span>
                                    {activity.status === 'success' && <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: '#34C759' }}></span>}
                                    {activity.status === 'warning' && <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: '#FF9500' }}></span>}
                                    {activity.status === 'info' && <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: '#0066FF' }}></span>}
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>
          </section>

        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
