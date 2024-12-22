use ast::structure::Programm;

mod ast;

fn main() {
    //let prog = Programm::new("test.stz".to_string());

    Programm::pars("test.stz".to_string());
}
