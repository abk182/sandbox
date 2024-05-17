import React, { useState } from "react";
import { useWasm } from "../wasm/use-wasm";
import css from "./index.css";
import { PpmImage } from "./ppm-image";

export const Main = () => {
  const [currentIncome, setCurrentIncome] = useState(100);
  const [years, setYears] = useState(10);
  const [inflation, setInflation] = useState(5);
  const [width, setWidth] = useState(256);
  const [height, setHeight] = useState(256);
  const wasm = useWasm();

  return (
    <div className={css.main}>
      <div>
        <input
          placeholder="currentIncome"
          value={currentIncome}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value))
              setCurrentIncome(+e.target.value);
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
          placeholder="inflation"
          value={inflation}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setInflation(+e.target.value);
          }}
        />
        <div className="text-3xl font-bold underline">
          {wasm ? wasm.calc(currentIncome, years, inflation) : "loading..."}
        </div>
      </div>
      <div>
        <input
          placeholder="width"
          value={width}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setWidth(+e.target.value);
          }}
        />
        <input
          placeholder="height"
          value={height}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setHeight(+e.target.value);
          }}
        />
        {wasm ? (
          <PpmImage
            width={width}
            height={height}
            data={wasm.draw_ppm_image(width, height)}
          />
        ) : (
          "loading..."
        )}
      </div>
    </div>
  );
};
