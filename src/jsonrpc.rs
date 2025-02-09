use std::collections::HashMap;

use json;
use reqwest;

#[derive(Debug)]
struct JsonRpcErrorDataContext {}

#[derive(Debug)]
struct JsonRpcErrorData {
    name: String,
    debug: String,
    message: String,
    arguments: Vec<String>,
    context: JsonRpcErrorDataContext,
}

#[derive(Debug)]
struct JsonRpcError {
    code: u32,
    message: String,
    data: Option<JsonRpcErrorData>,
}

#[derive(Debug)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<json::JsonValue>,
    id: u32,
}

impl JsonRpcRequest {
    pub fn new(method: String, params: Option<json::JsonValue>, id: u32) -> Self {
        Self {
            jsonrpc: String::from("2.0"),
            method,
            params,
            id,
        }
    }

    pub fn from(params: Option<json::JsonValue>, id: u32) -> Self {
        Self {
            jsonrpc: String::from("2.0"),
            method: String::from("call"),
            params,
            id,
        }
    }
}

impl Into<json::JsonValue> for JsonRpcRequest {
    fn into(self) -> json::JsonValue {
        json::object! {
            "jsonrpc" => self.jsonrpc,
            "method" => self.method,
            "params" => self.params,
            "id" => self.id
        }
    }
}

#[derive(Debug)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<json::JsonValue>,
    error: Option<json::JsonValue>,
    id: u32,
}

//impl From<Result<json::JsonValue, json::JsonError>> for JsonRpcResponse {
//    fn from(result: Result<json::JsonValue, json::JsonError>) -> JsonRpcResponse {
//        let mut entries = result.ok().unwrap().e
//        entries.g
//        JsonRpcResponse {
//           jsonrpc: "2.0".to_string(),
//           id: 0,
//           result: result.ok().unwrap().as,
//           error: None,
//        }
//    }
//}

impl From<Result<json::JsonValue, json::JsonError>> for JsonRpcResponse {
    fn from(result: Result<json::JsonValue, json::JsonError>) -> JsonRpcResponse {
        match result {
            Ok(json_value) => {
                //let result_value: HashMap<String, json::JsonValue> = json_value.into(); // Extract the "result" key

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: 0,
                    result: Some(json_value["result"].clone()), // Use the extracted result
                    error: Some(json_value["error"].clone()),
                }
            }
            Err(err) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: 0,
                result: None,
                error: None, // Convert error to string
            },
        }
    }
}

pub struct JsonRpcClient {
    url: String,
    client: reqwest::Client,
}

impl JsonRpcClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn send(self, request: JsonRpcRequest) -> Result<JsonRpcResponse, reqwest::Error> {
        //println!("{:?}", json::stringify(request));

        let resp = self
            .client
            .post(self.url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            .body(json::stringify(request))
            .send()
            .await?; 

        let resp_body = resp.text_with_charset("UTF8").await?;
        //println!("{:?}", &resp_body);

        let resp_object = json::parse(resp_body.as_str());
        let desirialized: JsonRpcResponse = resp_object.into();

        println!("{:?}", desirialized);

        Ok(desirialized)
    }
}
