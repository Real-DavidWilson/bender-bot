use lazy_static::lazy_static;
use tera::Tera;

pub mod track_info;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_) => {
                ::std::process::exit(1);
            }
        };
        tera.full_reload().unwrap();
        tera.autoescape_on(vec![]);
        tera
    };
}
