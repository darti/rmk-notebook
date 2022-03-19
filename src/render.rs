use std::io::Write;

use printpdf::{Mm, PdfDocument};

use crate::{Notebook, Result};

impl Notebook {
    pub fn render(&self) -> Result<Vec<u8>> {
        let (doc, page1, layer1) =
            PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
        let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0), "Page 2, Layer 1");

        for page in &self.pages {}

        doc.save_to_bytes().map_err(Into::into)
    }
}
