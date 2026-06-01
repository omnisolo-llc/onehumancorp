# [Marketing] Brand DNA Toolbox and Pomelli Parity

## Goal

OHC must be capable of every core Pomelli workflow while extending it into live business operations. Pomelli's public feature set centers on Business DNA, campaign ideas, editable brand assets, product photoshoots, brand books, and generated websites. OHC should treat those as the baseline, then connect the outputs to storefront publishing, checkout, booking, inventory, inbox automation, and 1-tap approvals.

## Parity Capabilities

| Pomelli Capability | OHC Capability Target | Status |
| --- | --- | --- |
| Website or material intake | Brand intake from description, website URL, product URL, uploaded files, and chat | Implemented |
| Business DNA | Structured Brand DNA shared by builder and agents | Implemented and persisted |
| Logo | Generated SVG logo concepts and usage notes | Implemented |
| Catalog | Starter catalog items with SEO and photo prompts | Implemented |
| Campaign ideas | Campaign ideas with goals, hooks, and channel mix | Implemented |
| Editable creatives | Multi-channel assets with editable fields and visual prompts | Implemented |
| Photoshoot | Product photo source, templates, shot briefs, image prompts, and refinement controls | Implemented |
| Brand books | Exportable brand guidance sections | Implemented |
| Websites | Mobile-first storefront draft | Existing builder plus scaffold |
| Download/export | PNG/JPG/PDF/HTML/calendar export plan | Implemented as export metadata |

## OHC Differentiators

1. Brand DNA should become tenant memory, not a one-off generation artifact.
2. The Promoter should use Brand DNA whenever it drafts social calendars, ads, email, SEO, and link-in-bio pages.
3. The storefront builder should publish directly into the transactional OHC site, not just a brochure site.
4. Product creation should trigger a complete launch kit: product photo prompts, description, SEO metadata, email, social posts, and ad copy.
5. The approval inbox should show generated assets as business actions: approve, edit, schedule, publish, or add to Brand DNA.

## Brand DNA Object

The shared Brand DNA should include:

- Business name and type
- Positioning
- Audience
- Tone of voice
- Color palette
- Fonts
- Image style
- Logo and uploaded reference assets
- Prohibited claims and off-brand rules
- Product/service grounding URLs

## Required User Workflows

### Create A Brand From Scratch

1. User describes the business.
2. OHC generates Brand DNA.
3. OHC generates a brand book, starter assets, product/photo prompts, and website draft.
4. User edits or approves.
5. OHC publishes storefront and queues marketing actions.

### Analyze An Existing Brand

1. User enters website URL and optional social/product URLs.
2. OHC extracts brand clues and generates Brand DNA.
3. User confirms or edits.
4. OHC generates campaigns and assets grounded in the confirmed DNA.

### Launch A Product Campaign

1. User adds product URL or uploads product photo.
2. OHC generates a Photoshoot plan, studio/lifestyle prompts, social posts, ad copy, email, and website update.
3. User approves.
4. OHC schedules or publishes across connected channels.

## API Scaffold

The initial implementation adds:

- `POST /api/v1/builder/brand_toolbox/generate`
- `GET /api/v1/builder/brand_toolbox`
- `GET /api/v1/builder/brand_toolbox/{toolbox_id}`
- `POST /api/v1/builder/brand_toolbox/{toolbox_id}/publish_website`
- Input: description, website URL, product URL, campaign prompt, uploaded asset names
- Output: persisted Brand DNA, logo concepts, brand book sections, starter catalog, campaign ideas, social calendar, generated assets, photoshoot plan, website draft, edit controls, export formats

The existing storefront generator also accepts optional brand and product context, reuses the latest persisted Brand DNA when available, and can fall back to a deterministic draft when no LLM provider is configured.

## Next Implementation Steps

1. Persist Brand DNA per tenant and version it.
2. Add website/product URL extraction workers.
3. Add asset upload storage and reference-image metadata.
4. Wire Brand DNA into the Promoter worker's social calendar prompts.
5. Package the Brand Studio UI into the canonical Tauri static output.
6. Add export jobs for PDF brand books and social calendar files.
7. Add channel-specific renderers for social/ad/email image sizes.
8. Add approval inbox actions for edit, schedule, publish, and add-to-DNA.
