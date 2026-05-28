"use client";

import React, { useState, useEffect, useCallback, useRef } from 'react';

// Types for our Offline Identity Mesh
type StaffIdentity = {
  id: string;
  name: string;
  avatar: string;
  role: string;
  pinHash: string; // Simplified for client-side evaluation
};

type AuditLogEvent = {
  event_id: string;
  staff_id: string;
  action: string;
  occurred_at: string;
  synced: boolean;
};

// Mock local secure enclave cache
const LOCAL_STAFF_CACHE: StaffIdentity[] = [
  { id: 'staff_1', name: 'Fatima', avatar: '👩🏽‍🍳', role: 'Owner', pinHash: 'btoa(1234)' },
  { id: 'staff_2', name: 'Carlos', avatar: '👨🏽‍🔧', role: 'Manager', pinHash: 'btoa(5678)' },
  { id: 'staff_3', name: 'Alex', avatar: '🧑🏻‍💻', role: 'Cashier', pinHash: 'btoa(9012)' },
];

// Helper to simulate hash check
const verifyPin = (pin: string, expectedHash: string) => {
    // In a real implementation this would use WebCrypto or native Secure Enclave calls
    return `btoa(${pin})` === expectedHash;
};

export default function TerminalPage() {
  const [isOffline, setIsOffline] = useState(false);
  const [activeStaff, setActiveStaff] = useState<StaffIdentity | null>(null);

  // UI States
  const [selectedAvatar, setSelectedAvatar] = useState<StaffIdentity | null>(null);
  const [pinInput, setPinInput] = useState('');
  const [authError, setAuthError] = useState(false);
  const [isBiometricPrompting, setIsBiometricPrompting] = useState(false);

  const [auditLog, setAuditLog] = useState<AuditLogEvent[]>([]);

  // Monitor network status
  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    // Initial state
    setIsOffline(!navigator.onLine);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  // Load audit log from local storage on mount
  useEffect(() => {
      try {
          const savedLog = localStorage.getItem('ohc_offline_queue_audit');
          if (savedLog) {
              setAuditLog(JSON.parse(savedLog));
          }
      } catch (e) {
          console.error("Failed to parse audit log", e);
      }
  }, []);

  // Sync Engine: when online, sync pending logs
  useEffect(() => {
    if (!isOffline && auditLog.length > 0) {
      const unsynced = auditLog.filter(log => !log.synced);
      if (unsynced.length > 0) {
        // Simulate network request
        console.log("Syncing offline audit logs to cloud...", unsynced);

        // Mark as synced
        const syncedLog = auditLog.map(log => ({ ...log, synced: true }));
        setAuditLog(syncedLog);
        try {
            localStorage.setItem('ohc_offline_queue_audit', JSON.stringify(syncedLog));
        } catch(e){}

        // Reset queue to keep it clean, but could just keep it marked as synced
        setTimeout(() => {
            setAuditLog([]);
            try {
                localStorage.removeItem('ohc_offline_queue_audit');
            } catch(e){}
        }, 1000);
      }
    }
  }, [isOffline, auditLog]);

  const logAction = (action: string, staff_id: string) => {
      const event: AuditLogEvent = {
          event_id: `evt_${Date.now()}_${Math.random().toString(36).substring(7)}`,
          staff_id,
          action,
          occurred_at: new Date().toISOString(),
          synced: !isOffline,
      };

      const newLog = [...auditLog, event];
      setAuditLog(newLog);

      // Always store locally first (Offline-First CRDT principle)
      try {
        localStorage.setItem('ohc_offline_queue_audit', JSON.stringify(newLog));
      } catch (e) {
        console.error("Failed to save audit log locally");
      }
  };

  const handleAvatarTap = (staff: StaffIdentity) => {
    setSelectedAvatar(staff);
    setPinInput('');
    setAuthError(false);
    setIsBiometricPrompting(true);

    // Simulate biometric check (fails back to PIN after 1s)
    setTimeout(() => {
        setIsBiometricPrompting(false);
    }, 1000);
  };

  const handlePinSubmit = () => {
    if (selectedAvatar && verifyPin(pinInput, selectedAvatar.pinHash)) {
      // Sub-100ms switch
      setActiveStaff(selectedAvatar);
      setSelectedAvatar(null);
      setPinInput('');
      logAction('login', selectedAvatar.id);
    } else {
      setAuthError(true);
      setPinInput('');
    }
  };

  const handleLogout = () => {
      if (activeStaff) {
          logAction('logout', activeStaff.id);
      }
      setActiveStaff(null);
  };

  const handleDashboardAction = (action: string) => {
      if (activeStaff) {
          logAction(action, activeStaff.id);
      }
  };

  // Render the actual dashboard (Active Staff View)
  if (activeStaff) {
      return (
          <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter py-10 relative">
            <div className="w-[375px] max-w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative rounded-[40px] border-[8px] border-black">
                {/* Status Bar */}
                <div className="flex justify-between items-center px-6 py-4 bg-gray-50 border-b border-gray-100">
                    <div className="flex items-center gap-2">
                        <span className="text-2xl">{activeStaff.avatar}</span>
                        <div>
                            <p className="text-xs font-bold text-gray-900">{activeStaff.name}</p>
                            <p className="text-[10px] text-gray-500 uppercase">{activeStaff.role}</p>
                        </div>
                    </div>

                    <div className="flex items-center gap-2">
                        {isOffline && (
                            <div className="flex items-center gap-1 bg-red-100 px-2 py-1 rounded-full" id="offline-badge">
                                <div className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
                                <span className="text-[10px] font-bold text-red-700">Offline</span>
                            </div>
                        )}
                        <button
                            onClick={handleLogout}
                            id="logout-btn"
                            className="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-600 hover:bg-gray-300"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" /></svg>
                        </button>
                    </div>
                </div>

                {/* Main Content */}
                <div className="flex-1 p-6 bg-gray-50 flex flex-col gap-4">
                    <h1 className="text-2xl font-bold font-outfit text-gray-900">Point of Sale</h1>

                    <div className="grid grid-cols-2 gap-4">
                        <button
                            onClick={() => handleDashboardAction('sale_coffee')}
                            className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 flex flex-col items-center justify-center gap-2 active:scale-95 transition-transform"
                            id="action-coffee"
                        >
                            <span className="text-3xl">☕️</span>
                            <span className="text-sm font-semibold text-gray-700">Coffee ($4)</span>
                        </button>
                        <button
                            onClick={() => handleDashboardAction('sale_pastry')}
                            className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 flex flex-col items-center justify-center gap-2 active:scale-95 transition-transform"
                            id="action-pastry"
                        >
                            <span className="text-3xl">🥐</span>
                            <span className="text-sm font-semibold text-gray-700">Pastry ($3)</span>
                        </button>
                    </div>

                    <div className="mt-auto">
                        <h3 className="text-xs font-bold text-gray-500 uppercase mb-2">Recent Offline Actions</h3>
                        <div className="bg-white rounded-xl border border-gray-100 p-2 max-h-32 overflow-y-auto" id="audit-log">
                            {auditLog.filter(l => l.staff_id === activeStaff.id).map(log => (
                                <div key={log.event_id} className="flex justify-between items-center py-1 border-b border-gray-50 last:border-0">
                                    <span className="text-xs text-gray-700">{log.action}</span>
                                    <span className={`text-[10px] ${log.synced ? 'text-green-500' : 'text-orange-500'}`}>
                                        {log.synced ? 'Synced' : 'Pending'}
                                    </span>
                                </div>
                            ))}
                            {auditLog.filter(l => l.staff_id === activeStaff.id).length === 0 && (
                                <p className="text-xs text-gray-400 text-center py-2">No recent actions</p>
                            )}
                        </div>
                    </div>
                </div>
            </div>
          </div>
      );
  }

  // Render Lock Screen
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter py-10 relative">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-indigo-900 to-slate-900 shadow-2xl overflow-hidden flex flex-col relative rounded-[40px] border-[8px] border-black">

        {/* Background Blur Image simulation */}
        <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1557683316-973673baf926?q=80&w=1000')] bg-cover bg-center opacity-30 mix-blend-overlay"></div>
        <div className="absolute inset-0 backdrop-blur-[60px] bg-black/20"></div>

        {/* Top Status Bar indicator */}
        <div className="absolute top-4 right-6 z-20 flex items-center gap-2">
            {isOffline && (
                <div className="flex items-center gap-1 bg-black/40 backdrop-blur-md px-3 py-1.5 rounded-full border border-white/10" id="lock-offline-badge">
                    <div className="w-2 h-2 bg-red-400 rounded-full animate-pulse shadow-[0_0_8px_rgba(248,113,113,0.8)]"></div>
                    <span className="text-[10px] font-bold text-white tracking-wide">OFFLINE</span>
                </div>
            )}
        </div>

        <div className="relative z-10 flex flex-col items-center justify-center flex-1 p-6">
            <h1 className="text-3xl font-bold font-outfit text-white mb-2 tracking-tight">Select Staff</h1>
            <p className="text-white/60 text-sm mb-12 text-center">Tap your face to start working</p>

            <div className="grid grid-cols-2 gap-6 w-full px-4">
                {LOCAL_STAFF_CACHE.map(staff => (
                    <button
                        key={staff.id}
                        id={`staff-btn-${staff.id}`}
                        onClick={() => handleAvatarTap(staff)}
                        className="flex flex-col items-center gap-3 transition-transform active:scale-95"
                    >
                        <div className="w-24 h-24 rounded-[32px] bg-white/10 backdrop-blur-xl border border-white/20 flex items-center justify-center text-5xl shadow-xl hover:bg-white/20 transition-colors">
                            {staff.avatar}
                        </div>
                        <div className="text-center">
                            <p className="text-white font-semibold">{staff.name}</p>
                            <p className="text-white/50 text-xs font-medium">{staff.role}</p>
                        </div>
                    </button>
                ))}
            </div>
        </div>

        {/* Challenge Modal (Glassmorphism) */}
        {selectedAvatar && (
            <div className="absolute inset-0 z-50 flex items-end justify-center pb-8 bg-black/40 backdrop-blur-sm animate-in fade-in duration-200" id="challenge-modal">
                <div className="w-[90%] bg-white/70 backdrop-blur-[40px] saturate-[210%] rounded-[32px] p-6 border border-white/40 shadow-2xl flex flex-col items-center translate-y-0 animate-in slide-in-from-bottom-8 duration-300">
                    <div className="w-12 h-1 bg-black/10 rounded-full mb-6"></div>

                    <div className="w-16 h-16 rounded-full bg-white flex items-center justify-center text-3xl shadow-sm mb-4 border border-gray-100">
                        {selectedAvatar.avatar}
                    </div>

                    <h3 className="text-xl font-bold text-gray-900 mb-1">Welcome back, {selectedAvatar.name}</h3>

                    {isBiometricPrompting ? (
                        <div className="flex flex-col items-center py-6 gap-3 w-full">
                            <svg className="w-12 h-12 text-[#0071E3] animate-pulse" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 11c0 3.517-1.009 6.799-2.753 9.571m-3.44-2.04l.054-.09A13.916 13.916 0 008 11a4 4 0 118 0c0 1.017-.07 2.019-.203 3m-2.118 6.844A21.88 21.88 0 0015.171 17m3.839 1.132c.645-2.266.99-4.659.99-7.132A8 8 0 008 4.07M3 15.364c.64-1.319 1-2.8 1-4.364 0-1.457.39-2.823 1.07-4" /></svg>
                            <p className="text-sm font-medium text-gray-600">Face ID</p>
                        </div>
                    ) : (
                        <div className="w-full flex flex-col items-center pt-4">
                            <p className="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-3">Enter PIN</p>

                            <div className="flex gap-2 mb-6">
                                {[0, 1, 2, 3].map(i => (
                                    <div key={i} className={`w-4 h-4 rounded-full border-2 ${pinInput.length > i ? 'bg-gray-800 border-gray-800' : 'border-gray-300'} transition-all`}></div>
                                ))}
                            </div>

                            {authError && <p className="text-red-500 text-xs mb-4 font-semibold">Incorrect PIN</p>}

                            <div className="grid grid-cols-3 gap-x-6 gap-y-4 w-full px-6 mb-6">
                                {[1, 2, 3, 4, 5, 6, 7, 8, 9].map(num => (
                                    <button
                                        key={num}
                                        onClick={() => {
                                            if (pinInput.length < 4) {
                                                const newPin = pinInput + num;
                                                setPinInput(newPin);
                                                if (newPin.length === 4) {
                                                    // Let react batch updates, submit on next tick
                                                    setTimeout(() => {
                                                        const event = new CustomEvent('submit-pin', { detail: newPin });
                                                        document.dispatchEvent(event);
                                                    }, 50);
                                                }
                                            }
                                        }}
                                        className="w-16 h-16 rounded-full bg-white/50 hover:bg-white flex items-center justify-center text-2xl font-medium text-gray-900 active:bg-gray-200 transition-colors"
                                        id={`pin-${num}`}
                                    >
                                        {num}
                                    </button>
                                ))}
                                <div></div>
                                <button
                                    onClick={() => {
                                        if (pinInput.length < 4) {
                                            const newPin = pinInput + '0';
                                            setPinInput(newPin);
                                            if (newPin.length === 4) {
                                                setTimeout(() => {
                                                    const event = new CustomEvent('submit-pin', { detail: newPin });
                                                    document.dispatchEvent(event);
                                                }, 50);
                                            }
                                        }
                                    }}
                                    className="w-16 h-16 rounded-full bg-white/50 hover:bg-white flex items-center justify-center text-2xl font-medium text-gray-900 active:bg-gray-200 transition-colors"
                                    id="pin-0"
                                >
                                    0
                                </button>
                                <button
                                    onClick={() => setPinInput(prev => prev.slice(0, -1))}
                                    className="w-16 h-16 rounded-full flex items-center justify-center text-gray-600 active:bg-black/10 transition-colors"
                                >
                                    <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2M3 12l6.414 6.414a2 2 0 001.414.586H19a2 2 0 002-2V7a2 2 0 00-2-2h-8.172a2 2 0 00-1.414.586L3 12z" /></svg>
                                </button>
                            </div>

                            {/* Hidden submit trigger for React batching trick */}
                            <button className="hidden" onClick={handlePinSubmit} id="hidden-submit"></button>
                        </div>
                    )}

                    <button
                        onClick={() => { setSelectedAvatar(null); setPinInput(''); setAuthError(false); }}
                        className="text-sm font-semibold text-gray-500 hover:text-gray-800 p-2"
                        id="cancel-login"
                    >
                        Cancel
                    </button>
                </div>
            </div>
        )}

      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}

// Hook up the custom event for PIN submission outside the main render loop
if (typeof document !== 'undefined') {
    document.addEventListener('submit-pin', (e: any) => {
        const btn = document.getElementById('hidden-submit');
        if (btn) btn.click();
    });
}
