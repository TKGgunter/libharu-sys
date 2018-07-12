/*
 * << Haru Free PDF Library 2.0.0 >> -- jpfont_demo.c
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
use std::fs::File;
use libharu_sys::*;
use std::io::prelude::*;

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
        let mut detail_font = Vec::<HPDF_Font>::new();
        let PAGE_HEIGHT = 210.0;

        let mut contents = Vec::new();
        let mut contents_length :i32 = 0;
        {
            let mut f = File::open("examples/mbtext/sjis.txt").expect("File could not be loaded.");
            f.read_to_end(&mut contents).unwrap();
            contents_length = contents.len() as i32;
        }
        let samp_text = CString::from_vec_unchecked(contents);


        let fname = cstring!("TEST.pdf");
        
        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }
        

        /* configure pdf-document to be compressed. */
        HPDF_SetCompressionMode (pdf, HPDF_COMP_ALL);

        /* declaration for using Japanese font, encoding. */
        HPDF_UseJPEncodings (pdf);
        HPDF_UseJPFonts (pdf);

        //Typo in the original Mincyo fails to find font should be Mincho
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-Mincho").as_ptr(),                cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-Mincho,Bold").as_ptr(),           cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-Mincho,Italic").as_ptr(),         cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-Mincho,BoldItalic").as_ptr(),     cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-PMincho").as_ptr(),               cstring!("90msp-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-PMincho,Bold").as_ptr(),          cstring!("90msp-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-PMincho,Italic").as_ptr(),        cstring!("90msp-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf, cstring!("MS-PMincho,BoldItalic").as_ptr(),    cstring!("90msp-RKSJ-H").as_ptr()));

        detail_font.push(HPDF_GetFont (pdf,  cstring!("MS-Gothic").as_ptr(),        cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push(HPDF_GetFont (pdf,  cstring!("MS-Gothic,Bold").as_ptr(),   cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push( HPDF_GetFont (pdf, cstring!("MS-Gothic,Italic").as_ptr(), cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push( HPDF_GetFont (pdf, cstring!("MS-Gothic,BoldItalic").as_ptr(), cstring!("90ms-RKSJ-H").as_ptr()));
        detail_font.push( HPDF_GetFont (pdf, cstring!("MS-PGothic").as_ptr(),           cstring!("90msp-RKSJ-H").as_ptr()));
        detail_font.push( HPDF_GetFont (pdf, cstring!("MS-PGothic,Bold").as_ptr(),      cstring!("90msp-RKSJ-H").as_ptr()));
        detail_font.push( HPDF_GetFont (pdf, cstring!("MS-PGothic,Italic").as_ptr(),    cstring!("90msp-RKSJ-H").as_ptr()));
        detail_font.push( HPDF_GetFont (pdf, cstring!("MS-PGothic,BoldItalic").as_ptr(),cstring!("90msp-RKSJ-H").as_ptr()));

        /* Set page mode to use outlines. */
        HPDF_SetPageMode(pdf, HPDF_PageMode::HPDF_PAGE_MODE_USE_OUTLINE);

        /* create outline root. */
        let root = HPDF_CreateOutline (pdf, ptr::null_mut(), cstring!("JP font demo").as_ptr(), ptr::null_mut());
        HPDF_Outline_SetOpened (root, HPDF_TRUE);

        for i in 0..15{

            /* add a new page object. */
            let page = HPDF_AddPage (pdf);

            /* create outline entry */
            let outline = HPDF_CreateOutline (pdf, root,
                    HPDF_Font_GetFontName (detail_font[i]), ptr::null_mut());
            let dst = HPDF_Page_CreateDestination (page);
            HPDF_Outline_SetDestination(outline, dst);

            let title_font = HPDF_GetFont (pdf, cstring!("Helvetica").as_ptr(), ptr::null_mut());
            HPDF_Page_SetFontAndSize (page, title_font, 10.0);

            HPDF_Page_BeginText (page);

            /* move the position of the text to top of the page. */
            HPDF_Page_MoveTextPos(page, 10.0, 190.0);
            HPDF_Page_ShowText (page, HPDF_Font_GetFontName (detail_font[i]));

            HPDF_Page_SetFontAndSize (page, detail_font[i], 15.0);
            HPDF_Page_MoveTextPos (page, 10.0, -20.0);
            HPDF_Page_ShowText (page, cstring!("abcdefghijklmnopqrstuvwxyz").as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -20.0);
            HPDF_Page_ShowText (page, cstring!("ABCDEFGHIJKLMNOPQRSTUVWXYZ").as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -20.0);
            HPDF_Page_ShowText (page, cstring!("1234567890").as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -20.0);

            HPDF_Page_SetFontAndSize (page, detail_font[i], 10.0);
            HPDF_Page_ShowText (page, samp_text.as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -18.0);

            HPDF_Page_SetFontAndSize (page, detail_font[i], 16.0);
            HPDF_Page_ShowText (page, samp_text.as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -27.0);

            HPDF_Page_SetFontAndSize (page, detail_font[i], 23.0);
            HPDF_Page_ShowText (page, samp_text.as_ptr());
            HPDF_Page_MoveTextPos (page, 0.0, -36.0);

            HPDF_Page_SetFontAndSize (page, detail_font[i], 30.0);
            HPDF_Page_ShowText (page, samp_text.as_ptr());

            let mut p = HPDF_Page_GetCurrentTextPos (page);

            /* finish to print text. */
            HPDF_Page_EndText (page);

            HPDF_Page_SetLineWidth (page, 0.5);

            let mut x_pos = 20.0;
            for j in 0..contents_length / 2 {
                HPDF_Page_MoveTo (page, x_pos, p.y - 10.0);
                HPDF_Page_LineTo (page, x_pos, p.y - 12.0);
                HPDF_Page_Stroke (page);
                x_pos = x_pos + 30.0;
            }

            HPDF_Page_SetWidth (page, p.x + 20.0);
            HPDF_Page_SetHeight (page, PAGE_HEIGHT);

            HPDF_Page_MoveTo (page, 10.0, PAGE_HEIGHT - 25.0);
            HPDF_Page_LineTo (page, p.x + 10.0, PAGE_HEIGHT - 25.0);
            HPDF_Page_Stroke (page);

            HPDF_Page_MoveTo (page, 10.0, PAGE_HEIGHT - 85.0);
            HPDF_Page_LineTo (page, p.x + 10.0, PAGE_HEIGHT - 85.0);
            HPDF_Page_Stroke (page);

            HPDF_Page_MoveTo (page, 10.0, p.y - 12.0);
            HPDF_Page_LineTo (page, p.x + 10.0, p.y - 12.0);
            HPDF_Page_Stroke (page);
        }

        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }
    return ;
}

