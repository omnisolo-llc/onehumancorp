export interface CatalogBlockItem {
  [key: string]: string | number | undefined;
  name?: string;
  price?: string | number;
  description?: string;
  image?: string;
  buttonText?: string;
}

export interface BlockProperties {
  [key: string]: string | number | boolean | CatalogBlockItem[] | undefined;
  headline?: string;
  copy?: string;
  image?: string;
  title?: string;
  availability?: string;
  items?: CatalogBlockItem[];
  offerTitle?: string;
  offerDescription?: string;
  url?: string;
  tenantId?: string;
  isPremium?: boolean;
  email?: string;
  phone?: string;
}

export interface BuilderBlock {
  id?: string;
  type: string;
  props: BlockProperties;
}

export interface GeneratedBlock {
  block_type: string;
  content: BlockProperties;
}

export interface OnboardingResult {
  organization_id?: string;
  website_url?: string;
  storefront_url?: string;
  url?: string;
  error?: string;
  message?: string;
  business_name?: string;
  status?: string;
  site?: { url?: string };
  agent_ids?: string[];
  product_ids?: string[];
}
