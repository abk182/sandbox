import React from "react";
import { TargetIncomeForm } from "./target-income-form";

export const appId = "app";

import css from "./index.css";

export const App = () => {
  return (
    <div className={css.app}>
      <TargetIncomeForm />
    </div>
  );
};
