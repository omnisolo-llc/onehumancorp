'use client';
import { useRouter } from 'next/navigation';

export default function Login() {
  const router = useRouter();

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    router.push('/dashboard');
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="max-w-md w-full p-8 bg-white rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-6">Login</h1>
        <form onSubmit={handleLogin}>
          <div className="mb-4">
            <input
              type="text"
              placeholder="Email or Username"
              className="w-full p-2 border rounded"
              required
            />
          </div>
          <div className="mb-6 relative">
            <input
              type="password"
              placeholder="Password"
              className="w-full p-2 border rounded"
              required
            />
            <button
              type="button"
              className="absolute right-2 top-2 text-sm text-gray-500"
            >
              Show
            </button>
          </div>
          <button
            type="submit"
            className="w-full bg-blue-600 text-white p-2 rounded font-bold"
          >
            Login
          </button>
        </form>
      </div>
    </div>
  );
}
