use once_cell::sync::Lazy;
use std::process;
use tera::Tera;

pub static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            process::exit(1);
        }
    };

    tera
});
