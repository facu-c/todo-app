mod modulos;

fn main() {
    use crate::modulos::app::run_app;

    println!("********************");
    println!("Bienvenido a Qacer");
    println!("********************");
    run_app();
}
