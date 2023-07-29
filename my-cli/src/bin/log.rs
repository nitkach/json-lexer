use log::{debug, info, warn};
use log::{Level, Metadata, Record};

#[derive(Debug)]
enum Sharpness {
    Sharp,
    Dull,
}

#[derive(Debug)]
struct Yak {
    fur: Option<Wool>,
}

#[derive(Debug)]
enum Wool {
    Black,
    // Brown,
    // Gray,
}

impl Yak {
    fn shave(&mut self, razor: Sharpness) -> Option<Wool> {
        match razor {
            Sharpness::Sharp => {
                let Some(wool) = std::mem::take(&mut self.fur) else {
                    return None
                };
                Some(wool)
            }
            Sharpness::Dull => return None,
        }
    }
}

#[derive(Debug)]
enum ShaveError {
    NoWool,
    DullBlade,
}

fn shave_the_yak(yak: &mut Yak, razor: Result<Sharpness, ShaveError>) {
    info!(target: "yak", "Commencing yak shaving for {:?}", yak);

    loop {
        match razor {
            Ok(razor) => {
                info!("Razor located: {:?}", razor);
                debug!("Shaved wool: {:?}", yak.shave(razor));
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {:?}, retrying", err);
                break;
            }
        }
    }
}

fn main() {
    //     log::set_boxed_logger(Box::new(SimpleLogger)).unwrap();
    // env_logger::init();
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();

    // log::set_max_level(log::LevelFilter::Info);
    shave_the_yak(
        &mut Yak {
            fur: Some(Wool::Black),
        },
        Ok(Sharpness::Sharp),
    );
    shave_the_yak(
        &mut Yak {
            fur: Some(Wool::Black),
        },
        Ok(Sharpness::Dull),
    );
    shave_the_yak(&mut Yak { fur: Some(Wool::Black) }, Err(ShaveError::DullBlade));
    shave_the_yak(&mut Yak { fur: None }, Err(ShaveError::NoWool));

    // println!("{:?}", yak)
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
