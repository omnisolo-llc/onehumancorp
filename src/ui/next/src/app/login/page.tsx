'use client';
import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function Login() {
  const router = useRouter();

  return (
    <div>
      <input type="text" placeholder="Email or Username" />
      <input type="password" placeholder="Password" />
      <button onClick={() => router.push('/dashboard')}>Login</button>
    </div>
  );
}
