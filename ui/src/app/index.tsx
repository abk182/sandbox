import React from "react";
import { TargetIncomeForm } from "./target-income-form";

export const appId = "app";

import "./index.global.css";

export const App = () => {
  return (
    <div className={"app"}>
      <TargetIncomeForm />
    </div>
  );
};
