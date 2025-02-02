use json::*;

mod jsonrpc;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let params = object! {
        "params": {
          "service": "common",
          "method": "authenticate",
          "args": [ "{{database}}", "{{username}}", "{{password}}", {} ]
        }
    };
    let req = jsonrpc::JsonRpcRequest::from(Some(params), 0);
    let client = jsonrpc::JsonRpcClient::new(String::from("http://localhost:80/jsonrpc"));

    let _resp = client.send(req).await; 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn json_parsing() {
        let parsed = json::parse(
            r#"

{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}

"#,
        )
        .unwrap();

        let instantiated = object! {
            // quotes on keys are optional
            "code": 200,
            success: true,
            payload: {
                features: [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ]
            }
        };

        assert_eq!(parsed, instantiated);
    }

    #[test]
    fn jsonrpc_test() {
        let params = object! {
            "params": {
              "service": "common",
              "method": "authenticate",
              "args": [ "{{database}}", "{{username}}", "{{password}}", {} ]
            }
        };
        let req = jsonrpc::JsonRpcRequest::from(Some(params), 0);
        let client =
            jsonrpc::JsonRpcClient::new(String::from("http://localhost:80/jsonrpc"));

        _ = client.send(req);
    }
}
