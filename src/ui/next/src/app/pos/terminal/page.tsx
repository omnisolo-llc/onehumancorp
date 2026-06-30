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
  const [isSyncingInitial, setIsSyncingInitial] = useState(true);
  const [selectedProduct, setSelectedProduct] = useState<any>(null);
  const [cart, setCart] = useState<{product: any, quantity: number}[]>([]);
  const [isCartOpen, setIsCartOpen] = useState(false);
  const [checkoutComplete, setCheckoutComplete] = useState(false);
  const [customerEmail, setCustomerEmail] = useState('');
  const [receiptSent, setReceiptSent] = useState(false);
  const [reserving, setReserving] = useState(false);
  const [orderStatus, setOrderStatus] = useState('');
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);
  const [offlineConversion, setOfflineConversion] = useState(false);

  const [sessionId, setSessionId] = useState<string | null>(null);
  const [deviceId, setDeviceId] = useState<string>('');


  useEffect(() => {
    const checkQueue = async () => {
      const qLen = await SyncManager.getInstance().getQueueLength();
      if (navigator.onLine && qLen > 0) {
        setSyncing(true);
      } else {
        setSyncing(false);
      }
    };

    const handleOnline = () => {
      setIsOffline(false);
      checkQueue();
    };
    const handleOffline = () => {
      setIsOffline(true);
      checkQueue();
    };

    const handleQueueUpdated = () => {
      checkQueue();
    };


    if (typeof window !== 'undefined') {
        let storedDeviceId = localStorage.getItem('ohc_pos_device_id');
        if (!storedDeviceId) {
            storedDeviceId = 'device_' + Date.now() + '_' + Math.floor(Math.random() * 1000);
            localStorage.setItem('ohc_pos_device_id', storedDeviceId);
        }
        setDeviceId(storedDeviceId);

        setIsOffline(!navigator.onLine);
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);
        window.addEventListener('ohc_queue_updated', handleQueueUpdated);

        checkQueue();

        return () => {
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
          window.removeEventListener('ohc_queue_updated', handleQueueUpdated);
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

            // Initialize terminal session
            try {
              const sessionRes = await fetch('/api/v1/payments/terminal/session/start', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json', 'x-tenant-id': data.staff.tenant_id },
                body: JSON.stringify({ device_id: deviceId })
              });
              const sessionData = await sessionRes.json();
              if (sessionData.success) {
                setSessionId(sessionData.session_id);
              } else {
                console.error("Failed to start terminal session", sessionData.error_message);
              }
            } catch(e) {
               console.error("Failed to fetch session", e);
            }

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
      const res = await fetch('/api/pos/inventory', { headers: { 'x-tenant-id': activeStaff?.tenant_id || 'default' } });
      const data = await res.json();
      setInventory(data.inventory || []);
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

  const handleAddToCart = (product: any) => {
    setCart(prev => {
      const existing = prev.find(item => item.product.id === product.id);
      if (existing) {
        return prev.map(item => item.product.id === product.id ? { ...item, quantity: item.quantity + 1 } : item);
      }
      return [...prev, { product, quantity: 1 }];
    });
  };

  const cartTotal = cart.reduce((sum: number, item: any) => sum + (item.product.price_cents * item.quantity), 0);
  const cartItemCount = cart.reduce((sum: number, item: any) => sum + item.quantity, 0);

  const handleCheckoutComplete = () => {
    setCheckoutComplete(true);
    setIsCartOpen(false);
  };

  const handleSendReceipt = () => {
    setReceiptSent(true);
    setTimeout(() => {
      setCheckoutComplete(false);
      setCart([]);
      setReceiptSent(false);
      setCustomerEmail('');
    }, 2000);
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

     const quickChargeProduct = {
         id: 'quick_charge',
         name: 'Quick Charge',
         description: 'Manual entry',
         price_cents: 5000,
         currency: 'usd',
         stock: 9999
     };

     setCart([{ product: quickChargeProduct, quantity: 1 }]);
     setReserving(false);
     setIsCartOpen(true);
  };

  if (locked) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] md:p-10 font-inter px-4 w-full overflow-hidden">
        <div className="w-full max-w-[375px] mx-auto bg-white rounded-3xl shadow-xl overflow-hidden p-8 border border-gray-100 relative">
           <div className="text-center mb-8">
             <div className="w-16 h-16 bg-gray-900 rounded-2xl mx-auto mb-4 flex items-center justify-center shadow-lg">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
             </div>
             <h1 className="text-2xl font-bold text-gray-900 font-outfit">{t('Terminal Locked')}</h1>
             <p className="text-gray-500 text-sm mt-2">{t('Enter PIN to access terminal')}</p>
             {isOffline && <p className="text-[#FF9500] font-bold text-xs mt-2 bg-orange-50 inline-block px-2 py-1 rounded">{t('Offline Mode Active')}</p>}
           </div>

           <div className="flex justify-center mb-8">
             <div className="flex space-x-4">
               {[...Array(4)].map((_, i) => (
                 <div key={i} className={`w-4 h-4 rounded-full transition-all ${i < pin.length ? 'bg-[#0071E3] scale-110 shadow-sm' : 'bg-gray-200'}`} />
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
     <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter md:py-10 w-full overflow-x-hidden">
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] md:h-[812px] md:min-h-0 bg-white md:shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200 mobile-pos-container">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">{activeStaff?.name}</h1>
            <p className="text-[#0071E3] font-medium text-sm mt-1">{t(activeStaff?.role)}</p>
            {isOffline ? (
              <div className="inline-flex items-center gap-1.5 mt-1 text-yellow-800 font-bold text-xs bg-yellow-100 px-2 py-1 rounded border border-yellow-200 shadow-sm">
                <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                {t('Offline - Syncing later')}
              </div>
            ) : (
              <span className="inline-block mt-1 text-green-800 font-bold text-xs bg-green-100 px-2 py-1 rounded border border-green-200 shadow-sm">{t('Online')}</span>
            )}
          </div>
          <div className="flex items-center gap-3">
            <LocalizationToggle />
            <button onClick={handleLock} className="text-sm font-semibold text-gray-500 hover:text-gray-900 min-h-[44px] min-w-[44px]">
              {t('Lock')}
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-[#F5F5F7]">

           <div className="app-card rounded-2xl p-6 shadow-lg mb-6 text-center bg-white/70 backdrop-blur-[32px] saturate-[200%] border border-white/50">
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
                 className="w-full py-4 rounded-xl bg-red-50 text-red-600 font-bold hover:bg-red-100 transition-colors min-h-[44px] min-w-[44px]"
               >
                 {t('Clock Out')}
               </button>
             ) : (
               <button
                 onClick={() => handleClockAction('CLOCK_IN')}
                 className="charge-btn w-full py-4 bg-[#0071E3] text-white font-bold shadow-md shadow-blue-500/20 hover:bg-blue-700 transition-colors min-h-[44px] min-w-[44px]"
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
                className={`charge-btn min-h-[44px] min-w-[44px] p-4 rounded-[8px] text-left shadow-lg bg-white/70 backdrop-blur-[32px] saturate-[200%] border border-white/50 ${reserving ? 'opacity-50' : 'active:scale-[0.98]'}`}
             >
               <div className="text-[#0066FF] mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
               </div>
               <span className="font-medium text-gray-900">{t('Quick Charge $50')}</span>
             </button>

             <button className="min-h-[44px] min-w-[44px] p-4 rounded-[8px] text-left shadow-lg active:scale-[0.98] bg-white/70 backdrop-blur-[32px] saturate-[200%] border border-white/50">
               <div className="text-[#FF9500] mb-2">
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
                  onClick={() => handleAddToCart(product)} disabled={reserving || isCartOpen}
                  className={`p-4 rounded-[8px] text-left transition-all active:scale-[0.98] min-h-[64px] min-w-[44px] shadow-lg backdrop-blur-[30px] saturate-[210%] ${selectedProduct?.id === product.id ? 'bg-white/80 ring-1 ring-[#0066FF] border border-[#0066FF]' : 'bg-white/65 border border-white/50'}`}
                >
                  <div className="flex justify-between items-center">
                    <div>
                      <div className="font-bold text-gray-900">{product.name}</div>
                      <div className="text-xs text-gray-500 line-clamp-1">{product.description} &bull; Stock: {product.stock}</div>
                    </div>
                    <div className="text-[#0071E3] font-bold">
                      ${(product.price_cents / 100).toFixed(2)}
                    </div>
                  </div>
                </button>
              ))}
           </div>

           {/* Bottom Bar */}
           {cartItemCount > 0 && !checkoutComplete && (
             <div className="fixed bottom-0 left-0 right-0 p-4 bg-white/80 backdrop-blur-[30px] border-t border-gray-200 z-40 pb-safe pb-8">
               <button
                 onClick={() => setIsCartOpen(true)}
                 className="w-full bg-[#0066FF] text-white rounded-xl min-h-[60px] text-lg font-bold flex justify-between items-center px-6 shadow-lg active:scale-[0.98]"
               >
                 <span className="bg-white/20 px-3 py-1 rounded-full text-sm">{cartItemCount} item{cartItemCount > 1 ? 's' : ''}</span>
                 <span>Charge ${(cartTotal / 100).toFixed(2)}</span>
               </button>
             </div>
           )}

           {/* Cart Drawer */}
           {isCartOpen && !checkoutComplete && (
             <div className="fixed inset-0 z-50 flex flex-col justify-end">
               <div className="absolute inset-0 bg-black/40 backdrop-blur-[30px] saturate-[210%]" onClick={() => setIsCartOpen(false)}></div>
               <div className="relative bg-white/85 backdrop-blur-[40px] saturate-[210%] border-t border-white/50 rounded-t-3xl p-6 shadow-2xl animate-in slide-in-from-bottom max-h-[90vh] overflow-y-auto">
                 <div className="flex justify-between items-center mb-6">
                   <h2 className="text-xl font-bold font-outfit text-gray-900">Current Order</h2>
                   <button onClick={() => setIsCartOpen(false)} className="p-2 bg-gray-100 rounded-full text-gray-500 hover:bg-gray-200">
                     <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                   </button>
                 </div>

                 <div className="space-y-4 mb-6">
                   {cart.map((item, idx) => (
                     <div key={idx} className="flex justify-between items-center p-4 bg-white/50 rounded-xl border border-white/60 shadow-sm">
                       <div className="flex flex-col">
                         <span className="font-bold text-gray-900">{item.product.name}</span>
                         <span className="text-sm text-gray-500">Qty: {item.quantity}</span>
                       </div>
                       <span className="font-bold text-gray-900">${(item.product.price_cents * item.quantity / 100).toFixed(2)}</span>
                     </div>
                   ))}
                 </div>

                 <div className="border-t border-gray-200 pt-4 mb-4">
                   <div className="flex justify-between items-center font-bold text-xl text-gray-900">
                     <span>Total</span>
                     <span>${(cartTotal / 100).toFixed(2)}</span>
                   </div>
                 </div>

                 <StripeTerminalClient
                    amount={cartTotal}
                    productId={cart[0].product.id}
                    cart={cart}
                    tenantId={activeStaff?.tenant_id || "default_tenant"}
                    onOptimisticReserve={() => cart.forEach(item => handleOptimisticReserve(item.product.id))}
                    onOptimisticRollback={() => cart.forEach(item => handleOptimisticRollback(item.product.id))}
                    onSuccess={handleCheckoutComplete}
                 />
               </div>
             </div>
           )}

           {/* Post-Sale Screen */}
           {checkoutComplete && (
             <div className="fixed inset-0 z-50 flex items-center justify-center p-6 bg-white/80 backdrop-blur-[30px] saturate-[210%]">
               <div className="bg-white rounded-3xl p-8 shadow-2xl border border-gray-200 w-full max-w-sm text-center animate-in zoom-in-95">
                 <div className="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
                   <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                 </div>
                 <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Payment Successful!</h2>
                 <p className="text-gray-500 mb-8">The total of ${(cartTotal / 100).toFixed(2)} was charged.</p>

                 {!receiptSent ? (
                   <div className="text-left">
                     <label className="block text-sm font-bold text-gray-700 mb-2">Add customer details for receipt?</label>
                     <input
                       type="email"
                       placeholder="Customer email"
                       value={customerEmail}
                       onChange={(e) => setCustomerEmail(e.target.value)}
                       className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-[#0066FF] focus:border-transparent outline-none mb-4"
                     />
                     <button
                       onClick={handleSendReceipt}
                       disabled={!customerEmail}
                       className="w-full bg-[#0066FF] text-white font-bold py-3 px-4 rounded-xl active:scale-[0.98] disabled:opacity-50 min-h-[44px]"
                     >
                       Send Receipt & Complete
                     </button>
                     <button
                       onClick={() => { setCheckoutComplete(false); setCart([]); }}
                       className="w-full mt-3 text-gray-500 font-bold py-3 px-4 rounded-xl hover:bg-gray-50 active:scale-[0.98] min-h-[44px]"
                     >
                       No Receipt
                     </button>
                   </div>
                 ) : (
                   <p className="text-green-600 font-bold animate-pulse">Receipt sent! Loading new order...</p>
                 )}
               </div>
             </div>
           )}

           {orderStatus && <p className="mt-4 rounded-xl bg-blue-50 px-4 py-3 text-sm font-semibold text-blue-800 animate-in fade-in slide-in-from-top-2" role="status">{orderStatus}</p>}
        </div>

        {syncing && (
          <div className="absolute bottom-6 left-1/2 -translate-x-1/2 bg-[#0071E3]/90 backdrop-blur-[30px] saturate-[210%] border border-white/20 text-white px-6 py-3 rounded-full shadow-lg font-bold min-h-[44px] flex items-center justify-center space-x-2 z-50">
            <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>{t('Syncing transactions...')}</span>
          </div>
        )}
        {offlineConversion && (
          <div className="absolute bottom-16 left-1/2 -translate-x-1/2 bg-amber-100/80 backdrop-blur-[30px] saturate-[210%] border border-amber-200 shadow-xl shadow-amber-500/20 text-amber-900 px-4 py-2 rounded-full text-xs font-bold animate-bounce">
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
