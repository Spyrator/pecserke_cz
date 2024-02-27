use payment_strings::{Currency, Payment, PaymentEncoding};
use spin_sdk::http::{conversions::TryFromBody, IntoResponse, Json, Params, Request, Response};
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct PaymentForm {
    iban: String,
    recipient: String,
    amount: String,
    currency: String,
    message: String,
}

pub fn pay_me_amount(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let amount = params.get("amount").expect("amount").parse::<u32>()?;
    let message = "".to_owned();

    let payment = Payment {
        iban: "CZ8120100000002000466782".into(),
        swift: "FIOBCZPPXXX".to_owned(),
        amount,
        currency: Currency::CZK,
        message,
        recipient_name: "Filip Pecsérke".to_owned(),
    };
    let payment_string = payment.string(&PaymentEncoding::SPD);
    let svg_qr = qr_from_str::svg(&payment_string);

    let res = Response::builder().status(200).body(svg_qr).build();

    return Ok(res);
}

pub fn pay_me_json(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let Json(PaymentForm {
        iban,
        recipient,
        amount,
        currency,
        message,
    }) = Json::<PaymentForm>::try_from_body(req.into_body())?;

    let payment = Payment {
        iban,
        swift: "".to_owned(),
        amount: amount.parse().expect("a number"),
        currency: Currency::from_str(&currency).unwrap_or(Currency::CZK),
        message,
        recipient_name: recipient,
    };
    let payment_string = payment.string(&PaymentEncoding::SPD);
    let svg_qr = qr_from_str::svg(&payment_string);

    let res = Response::builder().status(200).body(svg_qr).build();

    return Ok(res);
}
