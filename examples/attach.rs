/*
 * << Haru Free PDF Library 2.0.0 >> -- attach.c
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


const text: &str = "This PDF should have an attachment named attach.c";

extern fn error_handler  (error_no: HPDF_STATUS,
                detail_no: HPDF_STATUS,
                user_data : HPDF_HANDLE)
{
    println! ("ERROR: error_no={:#X}, detail_no={:#X}", error_no,
                detail_no);
}

macro_rules! cstring{
    ($fmt:expr) =>{
        CString::new($fmt).unwrap();
    }
}

fn main ()
{
    unsafe{
        let fname = cstring!("TEST.pdf"); 


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }


        /* create default-font */
        let font = HPDF_GetFont (pdf, cstring!("Helvetica").as_ptr(), ptr::null_mut());

        /* add a new page object. */
        let page = HPDF_AddPage (pdf);

        HPDF_Page_SetSize (page, HPDF_PageSizes::HPDF_PAGE_SIZE_LETTER, HPDF_PageDirection::HPDF_PAGE_PORTRAIT);

        HPDF_Page_BeginText (page);
        HPDF_Page_SetFontAndSize (page, font, 20.0);
        let tw = HPDF_Page_TextWidth (page, cstring!(text).as_ptr());
        HPDF_Page_MoveTextPos (page, (HPDF_Page_GetWidth (page) - tw) / 2.0,
                    (HPDF_Page_GetHeight (page)  - 20.0) / 2.0);
        HPDF_Page_ShowText (page, cstring!(text).as_ptr());
        HPDF_Page_EndText (page);

        /* attach a file to the document */
        HPDF_AttachFile (pdf, cstring!("examples/attach.rs").as_ptr());
        HPDF_AttachFile (pdf, cstring!("examples/jpeg_demo.rs").as_ptr());

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }

    return ;
}

