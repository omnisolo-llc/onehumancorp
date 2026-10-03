import type { BuilderBlock } from '@/lib/builder-types';
import { isBuilderBlocks } from './store';

/** Check fields actually rendered by the editor before replacing its owned draft. */
export function readGeneratedBlocks(value: unknown): BuilderBlock[] {
  const data = value && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : null;
  const page = data && Array.isArray(data.pages) && data.pages[0];
  if (!data || data.success !== undefined && data.success !== true || data.error != null || !page || !Array.isArray(page.blocks) || !page.blocks.length) throw new Error('The generated draft could not be confirmed.');
  const names: Record<string, string> = { HeroBlock: 'Hero', ProductGridBlock: 'Catalog', ServiceBookingBlock: 'Booking', TestimonialBlock: 'Testimonials', ReferralBlock: 'Referral' };
  const generated: unknown = page.blocks.map((block: Record<string, unknown>) => ({ type: typeof block?.block_type === 'string' ? names[block.block_type] || block.block_type : '', props: block?.content }));
  const textFields = ['headline', 'subtitle', 'copy', 'text', 'image', 'title', 'availability', 'email', 'phone', 'offerTitle', 'offerDescription', 'url', 'tenantId'];
  if (!isBuilderBlocks(generated) || generated.some(block =>
    textFields.some(key => block.props[key] !== undefined && typeof block.props[key] !== 'string')
    || block.type === 'Catalog' && !Array.isArray(block.props.items))) {
    throw new Error('The generated draft has unsupported content.');
  }
  return generated;
}
