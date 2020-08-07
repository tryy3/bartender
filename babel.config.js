var path = require("path");

function resolve(dir) {
    return path.join(__dirname, dir);
}

module.exports = {
    plugins: [
        require.resolve("babel-plugin-import-graphql"),
        require.resolve("babel-plugin-lodash"),
        [
            require.resolve("babel-plugin-module-resolver"),
            {
                alias: {
                    "@": resolve("src")
                }
            }
        ]
    ],
    presets: ["@vue/cli-plugin-babel/preset"]
};
