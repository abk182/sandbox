import React from "react";
import { TargetIncomeForm } from "./target-income-form";

export const rootDomNodeId = "app0.app1";

import "./index.global.css";

export const Ui = () => {
  return (
    <div className={"ui"}>
      <TargetIncomeForm />
    </div>
  );
};
