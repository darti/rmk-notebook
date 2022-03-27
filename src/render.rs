use std::io::Write;

use log::info;
use lopdf::{
    content::{Content, Operation},
    dictionary, Dictionary, Document, Object, Stream,
};

use crate::{
    rm::{Line, Point},
    Notebook, Result,
};

impl Notebook {
    pub fn render<W: Write>(&self, target: &mut W) -> Result<()> {
        let paper_width = 210.;
        let paper_height = 297.;

        let screen_width = 1404.;
        let screen_height = 1872.;

        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 48.into()]),
                Operation::new("Td", vec![100.into(), 600.into()]),
                Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
                Operation::new("ET", vec![]),
            ],
        };

        // [ a b tx]
        // [ c d ty]
        // [ 0 0 1 ]

        // [a b c d tx ty]

        let screen_paper_ratio = paper_width / screen_width;

        let mut page_ids: Vec<Object> = Vec::with_capacity(self.pages.len());

        // 595 * 842 points. 1 pt = 1 / 72 inch, which means that the page is 21 * 29 cm large  = A4 size
        let media_box: Object = vec![0.into(), 0.into(), 595.into(), 842.into()].into();

        for page in &self.pages {
            // current_layer.add_operation(CurTransMat::Translate(Pt(0.), Pt(screen_height)));
            // current_layer.add_operation(CurTransMat::Scale(1.0, -1.0));

            // let content = Content { operations: vec![] };
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

            let page_dict = dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
            };

            page_ids.push(doc.add_object(page_dict).into());

            for layer in &page.layers {
                for line in &layer.lines {
                    // self.render_line(line, &current_layer);
                }
            }
        }

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => Object::Array(page_ids),
            "Count" => 1,//page_ids.len() as i32,
            "Resources" => resources_id,
             "MediaBox" => media_box,

        };

        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        doc.trailer.set("Root", catalog_id);
        doc.compress();
        doc.save_to(target)?;

        Ok(())
    }

    // fn render_line(&self, line: &Line, layer: &PdfLayerReference) {
    //     line.points.windows(2).for_each(|points| {
    //         let p0 = &points[0];
    //         let p1 = &points[1];

    //         let l = PdfLine {
    //             points: vec![p0.into(), p1.into()],
    //             is_closed: true,
    //             has_fill: true,
    //             has_stroke: true,
    //             is_clipping_path: false,
    //         };
    //         layer.add_shape(l);
    //     });
    // }
}
