use sims_data_cacher::run;


/*
* This microservice serves as a cache for frequently queried data from my primary API,
* https://github.com/Adriancoding96/smart-inventory-management-system.
* The main API tracks the frequency of specific endpoint calls, when a call count surpasses a set limit, the data is cached using this service.
* If the cache size surpasses its capacity, the least recently accessed data is removed from the cache, and an event is triggered
* to notify the main API that the data is no longer available.
*/

#[tokio::main]
async fn main() {
    run().await;
}

