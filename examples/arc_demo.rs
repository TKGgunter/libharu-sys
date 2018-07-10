/*
 * << Haru Free PDF Library 2.0.0 >> -- arc_demo.c
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

fn main ()
{
    unsafe{
        let fname = cstring!("TEST.pdf");


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }


        /* add a new page object. */
        let page = HPDF_AddPage (pdf);

        HPDF_Page_SetHeight (page, 220.0);
        HPDF_Page_SetWidth (page, 200.0);

        /* draw grid to the page */
        //print_grid  (pdf, page);

        /* draw pie chart
         *
         *   A: 45% Red
         *   B: 25% Blue
         *   C: 15% green
         *   D: other yellow
         */

        /* A */
        HPDF_Page_SetRGBFill (page, 1.0, 0.0, 0.0);
        HPDF_Page_MoveTo (page, 100.0, 100.0);
        HPDF_Page_LineTo (page, 100.0, 180.0);
        HPDF_Page_Arc (page, 100.0, 100.0, 80.0, 0.0, 360.0 * 0.45);
        let mut pos = HPDF_Page_GetCurrentPos (page);
        HPDF_Page_LineTo (page, 100.0, 100.0);
        HPDF_Page_Fill (page);

        /* B */
        HPDF_Page_SetRGBFill (page, 0.0, 0.0, 1.0);
        HPDF_Page_MoveTo (page, 100.0, 100.0);
        HPDF_Page_LineTo (page, pos.x, pos.y);
        HPDF_Page_Arc (page, 100.0, 100.0, 80.0, 360.0 * 0.45, 360.0 * 0.7);
        pos = HPDF_Page_GetCurrentPos (page);
        HPDF_Page_LineTo (page, 100.0, 100.0);
        HPDF_Page_Fill (page);

        /* C */
        HPDF_Page_SetRGBFill (page, 0.0, 1.0, 0.0);
        HPDF_Page_MoveTo (page, 100.0, 100.0);
        HPDF_Page_LineTo (page, pos.x, pos.y);
        HPDF_Page_Arc (page, 100.0, 100.0, 80.0, 360.0 * 0.7, 360.0 * 0.85);
        pos = HPDF_Page_GetCurrentPos (page);
        HPDF_Page_LineTo (page, 100.0, 100.0);
        HPDF_Page_Fill (page);

        /* D */
        HPDF_Page_SetRGBFill (page, 1.0, 1.0, 0.0);
        HPDF_Page_MoveTo (page, 100.0, 100.0);
        HPDF_Page_LineTo (page, pos.x, pos.y);
        HPDF_Page_Arc (page, 100.0, 100.0, 80.0, 360.0 * 0.85, 360.0);
        pos = HPDF_Page_GetCurrentPos (page);
        HPDF_Page_LineTo (page, 100.0, 100.0);
        HPDF_Page_Fill (page);

        /* draw center circle */
        HPDF_Page_SetGrayStroke (page, 0.0);
        HPDF_Page_SetGrayFill (page, 1.0);
        HPDF_Page_Circle (page, 100.0, 100.0, 30.0);
        HPDF_Page_Fill (page);

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }
    return ;
}

