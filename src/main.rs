extern crate gtk;
mod module;

fn main() {
    if gtk::init().is_err() {
        println!("Erreur de l'affichage du logiciel RstQcM  ");
        return;
    }
    module::qcm::qcm();
}
