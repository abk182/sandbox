import React from "react";
import { TargetIncomeForm } from "./target-income-form";
import useScriptLoader from "./script-loader";

export const rootDomNodeId = "app0.app0";

import "./index.global.css";

export const Ui = () => {
  useScriptLoader("http://localhost:8081/assets/client.js");
  return (
    <div className={"ui"}>
      <TargetIncomeForm />
    </div>
  );
};
