/*
 * << Haru Free PDF Library 2.0.0 >> -- grid_sheet.c
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


fn print_grid  (pdf: HPDF_Doc,
                page: HPDF_Page)
{
    unsafe{
        let height = HPDF_Page_GetHeight (page);
        let width = HPDF_Page_GetWidth (page);
        let font = HPDF_GetFont (pdf, cstring!("Helvetica").as_ptr(), ptr::null_mut());
        let mut x = 0.0;
        let mut y = 0.0;

        HPDF_Page_SetFontAndSize (page, font, 5.0);
        HPDF_Page_SetGrayFill (page, 0.5);
        HPDF_Page_SetGrayStroke (page, 0.8);

        /* Draw horizontal lines */
        y = 0.0;
        while y < height {
            if y % 10.0 == 0.0{
                HPDF_Page_SetLineWidth (page, 0.5);
            }
            else {
                if HPDF_Page_GetLineWidth (page) != 0.25{
                    HPDF_Page_SetLineWidth (page, 0.25);
                }
            }

            HPDF_Page_MoveTo (page, 0.0, y);
            HPDF_Page_LineTo (page, width, y);
            HPDF_Page_Stroke (page);

            if y % 10.0 == 0.0 && y > 0.0 {
                HPDF_Page_SetGrayStroke (page, 0.5);

                HPDF_Page_MoveTo (page, 0.0, y);
                HPDF_Page_LineTo (page, 5.0, y);
                HPDF_Page_Stroke (page);

                HPDF_Page_SetGrayStroke (page, 0.8);
            }

            y += 5.0;
        }


        /* Draw virtical lines */
        x = 0.0;
        while x < width {
            if x % 10.0 == 0.0 {
                HPDF_Page_SetLineWidth (page, 0.5);
            }
            else {
                if HPDF_Page_GetLineWidth (page) != 0.25{
                    HPDF_Page_SetLineWidth (page, 0.25);
                }
            }

            HPDF_Page_MoveTo (page, x, 0.0);
            HPDF_Page_LineTo (page, x, height);
            HPDF_Page_Stroke (page);

            if x % 50.0 == 0.0 && x > 0.0 {
                HPDF_Page_SetGrayStroke (page, 0.5);

                HPDF_Page_MoveTo (page, x, 0.0);
                HPDF_Page_LineTo (page, x, 5.0);
                HPDF_Page_Stroke (page);

                HPDF_Page_MoveTo (page, x, height);
                HPDF_Page_LineTo (page, x, height - 5.0);
                HPDF_Page_Stroke (page);

                HPDF_Page_SetGrayStroke (page, 0.8);
            }

            x += 5.0;
        }

        /* Draw horizontal text */
        y = 0.0;
        while y < height {
            if y % 10.0 == 0.0 && y > 0.0 {

                HPDF_Page_BeginText (page);
                HPDF_Page_MoveTextPos (page, 5.0, y - 2.0);

                let buf = cstring!(format!("{}", y));

                HPDF_Page_ShowText (page, buf.as_ptr());
                HPDF_Page_EndText (page);
            }

            y += 5.0;
        }


        /* Draw virtical text */
        x = 0.0;
        while x < width {
            if x % 50.0 == 0.0 && x > 0.0 {

                HPDF_Page_BeginText (page);
                HPDF_Page_MoveTextPos (page, x, 5.0);

                let buf = cstring!(format!("{}", x));

                HPDF_Page_ShowText (page, buf.as_ptr());
                HPDF_Page_EndText (page);

                HPDF_Page_BeginText (page);
                HPDF_Page_MoveTextPos (page, x, height - 10.0);
                HPDF_Page_ShowText (page, buf.as_ptr());
                HPDF_Page_EndText (page);
            }

            x += 5.0;
        }

        HPDF_Page_SetGrayFill (page, 0.0);
        HPDF_Page_SetGrayStroke (page, 0.0);
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

        HPDF_Page_SetHeight (page, 600.0);
        HPDF_Page_SetWidth (page, 400.0);

        print_grid  (pdf, page);


        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }

    return ;
}



