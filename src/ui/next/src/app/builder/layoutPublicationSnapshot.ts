import type { BuilderBlock } from '@/lib/builder-types';

/** Explicitly select the current editor layout; owner/session/settings are never public content. */
export function layoutPublicationSnapshot(layout: { title: string; bio: string; blocks: BuilderBlock[] }) {
  const names: Record<string, string> = { Hero: 'HeroBlock', Catalog: 'ProductGridBlock', Booking: 'ServiceBookingBlock', Testimonials: 'TestimonialBlock', Referral: 'ReferralBlock' };
  return { domain: null, pages: [{ path: '/', title: layout.title || 'Home', seo_metadata: { description: layout.bio },
    blocks: layout.blocks.map((block, index) => ({ block_type: names[block.type] ?? block.type, content: block.props, sort_order: index })) }] };
}
