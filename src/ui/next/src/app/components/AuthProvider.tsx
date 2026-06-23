"use client";

import React, { createContext, useContext, useState, useEffect } from 'react';

interface AuthSession {
  tenant_id: string;
  access_token: string;
}

interface AuthContextType {
  session: AuthSession | null;
}

const AuthContext = createContext<AuthContextType>({ session: null });

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [session, setSession] = useState<AuthSession | null>(null);

  useEffect(() => {
    // Simulated session lookup for UI build/test
    const tid = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') : null;
    const tok = typeof localStorage !== 'undefined' ? localStorage.getItem('access_token') : null;
    if (tid && tok) {
      setSession({ tenant_id: tid, access_token: tok });
    }
  }, []);

  return (
    <AuthContext.Provider value={{ session }}>
      {children}
    </AuthContext.Provider>
  );
}

export const useAuth = () => useContext(AuthContext);
