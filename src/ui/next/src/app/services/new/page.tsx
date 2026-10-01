'use client';
import { useEffect, useRef, useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { sameOwner } from '@/lib/sync/queueIdentity';
import {
  fetchForOwnedBusinessAction, onboardingOwner, onboardingSessionEpoch,
  openOnboardingSession, readOwnedOnboardingItem, subscribeOnboardingInvalidation,
  writeOwnedOnboardingItem, type DraftOwner,
} from '../../onboarding/draftSession';

type Fields = { title: string; description: string; price: string };
const EMPTY: Fields = { title: '', description: '', price: '' };
const DRAFT = 'service-create-draft';
const REQUEST = 'service-create-request';
const UNKNOWN = 'Could not confirm whether the service was saved. This request is on hold to avoid creating a duplicate.';
const serviceId = (value: unknown): value is string => typeof value === 'string' && /^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/i.test(value);
function restoreFields(raw: string | null): Fields {
  if (!raw) return { ...EMPTY };
  const value: unknown = JSON.parse(raw);
  if (!value || typeof value !== 'object') throw new Error('Invalid service draft');
  const data = value as Record<string, unknown>;
  if (typeof data.title !== 'string' || typeof data.description !== 'string' || typeof data.price !== 'string' || data.title.length > 1000 || data.description.length > 20000 || data.price.length > 100) throw new Error('Invalid service draft');
  return { title: data.title, description: data.description, price: data.price };
}
function servicePayload(fields: Fields) {
  const title = fields.title.trim();
  if (!title) throw new Error('Enter a service title before saving.');
  if ([...title].length > 200) throw new Error('Keep the service title within 200 characters.');
  if ([...fields.description].length > 10000) throw new Error('Keep the description within 10,000 characters.');
  const price = fields.price.trim();
  if (price && !/^\d+(?:\.\d{1,2})?$/.test(price)) throw new Error('Enter a price from $0 to $10,000,000 with no more than two decimal places.');
  const [dollars = '0', cents = ''] = price.split('.');
  const priceCents = Number(dollars || '0') * 100 + Number(cents.padEnd(2, '0'));
  if (!Number.isSafeInteger(priceCents) || priceCents < 0 || priceCents > 1_000_000_000) throw new Error('Enter a price from $0 to $10,000,000 with no more than two decimal places.');
  return { title, description: fields.description, price_cents: priceCents };
}

function untilAborted<T>(operation: Promise<T>, signal: AbortSignal): Promise<T> {
  return new Promise((resolve, reject) => {
    const abort = () => reject(new DOMException('Service request interrupted', 'AbortError'));
    if (signal.aborted) abort();
    else signal.addEventListener('abort', abort, { once: true });
    operation.then(value => { signal.removeEventListener('abort', abort); resolve(value); },
      error => { signal.removeEventListener('abort', abort); reject(error); });
  });
}

export default function NewServicePage() {
  const router = useRouter();
  const [fields, setFields] = useState<Fields>({ ...EMPTY });
  const [ready, setReady] = useState(false);
  const [saving, setSaving] = useState(false);
  const [held, setHeld] = useState(false);
  const [receipt, setReceipt] = useState<{ id: string; cached: boolean } | null>(null);
  const [message, setMessage] = useState('Checking your session…');
  const [verification, setVerification] = useState(0);
  const owner = useRef<DraftOwner | null>(null);
  const mounted = useRef(false);
  const generation = useRef(0);
  const inFlight = useRef(false);
  const blocked = useRef(false);
  const activeRequest = useRef<AbortController | null>(null);
  const current = (expected: DraftOwner, token: number, epoch: number) => {
    const active = onboardingOwner();
    return mounted.current && generation.current === token && onboardingSessionEpoch() === epoch && !!owner.current && !!active && sameOwner(owner.current, expected) && sameOwner(active, expected);
  };

  useEffect(() => {
    mounted.current = true;
    let load = 0;
    const clear = () => {
      activeRequest.current?.abort(); activeRequest.current = null;
      generation.current += 1; owner.current = null; inFlight.current = false; blocked.current = false;
      setFields({ ...EMPTY }); setReady(false); setSaving(false); setHeld(false); setReceipt(null);
    };
    const verify = async () => {
      const sequence = ++load, epoch = onboardingSessionEpoch();
      try {
        const verified = await openOnboardingSession();
        if (!mounted.current || sequence !== load || epoch !== onboardingSessionEpoch()) return;
        const restored = restoreFields(readOwnedOnboardingItem(DRAFT));
        const request = readOwnedOnboardingItem(REQUEST);
        owner.current = verified; setFields(restored); setReady(true); setMessage('');
        if (request) {
          let previous: { status?: unknown; serviceId?: unknown } | null = null;
          try { previous = JSON.parse(request); } catch { /* Unreadable receipts stay held. */ }
          blocked.current = true;
          if (previous?.status === 'acknowledged' && serviceId(previous.serviceId)) setReceipt({ id: previous.serviceId, cached: true });
          else { setHeld(true); setMessage(UNKNOWN); }
        }
      } catch {
        if (mounted.current && sequence === load && epoch === onboardingSessionEpoch()) {
          setMessage('Could not verify your session or restore its local request status. Saving is blocked.');
        }
      }
    };
    clear(); void verify();
    const unsubscribe = subscribeOnboardingInvalidation(restart => {
      load += 1; clear(); setMessage('Your session changed. Verify your session before saving.');
      if (restart) void verify();
    });
    return () => { activeRequest.current?.abort(); activeRequest.current = null; mounted.current = false; generation.current += 1; load += 1; owner.current = null; unsubscribe(); };
  }, [verification]);

  const change = (name: keyof Fields, value: string) => {
    const expected = owner.current;
    if (!expected || inFlight.current || blocked.current || !current(expected, generation.current, onboardingSessionEpoch())) return;
    const next = { ...fields, [name]: value }; setFields(next); setMessage('');
    try { writeOwnedOnboardingItem(DRAFT, JSON.stringify(next)); }
    catch { setMessage('Changes could not be saved on this device. Keep this page open.'); }
  };
  const save = async () => {
    const expected = owner.current;
    if (!expected || inFlight.current || blocked.current) return;
    let payload: ReturnType<typeof servicePayload>;
    try { payload = servicePayload(fields); }
    catch (error) { setMessage(error instanceof Error ? error.message : 'Check the service fields.'); return; }
    const token = generation.current, epoch = onboardingSessionEpoch();
    if (!navigator.locks?.request) { setMessage('Service saving is unavailable in this browser. Your draft has been kept.'); return; }
    inFlight.current = true; setSaving(true); setMessage('');
    const controller = new AbortController(); activeRequest.current = controller;
    const timeout = window.setTimeout(() => controller.abort(), 30_000);
    let dispatched = false;
    try {
      await navigator.locks.request('omnisolo-service-create:' + JSON.stringify([expected.userId, expected.tenantId]), { mode: 'exclusive', signal: controller.signal }, () => untilAborted((async () => {
      if (controller.signal.aborted || !current(expected, token, epoch)) return;
      const previousRequest = readOwnedOnboardingItem(REQUEST);
      if (previousRequest) {
        blocked.current = true;
        let previous: { status?: unknown; serviceId?: unknown } | null = null;
        try { previous = JSON.parse(previousRequest); } catch { /* Preserve unknown outcomes. */ }
        if (previous?.status === 'acknowledged' && serviceId(previous.serviceId)) setReceipt({ id: previous.serviceId, cached: true });
        else { setHeld(true); setMessage(UNKNOWN); }
        return;
      }
      const response = await fetchForOwnedBusinessAction('/api/v1/booking/services', {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), signal: controller.signal,
      }, expected, () => {
        if (controller.signal.aborted || !current(expected, token, epoch)) throw new Error('The service view changed before dispatch');
        writeOwnedOnboardingItem(REQUEST, JSON.stringify({ status: 'unknown', payload }));
        dispatched = true; blocked.current = true;
      });
      const data: unknown = await response.json();
      if (controller.signal.aborted || !current(expected, token, epoch)) return;
      const result = data && typeof data === 'object' ? data as Record<string, unknown> : null;
      if (response.status === 200 && result?.success === true && result.error === null && serviceId(result.service_id)) {
        let stored = true;
        try { writeOwnedOnboardingItem(REQUEST, JSON.stringify({ status: 'acknowledged', serviceId: result.service_id })); }
        catch { stored = false; }
        if (controller.signal.aborted || !current(expected, token, epoch)) return;
        setReceipt({ id: result.service_id, cached: false }); setHeld(false);
        setMessage(stored ? '' : 'The service was saved, but this device could not store the receipt. Keep this confirmation.');
        if (stored) {
          try { router.push('/dashboard'); }
          catch { if (current(expected, token, epoch)) setMessage('The service was saved. Automatic navigation failed; use Back to dashboard.'); }
        }
      } else if (response.status === 400 && result?.success === false && result.service_id === null) {
        try {
          writeOwnedOnboardingItem(REQUEST, ''); blocked.current = false; setHeld(false);
          setMessage('Service was not saved. Check the title, description, and price before trying again.');
        } catch {
          setHeld(true); setMessage('Service was not saved, but the local request hold could not be cleared. No retry was sent.');
        }
      } else { setHeld(true); setMessage(UNKNOWN); }
      })(), controller.signal));
    } catch {
      if (!current(expected, token, epoch)) return;
      if (dispatched) { setHeld(true); setMessage(UNKNOWN); }
      else if (controller.signal.aborted) setMessage('The request timed out before it was sent. You can try again.');
      else setMessage('Could not record this request locally. No service was sent. Check local storage before trying again.');
    } finally {
      window.clearTimeout(timeout);
      if (activeRequest.current === controller) activeRequest.current = null;
      if (current(expected, token, epoch)) { inFlight.current = false; setSaving(false); }
    }
  };
  const startAnother = async () => {
    const expected = owner.current;
    if (!expected || inFlight.current || !receipt || !navigator.locks?.request) return;
    const token = generation.current, epoch = onboardingSessionEpoch(), expectedId = receipt.id;
    inFlight.current = true; setSaving(true);
    try {
      await navigator.locks.request('omnisolo-service-create:' + JSON.stringify([expected.userId, expected.tenantId]), { mode: 'exclusive' }, async () => {
        if (!current(expected, token, epoch)) return;
        if (readOwnedOnboardingItem(REQUEST) !== JSON.stringify({ status: 'acknowledged', serviceId: expectedId })) {
          blocked.current = true; setReceipt(null); setHeld(true); setMessage(UNKNOWN); return;
        }
        writeOwnedOnboardingItem(DRAFT, JSON.stringify(EMPTY)); writeOwnedOnboardingItem(REQUEST, '');
        if (!current(expected, token, epoch)) return;
        blocked.current = false; setReceipt(null); setFields({ ...EMPTY }); setHeld(false); setMessage('');
      });
    } catch { if (current(expected, token, epoch)) setMessage('Could not start a new local draft. The existing receipt is retained.'); }
    finally { if (current(expected, token, epoch)) { inFlight.current = false; setSaving(false); } }
  };

  if (receipt) return <div className="p-4 max-w-md mx-auto mt-20 text-center">
    <h2 className="text-2xl font-bold text-green-600 mb-2">{receipt.cached ? 'Previously acknowledged service' : 'Service Saved!'}</h2>
    <p>Service reference: <span>{receipt.id}</span></p>
    {message && <p role="status">{message}</p>}
    <button onClick={startAnother} disabled={saving} className="mt-4 text-blue-700">Add another service</button>
    <p className="mt-3"><Link href="/dashboard">Back to dashboard</Link></p>
  </div>;

  const disabled = !ready || saving || held;
  return <div className="p-4 max-w-md mx-auto">
    <div className="flex items-center mb-6"><Link href="/dashboard" className="mr-4 text-blue-700">&lt; Back</Link><h1 className="text-2xl font-bold">Add Service</h1></div>
    <div className="space-y-4">
      <div><label htmlFor="service-title" className="block text-sm font-medium mb-1">Service Title</label>
        <input id="service-title" type="text" value={fields.title} onChange={event => change('title', event.target.value)} disabled={disabled} className="w-full border rounded p-2 text-black" placeholder="e.g. Weekly Music Tutoring" required /></div>
      <div><div className="flex justify-between items-center mb-1"><label htmlFor="service-description" className="text-sm font-medium">Description</label>
        <button onClick={() => change('description', fields.title.trim())} disabled={disabled || !fields.title.trim() || fields.description === fields.title.trim()} className="text-xs text-purple-700">Copy title</button></div>
        <textarea id="service-description" value={fields.description} onChange={event => change('description', event.target.value)} disabled={disabled} className="w-full border rounded p-2 text-black h-24" placeholder="Describe the service..." /></div>
      <div><label htmlFor="service-price" className="block text-sm font-medium mb-1">Price</label>
        <div className="relative"><span className="absolute left-3 top-2">$</span><input id="service-price" type="text" inputMode="decimal" value={fields.price} onChange={event => change('price', event.target.value)} disabled={disabled} className="w-full border rounded p-2 pl-8 text-black" placeholder="0.00" /></div>
        <p className="text-sm text-gray-600">Leave blank for $0. Saving a service does not charge a customer.</p></div>
      <div className="border-t pt-4"><label className="flex gap-2"><input type="checkbox" aria-label="Recurring payment" checked={false} disabled readOnly />Recurring payment</label>
        <p className="text-sm text-gray-600">Recurring payments are not supported for saved services.</p></div>
      <div className="pt-4">{message && <p className="mb-3 text-sm" role="status">{message}</p>}
        {!ready && <button onClick={() => setVerification(value => value + 1)} className="mb-3 text-blue-700">Verify session</button>}
        <button onClick={save} disabled={disabled} className="w-full bg-black text-white font-medium py-3 rounded-lg disabled:opacity-50">{saving ? 'Saving…' : 'Save Service'}</button></div>
    </div>
  </div>;
}
