import React from 'react';
import { PostPurchaseShareWidget } from '../../components/PostPurchaseShareWidget';

export default async function PostPurchaseShareEmbedPage({
  searchParams,
}: {
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  const resolvedParams = await searchParams;
  const tenantId = typeof resolvedParams.tenantId === 'string' ? resolvedParams.tenantId : 'demo-tenant';
  const orderId = typeof resolvedParams.orderId === 'string' ? resolvedParams.orderId : undefined;
  const storeName = typeof resolvedParams.storeName === 'string' ? resolvedParams.storeName : 'Our Store';

  return (
    <div className="min-h-screen bg-transparent flex flex-col items-center justify-center p-4">
      <div className="w-full max-w-2xl">
        <PostPurchaseShareWidget
          tenantId={tenantId}
          orderId={orderId}
          storeName={storeName}
        />
      </div>
    </div>
  );
}
