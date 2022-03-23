use log::info;
use printpdf::{
    lopdf::{content::Operation, Object},
    CurTransMat, Line as PdfLine, Mm, PdfDocument, PdfLayerReference, Point as PdfPoint, Pt,
};

use crate::{
    rm::{Line, Point},
    Notebook, Result,
};

impl From<&Point> for (PdfPoint, bool) {
    fn from(p: &Point) -> Self {
        (PdfPoint::new(Mm(p.x as f64), Mm(p.y as f64)), false)
    }
}

impl Notebook {
    pub fn render(&self) -> Result<Vec<u8>> {
        let paper_width = 210.;
        let paper_height = 297.;

        let screen_width = 1404.;
        let screen_height = 1872.;

        let x_mm = Mm(paper_width);
        let y_mm = Mm(paper_height);

        // let x_mm = Mm(screen_width);
        // let y_mm = Mm(screen_height);

        // [ a b tx]
        // [ c d ty]
        // [ 0 0 1 ]

        // [a b c d tx ty]

        let screen_paper_ratio = paper_width / screen_width;

        info!("screen_paper_ratio: {}", 1. / screen_paper_ratio);

        let transform = CurTransMat::Raw([
            screen_paper_ratio,
            0.0,
            0.0,
            -screen_paper_ratio,
            0.0,
            840.0,
        ]);

        let doc = PdfDocument::empty(self.metadata.visible_name.clone());

        for page in &self.pages {
            let (page_index, layer_index) = doc.add_page(x_mm, y_mm, "Layer 1");
            let current_layer = doc.get_page(page_index).get_layer(layer_index);

            // current_layer.add_operation(CurTransMat::Translate(Pt(0.), Pt(screen_height)));
            // current_layer.add_operation(CurTransMat::Scale(1.0, -1.0));

            current_layer.add_operation(transform);

            for layer in &page.layers {
                for line in &layer.lines {
                    self.render_line(line, &current_layer);
                }
            }
        }

        doc.save_to_bytes().map_err(Into::into)
    }

    fn render_line(&self, line: &Line, layer: &PdfLayerReference) {
        line.points.windows(2).for_each(|points| {
            let p0 = &points[0];
            let p1 = &points[1];

            let l = PdfLine {
                points: vec![p0.into(), p1.into()],
                is_closed: true,
                has_fill: true,
                has_stroke: true,
                is_clipping_path: false,
            };
            layer.add_shape(l);
        });
    }
}
