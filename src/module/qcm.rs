use crate::module;
extern crate gtk;
use gtk::prelude::*;
use std::cell::RefCell; //Utilisation de la lib RefCell pour l'emplacement mémoire très utile pour la fonction ajout
use std::rc::Rc;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn qcm() {
    let glade_src = include_str!("../rust.glade");
    let builder = gtk::Builder::from_string(glade_src);

    //Affecter a des variables des widgets
    let window: gtk::Window = builder.get_object("applicationWindow").unwrap();
    let quitter: gtk::Button = builder.get_object("quitter").unwrap();
    let generer: gtk::Button = builder.get_object("generer").unwrap();
    let ajouter: gtk::Button = builder.get_object("ajouter").unwrap();
    let correction: gtk::Button = builder.get_object("correctio").unwrap();
    //Vecteur pour le stockage
    let stockage: Rc<RefCell<Vec<(String, String, String, String, String)>>> =
        Rc::new(RefCell::new(Vec::new()));
    let qcmm = module::QCM::build(builder.clone());
    //Boutton permettant de push d'autre struct dans un vecteur
    ajouter.connect_clicked(clone!(stockage => move |_| {
    let qcmstruct = qcmm.string();
    stockage.borrow_mut().push(qcmstruct);
    }));

    //Boutton qui va generer un fichier pdf
    generer.connect_clicked(clone!(stockage => move |_| {
        module::generate::generateqcm(
            &stockage
            );
    }));

    //Boutton anti-triche a venir
    correction.connect_clicked(clone!(stockage => move |_| {
        module::generate::gencorec(
            &stockage
            );
    }));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let window_clone = window.clone();
    quitter.connect_clicked(move |_| {
        window.close();
    });

    window_clone.show_all();

    gtk::main();
}
