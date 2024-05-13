import React, { useEffect, useState } from "react";
import { getTargetIncome } from "../../utils/get-target-salary";
import '../wasm/bootstrap.js';

import css from "./index.css";

export const TargetIncomeForm = () => {
  const [currentIncome, setCurrentIncome] = useState(0);
  const [years, setYears] = useState(0);
  useEffect(() => {
    // const asyncEffect = async () => {
    //   //@ts-ignore
    //   const t = await wasm.default;

    //   console.log(t);
    // };

    // asyncEffect();
  }, []);

  return (
    <div className={css.form}>
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
      <div className="text-3xl font-bold underline">
        {getTargetIncome(currentIncome, years)}
      </div>
    </div>
  );
};
