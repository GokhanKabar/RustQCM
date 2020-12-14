pub mod qcm;
use crate::module;
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
