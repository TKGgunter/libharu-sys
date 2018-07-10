/*
 * << Haru Free PDF Library 2.0.0 >> -- ttfont_demo.c
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




pub fn main ()
{
    unsafe{
        let mut SAMP_TXT = CString::new("The quick brown fox jumps over the lazy dog.").unwrap();

        let fname = CString::new("TEST.pdf").unwrap();
        let mut embed : HPDF_BOOL = HPDF_TRUE;
        let mut page_height :HPDF_REAL = 0.0;
        let mut page_width: HPDF_REAL = 0.0;
        let mut pw: HPDF_REAL = 0.0; 


        let mut pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            panic!("PDF could not be created!");
        }


        /* Add a new page object. */
        let mut page = HPDF_AddPage (pdf);

        

        /*
        if (argc > 2 && memcmp(argv[2], "-E", 2) == 0)
            embed = HPDF_TRUE;
        else
            embed = HPDF_FALSE;
        */

        let title_font = HPDF_GetFont (pdf, CString::new("Helvetica").unwrap().as_ptr(), ptr::null_mut());

        let detail_font_name = HPDF_LoadTTFontFromFile (pdf, CString::new("examples/ttfont/PenguinAttack.ttf").unwrap().as_ptr(), embed);

        let detail_font = HPDF_GetFont (pdf, detail_font_name, ptr::null_mut());

        HPDF_Page_SetFontAndSize (page, title_font, 10.0);

        HPDF_Page_BeginText (page);

        /* Move the position of the text to top of the page. */
        HPDF_Page_MoveTextPos(page, 10.0, 190.0);
        HPDF_Page_ShowText (page, detail_font_name);

        HPDF_Page_ShowText (page, CString::new("(Embedded Subset)").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, detail_font, 15.0);
        HPDF_Page_MoveTextPos (page, 10.0, -20.0);
        HPDF_Page_ShowText (page, CString::new("abcdefghijklmnopqrstuvwxyz").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, 0.0, -20.0);
        HPDF_Page_ShowText (page, CString::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, 0.0, -20.0);
        HPDF_Page_ShowText (page, CString::new("1234567890").unwrap().as_ptr());
        HPDF_Page_MoveTextPos (page, 0.0, -20.0);

        HPDF_Page_SetFontAndSize (page, detail_font, 10.0);
        HPDF_Page_ShowText (page, SAMP_TXT.as_ptr());
        HPDF_Page_MoveTextPos (page, 0.0, -18.0);

        HPDF_Page_SetFontAndSize (page, detail_font, 16.0);
        HPDF_Page_ShowText (page, SAMP_TXT.as_ptr());
        HPDF_Page_MoveTextPos (page, 0.0, -27.0);

        HPDF_Page_SetFontAndSize (page, detail_font, 23.0);
        HPDF_Page_ShowText (page, SAMP_TXT.as_ptr());
        HPDF_Page_MoveTextPos (page, 0.0, -36.0);

        HPDF_Page_SetFontAndSize (page, detail_font, 30.0);
        HPDF_Page_ShowText (page, SAMP_TXT.as_ptr());
        HPDF_Page_MoveTextPos (page, 0.0, -36.0);

        pw = HPDF_Page_TextWidth (page, SAMP_TXT.as_ptr());
        page_height = 210.0;
        page_width = pw + 40.0;

        HPDF_Page_SetWidth (page, page_width);
        HPDF_Page_SetHeight (page, page_height);

        /* Finish to print text. */
        HPDF_Page_EndText (page);

        HPDF_Page_SetLineWidth (page, 0.5);

        HPDF_Page_MoveTo (page, 10.0, page_height - 25.0);
        HPDF_Page_LineTo (page, page_width - 10.0, page_height - 25.0);
        HPDF_Page_Stroke (page);

        HPDF_Page_MoveTo (page, 10.0, page_height - 85.0);
        HPDF_Page_LineTo (page, page_width - 10.0, page_height - 85.0);
        HPDF_Page_Stroke (page);

        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }

}

