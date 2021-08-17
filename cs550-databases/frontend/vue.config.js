module.exports = {
  devServer: {
    proxy: `http://${process.env.DOCKER_UI ? "web" : "localhost"}:8888`,
  },
};
