'use client';
import { useCallback, useEffect, useRef, useState } from 'react';
import { sameOwner } from '@/lib/sync/queueIdentity';
import { fetchForOwnedAssistant, fetchForOwnedBusinessRead, onboardingOwner, onboardingSessionEpoch, openOnboardingSession, readOwnedOnboardingItem, subscribeOnboardingInvalidation, writeOwnedOnboardingItem, type DraftOwner } from '../onboarding/draftSession';
import type { AssistantTask } from './taskTypes';
const MARKER = 'assistant-text-request-v1';
const UNKNOWN = 'Acceptance is unconfirmed. Your request is held to prevent duplicate execution. Check acceptance or retry the same request.';
const uuid = /^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/;
type Pending = { requestId: string; path: string; method: 'POST' | 'PATCH'; body: string };
const record = (value: unknown): Record<string, unknown> | null => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string,unknown> : null;
function marker(value: string | null): Pending | null {
  if (!value) return null;
  const item = record(JSON.parse(value));
  if (!item || !uuid.test(String(item.requestId)) || typeof item.body !== 'string' || item.body.length > 100_000
    || !((item.path === '/api/v1/assistant/tasks' && item.method === 'POST') || (typeof item.path === 'string' && /^\/api\/v1\/assistant\/tasks\/[a-f0-9-]{36}$/.test(item.path) && item.method === 'PATCH'))) throw new Error('Unresolved request marker');
  return item as Pending;
}
export function assistantTask(value: unknown): AssistantTask | null {
  const task = record(value), execution = record(task?.execution);
  if (!task || typeof task.id !== 'string' || !task.id || typeof task.title !== 'string' || typeof task.workspace !== 'string' || typeof task.currentStep !== 'string' || !Array.isArray(task.messages) || !Array.isArray(task.artifacts) || !Array.isArray(task.changes)) return null;
  if (task.legacy === true) return task.status === 'blocked' && task.execution === null ? task as AssistantTask : null;
  const states:Record<string,string> = {queued:'queued',dispatching:'running',completed:'completed',cancelled:'cancelled',outcome_unknown:'outcome_unknown'};
  if (!execution || !uuid.test(String(execution.id)) || !uuid.test(String(execution.request_id)) || states[String(execution.phase)] !== task.status
    || (execution.phase === 'completed' ? typeof execution.output !== 'string' || !execution.output.trim() || task.output !== execution.output : execution.output != null || task.output != null)) return null;
  return task as AssistantTask;
}
export function useAssistantExecution(onRetire: () => void, onReceipt: (task: AssistantTask) => void, onRestore: (draft: Record<string,unknown>) => void) {
  const [ready,setReady] = useState(false), [configured,setConfigured] = useState(false), [busy,setBusy] = useState(false), [held,setHeld] = useState(false), [notice,setNotice] = useState('Checking the signed session and text runtime…');
  const [revision,setRevision] = useState(0);
  const owner = useRef<DraftOwner|null>(null), generation = useRef(0), mounted = useRef(false), inFlight = useRef(false), pending = useRef<Pending|null>(null), controller = useRef<AbortController|null>(null);
  const callbacks = useRef({onRetire,onReceipt,onRestore}); callbacks.current = {onRetire,onReceipt,onRestore};
  const current = useCallback((expected:DraftOwner,token:number,epoch:number) => mounted.current && generation.current === token && onboardingSessionEpoch() === epoch && !!owner.current && !!onboardingOwner() && sameOwner(owner.current,expected) && sameOwner(onboardingOwner()!,expected),[]);
  const acknowledge = useCallback((value:unknown, expected:DraftOwner, saved:Pending):boolean => {
    const envelope = record(value), task = assistantTask(envelope?.task);
    const retirePending = () => {
      const latest = readOwnedOnboardingItem(MARKER);
      if (latest && latest !== JSON.stringify(saved)) {
        pending.current = marker(latest); setHeld(true); return false;
      }
      // A missing marker is not proof by itself. Call this only after the
      // authenticated response confirms this exact saved request below.
      if (latest) writeOwnedOnboardingItem(MARKER,'');
      pending.current = null; setHeld(false); return true;
    };
    const cancelled = record(envelope?.cancelled_request);
    if (envelope?.effect === 'none' && cancelled?.phase === 'cancelled' && cancelled.request_id === saved.requestId && cancelled.tenant_id === expected.tenantId && cancelled.actor_id === expected.userId) {
      if (!retirePending()) return false;
      setNotice('This request was cancelled before dispatch. You can submit a new text task.'); return true;
    }
    if (!task?.execution || task.execution.tenant_id !== expected.tenantId || task.execution.actor_id !== expected.userId
      || ![task.execution.request_id,task.execution.root_request_id,task.matchedRequestId].includes(saved.requestId)) return false;
    // A late read for a prior operation must never retire a newer tab's marker.
    // Storage failure retains the original hold; it cannot cause a new request.
    if (!retirePending()) return false;
    setNotice(`Task accepted. Current status: ${task.status}.`); callbacks.current.onReceipt(task); return true;
  },[]);
  const recover = useCallback(async () => {
    const expected = owner.current, saved = pending.current, token = generation.current, epoch = onboardingSessionEpoch();
    if (!expected || !saved || inFlight.current) return;
    inFlight.current = true; setBusy(true);
    const abort = new AbortController(); controller.current = abort;
    const timeout = window.setTimeout(() => abort.abort(), 30_000);
    try {
      if (!navigator.locks?.request) { setNotice(UNKNOWN); return; }
      await navigator.locks.request('ohc-assistant:'+JSON.stringify([expected.userId,expected.tenantId]),{mode:'exclusive',signal:abort.signal},async()=>{
        if (!current(expected,token,epoch) || abort.signal.aborted) return;
        // Another tab may already have acknowledged this request or started a
        // newer one. Synchronize under the same lock, then verify the server.
        const active = marker(readOwnedOnboardingItem(MARKER)) ?? saved;
        pending.current = active;
        const response = await fetchForOwnedAssistant(`/api/v1/assistant/tasks/by-request/${active.requestId}`,{signal:abort.signal},expected);
        const data:unknown = await response.json();
        if (!current(expected,token,epoch) || abort.signal.aborted) return;
        if (!response.ok || !acknowledge(data,expected,active)) setNotice(UNKNOWN);
      });
    } catch {if(current(expected,token,epoch))setNotice(UNKNOWN);}
    finally {window.clearTimeout(timeout);if(controller.current===abort)controller.current=null;if(current(expected,token,epoch)){inFlight.current=false;setBusy(false);}}
  },[acknowledge,current]);
  useEffect(() => {
    mounted.current = true;
    const clear = () => {generation.current++;owner.current=null;controller.current?.abort();pending.current=null;inFlight.current=false;setReady(false);setConfigured(false);setBusy(false);setHeld(false);callbacks.current.onRetire();};
    const verify = async () => {
      const token=generation.current,epoch=onboardingSessionEpoch();
      try {
        const expected=await openOnboardingSession();
        if(!mounted.current||generation.current!==token||onboardingSessionEpoch()!==epoch)return;
        owner.current=expected;
        const response=await fetchForOwnedBusinessRead('/api/v1/agents/execution-policy',expected);
        const policy=record(await response.json());
        if(!current(expected,token,epoch))return;
        const available=response.ok&&policy?.available===true&&policy.mode==='text_analysis'&&policy.workspace_access===false&&Array.isArray(policy.tools)&&policy.tools.length===0&&record(policy.policy)!==null;
        setReady(true);setConfigured(available);setNotice(available?'One text response using the configured model. No file, coding, browsing, or delegation tools.':'Text execution is unavailable. Existing receipts can still be read.');
        const stored=readOwnedOnboardingItem(MARKER);
        if(stored){setHeld(true);setNotice(UNKNOWN);pending.current=marker(stored);if(pending.current?.method==='POST'){const draft=record(JSON.parse(pending.current.body));if(draft)callbacks.current.onRestore(draft);}await recover();}
        if(current(expected,token,epoch))setRevision(value=>value+1);
      } catch {if(mounted.current&&generation.current===token&&onboardingSessionEpoch()===epoch)setNotice('Could not verify this session or request. No new task can be submitted.');}
    };
    void verify();
    const unsubscribe=subscribeOnboardingInvalidation(restart=>{clear();setNotice('Your session changed. The prior task view was cleared.');if(restart)void verify();});
    return()=>{mounted.current=false;generation.current++;owner.current=null;controller.current?.abort();unsubscribe();};
  },[current,recover]);
  useEffect(() => {
    if (!ready || !held) return;
    const timer = window.setInterval(() => { void recover(); }, 3000);
    return () => window.clearInterval(timer);
  }, [ready, held, recover]);
  const read = useCallback(async(path:string):Promise<unknown>=>{
    const expected=owner.current,token=generation.current,epoch=onboardingSessionEpoch();
    if(!expected||!current(expected,token,epoch))throw new Error('Session unavailable');
    const response=await fetchForOwnedAssistant(path,{},expected);const data:unknown=await response.json();
    if(!current(expected,token,epoch))throw new Error('Your session changed. This reply was not applied.');
    if(!response.ok)throw new Error(typeof record(data)?.error === 'string' ? String(record(data)?.error) : 'Assistant records unavailable');return data;
  },[current]);
  const action = useCallback(async(path:string,method:'POST'|'PATCH',body:Record<string,unknown>):Promise<unknown>=>{
    const expected=owner.current,token=generation.current,epoch=onboardingSessionEpoch();
    if(!expected||!current(expected,token,epoch))throw new Error('Session unavailable');
    const response=await fetchForOwnedAssistant(path,{method,headers:{'Content-Type':'application/json'},body:JSON.stringify(body)},expected);
    const data:unknown=await response.json();
    if(!current(expected,token,epoch))throw new Error('Your session changed. This reply was not applied.');
    if(!response.ok)throw new Error(typeof record(data)?.error==='string'?String(record(data)?.error):'Action unconfirmed');
    return data;
  },[current]);
  const submit = async (path:string,method:'POST'|'PATCH',body:Record<string,unknown>,retry=false) => {
    const expected=owner.current,token=generation.current,epoch=onboardingSessionEpoch();
    if(!expected||!ready||!configured||inFlight.current||(!retry&&held))return;
    if (!retry && method === 'POST') {
      const prompt = typeof body.prompt === 'string' ? body.prompt : '';
      const constraints = typeof body.constraints === 'string' ? body.constraints : '';
      const workspace = typeof body.workspace === 'string' ? body.workspace : '';
      const input = constraints.trim() ? `${prompt}\n\nAdditional user constraints:\n${constraints}` : prompt;
      if (!prompt.trim() || !workspace.trim() || [...workspace].length > 80 || [...input].length > 16000 || [prompt,constraints,workspace].some(text => text.includes('\0'))) {
        setNotice('Use a workspace name of 1–80 characters and no more than 16,000 characters of task text and constraints. No task was submitted.'); return;
      }
    }
    if(!navigator.locks?.request){setNotice('Safe task submission is unavailable in this browser. No request was sent.');return;}
    inFlight.current=true;setBusy(true);
    const abort=new AbortController();controller.current=abort;const timeout=window.setTimeout(()=>abort.abort(),30_000);
    try {
      await navigator.locks.request('ohc-assistant:'+JSON.stringify([expected.userId,expected.tenantId]),{mode:'exclusive',signal:abort.signal},async()=>{
        if(!current(expected,token,epoch)||abort.signal.aborted)return;
        const stored=marker(readOwnedOnboardingItem(MARKER));
        if(stored&&!retry){pending.current=stored;setHeld(true);setNotice(UNKNOWN);return;}
        if(retry&&(!stored||stored.requestId!==pending.current?.requestId)){if(stored)pending.current=stored;setNotice(UNKNOWN);return;}
        const saved=stored??{requestId:crypto.randomUUID(),path,method,body:JSON.stringify(body)};
        const response=await fetchForOwnedAssistant(saved.path,{method:saved.method,headers:{'Content-Type':'application/json','Idempotency-Key':saved.requestId},body:saved.body,signal:abort.signal},expected,()=>{
          if(!current(expected,token,epoch)||abort.signal.aborted)throw new Error('Session changed');
          writeOwnedOnboardingItem(MARKER,JSON.stringify(saved));pending.current=saved;setHeld(true);
        });
        const data:unknown=await response.json();
        if(!current(expected,token,epoch)||abort.signal.aborted)return;
        const rejected = record(data);
        if ((response.status === 400 || response.status === 409) && rejected?.effect === 'none' && (rejected.rejected_before_admission === true || (rejected.rejected_before_dispatch === true && rejected.rejected_request_id === saved.requestId))) {
          if (readOwnedOnboardingItem(MARKER) !== JSON.stringify(saved)) { setNotice(UNKNOWN); return; }
          writeOwnedOnboardingItem(MARKER,''); pending.current = null; setHeld(false);
          setNotice(typeof rejected.error === 'string' ? rejected.error : 'The request was rejected before admission. Edit it and try again.');
        } else if(!response.ok||!acknowledge(data,expected,saved))setNotice(UNKNOWN);
      });
    }catch{if(current(expected,token,epoch))setNotice(pending.current?UNKNOWN:'The request could not be prepared. No task was submitted.');}
    finally{window.clearTimeout(timeout);if(controller.current===abort)controller.current=null;if(current(expected,token,epoch)){inFlight.current=false;setBusy(false);}}
  };
  const mutate = async(task:AssistantTask,action:'stop'|'archive'|'unarchive'|'resume')=>{
    if(!task.execution||task.legacy)return;
    const body={action,sourceReceiptId:task.execution.id};
    if(action==='resume')return submit(`/api/v1/assistant/tasks/${task.id}`,'PATCH',body);
    const expected=owner.current,token=generation.current,epoch=onboardingSessionEpoch();
    if(!expected||inFlight.current||!current(expected,token,epoch))return;
    inFlight.current=true;setBusy(true);
    try{
      const response=await fetchForOwnedAssistant(`/api/v1/assistant/tasks/${task.id}`,{method:'PATCH',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)},expected);
      const data=record(await response.json());if(!current(expected,token,epoch))return;
      const updated=assistantTask(data?.task);if(!response.ok||!updated)throw new Error('unconfirmed');
      callbacks.current.onReceipt(updated);setNotice(`Current task status: ${updated.status}${updated.archived?' (archived)':''}.`);
    }catch{if(current(expected,token,epoch))setNotice('The change is unconfirmed. Refresh the task before trying again.');}
    finally{if(current(expected,token,epoch)){inFlight.current=false;setBusy(false);}}
  };
  return {ready,configured,busy,held,notice,revision,read,action,recover,mutate,start:(body:Record<string,unknown>)=>submit('/api/v1/assistant/tasks','POST',body),retry:()=>submit('', 'POST',{},true)};
}
