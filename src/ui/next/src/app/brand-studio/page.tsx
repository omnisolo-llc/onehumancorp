"use client";

import React, { useState } from "react";

type BrandToolbox = {
  id?: string;
  brand_dna: {
    name: string;
    business_type: string;
    positioning: string;
    audience: string;
    tone_of_voice: string[];
    colors: string[];
    fonts: string[];
    image_style: string[];
  };
  logo_concepts: { title: string; svg: string; usage_notes: string[] }[];
  brand_book: { title: string; guidance: string[] }[];
  catalog: {
    name: string;
    price: string;
    description: string;
    photo_prompt: string;
    seo_title: string;
  }[];
  campaign_ideas: { title: string; goal: string; channels: string[]; hook: string }[];
  social_calendar: {
    day: string;
    channel: string;
    caption: string;
    visual_prompt: string;
    call_to_action: string;
  }[];
  assets: {
    asset_type: string;
    channel: string;
    title: string;
    copy: string;
    visual_prompt: string;
    editable_fields: string[];
  }[];
  photoshoot: {
    product_source: string;
    templates: string[];
    prompts: string[];
    shots: { title: string; format: string; prompt: string; usage: string; representation_svg: string }[];
    refinement_controls: string[];
  };
  store_profile?: {
    pages: { blocks: { block_type: string }[] }[];
  };
  website_draft?: {
    pages: { blocks: { block_type: string }[] }[];
  };
  export_formats: string[];
};

const defaultDescription =
  "I run a local bakery that sells custom cakes and weekend dessert boxes.";

export default function BrandStudioPage() {
  const [description, setDescription] = useState(defaultDescription);
  const [websiteUrl, setWebsiteUrl] = useState("");
  const [productUrl, setProductUrl] = useState("");
  const [campaignPrompt, setCampaignPrompt] = useState("launch a weekend offer");
  const [toolbox, setToolbox] = useState<BrandToolbox | null>(null);
  const [status, setStatus] = useState<"idle" | "generating" | "ready" | "error">("idle");
  const [publishStatus, setPublishStatus] = useState("");

  const generateToolbox = async () => {
    setStatus("generating");
    try {
      const response = await fetch("/api/v1/builder/brand_toolbox/generate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          description,
          website_url: websiteUrl || null,
          product_url: productUrl || null,
          campaign_prompt: campaignPrompt || null,
          uploaded_asset_names: [],
        }),
      });

      if (!response.ok) {
        throw new Error("Brand toolbox generation failed");
      }

      setToolbox(await response.json());
      setPublishStatus("");
      setStatus("ready");
    } catch (error) {
      console.error(error);
      setStatus("error");
    }
  };

  const publishWebsite = async () => {
    if (!toolbox?.id) return;
    setPublishStatus("Publishing website...");
    try {
      const response = await fetch(`/api/v1/builder/brand_toolbox/${toolbox.id}/publish_website`, {
        method: "POST",
      });
      if (!response.ok) {
        throw new Error("Website publish failed");
      }
      const site = await response.json();
      setPublishStatus(`Published domain: ${site.domain ?? "Website published"}`);
    } catch (error) {
      console.error(error);
      setPublishStatus("Could not publish the website.");
    }
  };

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
            onChange={(event) => setDescription(event.target.value)}
          />

          <label className="mb-2 block text-sm font-semibold text-gray-700" htmlFor="brand-toolbox-website">
            Website URL
          </label>
          <input
            id="brand-toolbox-website"
            className="mb-4 w-full rounded-lg border border-gray-300 bg-white p-3 text-sm outline-none focus:border-[#0071E3] focus:ring-2 focus:ring-blue-100"
            value={websiteUrl}
            onChange={(event) => setWebsiteUrl(event.target.value)}
            placeholder="https://example.com"
          />

          <label className="mb-2 block text-sm font-semibold text-gray-700" htmlFor="brand-toolbox-product">
            Product URL
          </label>
          <input
            id="brand-toolbox-product"
            className="mb-4 w-full rounded-lg border border-gray-300 bg-white p-3 text-sm outline-none focus:border-[#0071E3] focus:ring-2 focus:ring-blue-100"
            value={productUrl}
            onChange={(event) => setProductUrl(event.target.value)}
            placeholder="https://example.com/product"
          />

          <label className="mb-2 block text-sm font-semibold text-gray-700" htmlFor="brand-toolbox-campaign">
            Campaign
          </label>
          <input
            id="brand-toolbox-campaign"
            className="mb-5 w-full rounded-lg border border-gray-300 bg-white p-3 text-sm outline-none focus:border-[#0071E3] focus:ring-2 focus:ring-blue-100"
            value={campaignPrompt}
            onChange={(event) => setCampaignPrompt(event.target.value)}
          />

          <button
            className="flex h-12 w-full items-center justify-center rounded-lg bg-[#0071E3] px-4 text-sm font-bold text-white transition hover:bg-blue-700 disabled:bg-gray-300"
            onClick={generateToolbox}
            disabled={status === "generating" || description.trim().length < 8}
          >
            {status === "generating" ? "Generating..." : "Generate Toolbox"}
          </button>

          <button
            className="mt-3 flex h-12 w-full items-center justify-center rounded-lg border border-gray-300 bg-white px-4 text-sm font-bold text-gray-900 transition hover:bg-gray-50 disabled:text-gray-400"
            onClick={publishWebsite}
            disabled={!toolbox?.id || publishStatus === "Publishing website..."}
          >
            Publish Website
          </button>

          {publishStatus && (
            <p id="brand-toolbox-status" className="mt-4 rounded-lg bg-blue-50 p-3 text-sm font-medium text-blue-700">
              {publishStatus}
            </p>
          )}

          {status === "error" && (
            <p id="brand-toolbox-status" className="mt-4 rounded-lg bg-red-50 p-3 text-sm font-medium text-red-700">
              Could not generate the toolbox.
            </p>
          )}
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
                  {(toolbox.logo_concepts ?? []).map((logo) => (
                    <div key={logo.title} className="border-b border-gray-100 py-3 last:border-0">
                      <h3 className="font-semibold">{logo.title}</h3>
                      <div
                        className="mt-3 overflow-hidden rounded-lg border border-gray-100 bg-gray-50"
                        dangerouslySetInnerHTML={{ __html: logo.svg }}
                      />
                      <p className="mt-2 text-sm text-gray-600">{logo.usage_notes.join(" ")}</p>
                    </div>
                  ))}
                </OutputGroup>

                <OutputGroup title="Starter Catalog">
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

                <OutputGroup title="Photoshoot">
                  <p className="mb-3 text-sm text-gray-600">{toolbox.photoshoot?.product_source ?? "Product and campaign-ready imagery"}</p>
                  {(toolbox.photoshoot?.shots ?? []).map((shot) => (
                    <div key={shot.title} className="border-b border-gray-100 py-3 last:border-0">
                      <h3 className="font-semibold">{shot.title}</h3>
                      <div
                        className="mt-3 overflow-hidden rounded-lg border border-gray-100 bg-gray-50"
                        dangerouslySetInnerHTML={{ __html: shot.representation_svg }}
                      />
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
