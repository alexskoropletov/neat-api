## backend API built with Rust

TODO: 
    1) add .env file and expect variables to exist
    2) copy API routes structure from ur-be project backend
        GET /health
        POST /auth/login

        GET /account 
        GET /account/:id
        DELETE /account/:id
        POST /account

        GET /currency/symbols
        GET /currency/base-currency
        GET /currency/exchange-rates
        GET /currency/update-rates

        GET /distribution-by-account 
        GET /distribution-by-account/:id
        DELETE /distribution-by-account/:id
        POST /distribution-by-account

        GET /distribution-by-source-of-income 
        GET /distribution-by-source-of-income/:id
        DELETE /distribution-by-source-of-income/:id
        POST /distribution-by-source-of-income
        GET /distribution-by-source-of-income/year/:year

        GET /goal 
        GET /goal/:id
        DELETE /goal/:id
        POST /goal

        GET /income-period

        GET /user
        GET /user/me

        GET /source-of-income 
        GET /source-of-income/:id
        DELETE /source-of-income/:id
        POST /source-of-income
        GET /source-of-income/year/:year
    3) apply JWT logic
    4) rebuid whole ur-be DB structure in memory
    5) connect to DB (https://github.com/iamhabbeboy/rest-api-actix-web/blob/master/src/db.rs)
    5.1) DB migrations?
    6) use this backend in ur-be 