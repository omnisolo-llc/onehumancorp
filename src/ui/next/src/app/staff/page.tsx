'use client';
import { useState, useEffect } from 'react';

type StaffTask = {
    id: string;
    description: string;
    priority: string;
    status: string;
};

type ShiftSummary = {
    id: string;
    shift_date: string;
    summary_text: string;
};

export default function StaffPage() {
    const [tasks, setTasks] = useState<StaffTask[]>([]);
    const [summaries, setSummaries] = useState<ShiftSummary[]>([]);
    const [activeTab, setActiveTab] = useState<'tasks' | 'summaries'>('tasks');
    const [escalationText, setEscalationText] = useState('');
    const [escalations, setEscalations] = useState<{id: string, text: string, status: string}[]>([]);

    useEffect(() => {
        if (activeTab === 'tasks') {
            fetch('/api/staff/tasks')
                .then(res => res.json())
                .then(data => {
                    if (data && data.tasks) {
                        setTasks(data.tasks);
                    }
                })
                .catch(console.error);
        } else {
            fetch('/api/staff/summaries')
                .then(res => res.json())
                .then(data => {
                    if (data && data.summaries) {
                        setSummaries(data.summaries);
                    }
                })
                .catch(console.error);
        }
    }, [activeTab]);

    const markTaskComplete = async (taskId: string) => {
        // Optimistic update
        setTasks(prev => prev.map(t => t.id === taskId ? { ...t, status: 'completed' } : t));

        try {
            await fetch(`/api/staff/tasks/${taskId}`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    status: 'completed',
                    offline_timestamp: new Date().toISOString()
                })
            });
        } catch (error) {
            console.error('Failed to sync task completion. Will retry when online.');
        }
    };

    const handleEscalationSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!escalationText.trim()) return;

        // Optimistic update
        const tempId = `temp_${Date.now()}`;
        setEscalations(prev => [{ id: tempId, text: escalationText, status: 'pending' }, ...prev]);
        const text = escalationText;
        setEscalationText('');

        try {
            await fetch('/api/staff/escalations', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    escalation_text: text,
                    staff_id: 'staff_1'
                })
            });
        } catch (error) {
            console.error('Failed to send escalation.');
        }
    };

    return (
        <div className="p-4 bg-gray-50 min-h-screen">
            <h1 className="text-2xl font-bold mb-4">Staff Mesh</h1>

            <div className="flex gap-4 mb-4">
                <button
                    onClick={() => setActiveTab('tasks')}
                    data-testid="view-tasks-tab"
                    className={`px-4 py-2 rounded ${activeTab === 'tasks' ? 'bg-blue-600 text-white' : 'bg-white'}`}
                >
                    Tasks
                </button>
                <button
                    onClick={() => setActiveTab('summaries')}
                    data-testid="view-summaries-tab"
                    className={`px-4 py-2 rounded ${activeTab === 'summaries' ? 'bg-blue-600 text-white' : 'bg-white'}`}
                >
                    Summaries
                </button>
            </div>

            {activeTab === 'tasks' && (
                <div className="space-y-4">
                    {/* Escalation Form */}
                    <div className="bg-white/65 p-4 rounded-xl border border-white/40 shadow-sm mb-4">
                        <form onSubmit={handleEscalationSubmit} className="flex gap-2">
                            <input
                                type="text"
                                data-testid="escalation-input"
                                placeholder="Report an issue..."
                                className="flex-1 p-2 border rounded"
                                value={escalationText}
                                onChange={(e) => setEscalationText(e.target.value)}
                            />
                            <button
                                type="submit"
                                data-testid="submit-escalation-btn"
                                className="bg-red-500 text-white px-4 py-2 rounded"
                            >
                                Escalate
                            </button>
                        </form>
                    </div>

                    {/* Pending Escalations */}
                    {escalations.map(esc => (
                        <div key={esc.id} data-testid="escalation-card" className="bg-red-100 p-3 rounded">
                            <p className="font-bold">Escalation: {esc.text} ({esc.status})</p>
                        </div>
                    ))}

                    {/* Task List */}
                    {tasks.map(task => (
                        <div key={task.id} data-testid="staff-task-card" className="bg-white/65 p-4 rounded-xl border border-white/40 shadow-sm flex justify-between items-center">
                            <div>
                                <h3 className="font-bold">{task.description}</h3>
                                <p className="text-sm text-gray-500">Priority: {task.priority}</p>
                                <p className="text-sm">Status: {task.status}</p>
                            </div>
                            {task.status !== 'completed' && (
                                <button
                                    onClick={() => markTaskComplete(task.id)}
                                    data-testid="mark-complete-btn"
                                    className="bg-green-500 text-white px-3 py-1 rounded"
                                >
                                    Complete
                                </button>
                            )}
                        </div>
                    ))}
                    {tasks.length === 0 && <p>No tasks at the moment.</p>}
                </div>
            )}

            {activeTab === 'summaries' && (
                <div className="space-y-4">
                    {summaries.map(summary => (
                        <div key={summary.id} data-testid="shift-summary-card" className="bg-white/65 p-4 rounded-xl border border-white/40 shadow-sm">
                            <h3 className="font-bold">{summary.shift_date}</h3>
                            <p>{summary.summary_text}</p>
                        </div>
                    ))}
                    {summaries.length === 0 && <p>No summaries available.</p>}
                </div>
            )}
        </div>
    );
}
