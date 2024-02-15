const path = require("path");

const config = {
  mode: "development",
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/,
        use: "ts-loader",
        include: path.resolve(__dirname, "src"),
      },
      {
        test: /\.css$/,
        use: [
          "style-loader",
          {
            loader: "css-loader",
            options: {
              modules: {
                localIdentName: "[name]_[local]_[hash:base64]"
              },
            },
          },
        ],
        include: path.resolve(__dirname, "src"),
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
};

const serverConfig = Object.assign(
  {
    target: "node",
    entry: "./src/server/index.tsx",
    output: {
      filename: "server.js",
      path: path.resolve(__dirname, "dist"),
    },
  },
  config
);

const clientConfig = Object.assign(
  {
    target: "web",
    entry: "./src/client/index.tsx",
    output: {
      filename: "assets/client.js",
      path: path.resolve(__dirname, "dist"),
    },
  },
  config
);

module.exports = [serverConfig, clientConfig];
