const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

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
        exclude: /\.global\.css$/,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: "css-loader",
            options: {
              modules: {
                localIdentName: "[name]_[local]_[hash:base64]",
              },
            },
          },
          { loader: "postcss-loader" },
        ],
        include: path.resolve(__dirname, "src"),
      },
      {
        test: /\.global\.css$/,
        use: [MiniCssExtractPlugin.loader, , "css-loader", "postcss-loader"],
        include: path.resolve(__dirname, "src"),
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  plugins: [new MiniCssExtractPlugin()],
  experiments: {
    asyncWebAssembly: true,
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
      filename: "client.js",
      path: path.resolve(__dirname, "dist"),
    },
  },
  config
);

module.exports = [serverConfig, clientConfig];
