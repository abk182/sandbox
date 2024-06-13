import React, { useState } from "react";
import { useWasm } from "../../wasm/use-wasm";
import { Image } from "../image";
import styles from "./style.css";

export const Cube = () => {
  const wasm = useWasm();
  const [width, setWidth] = useState(800);
  const [height, setHeight] = useState(600);
  const [x, setX] = useState(10);
  const [y, setY] = useState(10);
  const [z, setZ] = useState(10);

  return (
    <div className={styles.cube}>
      <div className={styles.inputs}>
        <input
          type="number"
          placeholder="width"
          value={width}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setWidth(+e.target.value);
          }}
        />
        <input
          type="number"
          placeholder="height"
          value={height}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setHeight(+e.target.value);
          }}
        />
        <input
          type="number"
          placeholder="width"
          value={x}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setX(+e.target.value);
          }}
        />
        <input
          type="number"
          placeholder="width"
          value={y}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setY(+e.target.value);
          }}
        />
        <input
          type="number"
          placeholder="height"
          value={z}
          onChange={(e) => {
            if (!Number.isNaN(+e.target.value)) setZ(+e.target.value);
          }}
        />
      </div>
      {wasm ? (
        <Image
          width={width}
          height={height}
          data={wasm.draw_cube(width, height, x, y, z)}
        />
      ) : (
        "loading..."
      )}
    </div>
  );
};
