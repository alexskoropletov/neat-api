## Backend API built with Rust

# TODO: 
- [X] add .env file and expect variables to exist
- [ ] copy API routes structure from ur-be project backend
- [ ] apply JWT logic
- [ ] rebuid whole ur-be DB structure in memory
- [ ] connect to DB (https://github.com/iamhabbeboy/rest-api-actix-web/blob/master/src/db.rs)
    - [ ] DB migrations?
    - [ ] load common static data to memory (currencies, rates, etc.)
- [ ] use this backend in ur-be 
- [ ] build a cron job to update currency exchange rates
- [ ] cover with integration tests each endpoit


# API routes to implement
- [X] GET /health
- [X] POST /auth/login

- [ ] GET /account 
- [ ] GET /account/:id
- [ ] DELETE /account/:id
- [ ] POST /account

- [X] GET /currency/symbols 
- [X] GET /currency/base-currency
- [X] GET /currency/exchange-rates
- [X] GET /currency/update-rates

- [ ] GET /distribution-by-account 
- [ ] GET /distribution-by-account/:id
- [ ] DELETE /distribution-by-account/:id
- [ ] POST /distribution-by-account

- [ ] GET /distribution-by-source-of-income 
- [ ] GET /distribution-by-source-of-income/:id
- [ ] DELETE /distribution-by-source-of-income/:id
- [ ] POST /distribution-by-source-of-income
- [ ] GET /distribution-by-source-of-income/year/:year

- [ ] GET /goal 
- [ ] GET /goal/:id
- [ ] DELETE /goal/:id
- [ ] POST /goal

- [ ] GET /income-period

- [ ] GET /user
- [ ] GET /user/me

- [ ] GET /source-of-income 
- [ ] GET /source-of-income/:id
- [ ] DELETE /source-of-income/:id
- [ ] POST /source-of-income
- [ ] GET /source-of-income/year/:year
