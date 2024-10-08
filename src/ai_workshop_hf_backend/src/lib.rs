use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};

//https://api-inference.huggingface.co/models/google-t5/t5-base

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

    let cycles = 1_603_154_400u128;
    let args = CanisterHttpRequestArgument {
        url,
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: request_headers,
        body: Some(r#"{"inputs": "My name is Wolfgang and I live in Berlin"}"#.into()),
        transform: None,
    };

    let res = http_request(args, cycles).await.unwrap();
    ic_cdk::println!("{:?}", res.0);
    let response = String::from_utf8(res.0.body).unwrap();
    
    response
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
