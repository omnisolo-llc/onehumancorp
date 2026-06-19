import { create } from 'zustand';

interface CartItem {
  id: string;
  product_id: string;
  name: string;
  price_cents: number;
  quantity: number;
  image_url?: string;
}

interface CartState {
  items: CartItem[];
  addItem: (product: any) => void;
  removeItem: (productId: string) => void;
  updateQuantity: (productId: string, quantity: number) => void;
  clearCart: () => void;
  totalCents: () => number;
}

export const useCartStore = create<CartState>((set, get) => ({
  items: [],
  addItem: (product) => {
    const items = get().items;
    const existingItem = items.find((item) => item.product_id === product.id);
    if (existingItem) {
      set({
        items: items.map((item) =>
          item.product_id === product.id
            ? { ...item, quantity: item.quantity + 1 }
            : item
        ),
      });
    } else {
      set({
        items: [
          ...items,
          {
            id: crypto.randomUUID(),
            product_id: product.id,
            name: product.name || product.title,
            price_cents: product.price_cents,
            quantity: 1,
            image_url: product.metadata?.image_url || '/placeholder-product.png',
          },
        ],
      });
    }
  },
  removeItem: (productId) =>
    set({ items: get().items.filter((item) => item.product_id !== productId) }),
  updateQuantity: (productId, quantity) =>
    set({
      items: get().items.map((item) =>
        item.product_id === productId ? { ...item, quantity } : item
      ),
    }),
  clearCart: () => set({ items: [] }),
  totalCents: () =>
    get().items.reduce((acc, item) => acc + item.price_cents * item.quantity, 0),
}));
