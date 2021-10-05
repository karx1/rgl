# ConCurrency

[Live App](https://karx.xyz/rgl/concurrency/)

## Running Locally
1. Install dependencies
    ```bash
    cd /path/to/concurrency
    yarn install
    ```
2. Build project (be sure to supply the API_URL environment variable)
    - Linux/MacOS:
        ```bash
        API_URL=https://v6.exchangerate-api.com/v6/api-key/latest/USD/ yarn build
        ```
    - Windows:
        I'm not sure how to set environment variables on Windows, if anyone does, PRs are welcome!
3. Open in browser
    
    Simply open the `index.html` file in the `public/` directory.

<small>Created using [exchangerate-api](https://exchangerate-api.com) and React with TypeScript.</small>