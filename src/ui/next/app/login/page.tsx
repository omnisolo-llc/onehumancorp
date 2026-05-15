"use client";
import { useRouter } from 'next/navigation';
import { useState } from 'react';

export default function Login() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [signUpMode, setSignUpMode] = useState(false);

  const handleSignIn = () => {
    setLoading(true);
    setTimeout(() => {
      setLoading(false);
      router.push('/onboarding');
    }, 500);
  };

  const handleSignUp = () => {
    setLoading(true);
    setTimeout(() => {
      setLoading(false);
      router.push('/onboarding');
    }, 500);
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50 font-outfit">
      <div className="bg-white p-8 rounded-xl shadow-lg w-full max-w-md backdrop-blur-md bg-opacity-90">
        <h1 className="text-3xl font-bold text-center mb-6 text-gray-800">One Human Corp</h1>
        <p className="text-center text-gray-500 mb-8">Your business, live in minutes.</p>

        <div className="space-y-4">
          <input
            type="email"
            placeholder="Email or Username"
            className="w-full px-4 py-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all outline-none"
          />
          <input
            type="password"
            placeholder="Password"
            className="w-full px-4 py-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all outline-none"
          />

          {!signUpMode ? (
            <>
              <button
                onClick={handleSignIn}
                disabled={loading}
                className="w-full bg-blue-600 text-white font-semibold py-3 rounded-lg hover:bg-blue-700 transition-colors shadow-sm disabled:opacity-50"
              >
                {loading ? 'Signing in...' : 'Sign In'}
              </button>
              <button
                onClick={() => setSignUpMode(true)}
                disabled={loading}
                className="w-full text-blue-600 font-medium py-2 hover:underline disabled:opacity-50"
              >
                Don't have an account? Sign Up
              </button>
            </>
          ) : (
            <>
              <button
                onClick={handleSignUp}
                disabled={loading}
                className="w-full bg-blue-600 text-white font-semibold py-3 rounded-lg hover:bg-blue-700 transition-colors shadow-sm disabled:opacity-50"
              >
                {loading ? 'Creating account...' : 'Sign Up'}
              </button>
              <button
                onClick={() => setSignUpMode(false)}
                disabled={loading}
                className="w-full text-blue-600 font-medium py-2 hover:underline disabled:opacity-50"
              >
                Already have an account? Sign In
              </button>
            </>
          )}

          <div className="relative flex items-center py-4">
            <div className="flex-grow border-t border-gray-300"></div>
            <span className="flex-shrink-0 mx-4 text-gray-400 text-sm">Or</span>
            <div className="flex-grow border-t border-gray-300"></div>
          </div>

          <button className="w-full bg-white border border-gray-300 text-gray-700 font-medium py-3 rounded-lg hover:bg-gray-50 transition-colors shadow-sm flex items-center justify-center gap-2">
            Continue with Google/Apple
          </button>
        </div>

        <div className="mt-8 flex flex-col gap-3">
          <button
            onClick={() => router.push('/onboarding')}
            className="w-full bg-gradient-to-r from-blue-500 to-indigo-600 text-white font-bold py-4 rounded-xl hover:shadow-lg transform hover:-translate-y-0.5 transition-all"
          >
            🚀 Start Business Setup
          </button>

          <button
            onClick={() => router.push('/onboarding')}
            className="w-full bg-gray-900 text-white font-bold py-4 rounded-xl hover:bg-gray-800 transition-colors"
          >
            🚀 Start My Business
          </button>

          <button
            onClick={() => router.push('/onboarding')}
            className="w-full text-indigo-600 font-bold py-3 hover:bg-indigo-50 rounded-lg transition-colors"
          >
            ⚡ Instant Build (AI) →
          </button>

          <button className="w-full text-gray-500 text-sm py-2 hover:text-gray-700 underline mt-4">
            Fix App Issues
          </button>
        </div>
      </div>
    </div>
  );
}
