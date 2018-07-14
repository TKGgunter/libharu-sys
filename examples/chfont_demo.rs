/*
 * << Haru Free PDF Library 2.0.0 >> -- chfont_demo.c
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
use std::str;
use std::ptr;
use std::fs::File;
use std::io::prelude::*;
use std::ffi::CString;
use libharu_sys::*;

use std::fmt::Display;
use std::fmt::Debug;

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
        let mut buf = [0u8;1024];

        let mut cp932  = File::open("examples/mbtext/cp932.txt").unwrap();
        let mut cp936  = File::open("examples/mbtext/cp936.txt").unwrap();

        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }


        HPDF_SetCompressionMode (pdf, HPDF_COMP_ALL);
        HPDF_UseJPEncodings (pdf);
        HPDF_UseCNSEncodings (pdf);

        let fcp936_name = HPDF_LoadTTFontFromFile (pdf, cstring!("examples/ttfont/NotoSansCJKtc-Regular.ttf").as_ptr(), //3,//atoi(argv[2]),
                HPDF_TRUE);
        let fcp932_name = HPDF_LoadTTFontFromFile (pdf, cstring!("examples/ttfont/NotoSansCJKtc-Regular.ttf").as_ptr(), // 1,//atoi(argv[4]),
                HPDF_TRUE);

        /* add a new page object. */
        let page = HPDF_AddPage (pdf);

        HPDF_Page_SetHeight (page, 300.0);
        HPDF_Page_SetWidth (page, 550.0);

        let fcp936 = HPDF_GetFont (pdf, fcp936_name, cstring!("GBK-EUC-H").as_ptr());
        let fcp932 = HPDF_GetFont (pdf, fcp932_name, cstring!("90ms-RKSJ-H").as_ptr());

        //print_grid  (pdf, page);

        HPDF_Page_SetTextLeading (page, 20.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 50.0, 250.0);
        HPDF_Page_SetTextLeading (page, 25.0);

        match cp936.read (&mut buf) {
            Ok(..) => {
                HPDF_Page_SetFontAndSize (page, fcp936, 18.0);
                {
                    println!("{:?}", &buf[0..1024]);
                    let temp_str = CString::from_vec_unchecked(buf.to_vec());
                    //This is a zero terminator IDK if we need thisf.
                    //buf [strlen (buf)] = 0;
                    HPDF_Page_ShowText (page, temp_str.as_ptr());
                }
                match cp932.read(&mut buf) {
                    Ok(..)=>{
                            HPDF_Page_SetFontAndSize (page, fcp932, 18.0);
                            println!("{:?}", &buf[0..1024]);
                            let temp_str = CString::from_vec_unchecked(buf.to_vec());
                            //This is a zero terminator IDK if we need this.
                            //buf [strlen (buf)] = 0;
                            HPDF_Page_ShowText (page, temp_str.as_ptr());
                    },
                    Err(e)=>{ 
                        println!("cp932 error: {}", e);
                    },
                }

                HPDF_Page_MoveToNextLine (page);
            },
            Err(e)=>{ println!("cp936 error {}", e);}
        }

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);

    }
    return ;
}

