use crate::module;
mod generationqcm;
extern crate gtk;
use gtk::prelude::*;
use std::cell::RefCell; //Utilisation de la lib RefCell pour l'emplacement mémoire très utile pour la fonction ajout
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct QCM {
    question: String,
    choix1: String,
    choix2: String,
    choix3: String,
}

//https://gtk-rs.org/docs-src/tutorial/closures
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
    let corec: gtk::Button = builder.get_object("corec").unwrap();
    let ques: gtk::Entry = builder.get_object("question").unwrap();
    let ch1: gtk::Entry = builder.get_object("entrychoix1").unwrap();
    let ch2: gtk::Entry = builder.get_object("entrychoix2").unwrap();
    let ch3: gtk::Entry = builder.get_object("entrychoix3").unwrap();

    //Vecteur pour le stockage
    let stockage: Rc<RefCell<Vec<QCM>>> = Rc::new(RefCell::new(Vec::new()));

    //Boutton permettant de push d'autre struct dans un vecteur
    ajouter.connect_clicked(clone!(stockage => move |_| {
        let qcmstruct = QCM {
            question : ques.get_text().to_string(),//Récupération des données de l'utilisateur
            choix1 : ch1.get_text().to_string(),
            choix2 : ch2.get_text().to_string(),
            choix3 : ch3.get_text().to_string(),
        };
        stockage.borrow_mut().push(qcmstruct);//Ajout de la struct dans le vecteur
        println!("{:#?}",&stockage);//Affichage du vecteur
    }));

    //Boutton qui va generer un fichier pdf
    generer.connect_clicked(clone!(stockage => move |_| {
   
    }));
    
//Boutton correction a venir
    corec.connect_clicked(clone!(stockage => move |_| {
        
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
