pub trait InspectErr<Error> {
    fn inspect_error<F: FnOnce(&Error)>(self, f: F) -> Self;
}

impl<T, Error> InspectErr<Error> for Result<T, Error> {
    fn inspect_error<F: FnOnce(&Error)>(self, f: F) -> Self {
        if let Err(ref err) = self {
            f(err);
        }
        self
    }
}

#[cfg(test)]
mod test {
    use crate::error_logger::InspectErr;
    use log::{error, log, warn};

    #[test]
    fn log_err() {
        let error: Result<i32, String> = Err("Invalid number".to_string());

        if let Ok(nb) = error.inspect_error(|err| warn!("Got an error: {}", err)) {
            println!("Number: {}", nb);
        }
    }
}
