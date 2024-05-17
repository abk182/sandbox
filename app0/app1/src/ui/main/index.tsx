import React, { useEffect, useRef, useState } from "react";

import css from "./index.css";

export const Main = () => {
  const [currentIncome, setCurrentIncome] = useState(0);
  const [years, setYears] = useState(1);
  const [inflation, setInflation] = useState(5);
  const [wasmReady, setWasmReady] = useState(false);
  const wasmRef = useRef<typeof import("wasm")>(null);
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

  return (
    <div className={css.main}>
      <input
        placeholder="currentIncome"
        value={currentIncome}
        onChange={(e) => {
          if (!Number.isNaN(+e.target.value)) setCurrentIncome(+e.target.value);
        }}
      />
      <input
        placeholder="years"
        value={years}
        onChange={(e) => {
          if (!Number.isNaN(+e.target.value)) setYears(+e.target.value);
        }}
      />
      <input
        placeholder="years"
        value={inflation}
        onChange={(e) => {
          if (!Number.isNaN(+e.target.value)) setInflation(+e.target.value);
        }}
      />
      <div className="text-3xl font-bold underline">
        {wasmReady
          ? wasmRef.current.calc(currentIncome, years, inflation)
          : "loading..."}
      </div>
    </div>
  );
};
