use json;
use json::object;
use reqwest;
use std::mem::ManuallyDrop;

struct JsonRpcErrorDataContext {}

struct JsonRpcErrorData {
    name: String,
    debug: String,
    message: String,
    arguments: Vec<String>,
    context: JsonRpcErrorDataContext,
}

struct JsonRpcError {
    code: u32,
    message: String,
    data: Option<JsonRpcErrorData>,
}

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

pub struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<json::JsonValue>,
    error: Option<JsonRpcError>,
    id: u32,
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

    pub async fn send(self, request: JsonRpcRequest) -> Result<(), reqwest::Error>  {
        let resp = self
            .client
            .post(self.url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            .body(json::stringify(request))
            .send().await?;

        println!("{:?}", resp);

        Ok(())
    }
}
