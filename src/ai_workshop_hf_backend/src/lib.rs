use ic_cdk::{api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod
}, println};

//https://api-inference.huggingface.co/models/google-t5/t5-base

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    translation_text: String,
}

#[ic_cdk::update]
async fn translate_req(text: String) -> String{
    let host = "api-inference.huggingface.co";
    let url = format!("https://{host}/models/google-t5/t5-base");
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: "Bearer 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".to_string(),
        },
    ];

    let cycles = 0u128;
    let args = CanisterHttpRequestArgument {
        url,
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: request_headers,
        body: Some(format!(r#"{{"inputs": "{}"}}"#, text).into()),
        transform: None,
    };
    
    let res = http_request(args, cycles).await.unwrap();
    println!("{:?}", String::from_utf8(res.0.body.clone()));
    let response: (Response,) = serde_json::from_slice(&res.0.body).unwrap();
    
    response.0.translation_text
}
