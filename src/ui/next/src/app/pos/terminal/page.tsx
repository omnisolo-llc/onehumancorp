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
           const staff = { id: 'staff_1', name: 'Offline Manager', role: 'Manager' };
           setActiveStaff(staff);
           setLocked(false);
           setPin('');
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
          } else {
            alert(t('Invalid PIN'));
            setPin('');
          }
        } catch (e) {
           console.error("Auth failed, falling back to offline", e);
           const staff = { id: 'staff_1', name: 'Offline Manager (Fallback)', role: 'Manager' };
           setActiveStaff(staff);
           setLocked(false);
           setPin('');
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
      setInventory(data);
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

  const handleClockAction = async (action: 'CLOCK_IN' | 'CLOCK_OUT') => {
    if (!activeStaff) return;

    const isClockingIn = action === 'CLOCK_IN';
    setClockedIn(isClockingIn);

    const event = {
      type: action,
      payload: { staff_id: activeStaff.id, timestamp: new Date().toISOString() },
    };

    await SyncManager.getInstance().enqueue(event);
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

  const handleQuickCharge = async () => {
     if (!activeStaff) return;
     setReserving(true);

     if (isOffline) {
         setOrderStatus(t('Processing offline quick charge...'));
         const transactionId = `tx_offline_${Date.now()}_${Math.floor(Math.random() * 1000)}`;
         const tx = {
            id: transactionId,
            type: 'tap_to_pay',
            amount: 5000,
            currency: 'usd',
            product_id: 'quick_charge',
            quantity: 1,
            timestamp: new Date().toISOString()
         };

         await SyncManager.getInstance().enqueue(tx);

         setTimeout(() => {
            setOrderStatus(t('Offline Quick Charge Saved.'));
            setReserving(false);
            setTimeout(() => setOrderStatus(''), 3000);
         }, 1000);
         return;
     }

     try {
         // simulate quick charge
         await new Promise(r => setTimeout(r, 1000));
         setOrderStatus(t('Quick charge successful'));
     } catch (e) {
         setOrderStatus(t('Quick charge failed'));
     } finally {
         setReserving(false);
         setTimeout(() => setOrderStatus(''), 3000);
     }
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
                 className="charge-btn w-full py-4 rounded-[16px] bg-blue-600 text-white font-bold shadow-md shadow-blue-500/20 hover:bg-blue-700 transition-colors"
               >
                 {t('Clock In')}
               </button>
             )}
           </div>

           {/* Quick Actions */}
           <h3 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-2 mt-8">{t('Quick Actions')}</h3>
           <div className="grid grid-cols-2 gap-4 mb-8">
             <button
                onClick={handleQuickCharge}
                disabled={reserving}
                className={`charge-btn p-4 rounded-[16px] text-left bg-white/65 backdrop-blur-[30px] border border-white/40 shadow-sm ${reserving ? 'opacity-50' : 'active:scale-[0.98]'}`}
             >
               <div className="text-blue-500 mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
               </div>
               <span className="font-medium text-gray-900">{t('Quick Charge $50')}</span>
             </button>

             <button className="p-4 rounded-[16px] text-left bg-white/65 backdrop-blur-[30px] border border-white/40 shadow-sm active:scale-[0.98]">
               <div className="text-orange-500 mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 15v-1a4 4 0 00-4-4H8m0 0l3 3m-3-3l3-3m9 14V5a2 2 0 00-2-2H6a2 2 0 00-2 2v16l4-2 4 2 4-2 4 2z" /></svg>
               </div>
               <span className="font-medium text-gray-900">{t('Refunds')}</span>
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
               <div className="bg-green-50 border border-green-100 rounded-xl p-4 my-4 mb-4">
                 <div className="flex justify-between items-center">
                   <span className="text-green-800 text-sm font-bold">Available Rewards</span>
                   <span className="text-green-800 text-sm font-bold">1 Reward Available</span>
                 </div>
                 <p className="text-green-700 text-xs font-medium mt-1">
                   Tap to Pay to automatically apply reward to this transaction.
                 </p>
               </div>
                              <StripeTerminalClient
                  amount={selectedProduct.price_cents}
                  productId={selectedProduct.id}
                  tenantId={activeStaff?.tenant_id || "default_tenant"}
                  onOptimisticReserve={() => handleOptimisticReserve(selectedProduct.id)}
                  onOptimisticRollback={() => handleOptimisticRollback(selectedProduct.id)}
               />
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
