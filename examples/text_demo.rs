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
extern crate libharu_sys;
use std::ptr;
use libharu_sys::*;
use std::ffi::CString;
extern crate libc;

extern fn error_handler(error_no: HPDF_STATUS,  detail_no: HPDF_STATUS, user_details: HPDF_HANDLE){ panic!("Some Error"); }

extern fn show_stripe_pattern  (page: HPDF_Page,
                        x: HPDF_REAL,
                        y: HPDF_REAL)
{
    unsafe{
        let mut iy :HPDF_UINT = 0;

        while (iy < 50) {
            HPDF_Page_SetRGBStroke (page, 0.0, 0.0, 0.5);
            HPDF_Page_SetLineWidth (page, 1.0);
            HPDF_Page_MoveTo (page, x, y + iy as f32);
            HPDF_Page_LineTo (page, x + HPDF_Page_TextWidth (page, CString::new("ABCabc123").unwrap().as_ptr()),
                        y + iy as f32);
            HPDF_Page_Stroke (page);
            iy += 3;
        }

        HPDF_Page_SetLineWidth (page, 2.5);
    }
}


extern fn show_description  (page:  HPDF_Page,
                      x:     HPDF_REAL,
                      y:     HPDF_REAL,
                      text: *const libc::c_char)
{
    unsafe{
        let mut fsize = HPDF_Page_GetCurrentFontSize (page);
        let font = HPDF_Page_GetCurrentFont (page);
        let c = HPDF_Page_GetRGBFill (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_SetRGBFill (page, 0.0, 0.0, 0.0);
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_FILL);
        HPDF_Page_SetFontAndSize (page, font, 10.0);
        HPDF_Page_TextOut (page, x, y - 12.0, text);
        HPDF_Page_EndText (page);

        HPDF_Page_SetFontAndSize (page, font, fsize);
        HPDF_Page_SetRGBFill (page, c.r, c.g, c.b);
    }
}



fn main()->()
{
    unsafe{
        let page_title = CString::new("Text Demo").unwrap();

        let fname = CString::new("TEST.pdf").unwrap();

        let  samp_text_rs = "abcdefgABCDEFG123!#$%&+-@?".to_string();
        let  samp_text = CString::new("abcdefgABCDEFG123!#$%&+-@?").unwrap();
        let  samp_text2 = CString::new("The quick brown fox jumps over the lazy dog.").unwrap();
        let mut tw: libc::c_float;
        let mut fsize: libc::c_float;
        let mut i:  libc::c_int;
        let mut len: libc::c_int;

        let mut angle1: libc::c_float;
        let mut angle2: libc::c_float;
        let mut rad1 : libc::c_float;
        let mut rad2 : libc::c_float;

        let ypos: libc::c_float;


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println!("error: cannot create PdfDoc object");
            return ();
        }


        /* set compression mode */
        HPDF_SetCompressionMode (pdf, HPDF_COMP_ALL);

        /* create default-font */
        let font = HPDF_GetFont (pdf, CString::new("Helvetica").unwrap().as_ptr(), ptr::null_mut());

        /* add a new page object. */
        let page = HPDF_AddPage (pdf);

        /* draw grid to the page */
        //TODO: I think this was written in a different demo file
        //After I port that I'll get this working
        //print_grid  (pdf, page);

        /* print the lines of the page.
        HPDF_Page_SetLineWidth (page, 1.0);
        HPDF_Page_Rectangle (page, 50.0, 50.0, HPDF_Page_GetWidth(page) - 100.0,
                    HPDF_Page_GetHeight (page) - 110.0);
        HPDF_Page_Stroke (page);
        */

        /* print the title of the page (with positioning center). */
        HPDF_Page_SetFontAndSize (page, font, 24.0);
        tw = HPDF_Page_TextWidth (page, page_title.as_ptr());
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, (HPDF_Page_GetWidth(page) - tw) / 2.0,
                    HPDF_Page_GetHeight (page) - 50.0, page_title.as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 60.0, HPDF_Page_GetHeight(page) - 60.0);

        /*
         * font size
         */
        fsize = 8.0;
        loop{
            //let mut buf : [libc::c_char; 50]; //IDK Why this is needed]
            let mut len: u32;

            /* set style and size of font. */
            HPDF_Page_SetFontAndSize(page, font, fsize);

            /* set the position of the text. */
            HPDF_Page_MoveTextPos (page, 0.0, -5.0 - fsize);

            /* measure the number of characters which included in the page. */
            
            len = HPDF_Page_MeasureText (page, samp_text.as_ptr(),
                            HPDF_Page_GetWidth(page) - 120.0, HPDF_FALSE, ptr::null_mut() );

            /* truncate the text. */
            HPDF_Page_ShowText (page, samp_text.as_ptr());

            /* print the description. */
            HPDF_Page_MoveTextPos (page, 0.0, -10.0);
            HPDF_Page_SetFontAndSize(page, font, 8.0);

            let buf = CString::new(format!("Fontsize={}", fsize)).unwrap();
            HPDF_Page_ShowText (page, buf.as_ptr());

            fsize *= 1.5;
            if fsize >= 60.0{break;}
        }

        /*
         * font color
         */
        HPDF_Page_SetFontAndSize(page, font, 8.0);
        HPDF_Page_MoveTextPos (page, 0.0, -30.0);
        HPDF_Page_ShowText (page, CString::new("Font color").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 18.0);
        HPDF_Page_MoveTextPos (page, 0.0, -20.0);
        let len = samp_text_rs.len() ;
        for i in 0..len {
            let mut buf : [libc::c_char;2] = [0,0];
            let r : f32 = (i as f32) / (len as f32);
            let g : f32 = 1.0 - ((i as f32) / (len as f32));
            buf[0] = samp_text_rs.chars().nth(i).unwrap() as libc::c_char;
            buf[1] = 0x00 as libc::c_char;

            HPDF_Page_SetRGBFill (page, r, g, 0.0);
            HPDF_Page_ShowText (page, buf.as_ptr());
        }
        HPDF_Page_MoveTextPos (page, 0.0, -25.0);

        for i in 0..len {
            let mut buf : [libc::c_char;2] = [0,0];
            let r : f32 = (i as f32) / (len as f32);
            let b : f32 = 1.0 - ((i as f32) / (len as f32));
            buf[0] = samp_text_rs.chars().nth(i).unwrap() as libc::c_char;
            buf[1] = 0x00 as libc::c_char;

            HPDF_Page_SetRGBFill (page, r, 0.0, b);
            HPDF_Page_ShowText (page, buf.as_ptr());
        }
        HPDF_Page_MoveTextPos (page, 0.0, -25.0);

        for i in 0..len {
            let mut buf : [libc::c_char;2] = [0,0];
            let g : f32 = (i as f32) / (len as f32);
            let b : f32 = 1.0 - ((i as f32) / (len as f32));
            buf[0] = samp_text_rs.chars().nth(i).unwrap() as libc::c_char;
            buf[1] = 0x00 as libc::c_char;

            HPDF_Page_SetRGBFill (page, 0.0, g, b);
            HPDF_Page_ShowText (page, buf.as_ptr());
        }

        HPDF_Page_EndText (page);

        ypos = 450.0;

        /*
         * Font rendering mode
         */
        HPDF_Page_SetFontAndSize(page, font, 32.0);
        HPDF_Page_SetRGBFill (page, 0.5, 0.5, 0.0);
        HPDF_Page_SetLineWidth (page, 1.5);

         /* PDF_FILL */
        show_description (page,  60.0, ypos,
                    CString::new("RenderingMode=PDF_FILL").unwrap().as_ptr());
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_FILL);
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, ypos, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        /* PDF_STROKE */
        show_description (page, 60.0, ypos - 50.0,
                    CString::new("RenderingMode=PDF_STROKE").unwrap().as_ptr());
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_STROKE);
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, ypos - 50.0, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        /* PDF_FILL_THEN_STROKE */
        show_description (page, 60.0, ypos - 100.0,
                    CString::new("RenderingMode=PDF_FILL_THEN_STROKE").unwrap().as_ptr());
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_FILL_THEN_STROKE);
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, ypos - 100.0, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        /* PDF_FILL_CLIPPING */
        show_description (page, 60.0, ypos - 150.0,
                    CString::new("RenderingMode=PDF_FILL_CLIPPING").unwrap().as_ptr());
        HPDF_Page_GSave (page);
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_FILL_CLIPPING);
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, ypos - 150.0, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);
        show_stripe_pattern (page, 60.0, ypos - 150.0);
        HPDF_Page_GRestore (page);

        /* PDF_STROKE_CLIPPING */
        show_description (page, 60.0, ypos - 200.0,
                    CString::new("RenderingMode=PDF_STROKE_CLIPPING").unwrap().as_ptr());
        HPDF_Page_GSave (page);
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_STROKE_CLIPPING);
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, ypos - 200.0, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);
        show_stripe_pattern (page, 60.0, ypos - 200.0);
        HPDF_Page_GRestore (page);

        /* PDF_FILL_STROKE_CLIPPING */
        show_description (page, 60.0, ypos - 250.0,
                    CString::new("RenderingMode=PDF_FILL_STROKE_CLIPPING").unwrap().as_ptr());
        HPDF_Page_GSave (page);
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_FILL_STROKE_CLIPPING);
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, ypos - 250.0, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);
        show_stripe_pattern (page, 60.0, ypos - 250.0);
        HPDF_Page_GRestore (page);

        /* Reset text attributes */
        HPDF_Page_SetTextRenderingMode (page, HPDF_TextRenderingMode::HPDF_FILL);
        HPDF_Page_SetRGBFill (page, 0.0, 0.0, 0.0);
        HPDF_Page_SetFontAndSize(page, font, 30.0);


        /*
         * Rotating text
         */
        angle1 = 30.0;                   /* A rotation of 30 degrees. */
        rad1 = angle1 / 180.0 * 3.141592; /* Calcurate the radian value. */

        show_description (page, 320.0, ypos - 60.0, CString::new("Rotating text").unwrap().as_ptr());
        HPDF_Page_BeginText (page);
        HPDF_Page_SetTextMatrix (page, rad1.cos(), rad1.sin(), -rad1.sin(), rad1.cos(),
                    330.0, ypos - 60.0);
        HPDF_Page_ShowText (page, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);


        /*
         * Skewing text.
         */
        show_description (page, 320.0, ypos - 120.0, CString::new("Skewing text").unwrap().as_ptr());
        HPDF_Page_BeginText (page);

        angle1 = 10.0;
        angle2 = 20.0;
        rad1 = angle1 / 180.0 * 3.141592;
        rad2 = angle2 / 180.0 * 3.141592;

        HPDF_Page_SetTextMatrix (page, 1.0, rad1.tan(), rad2.tan(), 1.0, 320.0, ypos - 120.0);
        HPDF_Page_ShowText (page, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);


        /*
         * scaling text (X direction)
         */
        show_description (page, 320.0, ypos - 175.0, CString::new("Scaling text (X direction)").unwrap().as_ptr());
        HPDF_Page_BeginText (page);
        HPDF_Page_SetTextMatrix (page, 1.5, 0.0, 0.0, 1.0, 320.0, ypos - 175.0);
        HPDF_Page_ShowText (page, CString::new("ABCabc12").unwrap().as_ptr());
        HPDF_Page_EndText (page);


        /*
         * scaling text (Y direction)
         */
        show_description (page, 320.0, ypos - 250.0, CString::new("Scaling text (Y direction)").unwrap().as_ptr());
        HPDF_Page_BeginText (page);
        HPDF_Page_SetTextMatrix (page, 1.0, 0.0, 0.0, 2.0, 320.0, ypos - 250.0);
        HPDF_Page_ShowText (page, CString::new("ABCabc123").unwrap().as_ptr());
        HPDF_Page_EndText (page);


        /*
         * char spacing, word spacing
         */

        show_description (page, 60.0, 140.0, CString::new("char-spacing 0").unwrap().as_ptr());
        show_description (page, 60.0, 100.0, CString::new("char-spacing 1.5").unwrap().as_ptr());
        show_description (page, 60.0, 60.0,  CString::new("char-spacing 1.5, word-spacing 2.5").unwrap().as_ptr());

        HPDF_Page_SetFontAndSize (page, font, 20.0);
        HPDF_Page_SetRGBFill (page, 0.1, 0.3, 0.1);

        /* char-spacing 0 */
        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, 140.0, samp_text2.as_ptr());
        HPDF_Page_EndText (page);

        /* char-spacing 1.5 */
        HPDF_Page_SetCharSpace (page, 1.5);

        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, 100.0, samp_text2.as_ptr());
        HPDF_Page_EndText (page);

        /* char-spacing 1.5, word-spacing 3.5 */
        HPDF_Page_SetWordSpace (page, 2.5);

        HPDF_Page_BeginText (page);
        HPDF_Page_TextOut (page, 60.0, 60.0, samp_text2.as_ptr());
        HPDF_Page_EndText (page);

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);

    }
    return ();
}

