'use client';
import { useCallback, useEffect, useRef, useState } from 'react';
import { AppShell } from '../components/AppShell';
import { readQueueOwner, sameOwner, hasVerifiedOfflineQueueOwner, subscribeQueueIdentityReadiness } from '@/lib/sync/queueIdentity';
type Owner = {
    userId: string;
    tenantId: string;
    expiresAt: number;
};
type Appointment = {
    id: string;
    customer_id: string;
    customer_name: string;
    job_template_id: string;
    job_name: string;
    status: string;
    scheduled_start_time: string | null;
    scheduled_end_time: string | null;
    notes?: string | null;
};
type Phase = 'loading' | 'ready' | 'error' | 'held';
const validTime = (value: unknown) => value === null || (typeof value === 'string' && Number.isFinite(Date.parse(value)));
function appointmentsFrom(data: unknown): Appointment[] {
    if (!data || typeof data !== 'object' || Array.isArray(data))
        throw new Error('Invalid appointments');
    const receipt = data as Record<string, unknown>;
    if (receipt.error != null || ('success' in receipt && receipt.success !== true) || !Array.isArray(receipt.appointments))
        throw new Error('Invalid appointments');
    const ids = new Set<string>();
    for (const row of receipt.appointments) {
        if (!row || typeof row !== 'object' || ['id', 'customer_id', 'customer_name', 'job_template_id', 'job_name', 'status'].some(key => typeof row[key] !== 'string') || !row.id || ids.has(row.id) || !validTime(row.scheduled_start_time) || !validTime(row.scheduled_end_time) || (row.notes != null && typeof row.notes !== 'string'))
            throw new Error('Invalid appointment record');
        ids.add(row.id);
    }
    return receipt.appointments;
}
function ownerFrom(data: unknown): Owner {
    if (!data || typeof data !== 'object' || Array.isArray(data))
        throw new Error('Identity unavailable');
    const owner = data as Owner & {
        error?: unknown;
        success?: unknown;
    };
    if (owner.error != null || ('success' in owner && owner.success !== true) || typeof owner.userId !== 'string' || !owner.userId || typeof owner.tenantId !== 'string' || !owner.tenantId || !Number.isFinite(owner.expiresAt) || owner.expiresAt <= Date.now())
        throw new Error('Identity unavailable');
    return owner;
}
export default function OperationsPage() {
    const [appointments, setAppointments] = useState<Appointment[]>([]);
    const [phase, setPhase] = useState<Phase>('loading');
    const [status, setStatus] = useState('Loading verified appointments…');
    const [identityReady, setIdentityReady] = useState(false);
    const scope = useRef<Owner | null>(null);
    const epoch = useRef(0);
    const active = useRef(false);
    const request = useRef<AbortController | null>(null);
    const expiry = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
    const retire = useCallback(() => {
        ++epoch.current;
        request.current?.abort();
        clearTimeout(expiry.current);
        scope.current = null;
        setIdentityReady(false);
        setAppointments([]);
        setPhase('held');
        setStatus('Your session changed. Reload to verify appointment access.');
    }, []);
    const refreshReadiness = useCallback(() => {
        const expected = scope.current;
        const ready = hasVerifiedOfflineQueueOwner(expected);
        setIdentityReady(ready);
        if (expected && hasVerifiedOfflineQueueOwner() && !ready)
            retire();
    }, [retire]);
    const load = useCallback(async () => {
        const operation = ++epoch.current;
        scope.current = null;
        setIdentityReady(false);
        request.current?.abort();
        clearTimeout(expiry.current);
        const controller = new AbortController();
        request.current = controller;
        setAppointments([]);
        setPhase('loading');
        setStatus('Loading verified appointments…');
        const current = () => active.current && operation === epoch.current;
        try {
            const identityResponse = await fetch('/api/v1/auth/session-identity', { credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: controller.signal });
            if (identityResponse.status !== 200)
                throw new Error('Identity unavailable');
            const owner = ownerFrom(await identityResponse.json());
            if (!current())
                return;
            const headers = new Headers({ 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId });
            if (headers.get('x-ohc-expected-user') !== owner.userId || headers.get('x-ohc-expected-tenant') !== owner.tenantId)
                throw new Error('Identity cannot be represented');
            expiry.current = setTimeout(retire, Math.min(owner.expiresAt - Date.now(), 2147483647));
            scope.current = owner;
            // Preserve strict identity receipt validation above, then establish
            // the same owner in the shared readiness lifecycle before reading.
            const canonical = await readQueueOwner();
            if (!current())
                return;
            if (!sameOwner(owner, canonical)) {
                retire();
                return;
            }
            refreshReadiness();
            if (!current())
                return;
            const response = await fetch('/api/v1/field-ops/appointments', { headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: controller.signal });
            if (!current())
                return;
            if (response.status === 401 || response.status === 403) {
                retire();
                return;
            }
            const data = await response.json();
            if (!current())
                return;
            if (response.status === 409 && (data?.error === 'session_identity_changed' || data?.error === 'queued owner does not match the current session')) {
                retire();
                return;
            }
            if (response.status !== 200)
                throw new Error('Appointments unavailable');
            refreshReadiness();
            if (!current())
                return;
            const records = appointmentsFrom(data);
            setAppointments(records);
            setPhase('ready');
            setStatus('Recorded appointments loaded.');
        }
        catch {
            if (current()) {
                setAppointments([]);
                setPhase('error');
                setStatus('Appointments could not be loaded. Try again to read current records.');
            }
        }
    }, [retire, refreshReadiness]);
    useEffect(() => {
        active.current = true;
        const unsubscribe = subscribeQueueIdentityReadiness(refreshReadiness);
        const storage = (event: StorageEvent) => {
            if (event.key === null || event.key === 'omnisolo_queue_identity_epoch_v2')
                retire();
        };
        window.addEventListener('omnisolo_auth_changed', retire);
        window.addEventListener('pagehide', retire);
        window.addEventListener('storage', storage);
        void load();
        return () => { unsubscribe(); scope.current = null; active.current = false; ++epoch.current; request.current?.abort(); clearTimeout(expiry.current); window.removeEventListener('omnisolo_auth_changed', retire); window.removeEventListener('pagehide', retire); window.removeEventListener('storage', storage); };
    }, [load, retire, refreshReadiness]);
    return <AppShell title="Operations Copilot">
    <div className="p-4 sm:p-6 lg:p-8 max-w-7xl mx-auto space-y-6">
      <header>
        <h1 className="text-3xl font-bold">Appointment schedule</h1>
        <p className="mt-2 text-gray-600 dark:text-gray-300">Recorded appointments for your verified business.</p>
      </header>
      <p role={phase === 'error' ? 'alert' : 'status'} aria-label="Appointment status">{phase === 'ready' && !identityReady ? 'Appointment access is temporarily unverified. Reload to check again.' : status}</p>
      {phase === 'error' && <button onClick={() => void load()} className="px-4 py-2 rounded bg-blue-600 text-white">Retry loading appointments</button>}
      {phase === 'ready' && identityReady && <>
        <section className="glassmorphism p-6 border border-white/40 dark:border-white/10 shadow-sm">
          <h2 className="text-lg font-semibold">Schedule overview</h2>
          <p>{appointments.length} recorded {appointments.length === 1 ? 'appointment' : 'appointments'}.</p>
          <p className="text-sm text-gray-600 dark:text-gray-300">Times are shown in your browser timezone. Appointment status does not verify a payment or a reminder.</p>
        </section>
        {appointments.length === 0 ? <p>No appointments recorded for this business.</p> : <ul aria-label="Recorded appointments" className="space-y-4">
          {appointments.map(appointment => <li key={appointment.id} data-testid={`appointment-${appointment.id}`} className="glassmorphism p-4 border border-white/40 dark:border-white/10 shadow-sm">
            <h3 className="font-semibold">{appointment.job_name || 'Service details unavailable'}</h3>
            <p>{appointment.customer_name || 'Customer details unavailable'}</p>
            <p>{appointment.scheduled_start_time ? <time dateTime={appointment.scheduled_start_time}>{new Date(appointment.scheduled_start_time).toLocaleString()}</time> : 'Time not scheduled'}</p>
            {appointment.scheduled_end_time && <p>Ends: <time dateTime={appointment.scheduled_end_time}>{new Date(appointment.scheduled_end_time).toLocaleString()}</time></p>}
            <p>Status: <span>{appointment.status || 'Not recorded'}</span></p>
            {appointment.notes && <div><h4 className="font-medium">Appointment notes</h4><p className="whitespace-pre-wrap">{appointment.notes}</p></div>}
          </li>)}
        </ul>}
        <p className="text-sm text-gray-600 dark:text-gray-300">Messaging and reminder dispatch are not available from this schedule.</p>
      </>}
    </div>
  </AppShell>;
}
