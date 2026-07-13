export type WebSession = Readonly<{
  version: 1;
  iat: number;
  exp: number;
  accessToken: string;
  user: Readonly<{
    id: string;
    username: string;
    roles: readonly string[];
    organizationId: string;
  }>;
}>;

export type SessionCodecContext = Readonly<{
  audience: string;
  purpose: string;
}>;
