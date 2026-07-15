const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();
const MAX_ONBOARDING_INPUT_CHARS = 4_000;
const MAX_ONBOARDING_IMAGE_URL_CHARS = 2_048;

function hasAtMostChars(value: string, maximum: number): boolean {
  let count = 0;
  for (const _character of value) {
    count += 1;
    if (count > maximum) return false;
  }
  return true;
}

const ALLOWED_STATE_FIELDS = new Set([
  "step",
  "chatStep",
  "businessDescription",
  "businessGoal",
  "bio",
  "businessName",
  "whatYouSell",
  "location",
  "targetAudience",
  "businessType",
  "categories",
  "websiteTemplate",
  "domainChoice",
  "firstProductName",
  "firstProductPrice",
  "aiAgents",
  "aiAutoRespond",
  "instantImageUrl",
  "skipped",
  "error",
]);

const ALLOWED_START_FIELDS = new Set([
  "business_type",
  "company_name",
  "company_description",
  "selling_categories",
  "payment_pref",
  "website_template",
  "first_product_name",
  "first_product_price",
  "domain_choice",
  "price_type",
  "location",
  "target_audience",
  "initial_products",
  "ai_agents",
  "ai_auto_respond",
  "deposit_percentage",
  "lead_time_days",
]);

function sanitizeState(value: unknown): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("onboarding state must be an object");
  }
  const input = value as Record<string, unknown>;
  const output: Record<string, unknown> = {};
  for (const [name, field] of Object.entries(input)) {
    if (name === "wizardState") output.wizardState = sanitizeState(field);
    else if (ALLOWED_STATE_FIELDS.has(name)) output[name] = field;
  }
  return output;
}

export function sanitizeOnboardingStateRequest(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  return encoder.encode(JSON.stringify(sanitizeState(JSON.parse(decoder.decode(body)))));
}

export function sanitizeOnboardingStartRequest(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  const parsed = JSON.parse(decoder.decode(body)) as unknown;
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error("onboarding start request must be an object");
  }
  const output: Record<string, unknown> = {};
  for (const [name, field] of Object.entries(parsed)) {
    if (ALLOWED_START_FIELDS.has(name)) output[name] = field;
  }
  return encoder.encode(JSON.stringify(output));
}

export function sanitizeOnboardingZeroClickRequest(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  const parsed = JSON.parse(decoder.decode(body)) as unknown;
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error("zero-click onboarding request must be an object");
  }
  const input = parsed as Record<string, unknown>;
  if (
    typeof input.prompt !== "string" ||
    input.prompt.trim().length === 0 ||
    !hasAtMostChars(input.prompt, MAX_ONBOARDING_INPUT_CHARS)
  ) {
    throw new Error("zero-click onboarding prompt is invalid");
  }
  if (
    input.image_url !== undefined &&
    (typeof input.image_url !== "string" ||
      input.image_url.trim().length === 0 ||
      !hasAtMostChars(input.image_url, MAX_ONBOARDING_IMAGE_URL_CHARS))
  ) {
    throw new Error("zero-click onboarding image URL is invalid");
  }

  return encoder.encode(
    JSON.stringify({
      prompt: input.prompt,
      ...(input.image_url === undefined ? {} : { image_url: input.image_url }),
    }),
  );
}
