/*
 * << Alternative PDF Library 1.0.0 >> -- text_demo2.c
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



extern crate libharu_rs;
extern crate libc;
use std::ptr;
use std::ffi::CString;
use libharu_rs::*;

extern fn error_handler  (error_no: HPDF_STATUS,
                detail_no: HPDF_STATUS,
                user_data : HPDF_HANDLE)
{
    println! ("ERROR: error_no={:?}, detail_no={:?}", error_no,
                detail_no);
}

static mut no: i32 = 0;



fn main ()->()
{
    unsafe{
        let fname = CString::new("TEST.pdf").unwrap();
        let mut font : HPDF_Font;
        let mut angle1  : f32 ;
        let mut angle2  : f32 ;
        let mut rad1    : f32 ;
        let mut rad2    : f32 ;
        let mut page_height : HPDF_REAL;
        let mut rect : HPDF_Rect = HPDF_Rect{left: 0.0, top: 0.0, right: 0.0, bottom: 0.0};
        let mut i: i32;

        let SAMP_TXT = "The quick brown fox jumps over the lazy dog. ".to_string();
        let SAMP_TXT_CHARS : Vec<char> = SAMP_TXT.chars().collect();
        let SAMP_TXT_LEN = SAMP_TXT.len();
        let SAMP_TXT = CString::new(SAMP_TXT).unwrap();


        let mut pdf = HPDF_New (error_handler, ptr::null_mut());

        if pdf == ptr::null_mut() {
            HPDF_Free (pdf);
            return ();
        }

        /* add a new page object. */
        let mut page = HPDF_AddPage (pdf);
        HPDF_Page_SetSize (page, HPDF_PageSizes::HPDF_PAGE_SIZE_A5, HPDF_PageDirection::HPDF_PAGE_PORTRAIT);
        
        //Grid has yes to be ported
        //print_grid  (pdf, page);

        page_height = HPDF_Page_GetHeight (page);

        font = HPDF_GetFont (pdf, CString::new("Helvetica").unwrap().as_ptr(), ptr::null_mut());
        HPDF_Page_SetTextLeading (page, 20.0);

        /* text_rect method */

        /* HPDF_TALIGN_LEFT */
        rect.left = 25.0;
        rect.top = 545.0;
        rect.right = 200.0;
        rect.bottom = rect.top - 40.0;

        HPDF_Page_Rectangle (page, rect.left, rect.bottom, rect.right - rect.left,
                     rect.top - rect.bottom);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);

        HPDF_Page_SetFontAndSize (page, font, 10.0);
        HPDF_Page_TextOut (page, rect.left, rect.top + 3.0, CString::new("HPDF_TALIGN_LEFT").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 13.0);
        HPDF_Page_TextRect (page, rect.left, rect.top, rect.right, rect.bottom,
                    SAMP_TXT.as_ptr(), HPDF_TextAlignment::HPDF_TALIGN_LEFT, ptr::null_mut());

        HPDF_Page_EndText (page);

        /* HPDF_TALIGN_RIGTH */
        rect.left = 220.0;
        rect.right = 395.0;

        HPDF_Page_Rectangle (page, rect.left, rect.bottom, rect.right - rect.left,
                    rect.top - rect.bottom);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);

        HPDF_Page_SetFontAndSize (page, font, 10.0);
        HPDF_Page_TextOut (page, rect.left, rect.top + 3.0, CString::new("HPDF_TALIGN_RIGTH").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 13.0);
        HPDF_Page_TextRect (page, rect.left, rect.top, rect.right, rect.bottom,
                    SAMP_TXT.as_ptr(), HPDF_TextAlignment::HPDF_TALIGN_RIGHT, ptr::null_mut());

        HPDF_Page_EndText (page);

        /* HPDF_TALIGN_CENTER */
        rect.left = 25.0;
        rect.top = 475.0;
        rect.right = 200.0;
        rect.bottom = rect.top - 40.0;

        HPDF_Page_Rectangle (page, rect.left, rect.bottom, rect.right - rect.left,
                    rect.top - rect.bottom);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);

        HPDF_Page_SetFontAndSize (page, font, 10.0);
        HPDF_Page_TextOut (page, rect.left, rect.top + 3.0, CString::new("HPDF_TALIGN_CENTER").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 13.0);
        HPDF_Page_TextRect (page, rect.left, rect.top, rect.right, rect.bottom,
                    SAMP_TXT.as_ptr(), HPDF_TextAlignment::HPDF_TALIGN_CENTER, ptr::null_mut());

        HPDF_Page_EndText (page);

        /* HPDF_TALIGN_JUSTIFY */
        rect.left = 220.0;
        rect.right = 395.0;

        HPDF_Page_Rectangle (page, rect.left, rect.bottom, rect.right - rect.left,
                    rect.top - rect.bottom);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);

        HPDF_Page_SetFontAndSize (page, font, 10.0);
        HPDF_Page_TextOut (page, rect.left, rect.top + 3.0, CString::new("HPDF_TALIGN_JUSTIFY").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 13.0);
        HPDF_Page_TextRect (page, rect.left, rect.top, rect.right, rect.bottom,
                    SAMP_TXT.as_ptr(), HPDF_TextAlignment::HPDF_TALIGN_JUSTIFY, ptr::null_mut());

        HPDF_Page_EndText (page);



        /* Skewed coordinate system */
        HPDF_Page_GSave (page);

        angle1 = 5.0;
        angle2 = 10.0;
        rad1 = angle1 / 180.0 * 3.141592;
        rad2 = angle2 / 180.0 * 3.141592;

        HPDF_Page_Concat (page, 1.0, rad1.tan(), rad2.tan(), 1.0, 25.0, 350.0);
        rect.left = 0.0;
        rect.top = 40.0;
        rect.right = 175.0;
        rect.bottom = 0.0;

        HPDF_Page_Rectangle (page, rect.left, rect.bottom, rect.right - rect.left,
                    rect.top - rect.bottom);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);

        HPDF_Page_SetFontAndSize (page, font, 10.0);
        HPDF_Page_TextOut (page, rect.left, rect.top + 3.0, CString::new("Skewed coordinate system").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 13.0);
        HPDF_Page_TextRect (page, rect.left, rect.top, rect.right, rect.bottom,
                    SAMP_TXT.as_ptr(), HPDF_TextAlignment::HPDF_TALIGN_LEFT, ptr::null_mut());

        HPDF_Page_EndText (page);

        HPDF_Page_GRestore (page);


        /* Rotated coordinate system */
        HPDF_Page_GSave (page);

        angle1 = 5.0;
        rad1 = angle1 / 180.0 * 3.141592;

        HPDF_Page_Concat (page, rad1.cos(), rad1.sin(), -rad1.sin(), rad1.cos(), 220.0, 350.0);
        rect.left = 0.0;
        rect.top = 40.0;
        rect.right = 175.0;
        rect.bottom = 0.0;

        HPDF_Page_Rectangle (page, rect.left, rect.bottom, rect.right - rect.left,
                    rect.top - rect.bottom);
        HPDF_Page_Stroke (page);

        HPDF_Page_BeginText (page);

        HPDF_Page_SetFontAndSize (page, font, 10.0);
        HPDF_Page_TextOut (page, rect.left, rect.top + 3.0, CString::new("Rotated coordinate system").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 13.0);
        HPDF_Page_TextRect (page, rect.left, rect.top, rect.right, rect.bottom,
                    SAMP_TXT.as_ptr(), HPDF_TextAlignment::HPDF_TALIGN_LEFT, ptr::null_mut());

        HPDF_Page_EndText (page);

        HPDF_Page_GRestore (page);


        /* text along a circle */
        HPDF_Page_SetGrayStroke (page, 0.0);
        HPDF_Page_Circle (page, 210.0, 190.0, 145.0);
        HPDF_Page_Circle (page, 210.0, 190.0, 113.0);
        HPDF_Page_Stroke (page);

        angle1 = 360.0 / SAMP_TXT_LEN as f32;
        angle2 = 180.0;

        HPDF_Page_BeginText (page);
        font = HPDF_GetFont (pdf, CString::new("Courier-Bold").unwrap().as_ptr(), ptr::null_mut());
        HPDF_Page_SetFontAndSize (page, font, 30.0);

        for i in 0..SAMP_TXT_LEN {
            let mut buf: [i8;2] = [0,0];
            let mut x: f32;
            let mut y: f32;

            rad1 = (angle2 - 90.0) / 180.0 * 3.141592;
            rad2 = angle2 / 180.0 * 3.141592;

            x = 210.0 + rad2.cos() * 122.0;
            y = 190.0 + rad2.sin() * 122.0;

            HPDF_Page_SetTextMatrix(page, rad1.cos(), rad1.sin(), -rad1.sin(), rad1.cos(), x, y);

            buf[0] = SAMP_TXT_CHARS[i] as i8;
            buf[1] = 0;
            HPDF_Page_ShowText (page, buf.as_ptr());
            angle2 -= angle1;
        }

        HPDF_Page_EndText (page);

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }

    return ();
}



