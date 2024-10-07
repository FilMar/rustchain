use crate::cripto_protocol::CriptoCurrency;
use axum::{extract::State, http::status, response::IntoResponse, Json};
use serde_json::{Map, Value};

type ArcChain = State<CriptoCurrency>;
// user to node
pub async fn create_transaction(
    State(mut criptochain): ArcChain,
    Json(data): Json<Map<String, Value>>,
) -> impl IntoResponse {
    let receiver = match data.get("to") {
        Some(Value::String(a)) => a,
        _ => {
            return (
                status::StatusCode::BAD_REQUEST,
                "serve chi riceve il pagamento",
            )
        }
    }
    .to_string();
    let amount = match data.get("amount") {
        Some(Value::String(a)) => a,
        _ => return (status::StatusCode::BAD_REQUEST, "serve quanto paga"),
    }
    .parse()
    .unwrap();
    criptochain
        .add_transaction(receiver, amount, 0.05)
        .await;
    (status::StatusCode::CREATED, "ok")
}

pub async fn get_chain(State(criptochain): ArcChain) -> impl IntoResponse {
    let res = match serde_json::to_string(&criptochain.get_chain().await) {
        Ok(e) => e,
        Err(e) => return (status::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    (status::StatusCode::OK, res)
}
// node to node
pub async fn add_external_blocks(
    State(mut criptochain): ArcChain,
    Json(data): Json<Vec<Map<String, Value>>>,
) -> impl IntoResponse {
    match criptochain.add_external_blocks(data).await {
        Ok(_) => (status::StatusCode::INTERNAL_SERVER_ERROR, "").into_response(),
        Err(e) => (status::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn add_external_node(
    State(criptochain): ArcChain,
    Json(data): Json<Map<String, Value>>,
) -> impl IntoResponse {
    match data.get("url") {
        Some(Value::String(url)) => {
            criptochain.add_node(url.clone()).await;
            (status::StatusCode::OK, format!("nodo {url} aggiunto con successo")).into_response()
        }
        None => (status::StatusCode::BAD_REQUEST, "url necessario").into_response(),
        _ =>  (status::StatusCode::BAD_REQUEST, "url di tipo errato").into_response(),
    }
}

// WARNING: per ora riceve una transazione per volta
pub async fn add_external_transaction(
    State(mut criptochain): ArcChain,
    Json(transaction): Json<Map<String, Value>>,
) -> impl IntoResponse {
    match (
        transaction.get("sender"),
        transaction.get("receiver"),
        transaction.get("amount"),
    ) {
        (
            Some(Value::String(sender)),
            Some(Value::String(receiver)),
            Some(Value::Number(amount)),
        ) => {
            criptochain
                .add_external_transaction(
                    sender.to_string(),
                    receiver.to_string(),
                    amount.as_f64().unwrap() as f32,
                    0.05,
                )
                .await;
            (status::StatusCode::CREATED, "").into_response()
        }
        (None, _, _) => (status::StatusCode::BAD_REQUEST, "serve quanto paga").into_response(),
        (_, None, _) => (status::StatusCode::BAD_REQUEST, "serve chi riceve").into_response(),
        (_, _, None) => (status::StatusCode::BAD_REQUEST, "serve quanto paga").into_response(),
        (_, _, _) => (
            status::StatusCode::BAD_REQUEST,
            "formato transazione non corretto",
        )
            .into_response(),
    }
}
