use printpdf::*;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;

pub fn generateqcm(stockage: &Rc<RefCell<Vec<(String, String, String, String, String)>>>) {
    let (doc, page, layer) = PdfDocument::new("RstQcM", Mm(200.0), Mm(300.0), "Layer");
    let current_layer = doc.get_page(page).get_layer(layer);

    let font = doc
        .add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap())
        .unwrap();
    current_layer.add_line_break();
    //Commencement de la zone d'ecriture
    current_layer.begin_text_section();
    current_layer.set_text_cursor(Mm(10.0), Mm(280.0));
    current_layer.set_font(&font, 14.0);
    current_layer.set_line_height(15.0);
    current_layer.set_word_spacing(1500.0);
    current_layer.write_text(
        "                                                        *********RstQcM*******         ",
        &font,
    );
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Entourer la bonne réponse", &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    for qcmm in stockage.borrow_mut().iter() {
        current_layer.write_text("Question : ", &font);
        current_layer.write_text(qcmm.0.clone(), &font);
        current_layer.add_line_break();
        current_layer.add_line_break();
        current_layer.write_text("Choix 1 : ", &font);
        current_layer.write_text(qcmm.1.clone(), &font);
        current_layer.add_line_break();
        current_layer.write_text("Choix 2 : ", &font);
        current_layer.write_text(qcmm.2.clone(), &font);
        current_layer.add_line_break();
        current_layer.write_text("Choix 3 : ", &font);
        current_layer.write_text(qcmm.3.clone(), &font);
        current_layer.add_line_break();
        current_layer.add_line_break();
    }
    current_layer.end_text_section(); //Fin de la zone d'ecriture

    doc.save(&mut BufWriter::new(File::create("qcm.pdf").unwrap()))
        .unwrap();
}
pub fn gencorec(stockage: &Rc<RefCell<Vec<(String, String, String, String, String)>>>) {
    let (doc, page, layer) = PdfDocument::new("RstQcM", Mm(200.0), Mm(300.0), "Layer");
    let current_layer = doc.get_page(page).get_layer(layer);

    let font = doc
        .add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap())
        .unwrap();
    current_layer.add_line_break();
    //Commencement de la zone d'ecriture
    current_layer.begin_text_section();
    current_layer.set_text_cursor(Mm(10.0), Mm(280.0));
    current_layer.set_font(&font, 14.0);
    current_layer.set_line_height(15.0);
    current_layer.set_word_spacing(1500.0);
    current_layer.write_text(
        "                                                        *********RstQcM*******         ",
        &font,
    );
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    for qcmm in stockage.borrow_mut().iter() {
        current_layer.write_text("Réponse", &font);
        current_layer.add_line_break();
        current_layer.write_text(qcmm.4.clone(), &font);
    }
    current_layer.end_text_section(); //Fin de la zone d'ecriture

    doc.save(&mut BufWriter::new(File::create("correction.pdf").unwrap()))
        .unwrap();
}
