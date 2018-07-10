/*
 * << Haru Free PDF Library 2.0.0 >> -- text_demo.c
 *
 * Copyright (c) 1999-2006 Takeshi Kanno <takeshi_kanno@est.hi-ho.ne.jp>
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.
 * It is provided "as is" without express or implied warranty.
 *
 */

//Original C file altered by Thoth Gunter to port for Rust

extern crate libharu_rs;
use std::ptr;
use libharu_rs::*;
use std::ffi::CString;

const font_list: [& str;14] = [
    "Courier",
    "Courier-Bold",
    "Courier-Oblique",
    "Courier-BoldOblique",
    "Helvetica",
    "Helvetica-Bold",
    "Helvetica-Oblique",
    "Helvetica-BoldOblique",
    "Times-Roman",
    "Times-Bold",
    "Times-Italic",
    "Times-BoldItalic",
    "Symbol",
    "ZapfDingbats",
];


extern fn error_handler(error_no: HPDF_STATUS,  detail_no: HPDF_STATUS, user_details: HPDF_HANDLE){ panic!("Some Error"); }

fn main()->(){
    println!("Begin!");
    unsafe{
        let pdf = HPDF_New(error_handler, ptr::null_mut());


        if (pdf == ptr::null_mut()) {
            println!("error: cannot create PdfDoc object\n");
            return ();
        }

        //if (setjmp(env)) {
        //    HPDF_Free (pdf);
        //    return 1;
        //}

        /* Add a new page object. */
        let mut page = HPDF_AddPage (pdf);

        let height = HPDF_Page_GetHeight (page);
        let width = HPDF_Page_GetWidth (page);

        /* Print the lines of the page. */
        HPDF_Page_SetLineWidth (page, 1.0);
        HPDF_Page_Rectangle (page, 50.0, 50.0, width - 100.0, height - 110.0);
        HPDF_Page_Stroke (page);


        //END OF TEST
        /* Print the title of the page (with positioning center). */
        let def_font = HPDF_GetFont (pdf, CString::new("Helvetica").unwrap().as_ptr(), ptr::null_mut());

        let page_title =CString::new("Helvetica").unwrap();
        HPDF_Page_SetFontAndSize (page, def_font, 24.0);

        let tw = HPDF_Page_TextWidth (page, page_title.as_ptr());
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, (width - tw) / 2.0, height - 50.0, page_title.as_ptr());
        HPDF_Page_EndText (page);

        /* output subtitle. */
        HPDF_Page_BeginText (page);
        HPDF_Page_SetFontAndSize (page, def_font, 16.0);
        HPDF_Page_TextOut (page, 60.0, height - 80.0, CString::new("<Standerd Type1 fonts samples>").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 60.0, height - 105.0);

        for font_name in font_list.iter(){
            let samp_text = CString::new("abcdefgABCDEFG12345!#$%&+-@?").unwrap();
            let font = HPDF_GetFont (pdf, CString::new(*font_name).unwrap().as_ptr(), ptr::null_mut());

            /* print a label of text */
            HPDF_Page_SetFontAndSize (page, def_font, 9.0);
            HPDF_Page_ShowText (page,  CString::new(*font_name).unwrap().as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -18.0);

            /* print a sample text. */
            HPDF_Page_SetFontAndSize (page, font, 20.0);
            HPDF_Page_ShowText (page, samp_text.as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -20.0);

        }
        
        HPDF_Page_EndText (page);

        HPDF_SaveToFile(pdf, CString::new("TEST.pdf").unwrap().as_ptr());
        HPDF_Free(pdf);
    }
    return ();
}
