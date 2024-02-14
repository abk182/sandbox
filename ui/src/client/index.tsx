import React from "react";
import { hydrateRoot } from "react-dom/client";
import { App, appId } from "../app";

hydrateRoot(document.getElementById(appId), <App />);
