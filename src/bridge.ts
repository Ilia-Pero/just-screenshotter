/** The bridge to the Tauri shell: it calls commands written in Rust. */
export interface Bridge {
  invoke<T>(command: string, payload?: Uint8Array | Readonly<Record<string, unknown>>): Promise<T>;
}

declare global {
  interface Window {
    /**
     * Tauri adds this bridge to each of its windows. The package @tauri-apps/api calls it too;
     * calling it directly spares every window a script of 41 KB behind window.__TAURI__.
     */
    readonly __TAURI_INTERNALS__: Bridge;
  }
}
