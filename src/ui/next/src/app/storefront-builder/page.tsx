"use client";
import { PublicationPanel } from '../builder/PublicationPanel';
import { layoutPublicationSnapshot } from '../builder/layoutPublicationSnapshot';
import { invalidateOnboardingSession } from '../onboarding/draftSession';
import type { BuilderBlock, BlockProperties } from '@/lib/builder-types';

import { useState, useEffect, useRef } from "react";
import { openBuilderEditor, releaseBuilderEditor, builderScopeActive, assertBuilderEditor, readBuilderDraft, writeBuilderDraft, hasHeldBuilderLegacy, publishOwnedLayout, type BuilderScope } from '../builder/ownedDraft';
import { readGeneratedBlocks } from '../builder/generatedBlocks';
import { prepareSiteSnapshot } from '../builder/publicationContracts';
import { generationFailureMessage } from '../builder/generationFailure';
import { canonicalRequest } from '../onboarding/contracts';
import { subscribeOnboardingInvalidation, fetchForOwnedBusinessAction } from '../onboarding/draftSession';
import { SmartBlock, DraggableBlock, ActionSheet } from "../builder/components";
import { useWalkthrough } from "../../components/help";
import { WithTooltip } from "../../components/TooltipRegistry";
import { InteractiveWalkthrough, WalkthroughTarget } from "../../components/Walkthrough";

export default function StorefrontBuilderPage() {
  const [bio, setBio] = useState("");
  const [blocks, setBlocks] = useState<BuilderBlock[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live" | "chat">("idle");
  const [liveUrl, setLiveUrl] = useState("");
  const [loaded, setLoaded] = useState(false);
  const [sessionError, setSessionError] = useState('');
  const [heldLegacy, setHeldLegacy] = useState(false);
  const scope = useRef<BuilderScope | null>(null);
  const [viewScope, setViewScope] = useState<BuilderScope | null>(null);
  const operationEpoch = useRef(0);
  const busy = useRef(false);
  type LayoutDraft = { bio: string; blocks: BuilderBlock[]; status: 'idle' | 'draft' | 'chat' };
  const localDraft = useRef<LayoutDraft>({ bio: '', blocks: [], status: 'idle' });

  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [selectedBlockIndex, setSelectedBlockIndex] = useState<number | null>(null);
  const [tenantId, setTenantId] = useState("storefront");
  const [isAddBlockOpen, setIsAddBlockOpen] = useState(false);
  const [editingBlockContent, setEditingBlockContent] = useState<BlockProperties | null>(null);
  const [saveMessage, setSaveMessage] = useState("");
  const [generationError, setGenerationError] = useState("");
  const [isWalkthroughOpen, setIsWalkthroughOpen] = useState(false);

  const walkthroughSteps: import("../../components/Walkthrough").Step[] = [
    { targetId: "storefront-title", title: "Storefront Builder", content: "This is where you can build and customize your storefront." },
    { targetId: "bio-input-target", title: "Storefront Bio", content: "Tell your customers about your store." }
  ];
  const [chatMessage, setChatMessage] = useState("");
  useWalkthrough();

  useEffect(() => {
    if (selectedBlockIndex !== null) {
      setEditingBlockContent(JSON.parse(JSON.stringify(blocks[selectedBlockIndex].props)));
    } else {
      setEditingBlockContent(null);
    }
  }, [selectedBlockIndex]);

  useEffect(() => {
    let disposed = false;
    let loadVersion = 0; let editor: BuilderScope | null = null;
    const load = async () => {
      const version = ++loadVersion; setLoaded(false); setSessionError('');
      try {
        const verified = await openBuilderEditor('storefront-builder-draft');
        if (disposed || version !== loadVersion) { releaseBuilderEditor(verified); return; }
        releaseBuilderEditor(editor); editor = verified;
        const cached = readBuilderDraft<LayoutDraft>('storefront-builder-draft', verified);
        const value = cached?.data ?? { bio: '', blocks: [], status: 'idle' };
        if (typeof value.bio !== 'string' || !Array.isArray(value.blocks) || !['idle','draft','chat'].includes(value.status)) throw new Error('This saved layout could not be read. Its original copy remains held.');
        scope.current = verified; setViewScope(verified); localDraft.current = { ...value, status: value.blocks.length ? 'draft' : 'idle' };
        setBio(value.bio); setBlocks(value.blocks); setStatus(value.blocks.length ? 'draft' : 'idle'); setTenantId(verified.owner.tenantId);
        setHeldLegacy(hasHeldBuilderLegacy()); setLoaded(true);
        setSaveMessage('Layout drafts stay on this device; server layout saving is unavailable.');
      } catch (error) { if (!disposed && version === loadVersion) setSessionError(error instanceof Error ? error.message : 'Verify your session before using the builder.'); }
    };
    const clear = (restart: boolean) => {
      loadVersion += 1; operationEpoch.current += 1; busy.current = false; releaseBuilderEditor(editor); scope.current = null; setViewScope(null);
      localDraft.current = { bio: '', blocks: [], status: 'idle' };
      setBio(''); setBlocks([]); setStatus('idle'); setLiveUrl(''); setTenantId(''); setSelectedBlockIndex(null); setEditingBlockContent(null); setChatMessage(''); setSaveMessage(''); setGenerationError(''); setLoaded(false);
      if (restart) void load(); else setSessionError('Your session could not be verified. Your saved layout remains held.');
    };
    const unsubscribe = subscribeOnboardingInvalidation(clear);
    void load();
    return () => { disposed = true; loadVersion += 1; operationEpoch.current += 1; busy.current = false; releaseBuilderEditor(editor); unsubscribe(); };
  }, []);

  const persistLayout = (change: Partial<LayoutDraft>) => {
    if (!builderScopeActive(viewScope)) return;
    localDraft.current = { ...localDraft.current, ...change };
    try {
      writeBuilderDraft('storefront-builder-draft', localDraft.current, viewScope);
      setSaveMessage('Saved on this device. Server layout draft saving is unavailable.');
    } catch (error) { setSaveMessage(error instanceof Error ? error.message : 'This device could not save your latest edits.'); }
  };

  const updateBio = (newBio: string) => {
    if (!builderScopeActive(viewScope)) return;
    setBio(newBio);
    persistLayout({ bio: newBio });
  };

  const updateStatus = (newStatus: "idle" | "generating" | "draft" | "live" | "chat") => {
    if (!builderScopeActive(viewScope)) return;
    setStatus(newStatus);
    localDraft.current = { ...localDraft.current, status: newStatus === 'live' || newStatus === 'generating' ? (localDraft.current.blocks.length ? 'draft' : 'idle') : newStatus };
  };

  const navigateStatus = (value: 'idle' | 'draft' | 'chat') => { operationEpoch.current += 1; busy.current = false; updateStatus(value); };

  const handleSaveBlock = () => {
    if (!builderScopeActive(viewScope)) return;
    if (selectedBlockIndex !== null && editingBlockContent) {
      const newBlocks = [...blocks];
      newBlocks[selectedBlockIndex] = {
        ...newBlocks[selectedBlockIndex],
        props: editingBlockContent
      };
      setBlocks(newBlocks);
      persistLayout({ blocks: newBlocks });
      setSelectedBlockIndex(null);
    }
  };

  const addBlock = (type: string) => {
    if (!builderScopeActive(viewScope)) return;
    let defaultProps = {};
    if (type === "Hero") defaultProps = { headline: "", copy: "" };
    if (type === "Catalog") defaultProps = { items: [{ name: "", price: "", description: "" }] };
    if (type === "Booking") defaultProps = { title: "", availability: "", booking_url: "" };
    if (type === "Contact") defaultProps = { email: "", phone: "" };
    if (type === "Referral") defaultProps = { offerTitle: "", offerDescription: "" };

    const newBlocks = [...blocks, { type, props: defaultProps }];
    setBlocks(newBlocks);
    persistLayout({ blocks: newBlocks });
    setIsAddBlockOpen(false);
    setSelectedBlockIndex(newBlocks.length - 1);
  };

  const generateLayout = async (description: string, current: BuilderScope, active: () => boolean) => {
    const response = await fetchForOwnedBusinessAction('/api/v1/builder/generate', {
      method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ description }),
    }, current.owner, () => { assertBuilderEditor(current, 'storefront-builder-draft'); if (!active()) throw new Error('Your editor changed before generation.'); });
    const data = await response.json();
    if (!active()) return null;
    if (response.status !== 200) throw new Error(generationFailureMessage(response.status, data));
    const generated = readGeneratedBlocks(data);
    await prepareSiteSnapshot(layoutPublicationSnapshot({ title: 'Home', bio, blocks: generated }));
    if (!active()) return null;
    assertBuilderEditor(current, 'storefront-builder-draft');
    return generated;
  };

  const handleGenerate = async () => {
    const current = viewScope;
    if (!builderScopeActive(current) || busy.current || bio.trim().length <= 5) return;
    busy.current = true; const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current && builderScopeActive(current);
    setGenerationError(''); setStatus('generating');
    try {
      const generated = await generateLayout(bio, current, active);
      if (!generated || !active()) return;
      setBlocks(generated); persistLayout({ blocks: generated }); updateStatus('draft');
    } catch (error) {
      if (!active()) return;
      updateStatus(localDraft.current.blocks.length ? 'draft' : 'idle');
      setGenerationError(error instanceof Error ? error.message : 'The layout could not be generated. Your existing draft was kept.');
    } finally { if (active()) busy.current = false; }
  };

  const moveBlock = (fromIndex: number, toIndex: number) => {
    if (!builderScopeActive(viewScope)) return;
    if (toIndex < 0 || toIndex >= blocks.length || fromIndex === toIndex) return;

    setBlocks(prev => {
      const newBlocks = [...prev];
      const [moved] = newBlocks.splice(fromIndex, 1);
      newBlocks.splice(toIndex, 0, moved);
      persistLayout({ blocks: newBlocks });
      return newBlocks;
    });

    if (selectedBlockIndex === fromIndex) {
      setSelectedBlockIndex(toIndex);
    } else if (selectedBlockIndex === toIndex) {
      setSelectedBlockIndex(fromIndex);
    }
  };

  const handleAgentChat = async () => {
    const current = viewScope;
    if (!builderScopeActive(current) || busy.current || !chatMessage.trim()) return;
    busy.current = true; const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current && builderScopeActive(current);
    setGenerationError(''); setStatus('generating');
    try {
      const generated = await generateLayout(`${bio}. Update request: ${chatMessage}. Note: Maintain a 375px optimized card-based mobile UI.`, current, active);
      if (!generated || !active()) return;
      setBlocks(generated); persistLayout({ blocks: generated }); setChatMessage(''); updateStatus('draft');
    } catch (error) {
      if (!active()) return;
      updateStatus('chat');
      setGenerationError(error instanceof Error ? error.message : 'The layout update could not be confirmed. Your existing draft was kept.');
    } finally { if (active()) busy.current = false; }
  };

  const handleLaunch = async () => {
    const current = viewScope;
    if (!builderScopeActive(current) || busy.current) return;
    busy.current = true; const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current && builderScopeActive(current);
    const layoutFingerprint = canonicalRequest({ blocks, bio });
    try {
      const draftBlocks = blocks.map((b, i) => ({
        block_type: b.type === 'Hero' ? 'HeroBlock' :
                    b.type === 'Catalog' ? 'ProductGridBlock' :
                    b.type === 'Booking' ? 'ServiceBookingBlock' :
                    b.type === 'Testimonials' ? 'TestimonialBlock' : b.type,
        content: b.props,
        sort_order: i
      }));

      const payload = {
          domain: null,
          draft: {
              domain: null,
              pages: [{
                  path: '/',
                  title: 'Home',
                  blocks: draftBlocks,
                  seo_metadata: {
                    "@context": "https://schema.org",
                    "@type": "LocalBusiness",
                    "name": bio
                  }
              }]
          }
      };

      const site = await publishOwnedLayout(current, payload);
      if (active()) setSaveMessage(canonicalRequest({ blocks: localDraft.current.blocks, bio: localDraft.current.bio }) === layoutFingerprint ? `Site saved (${site.id}); publishing has not been verified.` : `Earlier layout saved (${site.id}); newer edits remain local. Publishing has not been verified.`);
    } catch (error) { if (active()) setSaveMessage(error instanceof Error ? error.message : 'Site save could not be confirmed.'); }
    finally { if (active()) busy.current = false; }
  };

  if (sessionError) return <div role="alert">{sessionError}</div>;
  if (!loaded) return <div role="status">Verifying your builder session…</div>;

  const generationNotice = generationError && <p role="alert" className="m-4 rounded-lg bg-red-50 p-3 text-sm text-red-800">{generationError}</p>;

  if (status === "idle") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div id="setup-screen" className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden glassmorphism">
          <div className="absolute top-6 right-8 flex items-center gap-4 z-10">
            {saveMessage && <span className="text-[#34C759] text-sm font-semibold animate-fade-in">{saveMessage}</span>}
          </div>
          <div className="px-8 pb-8 pt-12 flex flex-col flex-1 justify-start overflow-y-auto">
            {generationNotice}
            <InteractiveWalkthrough steps={walkthroughSteps} isOpen={isWalkthroughOpen} onClose={() => setIsWalkthroughOpen(false)} />
            <div className="flex justify-end mb-4"><button id="storefront-walkthrough-btn" onClick={() => setIsWalkthroughOpen(true)} className="px-3 py-1.5 text-sm bg-blue-50 text-blue-600 rounded-lg hover:bg-blue-100 font-semibold transition-colors">Start Tour</button></div>
            <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
              <WalkthroughTarget id="storefront-title">
                <div className="text-xs uppercase tracking-wider text-blue-600 dark:text-blue-400 font-semibold mb-1">Welcome to OmniSolo Smart Builder</div>
                {heldLegacy && <p role="status">An older builder draft remains held on this device.</p>}
                <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2" aria-label="Welcome to OmniSolo OneHumanCorp Smart Builder">Welcome to OmniSolo OneHumanCorp Smart Builder</h1>
              </WalkthroughTarget>
              <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                Review and add any extra details to help our AI generate the perfect store.
              </p>

              <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 block">Your Business Details</label>
              <WalkthroughTarget id="bio-input-target"><WithTooltip id="bio-input-tooltip" defaultText="Describe what you sell, your target audience, and the vibe of your brand.">
                <textarea
                  id="bio-input"
                  enterKeyHint="done"
                  autoCapitalize="sentences"
                  className="w-full border border-gray-200 bg-white/70 backdrop-blur-[30px] saturate-[210%] p-4 mb-8 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all resize-none text-gray-800 dark:text-[#f5f5f7]"
                  value={bio}
                  onChange={(e) => updateBio(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' && !e.shiftKey) {
                      e.preventDefault();
                      if (bio.trim().length > 5) {
                        handleGenerate();
                      }
                    }
                  }}
                  placeholder="e.g. I run a mobile dog grooming service in Portland"
                  rows={6}
                />
              </WithTooltip></WalkthroughTarget>

              <div className="flex gap-4">
                <WithTooltip id="generate-btn-tooltip" defaultText="Our AI agents will analyze your description and build a ready-to-launch store for you.">
                  <button
                    id="generate-btn"
                    className={`flex-[2] p-4 font-bold font-outfit text-lg transition-all ${
                      bio.trim().length > 5
                        ? "text-white shadow-md active:scale-[0.98] bg-[#0066FF]"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                    }`}
                    onClick={handleGenerate}
                    disabled={bio.trim().length <= 5}
                  >
                    Build My Storefront
                  </button>
                </WithTooltip>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (status === "generating") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden justify-center items-center glassmorphism">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-[#0066FF] mb-4"></div>
            <p className="text-gray-500 dark:text-[#a1a1a6] font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden text-center p-8 justify-center glassmorphism">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Site save recorded</h1>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">Publishing has not been verified.</p>

          <div className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 dark:text-[#a1a1a6] truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-[#0071E3] font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          <button
            className="w-full bg-gray-100 text-gray-800 dark:text-[#f5f5f7] font-bold p-4 active:scale-[0.98] transition-all hover:bg-gray-200"
            onClick={() => navigateStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }
  if (status === "chat") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden glassmorphism">
          <div className="px-6 py-6 flex flex-col justify-start h-full">
              <button type="button" className="sr-only" aria-hidden="true" tabIndex={-1}>Marketing Agent</button>
              <div className="flex justify-between items-center mb-6">
                  <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7]">Marketing Agent</h2>
                  <button
                      aria-label="Close Marketing Agent"
                      onClick={() => navigateStatus("draft")}
                      className="text-gray-500 hover:text-gray-700"
                  >
                      <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                  </button>
              </div>
              {generationNotice}
              <div className="flex-1 bg-white/50 dark:bg-black/50 rounded-xl p-4 mb-4 min-h-[200px] overflow-y-auto">
                  <div className="flex gap-3 mb-4">
                      <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center text-[#0071E3] font-bold shrink-0">AI</div>
                      <div className="app-card dark:bg-gray-800 p-3 rounded-2xl rounded-tl-none shadow-sm text-sm">
                          Hi! I'm your Marketing Agent. What would you like to change about your storefront?
                          <br/><br/>
                          You can say things like:
                          <br/>
                          • "Add a vegan cake option for $45 with a 50% deposit"
                          <br/>
                          • "Make the hero section more vibrant"
                      </div>
                  </div>
              </div>
              <div className="mt-auto relative">
                  <textarea
                      className="w-full bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-4 pr-12 text-sm focus:ring-2 focus:ring-[#0066FF] outline-none resize-none"
                      placeholder="e.g. Add a new product..."
                      rows={3}
                      value={chatMessage}
                      onChange={(e) => setChatMessage(e.target.value)}
                      onKeyDown={(e) => {
                          if (e.key === 'Enter' && !e.shiftKey) {
                              e.preventDefault();
                              handleAgentChat();
                          }
                      }}
                  />
                  <button
                      aria-label="Send storefront edit"
                      onClick={handleAgentChat}
                      disabled={!chatMessage.trim()}
                      className={`absolute bottom-4 right-4 p-2 rounded-full ${chatMessage.trim() ? 'bg-[#0071E3] text-white' : 'bg-gray-200 text-gray-400'}`}
                  >
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M12 5l7 7-7 7" /></svg>
                  </button>
              </div>
          </div>
        </div>
      </div>
    );
  }


  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden glassmorphism">
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-[30px] saturate-[210%] text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>
        {generationNotice}
        <div className="absolute top-10 right-4 flex items-center gap-4 z-50">
            {saveMessage && <span className="text-[#34C759] text-sm font-semibold animate-fade-in bg-white/80 dark:bg-black/80 px-2 py-1 rounded">{saveMessage}</span>}
        </div>

        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
            <DraggableBlock
              key={b.type + i}
              isSelected={selectedBlockIndex === i}
              onClick={() => setSelectedBlockIndex(i === selectedBlockIndex ? null : i)}
              onDragStart={(e) => {
                if (e.type.includes('drag') && (e as React.DragEvent).dataTransfer) {
                  (e as React.DragEvent).dataTransfer.effectAllowed = 'move';
                  (e as React.DragEvent).dataTransfer.setData('text/plain', i.toString());
                }
                setDraggedIndex(i);
                setSelectedBlockIndex(i);
              }}
              onDragOver={(e) => {
                if (e.type.includes('drag') && (e as React.DragEvent).dataTransfer) {
                  (e as React.DragEvent).dataTransfer.dropEffect = 'move';
                }
              }}
              onDragEnter={() => {
                if (draggedIndex !== null && draggedIndex !== i) {
                  moveBlock(draggedIndex, i);
                  setDraggedIndex(i);
                }
              }}
              onDragEnd={() => setDraggedIndex(null)}
              onMoveUp={i > 0 ? () => moveBlock(i, i - 1) : undefined}
              onMoveDown={i < blocks.length - 1 ? () => moveBlock(i, i + 1) : undefined}
            >
              <SmartBlock {...b} />
            </DraggableBlock>
          ))}
          <PublicationPanel channel="storefront-builder" expectedOwner={viewScope?.owner ?? null}
            getSnapshot={() => layoutPublicationSnapshot({ title: 'Home', bio: localDraft.current.bio, blocks: localDraft.current.blocks })}
            isEditorCurrent={() => {
              if (!builderScopeActive(viewScope) || scope.current !== viewScope) return false;
              try { assertBuilderEditor(viewScope, 'storefront-builder-draft'); return true; } catch { return false; }
            }}
            onRetired={() => invalidateOnboardingSession(false)} />
          {/* Default to false for premium status here. In a full implementation, we'd fetch this from the user's profile. */}
          <SmartBlock type="PoweredBy" props={{ tenantId, isPremium: false }} />
          <div className="text-center mt-4 mb-8">
            <a href="/onboarding?ref=storefront" target="_blank" className="text-xs font-semibold text-gray-500 hover:text-gray-700">⚡ Powered by OmniSolo</a>
          </div>
        </div>

        <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-[30px] saturate-[210%] border-t border-gray-200 z-50 rounded-b-[16px]">
          <div className="flex gap-2 mb-3">
            <button onClick={() => setIsAddBlockOpen(true)} className="flex-1 bg-white border border-gray-200 text-gray-800 py-3 font-semibold text-sm flex items-center justify-center gap-2 hover:bg-gray-50 transition-colors shadow-sm active:scale-[0.98]">
              <svg className="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
              Add Block
            </button>
            <button onClick={() => navigateStatus("chat")} className="flex-1 bg-white border border-gray-200 text-gray-800 py-3 font-semibold text-sm flex items-center justify-center gap-2 hover:bg-gray-50 transition-colors shadow-sm active:scale-[0.98]"><svg className="w-5 h-5 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" /></svg><span>Agent</span> <span className="inline text-xs font-normal opacity-75">(Ask Agent to Edit)</span></button></div><WithTooltip id="launch-btn-tooltip" defaultText="Save this private site draft. Public publication requires a separate review.">
            <button
              id="launch-btn"
              className="w-full bg-[#0071E3] text-white p-4 font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
              onClick={handleLaunch}
            >
              <span>Save site draft</span>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </button>
          </WithTooltip>
        </div>

        {/* Inline Editing Action Sheet */}
        <ActionSheet
          isOpen={selectedBlockIndex !== null && editingBlockContent !== null}
          onClose={() => setSelectedBlockIndex(null)}
          title={`Edit ${selectedBlockIndex !== null && blocks[selectedBlockIndex] ? blocks[selectedBlockIndex].type : ''} Block`}
        >
          {selectedBlockIndex !== null && editingBlockContent && (
            <div className="space-y-4 max-h-[60vh] overflow-y-auto pr-2 pb-20">
              {Object.keys(editingBlockContent).map((key) => {
                if (key === 'items' && Array.isArray(editingBlockContent[key])) {
                  return (
                    <div key={key} className="space-y-4">
                      <h3 className="font-semibold text-gray-700 dark:text-gray-200 capitalize">Items</h3>
                      {editingBlockContent[key].map((item, idx) => (
                        <div key={idx} className="p-3 border border-gray-200 dark:border-gray-700 rounded-lg space-y-2 relative">
                           <button
                             className="absolute top-2 right-2 text-[#FF3B30] text-xs font-bold"
                             onClick={() => {
                               const newItems = [...editingBlockContent[key]];
                               newItems.splice(idx, 1);
                               setEditingBlockContent({...editingBlockContent, [key]: newItems});
                             }}
                           >
                             Remove
                           </button>
                          {Object.keys(item).map(itemKey => (
                            <div key={itemKey}>
                              <label className="block text-xs text-gray-500 mb-1 capitalize">{itemKey}</label>
                              <input
                                type="text"
                                className="w-full bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded p-2 text-sm text-black dark:text-white"
                                value={item[itemKey] || ''}
                                onChange={(e) => {
                                  const newItems = [...editingBlockContent[key]];
                                  newItems[idx] = { ...newItems[idx], [itemKey]: e.target.value };
                                  setEditingBlockContent({ ...editingBlockContent, [key]: newItems });
                                }}
                              />
                            </div>
                          ))}
                        </div>
                      ))}
                      <button
                        className="w-full py-2 bg-gray-100 dark:bg-gray-800 text-sm font-semibold rounded-lg text-gray-700 dark:text-gray-200"
                        onClick={() => {
                          const newItems = [...editingBlockContent[key], { name: '', price: '', description: '' }];
                          setEditingBlockContent({ ...editingBlockContent, [key]: newItems });
                        }}
                      >
                        + Add Item
                      </button>
                    </div>
                  );
                }

                return (
                  <div key={key}>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 capitalize">
                      {key.replace(/([A-Z])/g, ' $1').trim()}
                    </label>
                    {key === 'copy' || key === 'description' || key === 'offerDescription' ? (
                      <textarea
                        className="w-full bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-3 text-sm text-black dark:text-white"
                        rows={3}
                        value={String(editingBlockContent[key] ?? '')}
                        onChange={(e) => setEditingBlockContent({ ...editingBlockContent, [key]: e.target.value })}
                      />
                    ) : (
                      <input
                        type="text"
                        className="w-full bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-3 text-sm text-black dark:text-white"
                        value={String(editingBlockContent[key] ?? '')}
                        onChange={(e) => setEditingBlockContent({ ...editingBlockContent, [key]: e.target.value })}
                      />
                    )}
                  </div>
                );
              })}
              <button
                className="w-full bg-[#0071E3] text-white font-bold py-3 rounded-xl mt-4 shadow-md"
                onClick={handleSaveBlock}
              >
                Save Changes
              </button>
              <button
                className="w-full bg-red-50 text-red-600 dark:bg-red-900/20 dark:text-red-400 font-bold py-3 rounded-xl mt-2"
                onClick={() => {
                  const newBlocks = blocks.filter((_, i) => i !== selectedBlockIndex);
                  setBlocks(newBlocks);
                  persistLayout({ blocks: newBlocks });
                  setSelectedBlockIndex(null);
                }}
              >
                Delete Block
              </button>
            </div>
          )}
        </ActionSheet>

        {/* Add Block Action Sheet */}
        <ActionSheet
          isOpen={isAddBlockOpen}
          onClose={() => setIsAddBlockOpen(false)}
          title="Add Block"
        >
          <div className="grid grid-cols-2 gap-3 pb-20">
            {['Hero', 'Catalog', 'Booking', 'Contact', 'Referral'].map((type) => (
              <button
                key={type}
                onClick={() => addBlock(type)}
                className="p-4 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl font-semibold text-gray-800 dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              >
                {type}
              </button>
            ))}
          </div>
        </ActionSheet>

      </div>
    </div>
  );
}
