/**
 * @type { import("next").NextConfig}
 */
const config = {
  webpack: (config) => ({
    ...config, experiments: {
      asyncWebAssembly: true,
      layers: true
    },
  })
};
module.exports = config;