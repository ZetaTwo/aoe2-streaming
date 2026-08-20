/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_STANDINGS_PROXY_URL: string
  /** Set to "true" to bypass the backend and render static fixture data instead. */
  readonly VITE_USE_MOCK_DATA?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
