import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface VoiceAgentState {
  phone_number: string;
  is_enabled: boolean;
  primary_language: string;
  custom_instructions: string;
  allow_orders: boolean;
  allow_booking: boolean;
  setPhoneNumber: (phone: string) => void;
  setIsEnabled: (enabled: boolean) => void;
  setPrimaryLanguage: (lang: string) => void;
  setCustomInstructions: (instructions: string) => void;
  setAllowOrders: (allow: boolean) => void;
  setAllowBooking: (allow: boolean) => void;
}

export const useVoiceAgentStore = create<VoiceAgentState>()(
  persist(
    (set) => ({
      phone_number: '',
      is_enabled: false,
      primary_language: 'English',
      custom_instructions: '',
      allow_orders: false,
      allow_booking: false,
      setPhoneNumber: (phone) => set({ phone_number: phone }),
      setIsEnabled: (enabled) => set({ is_enabled: enabled }),
      setPrimaryLanguage: (lang) => set({ primary_language: lang }),
      setCustomInstructions: (instructions) => set({ custom_instructions: instructions }),
      setAllowOrders: (allow) => set({ allow_orders: allow }),
      setAllowBooking: (allow) => set({ allow_booking: allow }),
    }),
    {
      name: 'voice-agent-storage',
    }
  )
);
