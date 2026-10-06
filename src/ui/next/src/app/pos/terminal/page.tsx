"use client";

import { useState, useEffect, useRef } from 'react';
import StripeTerminalClient from './StripeTerminalClient';
import { LocalizationToggle } from '../../../components/LocalizationToggle';
import { SyncManager } from '../../../lib/sync/SyncManager';
import { QUEUE_IDENTITY_EPOCH_KEY, currentVerifiedQueueOwner, currentVerifiedQueueLease, hasPendingQueueOwnerVerification, hasVerifiedOfflineQueueOwner, readQueueOwner, sameOwner, subscribeQueueIdentityReadiness, type QueueOwner } from '../../../lib/sync/queueIdentity';
import { fetchForOwnedBusinessRead, openOnboardingSession } from '../../onboarding/draftSession';
import type { ClockQueueSummary } from '../../utils/offlineQueue';
import { MutationService } from '../../../lib/sync/MutationService';

type TerminalStaff = { id: string; name: string; role: string; tenant_id: string };
type TerminalLease = NonNullable<ReturnType<typeof currentVerifiedQueueLease>>;
const TERMINAL_IDENTITY_TIMEOUT_MS = 3000;

function waitForTerminalLease(owner: QueueOwner, current: () => boolean, timeoutMs = TERMINAL_IDENTITY_TIMEOUT_MS): Promise<TerminalLease> {
  return new Promise((resolve, reject) => {
    let settled = false;
    let unsubscribe = () => {};
    const timer = window.setTimeout(() => finish(null), timeoutMs);
    const finish = (lease: TerminalLease | null) => {
      if (settled) return;
      settled = true; window.clearTimeout(timer); unsubscribe();
      if (lease) resolve(lease); else reject(new Error('Verified terminal identity unavailable'));
    };
    unsubscribe = subscribeQueueIdentityReadiness(() => {
      if (!current()) { finish(null); return; }
      const lease = currentVerifiedQueueLease();
      if (lease) finish(sameOwner(lease.owner, owner) ? lease : null);
      else if (!hasPendingQueueOwnerVerification()) finish(null);
    });
    if (settled) unsubscribe();
  });
}

function confirmedStaff(value: unknown): TerminalStaff | null {
  if (!value || typeof value !== 'object') return null;
  const result = value as Record<string, unknown>;
  if (result.success !== true || !result.staff || typeof result.staff !== 'object') return null;
  const staff = result.staff as Record<string, unknown>;
  if (typeof staff.id !== 'string' || !staff.id.trim() || typeof staff.tenant_id !== 'string' || !staff.tenant_id.trim()
      || typeof staff.name !== 'string' || typeof staff.role !== 'string' || !staff.role.trim()) return null;
  return { id: staff.id, name: staff.name, role: staff.role, tenant_id: staff.tenant_id };
}

const t = (text: string) => text;

export default function POSTerminal() {
  const [authenticationError, setAuthenticationError] = useState('');
  const [authenticating, setAuthenticating] = useState(false);
  const authenticationPending = useRef(false);
  const [closed, setClosed] = useState(true);
  const [clockedIn, setClockedIn] = useState(false);
  const [clockPending, setClockPending] = useState(false);
  const [clockError, setClockError] = useState('');
  const clockPendingRef = useRef(false);
  const clockCheckPendingRef = useRef(false);
  const [clockCheckPending, setClockCheckPending] = useState(false);
  const [clockCheckError, setClockCheckError] = useState('');
  const [clockSummary, setClockSummary] = useState<ClockQueueSummary>({ confirmed: 0, unconfirmed: 0, legacyHeld: 0 });
  const mounted = useRef(true);
  const terminalVersion = useRef(0);
  const terminalLease = useRef<TerminalLease | null>(null);
  const committedClock = useRef<{ lease: TerminalLease; action: 'CLOCK_IN' | 'CLOCK_OUT' } | null>(null);
  const [queueIdentityReady, setQueueIdentityReady] = useState(false);
  const [queueAccessible, setQueueAccessible] = useState(false);
  const leaseTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const inventoryVersion = useRef(0);
  const [inventoryError, setInventoryError] = useState('');
  const [activeStaff, setActiveStaff] = useState<TerminalStaff | null>(null);
  const [inventory, setInventory] = useState<import("@/lib/business-records").SaleProduct[]>([]);
  useState(true);
  const [selectedProduct] = useState<import("@/lib/business-records").SaleProduct | null>(null);
  const [cart, setCart] = useState<import("@/lib/business-records").CartItem[]>([]);
  const [isCartOpen, setIsCartOpen] = useState(false);
  const [checkoutComplete, setCheckoutComplete] = useState(false);
  const [checkoutQueued, setCheckoutQueued] = useState(false);
  const [checkoutAmount, setCheckoutAmount] = useState(0);
  const [customerEmail, setCustomerEmail] = useState('');
  const [receiptSent] = useState(false);
  const [reserving, setReserving] = useState(false);
  const [orderStatus, setOrderStatus] = useState('');
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);
  const [offlineConversion] = useState(false);
  const [pendingSyncCount, setPendingSyncCount] = useState(0);
  const [syncSuccess, setSyncSuccess] = useState(false);
  const [queueError, setQueueError] = useState('');
  const [chargeAmount, setChargeAmount] = useState('0');
  const [showPaymentSheet, setShowPaymentSheet] = useState(false);
  const [posMode, setPosMode] = useState<'catalog' | 'quick_charge'>('catalog');

  const [sessionRegistration, setSessionRegistration] = useState<'pending' | 'unconfirmed' | null>(null);
  const [deviceId, setDeviceId] = useState<string>('');

  const retireTerminal = () => {
    terminalVersion.current += 1; terminalLease.current = null; committedClock.current = null;
    clearTimeout(leaseTimer.current); inventoryVersion.current += 1; setInventoryError('');
    authenticationPending.current = false; setAuthenticating(false); setSessionRegistration(null);
    clockPendingRef.current = false; setClockPending(false); setClockError('');
    clockCheckPendingRef.current = false; setClockCheckPending(false); setClockCheckError('');
    setClockSummary({ confirmed: 0, unconfirmed: 0, legacyHeld: 0 });
    setQueueIdentityReady(false); setClockedIn(false); setClosed(true); setActiveStaff(null);
    setInventory([]); setCart([]); setCheckoutComplete(false); setCheckoutQueued(false);
  };

  useEffect(() => {
    mounted.current = true;
    const unsubscribe = subscribeQueueIdentityReadiness(() => {
      const lease = terminalLease.current;
      if (!lease) { setQueueIdentityReady(false); return; }
      try {
        if (lease.expiresAt <= Date.now() || lease.storageEpoch !== localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY)) { retireTerminal(); return; }
      } catch { retireTerminal(); return; }
      const owner = currentVerifiedQueueOwner();
      if (!owner) { setQueueIdentityReady(false); return; }
      if (!sameOwner(owner, lease.owner)) { retireTerminal(); return; }
      setQueueIdentityReady(true);
      if (committedClock.current?.lease === lease) {
        setClockedIn(committedClock.current.action === 'CLOCK_IN');
        committedClock.current = null; setClockError('');
      }
    });
    const retire = () => retireTerminal();
    window.addEventListener('pagehide', retire);
    return () => { mounted.current = false; terminalVersion.current += 1; terminalLease.current = null; clearTimeout(leaseTimer.current); inventoryVersion.current += 1; unsubscribe(); window.removeEventListener('pagehide', retire); };
  }, []);


  useEffect(() => {
    let active = true;
    let readVersion = 0;
    let lastKnownCount: number | null = null;
    let successTimer: ReturnType<typeof setTimeout> | undefined;
    const checkQueue = async () => {
      const version = ++readVersion;
      try {
        const qLen = await SyncManager.getInstance().getQueueLength();
        const clocks = await SyncManager.getInstance().getClockQueueSummary();
        if (!active || version !== readVersion) return;
        clearTimeout(successTimer);
        const cleared = navigator.onLine && lastKnownCount !== null && lastKnownCount > 0 && qLen === 0;
        lastKnownCount = qLen;
        setPendingSyncCount(qLen); setClockSummary(clocks); setQueueError(''); setQueueAccessible(true);
        setSyncSuccess(cleared); setSyncing(navigator.onLine && qLen > 0);
        if (cleared) successTimer = setTimeout(() => { if (active) setSyncSuccess(false); }, 3000);
      } catch {
        if (!active || version !== readVersion) return;
        lastKnownCount = null; clearTimeout(successTimer);
        setSyncSuccess(false); setSyncing(false);
        setQueueAccessible(false);
        setQueueError('Queue status is unavailable. Saved actions remain held until your session and local storage can be verified.');
      }
    };
    const handleIdentityChanged = () => {
      retireTerminal(); setQueueAccessible(false);
      lastKnownCount = null; clearTimeout(successTimer); setSyncSuccess(false);
      setPendingSyncCount(0); void checkQueue();
    };

    const handleStorage = (event: StorageEvent) => {
      if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) handleIdentityChanged();
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
        try {
          let storedDeviceId = localStorage.getItem('omnisolo_pos_device_id');
          if (!storedDeviceId) {
            storedDeviceId = 'device_' + Date.now() + '_' + Math.floor(Math.random() * 1000);
            localStorage.setItem('omnisolo_pos_device_id', storedDeviceId);
          }
          setDeviceId(storedDeviceId);
        } catch {
          // Keep recovery notices mounted and do not invent a device identity
          // when this browser cannot preserve its existing local records.
          setAuthenticationError('Device storage is unavailable. Enable browser storage before opening the terminal.');
        }

        setIsOffline(!navigator.onLine);
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);
        window.addEventListener('omnisolo_queue_updated', handleQueueUpdated);
        window.addEventListener('omnisolo_auth_changed', handleIdentityChanged);
        window.addEventListener('storage', handleStorage);

        checkQueue();

        return () => {
          active = false; readVersion += 1; clearTimeout(successTimer);
          window.removeEventListener('omnisolo_auth_changed', handleIdentityChanged);
          window.removeEventListener('storage', handleStorage);
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
          window.removeEventListener('omnisolo_queue_updated', handleQueueUpdated);
        };
    }
  }, []);

  const handleOpenTerminal = async () => {
    if (authenticationPending.current) return;
    setAuthenticationError('');
    if (!deviceId) {
      setAuthenticationError('Device storage is unavailable. Enable browser storage before opening the terminal.');
      return;
    }
    if (isOffline || !navigator.onLine) {
      setAuthenticationError('Connect to the server to verify your staff identity. Offline access has not been authorized.');
      return;
    }

    authenticationPending.current = true;
    setAuthenticating(true);
    const attempt = ++terminalVersion.current;
    try {
      const storageEpoch = localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY);
      const current = () => mounted.current && attempt === terminalVersion.current && storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY);
      const expectedOwner = await openOnboardingSession();
      const lease = await waitForTerminalLease(expectedOwner, current);
      if (!current()) return;
      const headers = { 'Content-Type': 'application/json', 'x-ohc-expected-user': expectedOwner.userId, 'x-ohc-expected-tenant': expectedOwner.tenantId };
      const res = await fetch('/api/v1/pos/auth', {
        method: 'POST', headers, body: JSON.stringify({}),
      });
      const staff = res.ok ? confirmedStaff(await res.json()) : null;
      if (!current()) return;
      if (!staff) {
        setAuthenticationError('Your staff identity could not be verified. The terminal remains closed.');
        return;
      }
      const owner = await readQueueOwner();
      await waitForTerminalLease(expectedOwner, current);
      if (!current()) return;
      if (!sameOwner(owner, expectedOwner) || owner.tenantId !== staff.tenant_id || owner.userId !== staff.id || lease.expiresAt <= Date.now()) throw new Error('Terminal staff and signed identity differ');
      terminalLease.current = lease;
      clearTimeout(leaseTimer.current);
      leaseTimer.current = setTimeout(() => { if (terminalLease.current === lease) retireTerminal(); }, Math.min(lease.expiresAt - Date.now(), 2_147_483_647));
      setQueueIdentityReady(hasVerifiedOfflineQueueOwner(owner));
      setActiveStaff(staff);
      setSessionRegistration('pending');
      setClosed(false);

      // Account verification does not establish terminal registration or payment readiness.
      try {
        const sessionRes = await fetch('/api/v1/payments/terminal/session/start', {
          method: 'POST', headers, body: JSON.stringify({ device_id: deviceId }),
        });
        const sessionData = await sessionRes.json();
        if (!current() || terminalLease.current !== lease) return;
        const confirmed = sessionRes.ok && sessionData?.success === true
          && typeof sessionData.session_id === 'string' && !!sessionData.session_id.trim();
        setSessionRegistration(confirmed ? null : 'unconfirmed');
      } catch {
        if (current() && terminalLease.current === lease) setSessionRegistration('unconfirmed');
      }
    } catch {
      if (!mounted.current || attempt !== terminalVersion.current) return;
      setAuthenticationError('The authentication service is unavailable. The terminal remains closed.');
      setActiveStaff(null);
      setClosed(true);
    } finally {
      if (mounted.current && attempt === terminalVersion.current) {
        authenticationPending.current = false; setAuthenticating(false);
      }
    }
  };

  const handleClose = () => {
    retireTerminal();
  };

  const loadDashboard = async () => {
    const lease = terminalLease.current;
    if (!lease) return;
    const version = ++inventoryVersion.current;
    const leaseCurrent = () => {
      try { return mounted.current && terminalLease.current === lease && version === inventoryVersion.current && lease.expiresAt > Date.now() && lease.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY); }
      catch { return false; }
    };
    const current = () => {
      const owner = currentVerifiedQueueOwner();
      return leaseCurrent() && !!owner && sameOwner(owner, lease.owner);
    };
    if (isOffline) {
       if (current()) { setInventory([]); setInventoryError('Inventory is unavailable while offline.'); }
       return;
    }
    try {
      const res = await fetchForOwnedBusinessRead('/api/v1/pos/inventory', lease.owner);
      const data = await res.json();
      // Clock/queue activity can revalidate the same owner while this body is
      // arriving. Keep its real outcome until readiness settles, without
      // replacing or extending the original terminal lease.
      const deadline = Date.now() + TERMINAL_IDENTITY_TIMEOUT_MS;
      while (true) {
        await waitForTerminalLease(lease.owner, leaseCurrent, Math.max(0, deadline - Date.now()));
        if (!leaseCurrent()) return;
        if (current()) break;
        // Another readiness listener may begin a check before this continuation.
        // Retain the body, but never restart the original waiting budget.
        if (!hasPendingQueueOwnerVerification() || Date.now() >= deadline) throw new Error('Inventory identity verification did not settle');
      }
      if (res.status !== 200 || !data || !Array.isArray(data.inventory)) throw new Error('Inventory unavailable');
      setInventory(data.inventory); setInventoryError('');
    } catch (e) {
      // An unresolved check must not become an apparently empty catalog when
      // readiness later returns. This status stays behind the identity gate.
      if (leaseCurrent()) { console.error("Failed to load inventory", e); setInventory([]); setInventoryError('Inventory is unavailable. Verify your session and retry.'); }
    }
  };

  useEffect(() => {
    if (!closed && activeStaff) {
      loadDashboard();
    }
  }, [closed, activeStaff]);

  const handleClockAction = async (action: 'CLOCK_IN' | 'CLOCK_OUT') => {
    const lease = terminalLease.current;
    if (!activeStaff || !lease || committedClock.current || clockPendingRef.current || !queueAccessible || !hasVerifiedOfflineQueueOwner(lease.owner)) return;
    const version = terminalVersion.current;
    const current = () => {
      try { return mounted.current && version === terminalVersion.current && terminalLease.current === lease && lease.expiresAt > Date.now() && lease.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY); }
      catch { return false; }
    };
    clockPendingRef.current = true; setClockPending(true); setClockError('');
    let committed = false;
    try {
      await SyncManager.getInstance().enqueue({
        type: 'staff_clock_event_v1',
        payload: { staff_id: lease.owner.userId, event_type: action },
      }, lease.owner);
      committed = true;
      if (!current()) { if (mounted.current && terminalLease.current === lease) retireTerminal(); return; }
      committedClock.current = { lease, action };
      const owner = await readQueueOwner();
      if (!current() || !sameOwner(owner, lease.owner)) { if (mounted.current) retireTerminal(); return; }
      setClockedIn(action === 'CLOCK_IN');
      committedClock.current = null;
    } catch {
      if (current()) setClockError(committed
        ? 'Clock change was saved locally, but the current session could not be verified. Reverify before continuing.'
        : 'Clock change could not be saved. Your previous clock state is unchanged.');
      else if (mounted.current && terminalLease.current === lease) retireTerminal();
    } finally {
      if (current()) { clockPendingRef.current = false; setClockPending(false); }
    }
  };

  const handleCheckClockStatus = async () => {
    const lease = terminalLease.current;
    if (!lease || clockCheckPendingRef.current || !queueAccessible || !hasVerifiedOfflineQueueOwner(lease.owner) || !navigator.onLine) return;
    const version = terminalVersion.current;
    const current = () => {
      const owner = currentVerifiedQueueOwner();
      try { return mounted.current && version === terminalVersion.current && terminalLease.current === lease && lease.expiresAt > Date.now()
        && lease.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY) && !!owner && sameOwner(owner, lease.owner); }
      catch { return false; }
    };
    clockCheckPendingRef.current = true; setClockCheckPending(true); setClockCheckError('');
    try {
      await SyncManager.getInstance().reconcileClockReceipts();
      const summary = await SyncManager.getInstance().getClockQueueSummary();
      if (current()) setClockSummary(summary);
    } catch {
      if (current()) setClockCheckError('Saved clock status could not be verified. Unconfirmed records remain held.');
    } finally {
      if (mounted.current && version === terminalVersion.current && terminalLease.current === lease) {
        clockCheckPendingRef.current = false; setClockCheckPending(false);
      }
    }
  };

  const handleAddToCart = (product: import("@/lib/business-records").SaleProduct) => {
    setCart(prev => {
      const existing = prev.find(item => item.product.id === product.id);
      if (existing) {
        return prev.map(item => item.product.id === product.id ? { ...item, quantity: item.quantity + 1 } : item);
      }
      return [...prev, { product, quantity: 1 }];
    });
  };

  const cartTotal = cart.reduce((sum, item) => sum + (item.product.price_cents * item.quantity), 0);
  const cartItemCount = cart.reduce((sum, item) => sum + item.quantity, 0);

  const handleKeypadPress = (val: string) => {
    setChargeAmount(prev => {
      if (val === 'backspace') {
        return prev.length > 1 ? prev.slice(0, -1) : '0';
      }
      if (prev === '0' && val !== '0') return val;
      if (prev === '0' && val === '0') return prev;
      if (prev.length < 8) return prev + val;
      return prev;
    });
  };

  const handleCheckoutComplete = (amount: number) => {
    setCheckoutQueued(false);
    setCheckoutAmount(amount);
    setCheckoutComplete(true);
    setIsCartOpen(false);
    setShowPaymentSheet(false);
    setChargeAmount('0');
  };



  const handleCheckoutQueued = (amount: number) => {
    setCheckoutQueued(true);
    setCheckoutAmount(amount);
    setCheckoutComplete(true);
    setIsCartOpen(false);
    setShowPaymentSheet(false);
    setChargeAmount('0');
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
         MutationService.getInstance().executeMutation(
             'tap_to_pay',
             {
                 amount_cents: 5000,
                 product_id: 'quick_charge',
                 quantity: 1
             },
             () => {
                 setOrderStatus(t('Processing offline quick charge...'));
                 setTimeout(() => {
                    setOrderStatus(t('Offline Quick Charge Saved.'));
                    setReserving(false);
                    setTimeout(() => setOrderStatus(''), 3000);
                 }, 1000);
             },
             () => {
                 setOrderStatus(t('Failed to queue offline charge.'));
                 setReserving(false);
             }
         );
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

  if (closed) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] md:p-10 font-inter px-4 w-full overflow-hidden">
        <div className="w-full max-w-[375px] mx-auto bg-white rounded-3xl shadow-xl overflow-hidden p-8 border border-gray-100 relative">
           <div className="text-center mb-8">
             <div className="w-16 h-16 bg-gray-900 rounded-2xl mx-auto mb-4 flex items-center justify-center shadow-lg">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4h16v12H4zM8 20h8m-4-4v4" /></svg>
             </div>
             <h1 className="text-2xl font-bold text-gray-900 font-outfit">{t('Open POS terminal')}</h1>
             <p className="text-gray-500 text-sm mt-2">{t('Access uses your signed-in account. Close the terminal to hide its contents; sign out of your account before leaving a shared device.')}</p>
             {authenticationError && <p role="alert" className="mt-3 text-sm text-red-700">{authenticationError}</p>}
             {queueError && <p role="status" className="mt-3 text-sm text-amber-800">{queueError}</p>}
             {isOffline && <p className="text-[#FF9500] font-bold text-xs mt-2 bg-orange-50 inline-block px-2 py-1 rounded">{t('Offline Mode Active')}</p>}
           </div>

           <button
             type="button"
             onClick={handleOpenTerminal}
             disabled={authenticating}
             className="w-full py-4 rounded-xl bg-[#0071E3] text-white font-semibold disabled:opacity-50 min-h-[44px]"
           >
             {authenticating ? t('Verifying signed-in account...') : t('Continue with signed-in account')}
           </button>

           {syncing && <div className="absolute bottom-4 left-4 text-xs text-blue-400">{t('Saved actions awaiting confirmation')}</div>}
        </div>
      </div>
    );
  }

  return (
    <>
     {!queueIdentityReady && <div role="status">Verifying your terminal session. Private details are hidden.{clockError && <p role="alert">{clockError}</p>}<button type="button" onClick={() => { void readQueueOwner().catch(() => {}); }}>Reverify terminal session</button></div>}
     <div style={{ display: queueIdentityReady ? undefined : 'none' }} inert={!queueIdentityReady} className="flex flex-col items-center justify-center min-h-screen bg-[#F5F5F7] font-inter md:py-10 w-full overflow-x-hidden">
       {queueError && <p role="status" className="p-3 text-sm text-amber-800">{queueError}</p>}
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] md:h-[812px] md:min-h-0 bg-white md:shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200 mobile-pos-container">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] border-b border-gray-200 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">{activeStaff?.name}</h1>
            <p className="text-[#0071E3] font-medium text-sm mt-1">{t(activeStaff?.role ?? '')}</p>
            {isOffline ? (
              <div className="inline-flex items-center gap-1.5 mt-1 text-yellow-800 font-bold text-xs bg-yellow-100 px-2 py-1 rounded border border-yellow-200 shadow-sm">
                <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                {t('Offline - Changes will sync later')}
              </div>
            ) : (
              <span className="inline-block mt-1 text-green-800 font-bold text-xs bg-green-100 px-2 py-1 rounded border border-green-200 shadow-sm">{t('Online')}</span>
            )}
          </div>
          <div className="flex items-center gap-3">
            <LocalizationToggle />
            <button onClick={handleClose} className="text-sm font-semibold text-gray-500 hover:text-gray-900 min-h-[44px] min-w-[44px]">
              {t('Close terminal')}
            </button>
          </div>
        </div>

        {sessionRegistration && <p role="status" aria-label="Terminal session status" className="p-3 text-sm text-amber-800">
          {sessionRegistration === 'pending'
            ? 'Registering this terminal session. Signing in does not confirm payment readiness.'
            : 'Terminal session registration could not be confirmed. Signing in does not confirm payment readiness.'}
        </p>}

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-[#F5F5F7]">

           <div className="app-card rounded-2xl p-6 shadow-lg mb-6 text-center bg-[rgba(255,255,255,0.65)] backdrop-blur-[32px] saturate-[200%] border border-[rgba(255,255,255,0.4)]">
             <div className={`w-16 h-16 mx-auto rounded-full flex items-center justify-center mb-4 ${clockedIn ? 'bg-green-100 text-green-600' : 'bg-gray-100 text-gray-400'}`}>
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
             </div>
             <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">
               {clockedIn ? t('Clocked In') : t('Not Clocked In')}
             </h2>
             <p className="text-sm text-gray-500 mb-6">
                {clockedIn ? t('Clock-in saved on this device. Server confirmation is shown below.') : t('Clock in to start your shift.')}
             </p>
             <p role="status" className="text-sm mb-3">{queueAccessible && queueIdentityReady ? 'Offline queue ready for this session.' : 'Verify this session and local storage before offline work.'}</p>
             {clockSummary.unconfirmed > 0 && <p role="status">{clockSummary.unconfirmed} saved clock {clockSummary.unconfirmed === 1 ? 'change' : 'changes'} awaiting server confirmation.</p>}
             {clockSummary.confirmed > 0 && <p role="status">{clockSummary.confirmed} saved clock {clockSummary.confirmed === 1 ? 'change' : 'changes'} confirmed by the server.</p>}
             {clockSummary.legacyHeld > 0 && <p role="status">{clockSummary.legacyHeld} historical clock {clockSummary.legacyHeld === 1 ? 'change requires' : 'changes require'} review. Original records are preserved.</p>}
             <button type="button" onClick={handleCheckClockStatus} disabled={clockCheckPending || isOffline || !queueAccessible || !queueIdentityReady} className="text-sm underline mb-3">
               {clockCheckPending ? 'Checking saved clock status...' : 'Check saved clock status'}
             </button>
             {clockCheckError && <p role="alert">{clockCheckError}</p>}
             {clockPending && <p role="status">Saving clock change...</p>}
             {clockError && <p role="alert">{clockError}</p>}
             {committedClock.current && <button type="button" onClick={() => { void readQueueOwner().catch(() => {}); }}>Reverify saved clock change</button>}

             {clockedIn ? (
               <button
                 onClick={() => handleClockAction('CLOCK_OUT')}
                 disabled={clockPending || !!committedClock.current || !queueAccessible || !queueIdentityReady}
                 className="w-full py-4 rounded-xl bg-red-50 text-red-600 font-bold hover:bg-red-100 transition-colors min-h-[44px] min-w-[44px]"
               >
                 {t('Clock Out')}
               </button>
             ) : (
               <button
                 onClick={() => handleClockAction('CLOCK_IN')}
                 disabled={clockPending || !!committedClock.current || !queueAccessible || !queueIdentityReady}
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
                className={`charge-btn min-h-[44px] min-w-[44px] p-4 rounded-[8px] text-left shadow-lg bg-[rgba(255,255,255,0.65)] backdrop-blur-[32px] saturate-[200%] border border-[rgba(255,255,255,0.4)] ${reserving ? 'opacity-50' : 'active:scale-[0.98]'}`}
             >
               <div className="text-[#0066FF] mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
               </div>
               <span className="font-medium text-gray-900">{t('New Order')}</span>
             </button>

             <button className="min-h-[44px] min-w-[44px] p-4 rounded-[8px] text-left shadow-lg active:scale-[0.98] bg-[rgba(255,255,255,0.65)] backdrop-blur-[32px] saturate-[200%] border border-[rgba(255,255,255,0.4)]">
               <div className="text-[#FF9500] mb-2">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 15v-1a4 4 0 00-4-4H8m0 0l3 3m-3-3l3-3m9 14V5a2 2 0 00-2-2H6a2 2 0 00-2 2v16l4-2 4 2 4-2 4 2z" /></svg>
               </div>
               <span className="font-medium text-gray-900">{t('Refunds')}</span>
             </button>
           </div>

           {clockedIn && !isCartOpen && !showPaymentSheet && !checkoutComplete && (
             <div className="mb-8 p-4 rounded-2xl bg-[rgba(255,255,255,0.65)] backdrop-blur-[32px] saturate-[200%] border border-[rgba(255,255,255,0.4)] shadow-lg">
               <StripeTerminalClient
                 amount={5000}
                 productId="quick_charge"
                 cart={[]}
                 tenantId={activeStaff?.tenant_id || "default_tenant"}
                 onSuccess={handleCheckoutComplete}
                    onQueued={handleCheckoutQueued}
               />
             </div>
           )}

           {/* View Toggle */}
           <div className="flex bg-gray-200/50 backdrop-blur-[30px] rounded-xl p-1 mb-6 mx-2 mt-8">
              <button
                onClick={() => setPosMode('catalog')}
                className={`flex-1 py-2 text-sm font-bold rounded-lg transition-all min-h-[44px] ${posMode === 'catalog' ? 'bg-white text-gray-900 shadow-sm' : 'text-gray-500'}`}
              >
                {t('Catalog')}
              </button>
              <button
                onClick={() => setPosMode('quick_charge')}
                className={`flex-1 py-2 text-sm font-bold rounded-lg transition-all min-h-[44px] ${posMode === 'quick_charge' ? 'bg-white text-gray-900 shadow-sm' : 'text-gray-500'}`}
              >
                {t('Quick Charge')}
              </button>
           </div>

           {/* Catalog Selection */}
           {posMode === 'catalog' && (
             <>
               <h3 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-2">{t('Product Catalog')}</h3>
               <div className="grid grid-cols-1 gap-3 mb-8">
                  {inventoryError ? (
                    <div role="status">{inventoryError}<button type="button" onClick={() => { void loadDashboard(); }}>Refresh inventory</button></div>
                  ) : inventory.length === 0 ? (
                    <p className="text-center text-gray-500 py-4 italic">{t('No products found in catalog')}</p>
                  ) : inventory.map(product => (
                    <button
                      key={product.id}
                      onClick={() => handleAddToCart(product)} disabled={reserving || isCartOpen}
                      className={`p-4 rounded-[8px] text-left transition-all active:scale-[0.98] min-h-[64px] min-w-[44px] shadow-lg backdrop-blur-[30px] saturate-[210%] ${selectedProduct?.id === product.id ? 'bg-[rgba(255,255,255,0.65)] ring-1 ring-[#0066FF] border border-[#0066FF]' : 'bg-[rgba(255,255,255,0.65)] border border-[rgba(255,255,255,0.4)]'}`}
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
             </>
           )}

           {/* Quick Charge Keypad */}
           {posMode === 'quick_charge' && (
             <div className="mb-8 px-2 flex flex-col items-center">
                <div className="w-full text-center mb-6">
                  <div className="text-5xl font-outfit font-bold text-gray-900">
                    ${(parseInt(chargeAmount || '0') / 100).toFixed(2)}
                  </div>
                </div>

                <div className="grid grid-cols-3 gap-4 w-full max-w-xs mb-8">
                  {['1', '2', '3', '4', '5', '6', '7', '8', '9'].map(num => (
                    <button
                      key={num}
                      onClick={() => handleKeypadPress(num)}
                      className="bg-white/70 backdrop-blur-[30px] border border-white/50 rounded-2xl h-16 text-2xl font-bold text-gray-900 shadow-sm active:bg-gray-200 transition-colors min-h-[44px]"
                    >
                      {num}
                    </button>
                  ))}
                  <button className="h-16 min-h-[44px]"></button>
                  <button
                    onClick={() => handleKeypadPress('0')}
                    className="bg-white/70 backdrop-blur-[30px] border border-white/50 rounded-2xl h-16 text-2xl font-bold text-gray-900 shadow-sm active:bg-gray-200 transition-colors min-h-[44px]"
                  >
                    0
                  </button>
                  <button
                    onClick={() => handleKeypadPress('backspace')}
                    className="bg-white/70 backdrop-blur-[30px] border border-white/50 rounded-2xl h-16 text-2xl font-bold text-gray-900 shadow-sm active:bg-gray-200 flex items-center justify-center transition-colors min-h-[44px]"
                  >
                    <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2M3 12l6.414 6.414a2 2 0 001.414.586H19a2 2 0 002-2V7a2 2 0 00-2-2h-8.172a2 2 0 00-1.414.586L3 12z" /></svg>
                  </button>
                </div>

                <button
                  onClick={() => setShowPaymentSheet(true)}
                  disabled={parseInt(chargeAmount || '0') === 0}
                  className="w-full bg-[#0066FF] text-white rounded-xl min-h-[60px] text-lg font-bold flex justify-center items-center px-6 shadow-lg active:scale-[0.98] disabled:opacity-50"
                >
                  Charge ${(parseInt(chargeAmount || '0') / 100).toFixed(2)}
                </button>
             </div>
           )}

           {/* Bottom Bar */}
           {cartItemCount > 0 && !checkoutComplete && (
             <div className="fixed bottom-0 left-0 right-0 p-4 bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] border-t border-gray-200 z-40 pb-safe pb-8">
               <button
                 onClick={() => setIsCartOpen(true)}
                 className="w-full bg-[#0066FF] text-white rounded-xl min-h-[60px] text-lg font-bold flex justify-between items-center px-6 shadow-lg active:scale-[0.98]"
               >
                 <span className="bg-white/20 px-3 py-1 rounded-full text-sm">{cartItemCount} item{cartItemCount > 1 ? 's' : ''}</span>
                 <span>Charge ${(cartTotal / 100).toFixed(2)}</span>
               </button>
             </div>
           )}

           {/* Cart Drawer & Payment Bottom Sheet */}
           {(isCartOpen || showPaymentSheet) && !checkoutComplete && (
             <div className="fixed inset-0 z-50 flex flex-col justify-end">
               <div className="absolute inset-0 bg-black/40 backdrop-blur-[30px] saturate-[210%]" onClick={() => { setIsCartOpen(false); setShowPaymentSheet(false); }}></div>
               <div className="relative bg-[rgba(255,255,255,0.65)] backdrop-blur-[40px] saturate-[210%] border-t border-[rgba(255,255,255,0.4)] rounded-t-3xl p-6 shadow-2xl animate-in slide-in-from-bottom max-h-[90vh] overflow-y-auto">
                 <div className="flex justify-between items-center mb-6">
                   <h2 className="text-xl font-bold font-outfit text-gray-900">{isCartOpen ? 'Current Order' : 'Payment Method'}</h2>
                   <button onClick={() => { setIsCartOpen(false); setShowPaymentSheet(false); }} className="p-2 bg-gray-100 rounded-full text-gray-500 hover:bg-gray-200">
                     <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                   </button>
                 </div>

                 {isCartOpen && (
                   <>
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
                   </>
                 )}

                 {showPaymentSheet && posMode === 'quick_charge' && (
                   <div className="border-b border-gray-200 pb-4 mb-6 text-center">
                     <div className="text-sm text-gray-500 uppercase tracking-wider font-bold mb-1">Amount Due</div>
                     <div className="text-4xl font-bold text-gray-900">${(parseInt(chargeAmount || '0') / 100).toFixed(2)}</div>
                   </div>
                 )}

                 <StripeTerminalClient
                    amount={posMode === 'quick_charge' ? parseInt(chargeAmount || '0') : cartTotal}
                    productId={posMode === 'quick_charge' ? 'custom-charge' : (cart[0]?.product?.id || 'custom-charge')}
                    cart={posMode === 'quick_charge' ? [] : cart}
                    tenantId={activeStaff?.tenant_id || "default_tenant"}
                    onOptimisticReserve={() => { if (posMode !== 'quick_charge') cart.forEach(item => handleOptimisticReserve(item.product.id)) }}
                    onOptimisticRollback={() => { if (posMode !== 'quick_charge') cart.forEach(item => handleOptimisticRollback(item.product.id)) }}
                    onSuccess={handleCheckoutComplete}
                    onQueued={handleCheckoutQueued}
                 />
               </div>
             </div>
           )}

           {/* Post-Sale Screen */}
           {checkoutComplete && (
             <div className="fixed inset-0 z-50 flex items-center justify-center p-6 bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%]">
               <div className="bg-white rounded-3xl p-8 shadow-2xl border border-gray-200 w-full max-w-sm text-center animate-in zoom-in-95">
                 <div className="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
                   <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                 </div>
                 <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">{checkoutQueued ? 'Sale queued offline' : 'Payment Successful!'}</h2>
                 <p className="text-gray-500 mb-8">{checkoutQueued ? `The $${(checkoutAmount / 100).toFixed(2)} sale is saved on this device and still needs to sync.` : `The total of $${(checkoutAmount / 100).toFixed(2)} was charged.`}</p>

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
                       disabled
                       className="w-full bg-[#0066FF] text-white font-bold py-3 px-4 rounded-xl active:scale-[0.98] disabled:opacity-50 min-h-[44px]"
                     >
                       Receipt service unavailable
                     </button>
                     <button
                       onClick={() => { setCheckoutComplete(false); setCart([]); }}
                       className="w-full mt-3 text-gray-500 font-bold py-3 px-4 rounded-xl hover:bg-gray-50 active:scale-[0.98] min-h-[44px]"
                     >
                       No Receipt
                     </button>
                   </div>
                 ) : null}
               </div>
             </div>
           )}

           {orderStatus && <p className="mt-4 rounded-xl bg-blue-50 px-4 py-3 text-sm font-semibold text-blue-800 animate-in fade-in slide-in-from-top-2" role="status">{orderStatus}</p>}

           {/* Operations Agent Notification Card */}
           {checkoutComplete && cart.some(item => {
             const invItem = inventory.find(i => i.id === item.product.id);
             return invItem && (invItem.available_quantity - item.quantity <= 0);
           }) && (
             <div className="mt-6 bg-[rgba(255,255,255,0.65)] backdrop-blur-[40px] saturate-[210%] border-l-4 border-l-[#FF9500] border border-[rgba(255,255,255,0.4)] rounded-2xl p-4 shadow-xl animate-in slide-in-from-bottom-4">
               <div className="flex items-start space-x-3">
                 <div className="w-10 h-10 rounded-full bg-[#FF9500]/10 flex items-center justify-center flex-shrink-0">
                   <svg className="w-6 h-6 text-[#FF9500]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                 </div>
                 <div>
                   <h3 className="font-bold text-gray-900 font-outfit">Operations Agent</h3>
                   <p className="text-sm text-gray-600 mt-1">
                     {cart.find(item => {
                       const invItem = inventory.find(i => i.id === item.product.id);
                       return invItem && (invItem.available_quantity - item.quantity <= 0);
                     })?.product.name} sold out. Would you like to draft a restock order?
                   </p>
                   <div className="mt-3 flex space-x-2">
                     <button className="px-4 py-2 bg-[#FF9500] text-white text-sm font-bold rounded-xl active:scale-[0.98]">Draft Restock</button>
                     <button className="px-4 py-2 bg-gray-100 text-gray-700 text-sm font-bold rounded-xl active:scale-[0.98]">Dismiss</button>
                   </div>
                 </div>
               </div>
             </div>
           )}
        </div>

        {isOffline && (
          <div className="absolute top-4 left-1/2 -translate-x-1/2 bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] shadow-lg text-gray-900 px-6 py-3 rounded-full font-bold min-h-[44px] flex items-center justify-center space-x-2 z-50">
            <svg className="w-5 h-5 text-[#FF9500]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
            <span>{t('Offline Mode')}{pendingSyncCount > 0 ? ` - ${pendingSyncCount} Pending` : ''}</span>
          </div>
        )}
        {syncing && !isOffline && (
          <div role="status" style={{ pointerEvents: 'none' }} className="absolute bottom-6 left-1/2 -translate-x-1/2 bg-[#0071E3]/90 backdrop-blur-[30px] saturate-[210%] border border-white/20 text-white px-6 py-3 rounded-full shadow-lg font-bold min-h-[44px] flex items-center justify-center space-x-2 z-50">
            <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>{t('Saved transactions awaiting confirmation')}</span>
          </div>
        )}
        {syncSuccess && !isOffline && (
          <div role="status" style={{ pointerEvents: 'none' }} className="absolute bottom-6 left-1/2 -translate-x-1/2 bg-[#34C759]/90 backdrop-blur-[30px] saturate-[210%] border border-white/20 text-white px-6 py-3 rounded-full shadow-lg font-bold min-h-[44px] flex items-center justify-center space-x-2 z-50">
            <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
            <span>{t('Synced')}</span>
          </div>
        )}
        {offlineConversion && (
          <div className="absolute bottom-16 left-1/2 -translate-x-1/2 bg-amber-100/80 backdrop-blur-[30px] saturate-[210%] border border-amber-200 shadow-xl shadow-amber-500/20 text-amber-900 px-4 py-2 rounded-full text-xs font-bold animate-bounce">
            {t('Using cached rates - Syncing soon')}
          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `

        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
    </>
  );
}
