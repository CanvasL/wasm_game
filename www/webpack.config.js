const path = require('path')
const CopyWebpackPlugin = require('copy-webpack-plugin')

module.exports = {
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, 'public'),
        filename: 'index.js'
    },
    mode: 'development',
    plugins: [
        new CopyWebpackPlugin({
            patterns: [{
                from: './src/index.html',
                to: './'
            }]
        })
    ]
}