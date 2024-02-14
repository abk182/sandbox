import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import express from "express";
import path from "path";
import { App } from "../app";
import { appId } from "../constants";

const startServer = () => {
  try {
    const port = 8080;

    const app = express();

    app.use("/client", express.static(path.join(__dirname, "../client")));

    app.get("/", (req, res) => {
      res.end(
        `<!DOCTYPE html>
          <html>
            <body>
              <div id=${appId}>${renderToStaticMarkup(<App />)}</div>
              <script src="client/index.js"></script>
            </body>
          </html>`
      );
    });

    app.listen(port, () => {
      console.log(`App listening on port ${port}`);
    });
  } catch (err) {
    console.log(err);
  }
};

startServer();
