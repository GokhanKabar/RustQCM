pub mod qcm;
extern crate gtk;
pub mod generate;
use gtk::prelude::{BuilderExtManual, EntryExt};

#[derive(Debug, Clone)]
pub struct QCM {
    question: gtk::Entry,
    choix1: gtk::Entry,
    choix2: gtk::Entry,
    choix3: gtk::Entry,
    reponse: gtk::Entry,
}

//On transforme la struct en tuple pour le fichier pdf afin de choisir les elements a écrire sur le pdf
impl QCM {
    fn string(&self) -> (String, String, String, String, String) {
        let question = self.question.get_text().to_string();
        let choix1 = self.choix1.get_text().to_string();
        let choix2 = self.choix2.get_text().to_string();
        let choix3 = self.choix3.get_text().to_string();
        let reponse = self.reponse.get_text().to_string();
        return (question, choix1, choix2, choix3, reponse);
    }
    //Affectation widget au variable
    fn build(builder: gtk::Builder) -> QCM {
        return QCM {
            question: builder.get_object("question").unwrap(),
            choix1: builder.get_object("entrychoix1").unwrap(),
            choix2: builder.get_object("entrychoix2").unwrap(),
            choix3: builder.get_object("entrychoix3").unwrap(),
            reponse: builder.get_object("correction").unwrap(),
        };
    }
}
