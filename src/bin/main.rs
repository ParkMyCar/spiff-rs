use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use spiff_rs::twilio::{read_config_file, TwilioClient, TwilioSMSRequestBody};
use twilio::twiml::{Action, Method, Sms};

fn spiff_hotline(_req: HttpRequest, form: web::Form<TwilioSMSRequestBody>) -> impl Responder {
    let twilio_sms_req = form.into_inner();
    println!(
        "Message from: {}\nBody: {}",
        twilio_sms_req.from().as_ref().unwrap(),
        twilio_sms_req.body().as_ref().unwrap()
    );
    let response = Sms {
        txt: String::from("This is a test message"),
        action: None,
        method: Method::Post,
        from: None,
        to: Some(twilio_sms_req.from().as_ref().unwrap().clone()),
        status_callback: None,
    };
    HttpResponse::Ok().body(response.as_twiml())
}

fn main() {
    HttpServer::new(|| {
        App::new().route(
            "/spiff",
            web::post()
                .guard(guard::Header("user-agent", "TwilioProxy/1.1"))
                .guard(guard::Header("connection", "close"))
                .to(spiff_hotline),
        )
    })
    .bind("0.0.0.0:767")
    .unwrap()
    .run()
    .unwrap();

    /*let config = read_config_file("api.toml.key").unwrap();
    let twilio = config.twilio.unwrap();
    let client = TwilioClient::new(
        twilio.account_sid.unwrap().clone(),
        twilio.auth_token.unwrap(),
    );
    client.send_sms(String::from("+12034480597"), String::from("this is a test"));*/
}
