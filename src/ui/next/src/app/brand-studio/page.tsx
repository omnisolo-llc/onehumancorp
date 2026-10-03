"use client";

import React, { useEffect, useRef, useState } from "react";
import { openBuilderEditor, releaseBuilderEditor, assertBuilderEditor, builderScopeActive, type BuilderScope } from '../builder/ownedDraft';
import { fetchForOwnedBusinessAction, subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { canonicalRequest } from '../onboarding/contracts';
import { sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';
import { generationFailureMessage } from '../builder/generationFailure';
import { PublicationPanel } from '../builder/PublicationPanel';
import { parsePublicationJson } from '../builder/publicationContracts';
import { readBrandToolbox, brandPublicationSnapshot, safeBrandSvg, type BrandToolbox } from './contracts';

const BRAND_DRAFT_KEY = 'brand-studio-draft';
export default function BrandStudioPage() {
  const [description, setDescription] = useState('');
  const [websiteUrl, setWebsiteUrl] = useState('');
  const [productUrl, setProductUrl] = useState('');
  const [campaignPrompt, setCampaignPrompt] = useState('');
  const [toolbox, setToolbox] = useState<BrandToolbox | null>(null);
  const [status, setStatus] = useState<'idle' | 'generating' | 'ready' | 'error'>('idle');
  const [errorMessage, setErrorMessage] = useState('');
  const [sessionError, setSessionError] = useState('');
  const [viewScope, setViewScope] = useState<BuilderScope | null>(null);
  const currentScope = useRef<BuilderScope | null>(null);
  const epoch = useRef(0);
  const pending = useRef<{ epoch: number; abort: AbortController } | null>(null);
  const generatedInput = useRef<string | null>(null);
  const currentInput = useRef({ description, websiteUrl, productUrl, campaignPrompt });
  currentInput.current = { description, websiteUrl, productUrl, campaignPrompt };
  const inputFingerprint = () => canonicalRequest(currentInput.current);
  const retireRequest = () => { ++epoch.current; pending.current?.abort.abort(); pending.current = null; generatedInput.current = null; };

  useEffect(() => {
    let disposed = false; let version = 0;
    const open = async () => {
      const attempt = ++version; setSessionError('');
      try {
        const opened = await openBuilderEditor(BRAND_DRAFT_KEY, () => !disposed && attempt === version);
        if (disposed || attempt !== version) { releaseBuilderEditor(opened); return; }
        releaseBuilderEditor(currentScope.current); currentScope.current = opened; setViewScope(opened);
      } catch (error) { if (!disposed && attempt === version) setSessionError(error instanceof Error ? error.message : 'Verify your Brand Studio session.'); }
    };
    const unsubscribe = subscribeOnboardingInvalidation(restart => {
      ++version; retireRequest(); releaseBuilderEditor(currentScope.current); currentScope.current = null; setViewScope(null);
      setToolbox(null); setDescription(''); setWebsiteUrl(''); setProductUrl(''); setCampaignPrompt(''); setStatus('idle'); setErrorMessage('');
      if (restart) void open(); else setSessionError('Your session could not be verified. Reopen Brand Studio.');
    });
    void open();
    return () => { disposed = true; ++version; retireRequest(); releaseBuilderEditor(currentScope.current); currentScope.current = null; unsubscribe(); };
  }, []);
  const retireEditor = (owner: QueueOwner, reason: string) => {
    if (!currentScope.current || !sameOwner(currentScope.current.owner, owner)) return;
    retireRequest(); releaseBuilderEditor(currentScope.current); currentScope.current = null; setViewScope(null);
    setToolbox(null); setDescription(''); setWebsiteUrl(''); setProductUrl(''); setCampaignPrompt(''); setStatus('idle'); setErrorMessage(''); setSessionError(reason);
  };
  const changeInput = (update: () => void) => {
    if (!builderScopeActive(viewScope)) return;
    try { assertBuilderEditor(viewScope, BRAND_DRAFT_KEY); }
    catch { retireEditor(viewScope.owner, 'Your Brand Studio editor changed. Reopen it to verify access.'); return; }
    retireRequest(); update(); setToolbox(null); setStatus('idle'); setErrorMessage('');
  };
  const generateToolbox = async () => {
    const scope = viewScope;
    if (!builderScopeActive(scope) || pending.current || description.trim().length < 8) return;
    const request = { epoch: ++epoch.current, abort: new AbortController() };
    const fingerprint = inputFingerprint();
    const active = () => pending.current === request && request.epoch === epoch.current && builderScopeActive(scope);
    pending.current = request; setStatus('generating'); setToolbox(null); setErrorMessage(''); generatedInput.current = null;
    try {
      assertBuilderEditor(scope, BRAND_DRAFT_KEY);
      if (websiteUrl.trim() || productUrl.trim()) throw new Error('Website and product URL fetching is unavailable in this flow.');
      const response = await fetchForOwnedBusinessAction('/api/v1/builder/brand_toolbox/generate', {
        method: 'POST', signal: request.abort.signal, headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description, website_url: null, product_url: null, campaign_prompt: campaignPrompt || null, uploaded_asset_names: [] }),
      }, scope.owner, () => { assertBuilderEditor(scope, BRAND_DRAFT_KEY); if (!active() || fingerprint !== inputFingerprint()) throw new Error('Your generation request changed before dispatch.'); });
      const payload = parsePublicationJson(await response.text());
      if (response.status !== 200) throw new Error(generationFailureMessage(response.status, payload));
      const data = await readBrandToolbox(payload);
      if (!active() || fingerprint !== inputFingerprint()) return;
      assertBuilderEditor(scope, BRAND_DRAFT_KEY); generatedInput.current = fingerprint; setToolbox(data); setStatus('ready');
    } catch (error) { if (active()) { setStatus('error'); setErrorMessage(error instanceof Error ? error.message : 'Could not generate the toolbox.'); } }
    finally { if (pending.current === request) pending.current = null; }
  };

  if (sessionError) return <main role="alert">{sessionError}</main>;
  if (!builderScopeActive(viewScope)) return <main role="status">Verifying your Brand Studio session…</main>;
  return (
    <main className="min-h-screen bg-[#F5F5F7] font-inter text-gray-950">
      <div className="mx-auto grid w-full max-w-7xl gap-6 px-4 py-6 lg:grid-cols-[360px_1fr] lg:px-8">
        <section className="rounded-lg border border-gray-200 bg-white p-5 shadow-sm">
          <div className="mb-5">
            <p className="text-xs font-semibold uppercase tracking-wider text-[#0071E3]">
              Brand Studio
            </p>
            <h1 className="mt-1 text-2xl font-bold font-outfit">Create Brand Toolbox</h1>
          </div>

          <label className="mb-2 block text-sm font-semibold text-gray-700" htmlFor="brand-toolbox-description">
            Business
          </label>
          <textarea
            id="brand-toolbox-description"
            className="mb-4 h-32 w-full resize-none rounded-lg border border-gray-300 bg-white p-3 text-sm outline-none focus:border-[#0071E3] focus:ring-2 focus:ring-blue-100"
            value={description}
            onChange={(event) => changeInput(() => setDescription(event.target.value))}
          />

          <label className="mb-2 block text-sm font-semibold text-gray-700" htmlFor="brand-toolbox-website">
            Website URL
          </label>
          <input
            id="brand-toolbox-website"
            className="mb-4 w-full rounded-lg border border-gray-300 bg-white p-3 text-sm outline-none focus:border-[#0071E3] focus:ring-2 focus:ring-blue-100"
            value={websiteUrl}
            disabled aria-describedby="brand-source-limits" onChange={(event) => changeInput(() => setWebsiteUrl(event.target.value))}
            placeholder="https://example.com"
          />

          <label className="mb-2 block text-sm font-semibold text-gray-700" htmlFor="brand-toolbox-product">
            Product URL
          </label>
          <input
            id="brand-toolbox-product"
            className="mb-4 w-full rounded-lg border border-gray-300 bg-white p-3 text-sm outline-none focus:border-[#0071E3] focus:ring-2 focus:ring-blue-100"
            value={productUrl}
            disabled aria-describedby="brand-source-limits" onChange={(event) => changeInput(() => setProductUrl(event.target.value))}
            placeholder="https://example.com/product"
          />

          <label className="mb-2 block text-sm font-semibold text-gray-700" htmlFor="brand-toolbox-campaign">
            Campaign
          </label>
          <input
            id="brand-toolbox-campaign"
            className="mb-5 w-full rounded-lg border border-gray-300 bg-white p-3 text-sm outline-none focus:border-[#0071E3] focus:ring-2 focus:ring-blue-100"
            value={campaignPrompt}
            onChange={(event) => changeInput(() => setCampaignPrompt(event.target.value))}
          />

          <button
            className="flex h-12 w-full items-center justify-center rounded-lg bg-[#0071E3] px-4 text-sm font-bold text-white transition hover:bg-blue-700 disabled:bg-gray-300"
            onClick={generateToolbox}
            disabled={status === "generating" || description.trim().length < 8 || !builderScopeActive(viewScope)}
          >
            {status === "generating" ? "Generating..." : "Generate Toolbox"}
          </button>

          <p id="brand-source-limits" className="mt-3 text-sm text-gray-600">Only supplied text is used. Website fetching and uploaded-media generation are unavailable in this flow.</p>
          {errorMessage && <p role="alert" className="mt-4 rounded-lg bg-red-50 p-3 text-sm">{errorMessage}</p>}
          {toolbox && <PublicationPanel channel="brand-studio" expectedOwner={viewScope.owner}
            getSnapshot={() => { assertBuilderEditor(viewScope, BRAND_DRAFT_KEY); if (!toolbox) throw new Error('Generate and review a current draft first.'); return brandPublicationSnapshot(toolbox); }}
            isEditorCurrent={() => { try { assertBuilderEditor(viewScope, BRAND_DRAFT_KEY); return !pending.current && generatedInput.current === inputFingerprint(); } catch { return false; } }}
            onRetired={retireEditor} />}
        </section>

        <section className="min-h-[720px] rounded-lg border border-gray-200 bg-white p-5 shadow-sm">
          {!toolbox ? (
            <div className="flex h-full min-h-[560px] items-center justify-center rounded-lg border border-dashed border-gray-300 text-center">
              <div>
                <h2 className="text-xl font-bold font-outfit">Brand output will appear here</h2>
                <p className="mt-2 max-w-md text-sm text-gray-500">
                  Generate a structured Brand DNA, brand book, campaign kit, photoshoot plan, and website draft.
                </p>
              </div>
            </div>
          ) : (
            <div className="grid gap-5">
              <p role="status">Draft suggestions; review before use. Business facts, source websites and media have not been verified.</p>
              <div className="rounded-lg border border-gray-200 p-4">
                <div className="flex flex-wrap items-start justify-between gap-4">
                  <div>
                    <p className="text-xs font-semibold uppercase tracking-wider text-[#0071E3]">
                      Brand DNA
                    </p>
                    <h2 className="mt-1 text-2xl font-bold font-outfit">{toolbox.brand_dna.name}</h2>
                    <p className="mt-2 max-w-3xl text-sm text-gray-600">{toolbox.brand_dna.positioning}</p>
                  </div>
                  <div className="flex gap-2">
                    {(toolbox.brand_dna.colors ?? []).map((color) => (
                      <span
                        key={color}
                        className="h-8 w-8 rounded border border-gray-200"
                        style={{ backgroundColor: color }}
                        title={color}
                      />
                    ))}
                  </div>
                </div>
              </div>

              <div className="grid gap-5 xl:grid-cols-2">
                <OutputGroup title="Brand Book">
                  {(toolbox.brand_book ?? []).map((section) => (
                    <div key={section.title} className="border-b border-gray-100 py-3 last:border-0">
                      <h3 className="font-semibold">{section.title}</h3>
                      <p className="mt-1 text-sm text-gray-600">{section.guidance.join(" ")}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Logo Concepts">
                  {toolbox.logo_concepts.length === 0 && <p>No logo assets were returned.</p>}
                  {(toolbox.logo_concepts ?? []).map((logo) => (
                    <div key={logo.title} className="border-b border-gray-100 py-3 last:border-0">
                      <h3 className="font-semibold">{logo.title}</h3>
                      <BrandImage title={logo.title} svg={logo.svg} />
                      <p className="mt-2 text-sm text-gray-600">{logo.usage_notes.join(" ")}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Catalog Suggestions">
                  {toolbox.catalog.length === 0 && <p>No catalog items or prices were supplied.</p>}
                  {(toolbox.catalog ?? []).map((item) => (
                    <div key={item.name} className="border-b border-gray-100 py-3 last:border-0">
                      <div className="flex items-start justify-between gap-3">
                        <h3 className="font-semibold">{item.name}</h3>
                        <span className="text-sm font-bold text-gray-700">{item.price}</span>
                      </div>
                      <p className="mt-1 text-sm text-gray-600">{item.description}</p>
                      <p className="mt-2 text-xs font-medium text-gray-500">{item.seo_title}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Campaign Ideas">
                  {(toolbox.campaign_ideas ?? []).map((idea) => (
                    <div key={idea.title} className="border-b border-gray-100 py-3 last:border-0">
                      <h3 className="font-semibold">{idea.title}</h3>
                      <p className="mt-1 text-sm text-gray-600">{idea.hook}</p>
                      <p className="mt-2 text-xs font-medium text-gray-500">{idea.channels.join(" / ")}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Social Calendar">
                  {(toolbox.social_calendar ?? []).map((item) => (
                    <div key={`${item.day}-${item.channel}`} className="border-b border-gray-100 py-3 last:border-0">
                      <p className="text-xs font-semibold uppercase tracking-wide text-gray-500">
                        {item.day} / {item.channel}
                      </p>
                      <p className="mt-1 text-sm text-gray-700">{item.caption}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Creative Assets">
                  {(toolbox.assets ?? []).map((asset) => (
                    <div key={`${asset.asset_type}-${asset.channel}`} className="border-b border-gray-100 py-3 last:border-0">
                      <p className="text-xs font-semibold uppercase tracking-wide text-gray-500">
                        {asset.asset_type} / {asset.channel}
                      </p>
                      <h3 className="mt-1 font-semibold">{asset.title}</h3>
                      <p className="mt-1 text-sm text-gray-600">{asset.copy}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Photoshoot Concepts">
                  {toolbox.photoshoot.shots.length === 0 && <p>No generated images were returned.</p>}
                  <p className="mb-3 text-sm text-gray-600">{toolbox.photoshoot.product_source}</p>
                  {(toolbox.photoshoot?.shots ?? []).map((shot) => (
                    <div key={shot.title} className="border-b border-gray-100 py-3 last:border-0">
                      <h3 className="font-semibold">{shot.title}</h3>
                      <BrandImage title={shot.title} svg={shot.mockup_svg} />
                      <p className="mt-1 text-xs font-medium text-gray-500">{shot.format} / {shot.usage}</p>
                      <p className="mt-2 text-sm text-gray-600">{shot.prompt}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Website Draft">
                  <p className="text-sm text-gray-600">
                    {(toolbox.website_draft ?? toolbox.store_profile)?.pages?.[0]?.blocks?.length ?? 0} ready-to-edit website blocks generated from the Brand DNA.
                  </p>
                </OutputGroup>
              </div>
            </div>
          )}
        </section>
      </div>
    </main>
  );
}

function OutputGroup({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <section className="rounded-lg border border-gray-200 p-4">
      <h2 className="text-lg font-bold font-outfit">{title}</h2>
      <div className="mt-2">{children}</div>
    </section>
  );
}

function BrandImage({ title, svg }: { title: string; svg: string }) {
  const source = safeBrandSvg(svg);
  return source ? <img src={source} alt={title} className="mt-3 max-h-64 w-full object-contain rounded-lg border border-gray-100 bg-gray-50" /> : <p>Preview unavailable for this returned asset.</p>;
}
