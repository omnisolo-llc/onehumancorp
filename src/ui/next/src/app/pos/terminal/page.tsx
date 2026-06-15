"use client";

import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'next/navigation';
import StripeTerminalClient from './StripeTerminalClient';
import { LocalizationToggle } from '../../../components/LocalizationToggle';
import { SyncManager } from '../../../lib/sync/SyncManager';

const t = (text: string) => text;

export default function POSTerminal() {
  const [pin, setPin] = useState('');
  const [locked, setLocked] = useState(true);
  const [clockedIn, setClockedIn] = useState(false);
  const [activeStaff, setActiveStaff] = useState<any>(null);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [showStartShift, setShowStartShift] = useState(false);
  const [openingBalance, setOpeningBalance] = useState('0');
  const [showDrawerOps, setShowDrawerOps] = useState(false);
  const [drawerOpAmount, setDrawerOpAmount] = useState('');
  const [drawerOpReason, setDrawerOpReason] = useState('');
  const [showEndShift, setShowEndShift] = useState(false);
  const [closingBalance, setClosingBalance] = useState('');
  const [sessionSummary, setSessionSummary] = useState<any>(null);
  const [inventory, setInventory] = useState<any[]>([]);
  const [selectedProduct, setSelectedProduct] = useState<any>(null);
  const [reserving, setReserving] = useState(false);
  const [orderStatus, setOrderStatus] = useState('');
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);
  const [offlineConversion, setOfflineConversion] = useState(false);

  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    if (typeof window !== 'undefined') {
        setIsOffline(!navigator.onLine);
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);

        return () => {
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
        };
    }
  }, []);

  const handlePinEntry = async (digit: string) => {
    if (pin.length < 4) {
      const newPin = pin + digit;
      setPin(newPin);
      if (newPin.length === 4) {
        if (isOffline) {
           const staff = { id: 'staff_1', name: 'Offline Manager', role: 'Manager', tenant_id: 'offline_tenant' };
           setActiveStaff(staff);
           setLocked(false);
           setPin('');
           const savedSession = localStorage.getItem('ohc_active_terminal_session_id');
           if (!savedSession) {
             setShowStartShift(true);
           } else {
             setSessionId(savedSession);
             setClockedIn(true);
           }
           return;
        }

        try {
          const res = await fetch('/api/v1/pos/auth', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ pin: newPin })
          });
          const data = await res.json();
          if (data.success) {
            setActiveStaff(data.staff);
            setLocked(false);
            setPin('');
            const savedSession = localStorage.getItem('ohc_active_terminal_session_id');
            if (!savedSession) {
              setShowStartShift(true);
            } else {
              setSessionId(savedSession);
              setClockedIn(true);
            }
          } else {
            alert(t('Invalid PIN'));
            setPin('');
          }
        } catch (e) {
           console.error("Auth failed, falling back to offline", e);
           const staff = { id: 'staff_1', name: 'Offline Manager (Fallback)', role: 'Manager', tenant_id: 'offline_tenant' };
           setActiveStaff(staff);
           setLocked(false);
           setPin('');
           setShowStartShift(true);
        }
      }
    }
  };

  const handleClear = () => setPin('');

  const handleLock = () => {
    setLocked(true);
    setActiveStaff(null);
  };

  const loadDashboard = async () => {
    if (isOffline) {
       // Just load something empty for now
       setInventory([]);
       return;
    }
    try {
      const res = await fetch('/api/pos/inventory');
      const data = await res.json();
      if (Array.isArray(data)) {
        setInventory(data);
      } else {
        console.error("Inventory data is not an array", data);
        setInventory([]);
      }
    } catch (e) {
      console.error("Failed to load inventory", e);
      setInventory([]);
    }
  };

  useEffect(() => {
    if (!locked && activeStaff) {
      loadDashboard();
    }
  }, [locked, activeStaff]);

  const handleStartShift = async () => {
    if (!activeStaff) return;

    const openingCents = parseInt(openingBalance) * 100;
    const deviceId = typeof window !== 'undefined' ? (localStorage.getItem('ohc_device_id') || 'dev_1') : 'dev_1';

    if (isOffline) {
      const newSessionId = `session_off_${Date.now()}`;
      setSessionId(newSessionId);
      setClockedIn(true);
      setShowStartShift(false);
      localStorage.setItem('ohc_active_terminal_session_id', newSessionId);

      const event = {
        type: 'START_TERMINAL_SESSION',
        payload: {
          session_id: newSessionId,
          device_id: deviceId,
          opening_balance_cents: openingCents,
          timestamp: new Date().toISOString()
        }
      };
      await SyncManager.getInstance().enqueue(event);
      return;
    }

    try {
      const res = await fetch('/api/v1/pos/sessions/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          device_id: deviceId,
          opening_balance_cents: openingCents,
          tenant_id: activeStaff.tenant_id
        })
      });
      if (res.ok) {
        const data = await res.json();
        if (data.success) {
          setSessionId(data.session_id);
          setClockedIn(true);
          setShowStartShift(false);
          localStorage.setItem('ohc_active_terminal_session_id', data.session_id);
          return;
        }
      }
      throw new Error(`Server returned ${res.status}`);
    } catch (e) {
      console.error("Failed to start session, falling back to local session", e);
      const newSessionId = `session_fallback_${Date.now()}`;
      setSessionId(newSessionId);
      setClockedIn(true);
      setShowStartShift(false);
      localStorage.setItem('ohc_active_terminal_session_id', newSessionId);
    }
  };

  const fetchSessionSummary = async () => {
    if (!sessionId || !activeStaff) return;
    try {
      const res = await fetch(`/api/v1/pos/sessions/summary?session_id=${sessionId}`);
      const data = await res.json();
      setSessionSummary(data);
    } catch (e) {
      console.error("Failed to fetch summary", e);
    }
  };

  const handleEndShift = async () => {
    if (!activeStaff || !sessionId) return;
    const closingCents = parseInt(closingBalance) * 100;

    if (isOffline) {
      const event = {
        type: 'END_TERMINAL_SESSION',
        payload: {
          session_id: sessionId,
          closing_balance_cents: closingCents,
          timestamp: new Date().toISOString()
        }
      };
      await SyncManager.getInstance().enqueue(event);
      setClockedIn(false);
      setSessionId(null);
      localStorage.removeItem('ohc_active_terminal_session_id');
      setShowEndShift(false);
      return;
    }

    try {
      const res = await fetch('/api/v1/pos/sessions/end', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          session_id: sessionId,
          closing_balance_cents: closingCents,
          tenant_id: activeStaff.tenant_id
        })
      });
      const data = await res.json();
      if (data.success) {
        setClockedIn(false);
        setSessionId(null);
        localStorage.removeItem('ohc_active_terminal_session_id');
        setShowEndShift(false);
      }
    } catch (e) {
      console.error("Failed to end session", e);
    }
  };

  const handleClockAction = async (action: 'CLOCK_IN' | 'CLOCK_OUT') => {
    if (!activeStaff) return;

    if (action === 'CLOCK_IN') {
      setShowStartShift(true);
    } else {
      await fetchSessionSummary();
      setShowEndShift(true);
    }
  };

  const handleSelectProduct = (product: any) => {
    setSelectedProduct(product);
    setOrderStatus('');
  };

  const handleOptimisticReserve = (productId: string) => {
    setInventory(prev => prev.map(p => {
      if (p.id === productId) {
        return { ...p, stock: p.stock - 1 };
      }
      return p;
    }));
  };

  const handleOptimisticRollback = (productId: string) => {
    setInventory(prev => prev.map(p => {
      if (p.id === productId) {
        return { ...p, stock: p.stock + 1 };
      }
      return p;
    }));
  };

  const handleCashCharge = async (amountCents: number, productId: string) => {
    if (!activeStaff || !sessionId) return;
    setReserving(true);

    const transactionId = `tx_cash_${Date.now()}_${Math.floor(Math.random() * 1000)}`;
    const event = {
      type: 'CASH_SALE',
      id: transactionId,
      payload: {
        session_id: sessionId,
        amount_cents: amountCents,
        currency: 'usd',
        product_id: productId,
        entry_type: 'SALE',
        timestamp: new Date().toISOString()
      }
    };

    await SyncManager.getInstance().enqueue(event);

    setOrderStatus(t('Cash payment recorded.'));
    setReserving(false);
    setSelectedProduct(null);
    setTimeout(() => setOrderStatus(''), 3000);
  };

  const handleDrawerOp = async (type: 'DROP' | 'PAYOUT') => {
    if (!activeStaff || !sessionId) return;
    const amountCents = parseInt(drawerOpAmount) * 100;

    const event = {
      type: 'DRAWER_OP',
      payload: {
        session_id: sessionId,
        entry_type: type,
        amount_cents: amountCents,
        currency: 'usd',
        reason: drawerOpReason,
        timestamp: new Date().toISOString()
      }
    };

    await SyncManager.getInstance().enqueue(event);
    setShowDrawerOps(false);
    setDrawerOpAmount('');
    setDrawerOpReason('');
    setOrderStatus(t(`${type} recorded.`));
    setTimeout(() => setOrderStatus(''), 3000);
  };

  const handleQuickCharge = async () => {
    if (!activeStaff) return;
    handleCashCharge(5000, 'quick_charge');
  };

  if (locked) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter px-4 w-full">
        <div className="w-[375px] bg-white rounded-3xl shadow-xl overflow-hidden p-8 border border-gray-100 relative">
           <div className="text-center mb-8">
             <div className="w-16 h-16 bg-gray-900 rounded-2xl mx-auto mb-4 flex items-center justify-center shadow-lg">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
             </div>
             <h1 className="text-2xl font-bold text-gray-900 font-outfit">{t('Terminal Locked')}</h1>
             <p className="text-gray-500 text-sm mt-2">{t('Enter PIN to access terminal')}</p>
             {isOffline && <p className="text-orange-500 font-bold text-xs mt-2 bg-orange-50 inline-block px-2 py-1 rounded">{t('Offline Mode Active')}</p>}
           </div>

           <div className="flex justify-center mb-8">
             <div className="flex space-x-4">
               {[...Array(4)].map((_, i) => (
                 <div key={i} className={`w-4 h-4 rounded-full transition-all ${i < pin.length ? 'bg-blue-600 scale-110 shadow-sm' : 'bg-gray-200'}`} />
               ))}
             </div>
           </div>

           <div className="grid grid-cols-3 gap-y-6 gap-x-6 max-w-[280px] mx-auto">
             {[1, 2, 3, 4, 5, 6, 7, 8, 9].map((num) => (
               <div key={num} className="flex justify-center">
                 <button
                   onClick={() => handlePinEntry(num.toString())}
                   className="w-16 h-16 sm:w-20 sm:h-20 rounded-full bg-gray-50 text-3xl font-light text-gray-800 hover:bg-gray-100 hover:shadow-inner active:bg-gray-200 transition-all flex items-center justify-center min-h-[44px] min-w-[44px]"
                 >
                   {num}
                 </button>
               </div>
             ))}
             <div className="col-start-2">
               <button
                 onClick={() => handlePinEntry('0')}
                 className="w-16 h-16 sm:w-20 sm:h-20 rounded-full bg-gray-800 text-3xl font-light hover:bg-gray-700 active:bg-gray-600 transition-colors flex items-center justify-center mx-auto min-h-[44px] min-w-[44px]"
               >
                 0
               </button>
             </div>
             <div className="col-start-3 flex items-center justify-center">
               <button
                 onClick={handleClear}
                 disabled={!pin}
                 className="text-gray-400 hover:text-white disabled:opacity-40 disabled:hover:text-gray-400 min-h-[44px] min-w-[44px]"
               >
                 {t('Clear')}
               </button>
             </div>
           </div>

           {syncing && <div className="absolute bottom-4 left-4 text-xs text-blue-400">{t('Syncing...')}</div>}
        </div>
      </div>
    );
  }

  return (
     <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter md:py-10 w-full overflow-hidden">
      <div className="w-full max-w-[375px] min-h-[100dvh] md:h-[812px] md:min-h-0 bg-white md:shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">{activeStaff?.name}</h1>
            <p className="text-blue-600 font-medium text-sm mt-1">{t(activeStaff?.role)}</p>
            {isOffline ? (
              <span className="inline-block mt-1 text-yellow-800 font-bold text-xs bg-yellow-100 px-2 py-1 rounded border border-yellow-200 shadow-sm">{t('Offline Mode')}</span>
            ) : (
              <span className="inline-block mt-1 text-green-800 font-bold text-xs bg-green-100 px-2 py-1 rounded border border-green-200 shadow-sm">{t('Online')}</span>
            )}
          </div>
          <div className="flex items-center gap-3">
            <LocalizationToggle />
            <button onClick={handleLock} className="text-sm font-semibold text-gray-500 hover:text-gray-900">
              {t('Lock')}
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-[#F5F5F7]">

           {showStartShift && (
              <div className="fixed inset-0 z-[100] flex items-center justify-center bg-[#1C1C1E]/80 backdrop-blur-xl p-4">
                <div className="bg-white rounded-[32px] w-full max-w-sm p-8 shadow-2xl animate-in zoom-in-95 duration-300 border border-gray-100">
                  <div className="w-16 h-16 bg-blue-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg mx-auto">
                    <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </div>
                  <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2 text-center">{t('Start Shift')}</h2>
                  <p className="text-gray-500 mb-8 text-center text-sm">{t('Prepare your drawer with the opening cash balance.')}</p>

                  <div className="relative mb-8">
                    <span className="absolute left-5 top-1/2 -translate-y-1/2 text-2xl font-bold text-blue-600">$</span>
                    <input
                      type="number"
                      id="opening-balance-input"
                      value={openingBalance}
                      onChange={(e) => setOpeningBalance(e.target.value)}
                      className="w-full pl-12 pr-6 py-5 bg-gray-50 border-none rounded-2xl text-3xl font-bold text-gray-900 focus:ring-2 focus:ring-blue-500 outline-none transition-all"
                      placeholder="0"
                      autoFocus
                    />
                  </div>

                  <div className="space-y-3">
                    <button
                      onClick={handleStartShift}
                      className="w-full py-5 rounded-[20px] bg-blue-600 text-white font-bold text-xl shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all"
                    >
                      {t('Open Terminal')}
                    </button>
                    <button
                      onClick={() => setLocked(true)}
                      className="w-full py-2 text-gray-400 font-bold hover:text-gray-600 transition-colors"
                    >
                      {t('Cancel')}
                    </button>
                  </div>
                </div>
              </div>
           )}

           <div className="app-card rounded-2xl p-6 shadow-sm border border-gray-100 mb-6 text-center">
             <div className={`w-16 h-16 mx-auto rounded-full flex items-center justify-center mb-4 ${clockedIn ? 'bg-green-100 text-green-600' : 'bg-gray-100 text-gray-400'}`}>
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
             </div>
             <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">
               {clockedIn ? t('Clocked In') : t('Not Clocked In')}
             </h2>
             <p className="text-sm text-gray-500 mb-6">
                {clockedIn ? t('Your time is being tracked locally.') : t('Clock in to start your shift.')}
             </p>

             {clockedIn ? (
               <button
                 onClick={() => handleClockAction('CLOCK_OUT')}
                 className="w-full py-4 rounded-xl bg-red-50 text-red-600 font-bold hover:bg-red-100 transition-colors"
               >
                 {t('Clock Out')}
               </button>
             ) : (
               <button
                 onClick={() => handleClockAction('CLOCK_IN')}
                 className="charge-btn w-full py-4 rounded-[8px] bg-blue-600 text-white font-bold shadow-md shadow-blue-500/20 hover:bg-blue-700 transition-colors"
               >
                 {t('Clock In')}
               </button>
             )}
           </div>

           {showEndShift && (
              <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm p-4 overflow-y-auto">
                <div className="bg-white rounded-3xl w-full max-w-sm p-8 shadow-2xl my-auto">
                  <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">{t('End Shift')}</h2>
                  <p className="text-gray-500 mb-6 text-sm">{t('Review session activity and count your drawer.')}</p>

                  {sessionSummary && (
                    <div className="bg-gray-50 rounded-2xl p-4 mb-6 space-y-2">
                      <div className="flex justify-between text-sm">
                        <span className="text-gray-500">{t('Total Sales')}</span>
                        <span className="font-bold text-gray-900">${(sessionSummary.total_sales_cents/100).toFixed(2)}</span>
                      </div>
                      <div className="flex justify-between text-sm">
                        <span className="text-gray-500">{t('Cash In (incl. Opening)')}</span>
                        <span className="font-bold text-green-600">+${(sessionSummary.total_cash_in_cents/100).toFixed(2)}</span>
                      </div>
                      <div className="flex justify-between text-sm">
                        <span className="text-gray-500">{t('Cash Out (Drops/Payouts)')}</span>
                        <span className="font-bold text-red-600">-${(sessionSummary.total_cash_out_cents/100).toFixed(2)}</span>
                      </div>
                      <div className="pt-2 border-t border-gray-200 flex justify-between font-bold">
                        <span>{t('Expected Cash')}</span>
                        <span className="text-blue-600">${(sessionSummary.expected_cash_cents/100).toFixed(2)}</span>
                      </div>
                    </div>
                  )}

                  <div className="mb-8">
                    <label className="block text-xs font-bold text-gray-400 uppercase mb-2 tracking-wider">{t('Actual Cash in Drawer')}</label>
                    <div className="relative">
                      <span className="absolute left-4 top-1/2 -translate-y-1/2 text-2xl font-bold text-gray-400">$</span>
                      <input
                        type="number"
                        value={closingBalance}
                        onChange={(e) => setClosingBalance(e.target.value)}
                        className="w-full pl-10 pr-4 py-4 bg-gray-50 border-none rounded-2xl text-2xl font-bold focus:ring-2 focus:ring-red-500 outline-none"
                        placeholder="0.00"
                        autoFocus
                      />
                    </div>
                    {sessionSummary && closingBalance && (
                      <p className={`text-xs font-bold mt-2 ${Math.abs(parseInt(closingBalance)*100 - sessionSummary.expected_cash_cents) < 1 ? 'text-green-600' : 'text-red-600'}`}>
                        {Math.abs(parseInt(closingBalance)*100 - sessionSummary.expected_cash_cents) < 1
                          ? t('Drawer is balanced.')
                          : `${t('Discrepancy:')} $${((parseInt(closingBalance)*100 - sessionSummary.expected_cash_cents)/100).toFixed(2)}`}
                      </p>
                    )}
                  </div>

                  <button
                    onClick={handleEndShift}
                    className="w-full py-4 rounded-2xl bg-gray-900 text-white font-bold text-lg shadow-lg hover:bg-black active:scale-[0.98] transition-all"
                  >
                    {t('Close Terminal')}
                  </button>
                  <button
                    onClick={() => setShowEndShift(false)}
                    className="w-full mt-2 py-2 text-gray-500 font-medium"
                  >
                    {t('Go Back')}
                  </button>
                </div>
              </div>
           )}

           {showDrawerOps && (
              <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm p-4">
                <div className="bg-white rounded-3xl w-full max-w-sm p-8 shadow-2xl">
                  <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-4">{t('Drawer Operation')}</h2>

                  <div className="space-y-4 mb-8">
                    <div>
                      <label className="block text-xs font-bold text-gray-400 uppercase mb-1">{t('Amount')}</label>
                      <input
                        type="number"
                        value={drawerOpAmount}
                        onChange={(e) => setDrawerOpAmount(e.target.value)}
                        className="w-full px-4 py-3 bg-gray-50 rounded-xl text-xl font-bold outline-none"
                        placeholder="0.00"
                      />
                    </div>
                    <div>
                      <label className="block text-xs font-bold text-gray-400 uppercase mb-1">{t('Reason')}</label>
                      <input
                        type="text"
                        value={drawerOpReason}
                        onChange={(e) => setDrawerOpReason(e.target.value)}
                        className="w-full px-4 py-3 bg-gray-50 rounded-xl outline-none"
                        placeholder={t('e.g. Mid-day drop')}
                      />
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <button
                      onClick={() => handleDrawerOp('DROP')}
                      className="py-4 rounded-2xl bg-orange-100 text-orange-700 font-bold"
                    >
                      {t('Cash Drop')}
                    </button>
                    <button
                      onClick={() => handleDrawerOp('PAYOUT')}
                      className="py-4 rounded-2xl bg-red-100 text-red-700 font-bold"
                    >
                      {t('Payout')}
                    </button>
                  </div>
                  <button
                    onClick={() => setShowDrawerOps(false)}
                    className="w-full mt-4 py-2 text-gray-500 font-medium"
                  >
                    {t('Close')}
                  </button>
                </div>
              </div>
           )}

           {/* Quick Actions */}
           <h3 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-2 mt-8">{t('Quick Actions')}</h3>
           <div className="grid grid-cols-2 gap-4 mb-8">
             <button
                onClick={handleQuickCharge}
                disabled={reserving}
                className={`charge-btn p-4 rounded-[16px] text-left bg-white border border-gray-200 shadow-sm ${reserving ? 'opacity-50' : 'active:scale-[0.98]'} min-h-[80px]`}
             >
               <div className="text-green-600 mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
               </div>
               <span className="font-bold text-gray-900">{t('Cash $50')}</span>
             </button>

             <button
                onClick={() => setShowDrawerOps(true)}
                className="p-4 rounded-[16px] text-left bg-white border border-gray-200 shadow-sm active:scale-[0.98] min-h-[80px]"
             >
               <div className="text-orange-600 mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 15v-1a4 4 0 00-4-4H8m0 0l3 3m-3-3l3-3m9 14V5a2 2 0 00-2-2H6a2 2 0 00-2 2v16l4-2 4 2 4-2 4 2z" /></svg>
               </div>
               <span className="font-bold text-gray-900">{t('Drawer Ops')}</span>
             </button>
           </div>

           {/* Catalog Selection */}
           <h3 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-2 mt-8">{t('Product Catalog')}</h3>
           <div className="grid grid-cols-1 gap-3 mb-8">
              {inventory.length === 0 ? (
                <p className="text-center text-gray-500 py-4 italic">{t('No products found in catalog')}</p>
              ) : inventory.map(product => (
                <button
                  key={product.id}
                  onClick={() => handleSelectProduct(product)}
                  className={`p-4 rounded-[16px] text-left transition-all active:scale-[0.98] min-h-[64px] min-w-[44px] ${selectedProduct?.id === product.id ? 'bg-white/80 border-[#0066FF] ring-1 ring-[#0066FF]' : 'bg-white/65 border-white/40'} backdrop-blur-[30px] border shadow-sm`}
                >
                  <div className="flex justify-between items-center">
                    <div>
                      <div className="font-bold text-gray-900">{product.name}</div>
                      <div className="text-xs text-gray-500 line-clamp-1">{product.description} &bull; Stock: {product.stock}</div>
                    </div>
                    <div className="text-blue-600 font-bold">
                      ${(product.price_cents / 100).toFixed(2)}
                    </div>
                  </div>
                </button>
              ))}
           </div>

           {selectedProduct && (
             <>
               <div className="bg-white/60 backdrop-blur-xl border border-white/40 rounded-3xl p-6 mb-4 shadow-xl">
                  <div className="flex justify-between items-start mb-6">
                    <div>
                      <h4 className="text-xl font-bold text-gray-900">{selectedProduct.name}</h4>
                      <p className="text-gray-500 text-sm">${(selectedProduct.price_cents/100).toFixed(2)}</p>
                    </div>
                    <button onClick={() => setSelectedProduct(null)} className="text-gray-400 p-2">
                      <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                    </button>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <button
                      onClick={() => handleCashCharge(selectedProduct.price_cents, selectedProduct.id)}
                      className="flex flex-col items-center justify-center p-6 bg-green-50 rounded-2xl border border-green-100 active:scale-[0.98] transition-all"
                    >
                      <div className="w-12 h-12 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-2">
                        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                      </div>
                      <span className="font-bold text-green-800">{t('Cash')}</span>
                    </button>

                    <div className="flex flex-col">
                       <StripeTerminalClient
                          amount={selectedProduct.price_cents}
                          productId={selectedProduct.id}
                          tenantId={activeStaff?.tenant_id || "default_tenant"}
                          onOptimisticReserve={() => handleOptimisticReserve(selectedProduct.id)}
                          onOptimisticRollback={() => handleOptimisticRollback(selectedProduct.id)}
                       />
                    </div>
                  </div>
               </div>

               <div className="bg-green-50 border border-green-100 rounded-xl p-4 my-4 mb-4">
                 <div className="flex justify-between items-center">
                   <span className="text-green-800 text-sm font-bold">Available Rewards</span>
                   <span className="text-green-800 text-sm font-bold">1 Reward Available</span>
                 </div>
                 <p className="text-green-700 text-xs font-medium mt-1">
                   Payment automatically applies reward to this transaction.
                 </p>
               </div>
             </>
           )}

           {orderStatus && <p className="mt-4 rounded-xl bg-blue-50 px-4 py-3 text-sm font-semibold text-blue-800 animate-in fade-in slide-in-from-top-2" role="status">{orderStatus}</p>}
        </div>

        {syncing && (
          <div className="absolute bottom-6 left-1/2 -translate-x-1/2 bg-blue-600/90 backdrop-blur-[30px] border border-white/20 text-white px-6 py-3 rounded-full shadow-lg font-bold min-h-[44px] flex items-center justify-center space-x-2 z-50">
            <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>{t('Syncing transactions...')}</span>
          </div>
        )}
        {offlineConversion && (
          <div className="absolute bottom-16 left-1/2 -translate-x-1/2 bg-amber-100 text-amber-800 px-4 py-2 rounded-full text-xs font-bold border border-amber-200 shadow-lg animate-bounce">
            {t('Using cached rates - Syncing soon')}
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
