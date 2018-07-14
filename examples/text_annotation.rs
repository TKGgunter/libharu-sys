/*
 * << Haru Free PDF Library 2.0.0 >> -- text_annotation.c
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

//Original C file altered by Thoth Gunter to port to Rust

extern crate libharu_sys;
extern crate libc;
use std::ptr;
use std::ffi::CString;
use libharu_sys::*;

extern fn error_handler  (error_no: HPDF_STATUS,
                detail_no: HPDF_STATUS,
                user_data : HPDF_HANDLE)
{
    println! ("ERROR: error_no={:#X}, detail_no={:#X}", error_no,
                detail_no);
}

macro_rules! cstring{
    ($fmt:expr) => {
        CString::new($fmt).unwrap()
    }
}



fn main()
{
    unsafe{
         let rect1  =   HPDF_Rect{left: 50.0,  right: 350.0, bottom: 150.0, top: 400.0};
         let rect2  =   HPDF_Rect{left: 210.0, right: 350.0, bottom: 350.0, top: 400.0};
         let rect3  =   HPDF_Rect{left: 50.0,  right: 250.0, bottom: 150.0, top: 300.0};
         let rect4  =   HPDF_Rect{left: 210.0, right: 250.0, bottom: 350.0, top: 300.0};
         let rect5  =   HPDF_Rect{left: 50.0,  right: 150.0, bottom: 150.0, top: 200.0};
         let rect6  =   HPDF_Rect{left: 210.0, right: 150.0, bottom: 350.0, top: 200.0};
         let rect7  =   HPDF_Rect{left: 50.0,  right:  50.0, bottom: 150.0, top: 100.0};
         let rect8  =   HPDF_Rect{left: 210.0, right:  50.0, bottom: 350.0, top: 100.0};

        let fname = cstring!("TEST.pdf");


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }


        /* use Times-Roman font. */
        let font = HPDF_GetFont (pdf, cstring!("Times-Roman").as_ptr(), cstring!("WinAnsiEncoding").as_ptr());

        let page = HPDF_AddPage (pdf);

        HPDF_Page_SetWidth (page, 400.0);
        HPDF_Page_SetHeight (page, 500.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_SetFontAndSize (page, font, 16.0);
        HPDF_Page_MoveTextPos (page, 130.0, 450.0);
        HPDF_Page_ShowText (page, cstring!("Annotation Demo").as_ptr());
        HPDF_Page_EndText (page);


        let mut  annot = HPDF_Page_CreateTextAnnot (page, rect1, cstring!("Annotation with Comment Icon. \n This annotation set to be opened initially.").as_ptr(),
                    ptr::null_mut());

        HPDF_TextAnnot_SetIcon (annot, HPDF_AnnotIcon::HPDF_ANNOT_ICON_COMMENT);
        HPDF_TextAnnot_SetOpened (annot, HPDF_TRUE);

        annot = HPDF_Page_CreateTextAnnot (page, rect2,
                    cstring!("Annotation with Key Icon").as_ptr(), ptr::null_mut());
        HPDF_TextAnnot_SetIcon (annot, HPDF_AnnotIcon::HPDF_ANNOT_ICON_PARAGRAPH);

        annot = HPDF_Page_CreateTextAnnot (page, rect3,
                    cstring!("Annotation with Note Icon").as_ptr(), ptr::null_mut());
        HPDF_TextAnnot_SetIcon (annot, HPDF_AnnotIcon::HPDF_ANNOT_ICON_NOTE);

        annot = HPDF_Page_CreateTextAnnot (page, rect4,
                    cstring!("Annotation with Help Icon").as_ptr(), ptr::null_mut());
        HPDF_TextAnnot_SetIcon (annot, HPDF_AnnotIcon::HPDF_ANNOT_ICON_HELP);

        annot = HPDF_Page_CreateTextAnnot (page, rect5,
                    cstring!("Annotation with NewParagraph Icon").as_ptr(), ptr::null_mut());
        HPDF_TextAnnot_SetIcon (annot, HPDF_AnnotIcon::HPDF_ANNOT_ICON_NEW_PARAGRAPH);

        annot = HPDF_Page_CreateTextAnnot (page, rect6,
                    cstring!("Annotation with Paragraph Icon").as_ptr(), ptr::null_mut());
        HPDF_TextAnnot_SetIcon (annot, HPDF_AnnotIcon::HPDF_ANNOT_ICON_PARAGRAPH);

        annot = HPDF_Page_CreateTextAnnot (page, rect7,
                    cstring!("Annotation with Insert Icon").as_ptr(), ptr::null_mut());
        HPDF_TextAnnot_SetIcon (annot, HPDF_AnnotIcon::HPDF_ANNOT_ICON_INSERT);

        let encoding = HPDF_GetEncoder (pdf, cstring!("ISO8859-2").as_ptr());

        HPDF_Page_CreateTextAnnot (page, rect8,
                    cstring!("Annotation with ISO8859 text ").as_ptr(), encoding);

        HPDF_Page_SetFontAndSize (page, font, 11.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect1.left + 35.0, rect1.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("Comment Icon.").as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect2.left + 35.0, rect2.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("Key Icon").as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect3.left + 35.0, rect3.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("Note Icon.").as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect4.left + 35.0, rect4.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("Help Icon").as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect5.left + 35.0, rect5.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("NewParagraph Icon").as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect6.left + 35.0, rect6.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("Paragraph Icon").as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect7.left + 35.0, rect7.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("Insert Icon").as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, rect8.left + 35.0, rect8.top - 20.0);
        HPDF_Page_ShowText (page, cstring!("Text Icon(ISO8859-2 text)").as_ptr());
        HPDF_Page_EndText (page);


        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }
    return ;
}
