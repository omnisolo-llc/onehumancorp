export type CustomerInteraction = {
  id?: string;
  channel?: string;
  type?: string;
  date?: string;
  created_at?: string | number;
  description?: string;
  raw_content?: string;
};

export type CustomerMemorySummary = {
  summary?: string;
  customer_name?: string;
  total_interactions?: number;
  segments?: string[];
  events?: CustomerInteraction[];
  interactions?: CustomerInteraction[];
  context_graph?: unknown;
};

function record(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function interaction(value: unknown): value is CustomerInteraction {
  if (!record(value)) return false;
  for (const key of ['id', 'channel', 'type', 'date', 'description', 'raw_content']) {
    if (value[key] !== undefined && typeof value[key] !== 'string') return false;
  }
  return value.created_at === undefined || typeof value.created_at === 'string'
    || (typeof value.created_at === 'number' && Number.isFinite(value.created_at));
}

/** Narrow external data before it becomes React children or array operations. */
export function parseMemorySummary(value: unknown): CustomerMemorySummary {
  if (!record(value)) throw new Error('Invalid customer history');
  for (const key of ['summary', 'customer_name']) {
    if (value[key] !== undefined && typeof value[key] !== 'string') throw new Error('Invalid customer history');
  }
  if (value.total_interactions !== undefined &&
      (typeof value.total_interactions !== 'number' || !Number.isSafeInteger(value.total_interactions) || value.total_interactions < 0)) {
    throw new Error('Invalid interaction count');
  }
  if (value.segments !== undefined && (!Array.isArray(value.segments) || !value.segments.every(item => typeof item === 'string'))) {
    throw new Error('Invalid customer segments');
  }
  for (const key of ['events', 'interactions']) {
    if (value[key] !== undefined && (!Array.isArray(value[key]) || !value[key].every(interaction))) {
      throw new Error('Invalid customer interactions');
    }
  }
  return value as CustomerMemorySummary;
}
