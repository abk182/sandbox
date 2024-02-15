import React from "react";
import { TargetIncomeForm } from "./target-income-form";

export const rootDomNodeId = "rootDomNodeId";

import "./index.global.css";

export const Ui = () => {
  return (
    <div className={"app"}>
      <TargetIncomeForm />
    </div>
  );
};
