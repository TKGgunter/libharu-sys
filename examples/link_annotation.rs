/*
 * << Haru Free PDF Library 2.0.0 >> -- link_annotation.c
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
    };
}

fn print_page  (page : HPDF_Page, font : HPDF_Font, page_num : i32 )
{
    unsafe{
        HPDF_Page_SetWidth (page, 200.0);
        HPDF_Page_SetHeight (page, 200.0);

        HPDF_Page_SetFontAndSize (page, font, 20.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 50.0, 150.0);

        let buf = format!("Page:{}", page_num);
        let temp_buf =    buf.as_str();

        HPDF_Page_ShowText (page, cstring!(temp_buf).as_ptr());
        HPDF_Page_EndText (page);
    }
}

fn main()
{
    unsafe{
        let mut page = Vec::<HPDF_Page>::new();
        let fname = cstring!("TEST.pdf");
        let mut rect = HPDF_Rect{left: 0.0, right: 0.0, top:0.0, bottom:0.0};
        let url = cstring!("http://libharu.org");


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }


        /* create default-font */
        let font = HPDF_GetFont (pdf, cstring!("Helvetica").as_ptr(), ptr::null_mut());

        /* create index page */
        let index_page = HPDF_AddPage (pdf);
        HPDF_Page_SetWidth (index_page, 300.0);
        HPDF_Page_SetHeight (index_page, 220.0);

        /* Add 7 pages to the document. */
        for i in 0..7{
            page.push( HPDF_AddPage (pdf) );
            print_page(page[i], font, i as i32 + 1);
        }

        HPDF_Page_BeginText (index_page);
        HPDF_Page_SetFontAndSize (index_page, font, 10.0);
        HPDF_Page_MoveTextPos (index_page, 15.0, 200.0);
        HPDF_Page_ShowText (index_page, cstring!("Link Annotation Demo").as_ptr());
        HPDF_Page_EndText (index_page);

        /*
         * Create Link-Annotation object on index page.
         */
        HPDF_Page_BeginText(index_page);
        HPDF_Page_SetFontAndSize (index_page, font, 8.0);
        HPDF_Page_MoveTextPos (index_page, 20.0, 180.0);
        HPDF_Page_SetTextLeading (index_page, 23.0);

        /* page1 (HPDF_ANNOT_NO_HIGHTLIGHT) */
        let mut tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("Jump to Page1 (HilightMode=HPDF_ANNOT_NO_HIGHTLIGHT)").as_ptr());
        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_MoveToNextLine (index_page);

        let mut dst = HPDF_Page_CreateDestination (page[0]);

        let mut annot = HPDF_Page_CreateLinkAnnot (index_page, rect, dst);

        HPDF_LinkAnnot_SetHighlightMode (annot, HPDF_AnnotHighlightMode::HPDF_ANNOT_NO_HIGHTLIGHT);


        /* page2 (HPDF_ANNOT_INVERT_BOX) */
        tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("Jump to Page2 (HilightMode=HPDF_ANNOT_INVERT_BOX)").as_ptr());
        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_MoveToNextLine (index_page);

        dst = HPDF_Page_CreateDestination (page[1]);

        annot = HPDF_Page_CreateLinkAnnot (index_page, rect, dst);

        HPDF_LinkAnnot_SetHighlightMode (annot, HPDF_AnnotHighlightMode::HPDF_ANNOT_INVERT_BOX);


        /* page3 (HPDF_ANNOT_INVERT_BORDER) */
        tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("Jump to Page3 (HilightMode=HPDF_ANNOT_INVERT_BORDER)").as_ptr());
        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_MoveToNextLine (index_page);

        dst = HPDF_Page_CreateDestination (page[2]);

        annot = HPDF_Page_CreateLinkAnnot (index_page, rect, dst);

        HPDF_LinkAnnot_SetHighlightMode (annot, HPDF_AnnotHighlightMode::HPDF_ANNOT_INVERT_BORDER);


        /* page4 (HPDF_ANNOT_DOWN_APPEARANCE) */
        tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("Jump to Page4 (HilightMode=HPDF_ANNOT_DOWN_APPEARANCE)").as_ptr());
        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_MoveToNextLine (index_page);

        dst = HPDF_Page_CreateDestination (page[3]);

        annot = HPDF_Page_CreateLinkAnnot (index_page, rect, dst);

        HPDF_LinkAnnot_SetHighlightMode (annot, HPDF_AnnotHighlightMode::HPDF_ANNOT_DOWN_APPEARANCE);


        /* page5 (dash border) */
        tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("Jump to Page5 (dash border)").as_ptr());
        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_MoveToNextLine (index_page);

        dst = HPDF_Page_CreateDestination (page[4]);

        annot = HPDF_Page_CreateLinkAnnot (index_page, rect, dst);

        HPDF_LinkAnnot_SetBorderStyle (annot, 1.0, 3, 2);


        /* page6 (no border) */
        tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("Jump to Page6 (no border)").as_ptr());
        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_MoveToNextLine (index_page);

        dst = HPDF_Page_CreateDestination (page[5]);

        annot = HPDF_Page_CreateLinkAnnot (index_page, rect, dst);

        HPDF_LinkAnnot_SetBorderStyle (annot, 0.0, 0, 0);


        /* page7 (bold border) */
        tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("Jump to Page7 (bold border)").as_ptr());
        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_MoveToNextLine (index_page);

        dst = HPDF_Page_CreateDestination (page[6]);

        annot = HPDF_Page_CreateLinkAnnot (index_page, rect, dst);

        HPDF_LinkAnnot_SetBorderStyle (annot, 2.0, 0, 0);


        /* URI link */
        tp = HPDF_Page_GetCurrentTextPos (index_page);

        HPDF_Page_ShowText (index_page, cstring!("URI (").as_ptr());
        HPDF_Page_ShowText (index_page, url.as_ptr());
        HPDF_Page_ShowText (index_page, cstring!(")").as_ptr());

        rect.left = tp.x - 4.0;
        rect.bottom = tp.y - 4.0;
        rect.right = HPDF_Page_GetCurrentTextPos (index_page).x + 4.0;
        rect.top = tp.y + 10.0;

        HPDF_Page_CreateURILinkAnnot (index_page, rect, url.as_ptr());

        HPDF_Page_EndText (index_page);

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    } 
    return ;
}
