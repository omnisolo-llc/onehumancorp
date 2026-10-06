"use client";

import { useCallback, useEffect, useRef, useState } from 'react';
import { QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';

type Owner = { userId: string; tenantId: string; expiresAt: number };
type Preferences = { urgent_booking: boolean; failed_payment: boolean; new_order: boolean };
type Challenge = { challenge_id: string; phone: string; state: string; expires_at: number };
type Snapshot = { status: 'verified' | 'unverified'; phone: string | null; verification_id: string | null; preferences: Preferences; challenge: Challenge | null; provider_configured: boolean };
const emptyPreferences: Preferences = { urgent_booking: false, failed_payment: false, new_order: false };
const phoneNumber = (value: unknown): value is string => typeof value === 'string' && /^\+[1-9]\d{7,14}$/.test(value);
const identifier = (value: unknown): value is string => typeof value === 'string' && /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid SMS response');
  return value as Record<string, unknown>;
}
function receipt(value: unknown, owner: Owner) {
  const data = record(value);
  if (data.success !== true || data.error != null || data.organization_id !== owner.tenantId || data.user_id !== owner.userId) throw new Error('Unconfirmed SMS response');
  return data;
}
function parseSnapshot(value: unknown, owner: Owner): Snapshot {
  const data = receipt(value, owner), preferences = record(data.preferences);
  if (!['verified', 'unverified'].includes(String(data.status)) || typeof data.provider_configured !== 'boolean'
    || Object.keys(emptyPreferences).some(key => typeof preferences[key] !== 'boolean')) throw new Error('Invalid SMS settings');
  if (data.status === 'unverified' && Object.values(preferences).some(value => value !== false)) throw new Error('Unverified notification subscription');
  if (data.status === 'verified' ? !phoneNumber(data.phone) || !identifier(data.verification_id) : data.phone !== null || data.verification_id !== null) throw new Error('Invalid verification receipt');
  let challenge: Challenge | null = null;
  if (data.challenge !== null) {
    const item = record(data.challenge);
    if (!identifier(item.challenge_id) || !phoneNumber(item.phone) || !['sending', 'accepted', 'unknown', 'rejected', 'verified', 'superseded'].includes(String(item.state)) || typeof item.expires_at !== 'number' || !Number.isSafeInteger(item.expires_at)) throw new Error('Invalid challenge receipt');
    challenge = item as Challenge;
  }
  return { status: data.status as Snapshot['status'], phone: data.phone as string | null, verification_id: data.verification_id as string | null,
    preferences: preferences as Preferences, challenge, provider_configured: data.provider_configured };
}

export function SmsSettingsPanel() {
  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);
  const [phone, setPhone] = useState(''), [otp, setOtp] = useState(''), [message, setMessage] = useState('Checking SMS settings…');
  const [pending, setPending] = useState(false), [sendHeld, setSendHeld] = useState(false);
  const owner = useRef<Owner | null>(null), generation = useRef(0), busy = useRef(false);
  const active = (epoch: number) => epoch === generation.current && owner.current !== null && owner.current.expiresAt > Date.now();
  const read = useCallback(async (expected: Owner, epoch: number) => {
    const response = await fetch('/api/v1/settings/sms-preferences', { cache: 'no-store', credentials: 'same-origin', headers: { 'x-ohc-expected-user': expected.userId, 'x-ohc-expected-tenant': expected.tenantId } });
    const body = await response.json();
    if (!response.ok) throw new Error('SMS settings unavailable');
    const result = parseSnapshot(body, expected);
    if (epoch !== generation.current) return;
    setSnapshot(result); setPhone(result.phone ?? result.challenge?.phone ?? '');
    setSendHeld(result.challenge !== null && ['sending', 'unknown'].includes(result.challenge.state));
    return result;
  }, []);
  useEffect(() => {
    const epoch = ++generation.current;
    const invalidate = () => { generation.current++; owner.current = null; busy.current = true; setPending(false); setSnapshot(null); setPhone(''); setOtp(''); setMessage('Your session changed. Reload to verify SMS settings.'); };
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) invalidate(); };
    window.addEventListener('omnisolo_auth_changed', invalidate); window.addEventListener('storage', storage);
    let expiry: ReturnType<typeof setTimeout> | undefined;
    void (async () => {
      try {
        const response = await fetch('/api/v1/auth/session-identity', { credentials: 'same-origin', cache: 'no-store' });
        const data = record(await response.json());
        if (!response.ok || typeof data.userId !== 'string' || !data.userId || typeof data.tenantId !== 'string' || !data.tenantId || typeof data.expiresAt !== 'number' || !Number.isSafeInteger(data.expiresAt) || data.expiresAt <= Date.now()) throw new Error('Identity unavailable');
        if (epoch !== generation.current) return;
        const expected = data as Owner; owner.current = expected;
        expiry = setTimeout(invalidate, Math.min(expected.expiresAt - Date.now(), 2_147_483_647));
        await read(expected, epoch);
        if (epoch === generation.current) setMessage('');
      } catch { if (epoch === generation.current) { setSnapshot(null); setMessage('SMS settings could not be verified. Reload to check the saved state.'); } }
    })();
    return () => { generation.current++; clearTimeout(expiry); window.removeEventListener('omnisolo_auth_changed', invalidate); window.removeEventListener('storage', storage); };
  }, [read]);
  async function post(path: string, payload: unknown, expected: Owner) {
    const response = await fetch(`/api/v1/settings/${path}`, { method: 'POST', credentials: 'same-origin', headers: { 'Content-Type': 'application/json', 'x-ohc-expected-user': expected.userId, 'x-ohc-expected-tenant': expected.tenantId }, body: JSON.stringify(payload) });
    const data = await response.json();
    if (response.status !== 200) throw new Error('SMS request rejected');
    return receipt(data, expected);
  }
  async function verify() {
    if (busy.current || !owner.current || !snapshot?.provider_configured || sendHeld || !phoneNumber(phone)) return;
    const expected = owner.current, epoch = generation.current, requestId = crypto.randomUUID();
    busy.current = true; setPending(true); setMessage('Requesting a verification SMS…');
    try {
      const data = await post('sms-verify', { phone, request_id: requestId }, expected);
      if (!active(epoch)) return;
      if (data.status !== 'provider_accepted' || data.challenge_id !== requestId || data.phone !== phone || typeof data.expires_at !== 'number' || !Number.isSafeInteger(data.expires_at) || data.expires_at * 1000 <= Date.now()) throw new Error('SMS acceptance was not acknowledged');
      setSnapshot(current => current && ({ ...current, challenge: { challenge_id: requestId, phone, state: 'accepted', expires_at: data.expires_at as number } }));
      setMessage('The SMS provider accepted your verification message. Delivery is not yet confirmed.');
    } catch { if (active(epoch)) { setSendHeld(true); setMessage('SMS request could not be confirmed. Check its saved status before requesting another code.'); } }
    finally { if (active(epoch)) { busy.current = false; setPending(false); } }
  }
  async function confirm() {
    const challenge = snapshot?.challenge;
    if (busy.current || !owner.current || !challenge || !/^\d{6}$/.test(otp)) return;
    const expected = owner.current, epoch = generation.current;
    busy.current = true; setPending(true);
    try {
      const data = await post('sms-confirm', { phone: challenge.phone, challenge_id: challenge.challenge_id, otp }, expected);
      const confirmed = parseSnapshot(data, expected);
      if (confirmed.status !== 'verified' || confirmed.phone !== challenge.phone || confirmed.verification_id !== challenge.challenge_id) throw new Error('Unconfirmed verification');
      if (!active(epoch)) return;
      setSnapshot(confirmed); setPhone(confirmed.phone!); setOtp(''); setMessage('Phone number verified.');
    } catch { if (active(epoch)) setMessage('The code could not be verified. It may be invalid or expired. Check the saved status if a response was lost.'); }
    finally { if (active(epoch)) { busy.current = false; setPending(false); } }
  }
  async function save(key: keyof Preferences, checked: boolean) {
    if (busy.current || !owner.current || snapshot?.status !== 'verified') return;
    const expected = owner.current, epoch = generation.current, wanted = { ...snapshot.preferences, [key]: checked };
    busy.current = true; setPending(true);
    try {
      const data = await post('sms-preferences', { phone: snapshot.phone, verification_id: snapshot.verification_id, ...wanted }, expected);
      const result = parseSnapshot(data, expected);
      if (result.status !== 'verified' || result.phone !== snapshot.phone || result.verification_id !== snapshot.verification_id || Object.keys(wanted).some(key => wanted[key as keyof Preferences] !== result.preferences[key as keyof Preferences])) throw new Error('Preference not acknowledged');
      if (active(epoch)) { setSnapshot(result); setMessage('SMS preferences saved.'); }
    } catch {
      if (active(epoch)) {
        try { await read(expected, epoch); } catch { if (active(epoch)) setSnapshot(null); }
        if (active(epoch)) setMessage('SMS preference change could not be confirmed. The saved status was checked; reload if unavailable.');
      }
    } finally { if (active(epoch)) { busy.current = false; setPending(false); } }
  }
  const verified = snapshot?.status === 'verified';
  const challenge = snapshot?.challenge;
  const canConfirm = !verified && challenge?.state === 'accepted' && challenge.expires_at * 1000 > Date.now();
  return <section className="app-panel glassmorphism overflow-hidden" aria-labelledby="sms-settings-title">
    <div className="app-panel-header px-6 py-4"><h3 id="sms-settings-title" className="font-bold">Critical SMS Alerts</h3><p className="text-sm">Verify your own phone before choosing business alerts.</p></div>
    <div className="app-panel-body p-6 space-y-4">
      <label className="block" htmlFor="sms-phone">Mobile Number</label>
      <input id="sms-phone" aria-label="Mobile Number" type="tel" autoComplete="tel" className="rounded-xl border px-4 py-3" value={phone} onChange={e => setPhone(e.target.value)} disabled={!snapshot || pending || verified || Boolean(canConfirm) || sendHeld} />
      {!verified && !canConfirm && <button type="button" className="app-button min-h-[44px]" disabled={!snapshot?.provider_configured || pending || sendHeld || !phoneNumber(phone)} onClick={() => void verify()}>Verify Number</button>}
      {!snapshot?.provider_configured && snapshot && <p>SMS verification is unavailable because the provider is not configured.</p>}
      {message && <p role="status">{message}</p>}
      {sendHeld && <p>Another SMS will not be sent while the previous provider outcome is unknown.</p>}
      {canConfirm && <div className="flex gap-3"><input aria-label="Verification code" inputMode="numeric" autoComplete="one-time-code" maxLength={6} value={otp} onChange={e => setOtp(e.target.value.replace(/\D/g, ''))} disabled={pending} className="rounded-xl border px-4 py-3" /><button type="button" disabled={pending || otp.length !== 6} className="app-button min-h-[44px]" onClick={() => void confirm()}>Confirm OTP</button></div>}
      {verified && <p>✓ Number Verified</p>}
      <div className="grid gap-4 sm:grid-cols-2">{([['urgent_booking', 'Urgent Bookings'], ['failed_payment', 'Failed Payments'], ['new_order', 'New Orders']] as const).map(([key, label]) => <label key={key} className="flex gap-3"><input type="checkbox" aria-label={label} aria-describedby={key === 'new_order' ? 'sms-new-order-availability' : undefined} checked={snapshot?.preferences[key] ?? false} disabled={!verified || pending} onChange={e => void save(key, e.target.checked)} />{label}</label>)}</div>
      <p id="sms-new-order-availability" className="text-sm">You can save this preference. Automatic new-order SMS is currently unavailable until orders have a confirmed, saved receipt.</p>
      <button type="button" className="app-button min-h-[44px]" disabled={pending || !owner.current} onClick={() => {
        if (!owner.current || busy.current) return;
        const expected = owner.current, epoch = generation.current; busy.current = true; setPending(true);
        void read(expected, epoch).then(() => { if (active(epoch)) setMessage('Saved SMS status checked.'); }).catch(() => { if (active(epoch)) { setSnapshot(null); setMessage('SMS settings could not be verified. Reload to check the saved state.'); } }).finally(() => { if (active(epoch)) { busy.current = false; setPending(false); } });
      }}>Check saved SMS status</button>
    </div>
  </section>;
}
