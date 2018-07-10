/*
 * << Haru Free PDF Library 2.0.0 >> -- line_demo.c
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
    println! ("ERROR: error_no={:?}, detail_no={:?}", error_no,
                detail_no);
}


fn draw_line  (page: HPDF_Page,
               x:    f32,
               y:    f32,
               label: *const libc::c_char)
{
    unsafe{
        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, x, y - 10.0);
        HPDF_Page_ShowText (page, label);
        HPDF_Page_EndText (page);

        HPDF_Page_MoveTo (page, x, y - 15.0);
        HPDF_Page_LineTo (page, x + 220.0, y - 15.0);
        HPDF_Page_Stroke (page);
    }
}


fn draw_line2  (page: HPDF_Page,
                x: f32,
                y: f32,
                label: *const libc::c_char)
{
    unsafe{
        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, x, y);
        HPDF_Page_ShowText (page, label);
        HPDF_Page_EndText (page);

        HPDF_Page_MoveTo (page, x + 30.0, y - 25.0);
        HPDF_Page_LineTo (page, x + 160.0, y - 25.0);
        HPDF_Page_Stroke (page);
    }
}


fn draw_rect (page:  HPDF_Page,
              x:     f32   ,
              y:     f32   ,
              label:  *const libc::c_char)
{
    unsafe{
        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, x, y - 10.0);
        HPDF_Page_ShowText (page, label);
        HPDF_Page_EndText (page);

        HPDF_Page_Rectangle(page, x, y - 40.0, 220.0, 25.0);
    }
}

fn main ()
{
    unsafe{
        let mut page_title = CString::new("Line Example").unwrap();

        let mut fname = CString::new("TEST.pdf").unwrap();

        let DASH_MODE1 : [HPDF_UINT16; 1] = [3];
        let DASH_MODE2 : [HPDF_UINT16; 2] = [3, 7];
        let DASH_MODE3 : [HPDF_UINT16; 4] = [8, 7, 2, 7];

        let mut x : f32 = 0.0;
        let mut y : f32 = 0.0;
        let mut x1 : f32 = 0.0;
        let mut y1 : f32 = 0.0;
        let mut x2 : f32 = 0.0;
        let mut y2 : f32 = 0.0;
        let mut x3 : f32 = 0.0;
        let mut y3 : f32 = 0.0;

        let mut tw: f32 = 0.0;


        let mut pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            panic!("pdf creation failed.");
        }


        /* create default-font */
        let font = HPDF_GetFont (pdf, CString::new("Helvetica").unwrap().as_ptr(),ptr::null_mut() );

        /* add a new page object. */
        let mut page = HPDF_AddPage (pdf);

        /* print the lines of the page. */
        HPDF_Page_SetLineWidth (page, 1.0);
        HPDF_Page_Rectangle (page, 50.0, 50.0, HPDF_Page_GetWidth(page) - 100.0,
                    HPDF_Page_GetHeight (page) - 110.0);
        HPDF_Page_Stroke (page);

        /* print the title of the page (with positioning center). */
        HPDF_Page_SetFontAndSize (page, font, 24.0);
        tw = HPDF_Page_TextWidth (page, page_title.as_ptr());
        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, (HPDF_Page_GetWidth(page) - tw) / 2.0,
                    HPDF_Page_GetHeight (page) - 50.0);
        HPDF_Page_ShowText (page, page_title.as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_SetFontAndSize (page, font, 10.0);

        /* Draw verious widths of lines. */
        HPDF_Page_SetLineWidth (page, 0.0);
        draw_line (page, 60.0, 770.0, CString::new("line width = 0").unwrap().as_ptr());

        HPDF_Page_SetLineWidth (page, 1.0);
        draw_line (page, 60.0, 740.0, CString::new("line width = 1.0").unwrap().as_ptr());

        HPDF_Page_SetLineWidth (page, 2.0);
        draw_line (page, 60.0, 710.0, CString::new("line width = 2.0").unwrap().as_ptr());

        /* Line dash pattern */
        HPDF_Page_SetLineWidth (page, 1.0);

        HPDF_Page_SetDash (page, DASH_MODE1.as_ptr(), 1, 1);
        draw_line (page, 60.0, 680.0, CString::new("dash_ptn=[3], phase=1 -- 2 on, 3 off, 3 on...").unwrap().as_ptr());

        HPDF_Page_SetDash (page, DASH_MODE2.as_ptr(), 2, 2);
        draw_line (page, 60.0, 650.0, CString::new("dash_ptn=[7, 3], phase=2 -- 5 on 3 off, 7 on,...").unwrap().as_ptr());

        HPDF_Page_SetDash (page, DASH_MODE3.as_ptr(), 4, 0);
        draw_line (page, 60.0, 620.0, CString::new("dash_ptn=[8, 7, 2, 7], phase=0").unwrap().as_ptr());

        HPDF_Page_SetDash (page, ptr::null_mut() , 0, 0);

        HPDF_Page_SetLineWidth (page, 30.0);
        HPDF_Page_SetRGBStroke (page, 0.0, 0.5, 0.0);

        /* Line Cap Style */
        HPDF_Page_SetLineCap (page, HPDF_LineCap::HPDF_BUTT_END);
        draw_line2 (page, 60.0, 570.0, CString::new("PDF_BUTT_END").unwrap().as_ptr());

        HPDF_Page_SetLineCap (page, HPDF_LineCap::HPDF_ROUND_END);
        draw_line2 (page, 60.0, 505.0, CString::new("PDF_ROUND_END").unwrap().as_ptr());

        HPDF_Page_SetLineCap (page, HPDF_LineCap::HPDF_PROJECTING_SCUARE_END);
        draw_line2 (page, 60.0, 440.0, CString::new("PDF_PROJECTING_SCUARE_END").unwrap().as_ptr());

        /* Line Join Style */
        HPDF_Page_SetLineWidth (page, 30.0);
        HPDF_Page_SetRGBStroke (page, 0.0, 0.0, 0.5);

        HPDF_Page_SetLineJoin (page, HPDF_LineJoin::HPDF_MITER_JOIN);
        HPDF_Page_MoveTo (page, 120.0, 300.0);
        HPDF_Page_LineTo (page, 160.0, 340.0);
        HPDF_Page_LineTo (page, 200.0, 300.0);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 60.0, 360.0);
        HPDF_Page_ShowText (page, CString::new("PDF_MITER_JOIN").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_SetLineJoin (page, HPDF_LineJoin::HPDF_ROUND_JOIN);
        HPDF_Page_MoveTo (page, 120.0, 195.0);
        HPDF_Page_LineTo (page, 160.0, 235.0);
        HPDF_Page_LineTo (page, 200.0, 195.0);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 60.0, 255.0);
        HPDF_Page_ShowText (page, CString::new("PDF_ROUND_JOIN").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_SetLineJoin (page, HPDF_LineJoin::HPDF_BEVEL_JOIN);
        HPDF_Page_MoveTo (page, 120.0, 90.0);
        HPDF_Page_LineTo (page, 160.0, 130.0);
        HPDF_Page_LineTo (page, 200.0, 90.0);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 60.0, 150.0);
        HPDF_Page_ShowText (page, CString::new("PDF_BEVEL_JOIN").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        /* Draw Rectangle */
        HPDF_Page_SetLineWidth (page, 2.0);
        HPDF_Page_SetRGBStroke (page, 0.0, 0.0, 0.0);
        HPDF_Page_SetRGBFill (page, 0.75, 0.0, 0.0);

        draw_rect (page, 300.0, 770.0, CString::new("Stroke").unwrap().as_ptr());
        HPDF_Page_Stroke (page);

        draw_rect (page, 300.0, 720.0, CString::new("Fill").unwrap().as_ptr());
        HPDF_Page_Fill (page);

        draw_rect (page, 300.0, 670.0, CString::new("Fill then Stroke").unwrap().as_ptr());
        HPDF_Page_FillStroke (page);

        /* Clip Rect */
        HPDF_Page_GSave (page);  /* Save the current graphic state */
        draw_rect (page, 300.0, 620.0, CString::new("Clip Rectangle").unwrap().as_ptr());
        HPDF_Page_Clip (page);
        HPDF_Page_Stroke (page);
        HPDF_Page_SetFontAndSize (page, font, 13.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 290.0, 600.0);
        HPDF_Page_SetTextLeading (page, 12.0);
        HPDF_Page_ShowText (page,
                    CString::new("Clip Clip Clip Clip Clip Clipi Clip Clip Clip").unwrap().as_ptr());
        HPDF_Page_ShowTextNextLine (page,
                    CString::new("Clip Clip Clip Clip Clip Clip Clip Clip Clip").unwrap().as_ptr());
        HPDF_Page_ShowTextNextLine (page,
                    CString::new("Clip Clip Clip Clip Clip Clip Clip Clip Clip").unwrap().as_ptr());
        HPDF_Page_EndText (page);
        HPDF_Page_GRestore (page);

        /* Curve Example(CurveTo2) */
        x = 330.0;
        y = 440.0;
        x1 = 430.0;
        y1 = 530.0;
        x2 = 480.0;
        y2 = 470.0;
        x3 = 480.0;
        y3 = 90.0;

        HPDF_Page_SetRGBFill (page, 0.0, 0.0, 0.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 300.0, 540.0);
        HPDF_Page_ShowText (page, CString::new("CurveTo2(x1, y1, x2. y2)").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, x + 5.0, y - 5.0);
        HPDF_Page_ShowText (page, CString::new("Current point").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, x1 - x, y1 - y);
        HPDF_Page_ShowText (page, CString::new("(x1, y1)").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, x2 - x1, y2 - y1);
        HPDF_Page_ShowText (page, CString::new("(x2, y2)").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_SetDash (page, DASH_MODE1.as_ptr(), 1, 0);

        HPDF_Page_SetLineWidth (page, 0.5);
        HPDF_Page_MoveTo (page, x1, y1);
        HPDF_Page_LineTo (page, x2, y2);
        HPDF_Page_Stroke (page);

        HPDF_Page_SetDash (page,ptr::null_mut() , 0, 0);

        HPDF_Page_SetLineWidth (page, 1.5);

        HPDF_Page_MoveTo (page, x, y);
        HPDF_Page_CurveTo2 (page, x1, y1, x2, y2);
        HPDF_Page_Stroke (page);

        /* Curve Example(CurveTo3) */
        y -= 150.0;
        y1 -= 150.0;
        y2 -= 150.0;

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 300.0, 390.0);
        HPDF_Page_ShowText (page, CString::new("CurveTo3(x1, y1, x2. y2)").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, x + 5.0, y - 5.0);
        HPDF_Page_ShowText (page, CString::new("Current point").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, x1 - x, y1 - y);
        HPDF_Page_ShowText (page, CString::new("(x1, y1)").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, x2 - x1, y2 - y1);
        HPDF_Page_ShowText (page, CString::new("(x2, y2)").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_SetDash (page, DASH_MODE1.as_ptr(), 1, 0);

        HPDF_Page_SetLineWidth (page, 0.5);
        HPDF_Page_MoveTo (page, x, y);
        HPDF_Page_LineTo (page, x1, y1);
        HPDF_Page_Stroke (page);

        HPDF_Page_SetDash (page,ptr::null_mut() , 0, 0);

        HPDF_Page_SetLineWidth (page, 1.5);
        HPDF_Page_MoveTo (page, x, y);
        HPDF_Page_CurveTo3 (page, x1, y1, x2, y2);
        HPDF_Page_Stroke (page);

        /* Curve Example(CurveTo) */
        y -= 150.0;
        y1 -= 160.0;
        y2 -= 130.0;
        x2 += 10.0;

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 300.0, 240.0);
        HPDF_Page_ShowText (page, CString::new("CurveTo(x1, y1, x2. y2, x3, y3)").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, x + 5.0, y - 5.0);
        HPDF_Page_ShowText (page, CString::new("Current point").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, x1 - x, y1 - y);
        HPDF_Page_ShowText (page, CString::new("(x1, y1)").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, x2 - x1, y2 - y1);
        HPDF_Page_ShowText (page, CString::new("(x2, y2)").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, x3 - x2, y3 - y2);
        HPDF_Page_ShowText (page, CString::new("(x3, y3)").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_SetDash (page, DASH_MODE1.as_ptr(), 1, 0);

        HPDF_Page_SetLineWidth (page, 0.5);
        HPDF_Page_MoveTo (page, x, y);
        HPDF_Page_LineTo (page, x1, y1);
        HPDF_Page_Stroke (page);
        HPDF_Page_MoveTo (page, x2, y2);
        HPDF_Page_LineTo (page, x3, y3);
        HPDF_Page_Stroke (page);

        HPDF_Page_SetDash (page, ptr::null_mut(), 0, 0);

        HPDF_Page_SetLineWidth (page, 1.5);
        HPDF_Page_MoveTo (page, x, y);
        HPDF_Page_CurveTo (page, x1, y1, x2, y2, x3, y3);
        HPDF_Page_Stroke (page);

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }
}

