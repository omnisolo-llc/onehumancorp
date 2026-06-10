"use client";

import React, { useState, useEffect } from 'react';
import { useTranslation, useCurrency } from '../../../lib/localizationStore';
import { LocalizationToggle } from '../../../components/LocalizationToggle';
import StripeTerminalClient from './StripeTerminalClient';

// Offline storage helper for staff data
const OfflineStore = {
  getStaff: () => JSON.parse(localStorage.getItem('ohc_offline_staff') || '[]'),
  setStaff: (staff: any[]) => localStorage.setItem('ohc_offline_staff', JSON.stringify(staff)),

  getEvents: () => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'),
  addEvent: (event: any) => {
    const events = OfflineStore.getEvents();
    events.push(event);
    localStorage.setItem('ohc_offline_events', JSON.stringify(events));
  },
  clearEvents: () => localStorage.setItem('ohc_offline_events', '[]'),

  getPosTransactions: () => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'),
  setPosTransactions: (transactions: any[]) => localStorage.setItem('ohc_offline_pos_tx', JSON.stringify(transactions)),
  addPosTransaction: (tx: any) => {
    const transactions = OfflineStore.getPosTransactions();
    transactions.push(tx);
    localStorage.setItem('ohc_offline_pos_tx', JSON.stringify(transactions));
  },
  clearPosTransactions: () => localStorage.setItem('ohc_offline_pos_tx', '[]')
};

export default function TerminalPage() {
  const { t } = useTranslation();
  const { currency, convert } = useCurrency();
  const [pin, setPin] = useState('');
  const [activeStaff, setActiveStaff] = useState<any | null>(null);
  const [clockedIn, setClockedIn] = useState(false);
  const [error, setError] = useState('');
  const [syncing, setSyncing] = useState(false);
  const [offlineConversion, setOfflineConversion] = useState(false);
  const [syncCount, setSyncCount] = useState(0);
  const [orderStatus, setOrderStatus] = useState('');
  const [reserving, setReserving] = useState(false);

  useEffect(() => {
    if (navigator.onLine) {
      fetch('/api/staff')
        .then(res => res.json())
        .then(data => {
          if (Array.isArray(data)) {
            OfflineStore.setStaff(data);
          } else if (data && data.staff) {
            OfflineStore.setStaff(data.staff);
          }
        })
        .catch(console.error);
    }
  }, []);
  const [isOffline, setIsOffline] = useState(false);

  // Network listener
  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    // Set initial state safely
    setIsOffline(!navigator.onLine);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  // Background sync
  useEffect(() => {
    const syncInterval = setInterval(async () => {
      if (navigator.onLine) {
        const events = OfflineStore.getEvents();
        const posTransactions = OfflineStore.getPosTransactions();

        if (events.length > 0 || posTransactions.length > 0) {
          setSyncCount(events.length + posTransactions.length);
          setSyncing(true);
          try {
            const syncTasks = [];
            if (events.length > 0) {
              syncTasks.push(
                fetch("/api/staff/timecard", {
                  method: "POST",
                  headers: { "Content-Type": "application/json" },
                  body: JSON.stringify(events)
                }).then(res => { if (res.ok) OfflineStore.clearEvents(); })
              );
            }

            if (posTransactions.length > 0) {
              const sessionId = localStorage.getItem("ohc_active_terminal_session_id");
              syncTasks.push(
                fetch("/api/v1/payments/terminal/sync_offline", {
                  method: "POST",
                  headers: { "Content-Type": "application/json" },
                  body: JSON.stringify({ session_id: sessionId, transactions: posTransactions })
                }).then(async (res) => {
                  if (res.ok) {
                    const data = await res.json();
                    if (data.failed_transaction_ids && data.failed_transaction_ids.length > 0) {
                      const failedTxs = posTransactions.filter((tx: any) => data.failed_transaction_ids.includes(tx.client_id || tx.id));
                      OfflineStore.setPosTransactions(failedTxs);
                    } else {
                      OfflineStore.clearPosTransactions();
                    }
                  }
                })
              );
            }
            await Promise.all(syncTasks);

          } catch (e) {
            console.error("Sync failed", e);
          } finally {
            setSyncing(false);
          }
        }
      }
    }, 10000); // Try syncing every 10 seconds

    return () => clearInterval(syncInterval);
  }, []);

  const handlePinEntry = (digit: string) => {
    if (pin.length < 4) {
      const newPin = pin + digit;
      setPin(newPin);
      setError('');

      if (newPin.length === 4) {
        // Attempt to authenticate offline
        const staff = OfflineStore.getStaff();
        const found = staff.find((s: any) => s.pin_hash === newPin); // Simple mock check

        if (found) {
          setActiveStaff(found);
          // Check last event locally to determine clockedIn status
          const events = OfflineStore.getEvents().filter((e: any) => e.staff_id === found.id);
          if (events.length > 0) {
             const lastEvent = events[events.length - 1];
             setClockedIn(lastEvent.event_type === 'CLOCK_IN');
          }
        } else {
          setError(t('Invalid PIN'));
          setPin('');
        }
      }
    }
  };

  const handleClear = () => setPin('');
  const handleLock = () => {
      setActiveStaff(null);
      setPin('');
  };

  const handleClockAction = (type: 'CLOCK_IN' | 'CLOCK_OUT') => {
    if (!activeStaff) return;

    const event = {
      staff_id: activeStaff.id,
      tenant_id: 'default_tenant',
      event_type: type,
      timestamp: new Date().toISOString(),
      sync_status: 'PENDING'
    };

    OfflineStore.addEvent(event);
    setClockedIn(type === 'CLOCK_IN');
  };

  const handleNewOrder = async () => {
    const basePrice = 5000; // $50.00
    const converted = convert(basePrice, 'USD', currency);
    if (converted.isOffline) {
      setOfflineConversion(true);
      setTimeout(() => setOfflineConversion(false), 3000);
    }

    if (isOffline) {
      setOrderStatus(`${t('New Order Total')}: ${converted.amount / 100} ${currency}`);
      // Bypass Stripe Terminal and save offline
      const tx = {
        id: `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        amount_cents: converted.amount,
        currency: currency,
        payload: JSON.stringify([{ product_id: 'prod_123', quantity: 1 }]),
        client_id: 'terminal_1',
        timestamp: new Date().toISOString()
      };
      OfflineStore.addPosTransaction(tx);
      setOrderStatus(`${t('Payment Saved Offline')} - ${converted.amount / 100} ${currency}`);
    } else {
      setReserving(true);
      setOrderStatus(t('Processing/Reserving...'));

      try {
        const reserveRes = await fetch('/api/v1/payments/terminal/reserve', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tenant_id: activeStaff?.tenant_id || "default_tenant", product_id: 'prod_123', quantity: 1, ttl_seconds: 15 })
        });

        const reserveData = await reserveRes.json();

        if (!reserveData.success) {
          setOrderStatus(t('Failed to reserve: ') + reserveData.error_message);
          setReserving(false);
          return;
        }

        setOrderStatus(`${t('New Order Total')}: ${converted.amount / 100} ${currency}`);

        await fetch('/api/v1/payments/terminal/commit', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tenant_id: activeStaff?.tenant_id || "default_tenant", product_id: 'prod_123', quantity: 1, lock_id: reserveData.lock_id })
        });
        setOrderStatus(`${t('Payment Completed')}`);
      } catch (err) {
        setOrderStatus(t('Error connecting to server'));
      } finally {
        setReserving(false);
      }
    }
  };

  if (!activeStaff) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 font-inter w-full overflow-hidden">
        <div className="w-full max-w-[375px] min-h-[100dvh] md:h-[812px] md:min-h-0 bg-black text-white p-8 flex flex-col items-center relative overflow-x-hidden md:shadow-2xl">
           <div className="absolute top-8 right-8 flex items-center gap-4">
              {isOffline && <span className="text-red-500 font-bold text-xs bg-red-100/10 px-2 py-1 rounded">{t('Offline Mode')}</span>}
              <LocalizationToggle />
           </div>

           <div className="mt-20 mb-12 text-center">
             <h1 className="text-2xl font-bold font-outfit mb-2">{t('Terminal Locked')}</h1>
             <p className="text-gray-400">{t('Enter your PIN to unlock')}</p>
           </div>

           <div className="flex gap-4 mb-12">
              {[...Array(4)].map((_, i) => (
                <div key={i} className={`w-4 h-4 rounded-full border-2 ${pin.length > i ? 'bg-white border-white' : 'border-gray-600'}`}></div>
              ))}
           </div>

           {error && <p className="text-red-500 mb-4 animate-bounce">{error}</p>}

           <div className="grid grid-cols-3 gap-4 sm:gap-6 w-full max-w-[280px]">
             {[1, 2, 3, 4, 5, 6, 7, 8, 9].map(num => (
               <button
                 key={num}
                 onClick={() => handlePinEntry(num.toString())}
                 className="w-16 h-16 sm:w-20 sm:h-20 rounded-full bg-gray-800 text-3xl font-light hover:bg-gray-700 active:bg-gray-600 transition-colors flex items-center justify-center mx-auto min-h-[44px] min-w-[44px]"
               >
                 {num}
               </button>
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
     <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter md:py-10 w-full overflow-hidden">
      <div className="w-full max-w-[375px] min-h-[100dvh] md:h-[812px] md:min-h-0 bg-white md:shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">{activeStaff.name}</h1>
            <p className="text-blue-600 font-medium text-sm mt-1">{t(activeStaff.role)}</p>
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
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-gray-50">

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
                 className="w-full py-4 rounded-xl bg-blue-600 text-white font-bold shadow-md shadow-blue-500/20 hover:bg-blue-700 transition-colors"
               >
                 {t('Clock In')}
               </button>
             )}
           </div>

           {/* Role-based UI rendering */}
           <h3 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-2 mt-8">{t('Quick Actions')}</h3>

           <div className="grid grid-cols-2 gap-4">
             <button
                onClick={handleNewOrder}
                disabled={reserving}
                className={`bg-white p-4 rounded-2xl shadow-sm border border-gray-100 text-left ${reserving ? 'opacity-50' : 'active:scale-[0.98]'}`}
             >
               <div className="text-blue-500 mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
               </div>
               <span className="font-medium text-gray-900">{t('New Order')}</span>
             </button>

             {activeStaff.role === 'Manager' && (
               <button className="app-card p-4 rounded-2xl shadow-sm border border-gray-100 text-left active:scale-[0.98]">
                 <div className="text-purple-500 mb-2">
                   <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                 </div>
                 <span className="font-medium text-gray-900">{t('Reports')}</span>
               </button>
             )}

             <button className="app-card p-4 rounded-2xl shadow-sm border border-gray-100 text-left active:scale-[0.98]">
               <div className="text-orange-500 mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
               </div>
               <span className="font-medium text-gray-900">{t('Refunds')}</span>
             </button>
           </div>

           <StripeTerminalClient amount={activeStaff?.id ? 5000 : 0} productId="prod_123" tenantId={activeStaff?.tenant_id || "default_tenant"} />
           {orderStatus && <p className="mt-4 rounded-xl bg-blue-50 px-4 py-3 text-sm font-semibold text-blue-800" role="status">{orderStatus}</p>}
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
