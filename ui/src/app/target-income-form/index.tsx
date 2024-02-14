import React, { useState } from "react";
import { getTargetIncome } from "../../utils/get-target-salary";

export const TargetIncomeForm = () => {
  const [currentIncome, setCurrentIncome] = useState(0);
  const [years, setYears] = useState(0);

  return (
    <div>
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
      <div>{getTargetIncome(currentIncome, years)}</div>
    </div>
  );
};
