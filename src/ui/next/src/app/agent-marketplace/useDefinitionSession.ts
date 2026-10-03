'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { assertBuilderScope, builderDraftKey, openBuilderEditor, openBuilderScope, releaseBuilderEditor, type BuilderScope } from '../builder/ownedDraft';
import { subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { executeDefinitionOperation, readSavedDefinitionOperation, recoverDefinitionOperation, type SavedOperation } from './definitionOperations';
import type { Operation, Receipt } from './definitions';

type Options = { editor?: string; opened?: (scope: BuilderScope) => void; retired?: () => void };
export function useDefinitionSession(options: Options = {}) {
  const callbacks = useRef(options); callbacks.current = options;
  const scope = useRef<BuilderScope | null>(null);
  const generation = useRef(0); const busy = useRef(false);
  const [version, setVersion] = useState(0);
  const [phase, setPhase] = useState<'verifying' | 'ready' | 'working' | 'held'>('verifying');
  const [saved, setSaved] = useState<SavedOperation | null>(null);
  const [message, setMessage] = useState('Verifying marketplace access…');
  const editor = options.editor;
  useEffect(() => {
    let mounted = true;
    const retire = () => {
      ++generation.current; busy.current = false; releaseBuilderEditor(scope.current); scope.current = null;
      setSaved(null); setPhase('held'); setMessage('Your session changed. Verify access before continuing.'); callbacks.current.retired?.();
    };
    const open = async () => {
      const current = ++generation.current; const active = () => mounted && current === generation.current;
      setPhase('verifying');
      try {
        const next = editor ? await openBuilderEditor(editor, active) : await openBuilderScope();
        if (!active()) { releaseBuilderEditor(next); return; }
        scope.current = next; callbacks.current.opened?.(next);
        let previous = await readSavedDefinitionOperation(next);
        if (!active()) return;
        if (previous && previous.state !== 'rejected') {
          setSaved(previous);
          previous = await recoverDefinitionOperation(next);
          if (!active()) return;
        }
        setSaved(previous); setPhase(previous?.state === 'pending' ? 'held' : 'ready');
        setMessage(previous?.state === 'pending' ? 'A previous request is unconfirmed. Check its saved status before submitting another request.' : previous?.state === 'confirmed' ? 'The previous request has a verified saved receipt.' : 'Verified marketplace access. Review all fields before submitting.');
        setVersion(value => value + 1);
      } catch (error) {
        if (active()) { setPhase('held'); setMessage(error instanceof Error ? error.message : 'Marketplace access is unavailable.'); }
      }
    };
    const unsubscribe = subscribeOnboardingInvalidation(restart => { retire(); if (restart && mounted) void open(); });
    const storage = (event: StorageEvent) => {
      const currentScope = scope.current;
      if (!currentScope) return;
      try {
        if (event.key !== null && event.key !== builderDraftKey('agent-definition-operation', currentScope)) return;
        // Another view may have dispatched a request. The operation gate rechecks
        // the exact persisted record; this view requires an explicit status read.
        setPhase('held'); setMessage('The saved operation changed in another view. Check its status.');
        void readSavedDefinitionOperation(currentScope).then(value => {
          if (mounted && scope.current === currentScope) setSaved(value);
        }).catch(() => {});
      } catch { retire(); }
    };
    window.addEventListener('storage', storage); void open();
    return () => { mounted = false; ++generation.current; releaseBuilderEditor(scope.current); scope.current = null; unsubscribe(); window.removeEventListener('storage', storage); };
  }, [editor]);

  const perform = useCallback(async (operation?: Operation, replay = false): Promise<Receipt | null> => {
    const currentScope = scope.current; if (!currentScope || busy.current) return null;
    const current = generation.current; const active = () => current === generation.current && scope.current === currentScope;
    busy.current = true; setPhase('working'); setMessage(operation ? 'Waiting for the saved operation receipt…' : 'Checking the saved operation receipt…');
    try {
      assertBuilderScope(currentScope);
      const receipt = operation ? await executeDefinitionOperation(currentScope, operation, replay, active) : null;
      const next = operation ? await readSavedDefinitionOperation(currentScope) : await recoverDefinitionOperation(currentScope);
      if (!active()) return null;
      setSaved(next); setPhase(next?.state === 'pending' ? 'held' : 'ready');
      setMessage(next?.state === 'pending' ? 'The original request remains unconfirmed. No new request was sent.' : next?.state === 'confirmed' ? 'The request has a verified saved receipt.' : 'No pending request was found. Review a new request before submitting.');
      return receipt ?? next?.receipt ?? null;
    } catch (error) {
      if (!active()) return null;
      try {
        const next = await readSavedDefinitionOperation(currentScope);
        if (!active()) return null;
        setSaved(next); setPhase(next?.state === 'rejected' ? 'ready' : 'held');
      } catch { if (active()) setPhase('held'); }
      if (active()) setMessage(error instanceof Error ? error.message : 'The request could not be confirmed.');
      return null;
    } finally { if (active()) busy.current = false; }
  }, []);
  return { scope, version, phase, saved, message, perform, setMessage };
}
