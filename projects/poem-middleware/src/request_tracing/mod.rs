use ansi_term::Colour;
use poem::{Body, Endpoint, IntoResponse, Middleware, Request, Response};
use reqwest::Url;
use tracing::{
    Callsite, Event, Level, Metadata, Subscriber,
    callsite::Identifier,
    field::{Field, FieldSet, Visit},
    metadata::Kind,
    subscriber::Interest,
};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

/// Middleware for [`tracing`](https://crates.io/crates/tracing).
#[derive(Default)]
pub struct RequestTracing {}

impl<E: Endpoint> Middleware<E> for RequestTracing {
    type Output = TracingEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        TracingEndpoint { inner: ep }
    }
}

/// Endpoint for the `Tracing` middleware.
pub struct TracingEndpoint<E> {
    inner: E,
}

impl<E: Endpoint> Endpoint for TracingEndpoint<E> {
    type Output = Response;
    async fn call(&self, input: Request) -> poem::Result<Self::Output> {
        tracing::trace!("{:#?}", input);
        let (head, body) = input.into_parts();
        let next = match body.into_string().await {
            Ok(o) => {
                tracing::trace!("{}", o);
                Request::from_parts(head, Body::from(o))
            }
            Err(_) => Request::from_parts(head, Body::from(())),
        };
        Ok(self.inner.call(next).await?.into_response())
    }
}

//     Event::dispatch(&CUSTOM_META_DATA, &CUSTOM_META_DATA.fields().value_set(&[]));
pub struct CustomCallSite;
pub static CUSTOM_META_DATA: Metadata<'static> = Metadata::new(
    "log interest cache",
    "tracing",
    Level::TRACE,
    None,
    None,
    None,
    FieldSet::new(&["message"], Identifier(&CustomCallSite)),
    Kind::EVENT,
);

impl Callsite for CustomCallSite {
    fn set_interest(&self, _: Interest) {}

    fn metadata(&self) -> &Metadata<'_> {
        &CUSTOM_META_DATA
    }
}

pub struct PrintTracing {
    hiding: (),
}

impl PrintTracing {
    pub fn enable() {
        tracing_subscriber::registry().with(PrintTracing { hiding: () }).init();
    }
}

impl<S> Layer<S> for PrintTracing
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut visitor = TracingVisitor {};
        // print!("[{}]", Colour::Fixed(245).paint(Local::now().to_string()));
        let level_style = match *event.metadata().level() {
            Level::ERROR => Colour::Red.bold().paint("FATAL"),
            Level::WARN => Colour::Yellow.bold().paint("ERROR"),
            Level::INFO => Colour::Green.bold().paint("PRINT"),
            Level::DEBUG => Colour::Cyan.bold().paint("DEBUG"),
            Level::TRACE => Colour::Purple.bold().paint("TRACE"),
        };
        let module = match event.metadata().module_path() {
            Some(s) => {
                format!(" {}", Colour::White.dimmed().paint(s))
            }
            None => String::new(),
        };
        let path = match event.metadata().file() {
            Some(s) => Url::from_file_path(s).map(|s| s.to_string()).unwrap_or("Anonymous".to_string()),
            None => "Anonymous".to_string(),
        };
        let line = event.metadata().line().unwrap_or(0);
        println!("[{}{}] at {}:{}", level_style, module, path, line);
        event.record(&mut visitor);
    }
}

pub struct TracingVisitor {}

impl Visit for TracingVisitor {
    fn record_f64(&mut self, field: &Field, value: f64) {
        println!("[F] {}={}", field.name(), value)
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        println!("[I] {}={}", field.name(), value)
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        println!("[U] {}={}", field.name(), value)
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        println!("[B] {}={}", field.name(), value)
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "summary" => {}
            "json" => println!("```json5\n{}\n```", value.trim()),
            "db.statement" => println!("```sql\n{}\n```", value.trim()),
            _ => println!("[S] {}={:?}", field.name(), value),
        }
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        println!("[E] {}={:?}", field.name(), value)
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "message" => {
                println!("{:?}", value)
            }
            _ => println!("[D] {}={:?}", field.name(), value),
        }
    }
}
