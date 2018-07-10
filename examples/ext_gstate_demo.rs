/*
 * << Haru Free PDF Library 2.0.0 >> -- ext_gstate_demo.c
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




fn draw_circles (page: HPDF_Page, description: *const libc::c_char, x: HPDF_REAL, y: HPDF_REAL)
{
    unsafe{
        HPDF_Page_SetLineWidth (page, 1.0);
        HPDF_Page_SetRGBStroke (page, 0.0, 0.0, 0.0);
        HPDF_Page_SetRGBFill (page, 1.0, 0.0, 0.0);
        HPDF_Page_Circle (page, x + 40.0, y + 40.0, 40.0);
        HPDF_Page_ClosePathFillStroke (page);
        HPDF_Page_SetRGBFill (page, 0.0, 1.0, 0.0);
        HPDF_Page_Circle (page, x + 100.0, y + 40.0, 40.0);
        HPDF_Page_ClosePathFillStroke (page);
        HPDF_Page_SetRGBFill (page, 0.0, 0.0, 1.0);
        HPDF_Page_Circle (page, x + 70.0, y + 74.64, 40.0);
        HPDF_Page_ClosePathFillStroke (page);

        HPDF_Page_SetRGBFill (page, 0.0, 0.0, 0.0);
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, x + 0.0, y + 130.0, description);
        HPDF_Page_EndText (page);
    }
}


fn main ()
{
    unsafe{
        let fname = cstring!("TEST.pdf");
        let PAGE_WIDTH  : HPDF_REAL = 600.0;
        let PAGE_HEIGHT : HPDF_REAL  = 900.0;


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }


        let hfont = HPDF_GetFont (pdf, cstring!("Helvetica-Bold").as_ptr(), ptr::null_mut());

        /* add a new page object. */
        let page = HPDF_AddPage (pdf);

        HPDF_Page_SetFontAndSize (page, hfont, 10.0);

        HPDF_Page_SetHeight (page, PAGE_HEIGHT);
        HPDF_Page_SetWidth (page, PAGE_WIDTH);

        /* normal */
        HPDF_Page_GSave (page);
        draw_circles (page, cstring!("normal").as_ptr(), 40.0, PAGE_HEIGHT - 170.0);
        HPDF_Page_GRestore (page);

        /* transparency (0.8) */
        HPDF_Page_GSave (page);
        let mut gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetAlphaFill (gstate, 0.8);
        HPDF_ExtGState_SetAlphaStroke (gstate, 0.8);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("alpha fill = 0.8").as_ptr(), 230.0, PAGE_HEIGHT - 170.0);
        HPDF_Page_GRestore (page);

        /* transparency (0.4) */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetAlphaFill (gstate, 0.4);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("alpha fill = 0.4").as_ptr(), 420.0, PAGE_HEIGHT - 170.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_MULTIPLY */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_MULTIPLY);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_MULTIPLY").as_ptr(), 40.0, PAGE_HEIGHT - 340.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_SCREEN */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_SCREEN);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_SCREEN").as_ptr(), 230.0, PAGE_HEIGHT - 340.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_OVERLAY */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_OVERLAY);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_OVERLAY").as_ptr(), 420.0, PAGE_HEIGHT - 340.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_DARKEN */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_DARKEN);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_DARKEN").as_ptr(), 40.0, PAGE_HEIGHT - 510.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_LIGHTEN */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_LIGHTEN);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_LIGHTEN").as_ptr(), 230.0, PAGE_HEIGHT - 510.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_COLOR_DODGE */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_COLOR_DODGE);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_COLOR_DODGE").as_ptr(), 420.0, PAGE_HEIGHT - 510.0);
        HPDF_Page_GRestore (page);


        /* blend-mode=HPDF_BM_COLOR_BUM */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_COLOR_BUM);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_COLOR_BUM").as_ptr(), 40.0, PAGE_HEIGHT - 680.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_HARD_LIGHT */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_HARD_LIGHT);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_HARD_LIGHT").as_ptr(), 230.0, PAGE_HEIGHT - 680.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_SOFT_LIGHT */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_SOFT_LIGHT);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_SOFT_LIGHT").as_ptr(), 420.0, PAGE_HEIGHT - 680.0);
        HPDF_Page_GRestore (page);

        /* blend-mode=HPDF_BM_DIFFERENCE */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_DIFFERENCE);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_DIFFERENCE").as_ptr(), 40.0, PAGE_HEIGHT - 850.0);
        HPDF_Page_GRestore (page);


        /* blend-mode=HPDF_BM_EXCLUSHON */
        HPDF_Page_GSave (page);
        gstate = HPDF_CreateExtGState (pdf);
        HPDF_ExtGState_SetBlendMode (gstate, HPDF_BlendMode::HPDF_BM_EXCLUSHON);
        HPDF_Page_SetExtGState (page, gstate);
        draw_circles (page, cstring!("HPDF_BM_EXCLUSHON").as_ptr(), 230.0, PAGE_HEIGHT - 850.0);
        HPDF_Page_GRestore (page);


        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }
    return ;
}

