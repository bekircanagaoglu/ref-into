use ref_into::RefInto;

fn main() {
    let endpoint = Endpoints::Orders;
    let api_route = endpoint.ref_into();

    println!("Api route: {:?}", api_route);
    println!("We still have the endpoint : {:?}", endpoint);

}

#[derive(Debug)]
struct ApiRoute(String);

#[derive(Debug)]
enum Endpoints {
    Orders,
    Customers
}

impl RefInto<ApiRoute> for Endpoints {
    fn ref_into(&self) -> ApiRoute {
        match self {
            Endpoints::Orders => ApiRoute("/api/v1/orders".to_string()),
            Endpoints::Customers => ApiRoute("/api/v1/orders".to_string()),
        }
    }
}
