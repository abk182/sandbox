import { useEffect, useRef, useState } from "react";

export const useWasm = () => {
  const [wasmReady, setWasmReady] = useState(false);
  const wasmRef = useRef<typeof import("wasm") | null>(null);
  useEffect(() => {
    import("wasm")
      .then((m) => {
        return m.default;
      })
      .then((m) => {
        wasmRef.current = m;
        setWasmReady(true);
      });
  }, []);

  if (wasmReady) {
    return wasmRef.current;
  }

  return null;
};
