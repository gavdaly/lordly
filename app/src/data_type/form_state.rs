use std::marker::PhantomData;

struct FormState<P> {
    state: PhantomData<P>,
}

struct Waiting;
struct Sending;
struct Success;
struct Invalid(String);

impl FormState {
    fn new() -> Self {
        FormState { state: PhantomData }
    }
}

trait FormState {
    fn process(data: &str) -> Self;
}

impl FormState<Waiting> {
    fn process(data: &str) -> FormState<Sending> {
        println!("Processing data: {}", data);
        match reqwest::blocking::Client::new()
            .post(data)
            .body(data.to_string())
            .send() {
                Some(_) => FormState { state: PhantomData::<Success> },
                Err(e) => FormState { state: PhantomData::<Invalid(e.to_string())> },
            }
        FormState { state: PhantomData }
    }
}
