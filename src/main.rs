use lambda_http::{aws_lambda_events::serde_json::json, Body, IntoResponse, Response};
use serde::Serialize;

struct PizzaList {
    pizzas: Vec<Pizza>,
}
#[derive(Serialize)]
struct Pizza {
    name: String,
    price: i32,
}

impl PizzaList {
    fn new() -> PizzaList {
        PizzaList {
            pizzas: vec![
                Pizza {
                    name: "veggie".to_string(),
                    price: 10,
                },
                Pizza {
                    name: "regina".to_string(),
                    price: 12,
                },
                Pizza {
                    name: "deluxe".to_string(),
                    price: 14,
                },
            ],
        }
    }
}

fn get_pizza_from_name<'a>(name: &'a str, pizza_list: &'a PizzaList) -> Option<&'a Pizza> {
    let mut iter = pizza_list.pizzas.iter();
    iter.find(|&pizza: &&Pizza| pizza.name == name)
}

async fn build_success_response(pizza: &Pizza) -> Response<Body> {
    json!(pizza).into_response().await
}

async fn build_failure_response(error_message: &str) -> Response<Body> {
    Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "error": error_message }).to_string()))
        .expect("could not build the error repsonse")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_pizza_list_test() {
        let all_pizza = PizzaList::new();
        assert_eq!(3, all_pizza.pizzas.len());

        let veggie = get_pizza_from_name("veggie", &all_pizza);
        assert_eq!(10, veggie.unwrap().price);

        let veggie = get_pizza_from_name("regina", &all_pizza);
        assert_eq!(12, veggie.unwrap().price);

        let veggie = get_pizza_from_name("deluxe", &all_pizza);
        assert_eq!(14, veggie.unwrap().price);
    }

    #[tokio::test]
    async fn build_success_response_test() {
        let test_pizza = Pizza {
            name: String::from("test_pizza"),
            price: 100,
        };
        let result = build_success_response(&test_pizza).await;
        let (parts, body) = result.into_parts();

        assert_eq!(200, parts.status);
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        assert_eq!(
            "{\"name\":\"test_pizza\",\"price\":100}",
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        );
    }

    #[tokio::test]
    async fn build_success_failure_test() {
        let result = build_failure_response("test error message").await;
        let (parts, body) = result.into_parts();

        assert_eq!(400, parts.status);
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        assert_eq!(
            "{\"error\":\"test error message\"}",
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        );
    }
}
