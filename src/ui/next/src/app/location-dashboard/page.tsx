"use client";
import { useState, useEffect, useRef } from 'react';

interface Task {
  id: string;
  title: string;
  status: string;
  auto_dreamed?: boolean;
}

interface Alert {
  id: string;
  message: string;
  severity: string;
}

interface Staff {
  id: string;
  name: string;
  role: string;
  status?: string;
}

export default function LocationManagerDashboard() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [staff, setStaff] = useState<Staff[]>([]);
  const [showEscalationModal, setShowEscalationModal] = useState(false);
  const [selectedAlert, setSelectedAlert] = useState<Alert | null>(null);
  const [escalationDraft, setEscalationDraft] = useState('');
  const [isDrafting, setIsDrafting] = useState(false);

  const [dataError, setDataError] = useState('');
  const [loaded, setLoaded] = useState(false);
  const [escalationError, setEscalationError] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const requestEpoch = useRef(0);
  const pending = useRef<AbortController | null>(null);
  const submitLock = useRef(false);

  useEffect(() => {
    const controller = new AbortController();
    const read = async () => {
      try {
        const response = await fetch('/api/v1/location/dashboard', { signal: controller.signal });
        if (response.status !== 200) throw new Error('unavailable');
        const data = await response.json();
        const rows = (value: unknown, fields: string[]) => Array.isArray(value) && value.length <= 1000
          && value.every(row => row && typeof row === 'object' && fields.every(field => typeof row[field] === 'string'));
        if (!data || data.error != null || data.success !== undefined && data.success !== true
          || !rows(data.tasks, ['id','title','status']) || !rows(data.alerts, ['id','message','severity'])
          || !rows(data.staff, ['id','name','role'])) throw new Error('invalid data');
        if (!controller.signal.aborted) { setTasks(data.tasks); setAlerts(data.alerts); setStaff(data.staff); setLoaded(true); }
      } catch {
        if (!controller.signal.aborted) setDataError('Location information could not be loaded. No staff, tasks or summaries were inferred.');
      }
    };
    void read();
    return () => { controller.abort(); ++requestEpoch.current; pending.current?.abort(); };
  }, []);

  const closeEscalation = () => {
    ++requestEpoch.current; pending.current?.abort(); pending.current = null;
    submitLock.current = false; setIsSubmitting(false); setIsDrafting(false);
    setShowEscalationModal(false); setEscalationDraft(''); setEscalationError('');
  };

  const handleEscalateClick = (alert: Alert) => {
    pending.current?.abort(); const epoch = ++requestEpoch.current;
    const controller = new AbortController(); pending.current = controller;
    setSelectedAlert(alert); setEscalationDraft(''); setEscalationError(''); setShowEscalationModal(true); setIsDrafting(true);
    const draft = async () => {
      try {
        const response = await fetch('/api/v1/agent/draft-escalation', {
          method:'POST', signal:controller.signal, headers:{'Content-Type':'application/json'},
          body:JSON.stringify({alertId:alert.id,context:alert.message}),
        });
        if (response.status !== 200) throw new Error('unavailable');
        const data=await response.json();
        if (!data || data.error != null || data.success !== undefined && data.success !== true
          || typeof data.draft !== 'string' || !data.draft.trim() || data.draft.length > 200_000) throw new Error('invalid draft');
        if (requestEpoch.current === epoch && !controller.signal.aborted) setEscalationDraft(data.draft);
      } catch {
        if (requestEpoch.current === epoch && !controller.signal.aborted) setEscalationError('No escalation draft was confirmed. The recorded summary remains unchanged.');
      } finally {
        if (requestEpoch.current === epoch && !controller.signal.aborted) setIsDrafting(false);
      }
    };
    void draft();
  };

  const submitEscalation = async () => {
    if (!selectedAlert || isDrafting || !escalationDraft.trim() || submitLock.current) return;
    submitLock.current=true; setIsSubmitting(true); setEscalationError('');
    const epoch=requestEpoch.current; const controller=new AbortController();pending.current=controller;
    try {
      const response=await fetch('/api/v1/location/escalate', {
        method:'POST',signal:controller.signal,headers:{'Content-Type':'application/json'},
        body:JSON.stringify({alertId:selectedAlert.id,draft:escalationDraft}),
      });
      // The current backend explicitly reports this capability unavailable.
      // A transport success alone cannot prove a durable escalation or delivery.
      if (!response.ok) throw new Error('unavailable');
      throw new Error('A verified durable escalation receipt is required');
    } catch {
      if (requestEpoch.current === epoch && !controller.signal.aborted) setEscalationError('No escalation was confirmed. Your draft and the recorded summary are retained.');
    } finally {
      if (requestEpoch.current === epoch) {submitLock.current=false;setIsSubmitting(false);}
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#000000] text-[#1D1D1F] dark:text-[#F5F5F7] font-sans">
      <header className="p-4 bg-white/70 dark:bg-black/70 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-50 border-b border-gray-200 dark:border-gray-800">
        <h1 className="text-xl font-semibold">Location Dashboard</h1>
        <p className="text-sm text-gray-500">Recorded staff and task information</p>
      </header>

      <main className="p-4 max-w-[375px] mx-auto md:max-w-2xl lg:max-w-4xl space-y-6">
        {dataError && <p role="alert">{dataError}</p>}
        {!loaded && !dataError && <p role="status">Loading recorded location information…</p>}

        {/* Active Alerts */}
        <section>
          <h2 className="text-lg font-medium mb-3">Staff Summaries</h2>
          {alerts.length === 0 ? (
            <div className="p-4 bg-white dark:bg-gray-900 rounded-xl shadow-sm text-center text-sm text-gray-500">
              {loaded ? 'No recorded staff summaries.' : 'Staff summaries are unavailable.'}
            </div>
          ) : (
            <div className="space-y-3">
              {alerts.map(alert => (
                <div key={alert.id} className="p-4 bg-red-50 dark:bg-red-900/20 rounded-xl border border-red-100 dark:border-red-800/30 flex flex-col gap-3">
                  <p className="text-sm font-medium text-red-800 dark:text-red-200">{alert.message}</p>
                  <button
                    onClick={() => handleEscalateClick(alert)}
                    className="self-start text-xs font-semibold bg-red-600 text-white px-3 py-1.5 rounded-full hover:bg-red-700 transition"
                  >
                    Escalate to Owner
                  </button>
                </div>
              ))}
            </div>
          )}
        </section>

        {/* Today's Local Tasks */}
        <section>
          <h2 className="text-lg font-medium mb-3">Recorded Tasks</h2>
          <div className="bg-white dark:bg-gray-900 rounded-xl shadow-sm divide-y divide-gray-100 dark:divide-gray-800">
            {tasks.map(task => (
              <div key={task.id} className="p-4 flex items-center justify-between">
                <span className={`text-sm ${task.status === 'COMPLETED' ? 'line-through text-gray-400' : ''}`}>
                  {task.title}
                </span>
                <span className={`text-xs px-2 py-1 rounded-full ${
                  task.status === 'COMPLETED' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300' : 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300'
                }`}>
                  {task.status}
                </span>
              </div>
            ))}
          </div>
        </section>

        {/* Staff on Shift */}
        <section>
          <h2 className="text-lg font-medium mb-3">Staff</h2>
          <div className="bg-white dark:bg-gray-900 rounded-xl shadow-sm p-4 space-y-3">
            {staff.map(member => (
              <div key={member.id} className="flex justify-between items-center text-sm">
                <div>
                  <p className="font-medium">{member.name}</p>
                  <p className="text-xs text-gray-500">{member.role}</p>
                </div>
                <div className="flex items-center gap-2">
                  <span className="w-2 h-2 rounded-full bg-gray-400"></span>
                  <span className="text-xs text-gray-600 dark:text-gray-400">{member.status ?? 'Shift status unavailable'}</span>
                </div>
              </div>
            ))}
          </div>
        </section>
      </main>

      {/* Escalation Modal */}
      {showEscalationModal && (
        <div role="dialog" aria-modal="true" aria-label="Escalate Issue" className="fixed inset-0 bg-black/50 backdrop-blur-[30px] saturate-[210%] z-[100] flex items-end md:items-center justify-center p-4">
          <div className="bg-white dark:bg-gray-900 w-full max-w-md rounded-3xl p-6 shadow-2xl animate-in slide-in-from-bottom-4 md:slide-in-from-bottom-0 md:zoom-in-95">
            <h3 className="text-xl font-semibold mb-2">Escalate Issue</h3>
            <p className="text-sm text-gray-500 mb-6">Review the draft. Sending requires a verified escalation service.</p>

            {escalationError && <p role="alert">{escalationError}</p>}
            <button type="button" onClick={closeEscalation}>Cancel</button>
            {isDrafting ? (
              <div className="flex items-center justify-center py-8">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-[#0066FF]"></div>
              </div>
            ) : (
              <div className="space-y-4">
                <textarea
                  aria-label="Escalation draft"
                  value={escalationDraft}
                  onChange={(e) => setEscalationDraft(e.target.value)}
                  className="w-full h-32 p-3 text-sm bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                />
                <div className="flex gap-3">
                  <button
                    onClick={submitEscalation}
                    disabled={isSubmitting || !escalationDraft.trim()}
                    className="flex-1 py-3 text-sm font-semibold rounded-xl bg-[#0066FF] text-white hover:bg-[#0071E3] transition"
                  >
                    Send to Owner
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
