import { create } from 'zustand';

export interface ProductData {
  title: string;
  description: string;
  price: string;
  category: string;
  type: string;
  isSubscription?: boolean;
}

interface OfferingState {
  intent: string;
  loading: boolean;
  productData: ProductData | null;
  error: string;
  isSuccess: boolean;
  setIntent: (intent: string) => void;
  setLoading: (loading: boolean) => void;
  setProductData: (data: ProductData | null) => void;
  setError: (error: string) => void;
  setIsSuccess: (success: boolean) => void;
  generateOffering: (intent: string) => Promise<void>;
  publishOffering: (data: ProductData) => Promise<void>;
}

export const useOfferingStore = create<OfferingState>((set) => ({
  intent: '',
  loading: false,
  productData: null,
  error: '',
  isSuccess: false,
  setIntent: (intent) => set({ intent }),
  setLoading: (loading) => set({ loading }),
  setProductData: (data) => set({ productData: data }),
  setError: (error) => set({ error }),
  setIsSuccess: (isSuccess) => set({ isSuccess }),
  generateOffering: async (intent) => {
    set({ loading: true, error: '' });
    try {
      const response = await fetch('/api/offerings/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ intent })
      });

      if (!response.ok) {
        throw new Error('Failed to generate offering from AI.');
      }

      const data = await response.json();
      if (data.error) throw new Error(data.error);

      set({ productData: data });
    } catch (err: any) {
      set({ error: err.message || 'Failed to generate offering.' });
    } finally {
      set({ loading: false });
    }
  },
  publishOffering: async (data) => {
    try {
      const response = await fetch('/api/product', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          title: data.title,
          description: data.description,
          price: parseFloat(data.price),
          category: data.category,
          product_type: (data.type || 'Product').toLowerCase(),
          is_subscription: !!data.isSubscription
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to publish offering to storefront.');
      }

      set({ isSuccess: true });
    } catch (err: any) {
      set({ error: err.message || 'Failed to publish offering.' });
    }
  }
}));
