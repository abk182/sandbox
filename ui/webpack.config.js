const path = require("path");

const config = {
  mode: "development",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
    fallback: {
      http: false,
      "buffer": require.resolve("buffer"),
      "url": require.resolve("url")
    },
  },
}

const serverConfig = Object.assign({
  target: 'node',
  entry: "./src/server/index.tsx",
  output: {
    filename: "server/index.js",
    path: path.resolve(__dirname, "dist"),
  },
}, config);

const clientConfig = Object.assign({
  target: 'web',
  entry: "./src/client/index.tsx",
  output: {
    filename: "client/index.js",
    path: path.resolve(__dirname, "dist"),
  },
}, config)

module.exports = [serverConfig, clientConfig];