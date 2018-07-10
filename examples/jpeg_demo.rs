/*
 * << Haru Free PDF Library 2.0.0 >> -- jpeg_demo.c
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


fn draw_image (  pdf      : HPDF_Doc, 
                 filename : &str,  
                 x        : f32,       
                 y        : f32,       
                 text     : &str )
{
    unsafe{
        let filename1 = CString::new("examples/images/".to_owned()+filename).unwrap();

        let page = HPDF_GetCurrentPage (pdf);


        let image = HPDF_LoadJpegImageFromFile (pdf, CString::new(filename1).unwrap().as_ptr());

        /* Draw image to the canvas. */
        HPDF_Page_DrawImage (page, image, x, y, HPDF_Image_GetWidth (image) as f32,
                    HPDF_Image_GetHeight (image) as f32);

        /* Print the text. */
        HPDF_Page_BeginText (page);
        HPDF_Page_SetTextLeading (page, 16.0);
        HPDF_Page_MoveTextPos (page, x, y);
        HPDF_Page_ShowTextNextLine (page, CString::new(filename).unwrap().as_ptr());
        HPDF_Page_ShowTextNextLine (page, CString::new(text).unwrap().as_ptr());
        HPDF_Page_EndText (page);
    }
}


fn main ()
{
    unsafe{
        let fname = CString::new("TEST.pdf").unwrap();

        let mut pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            panic!("")
        }

        HPDF_SetCompressionMode (pdf, HPDF_COMP_ALL);

        /* create default-font */
        let font = HPDF_GetFont (pdf, CString::new("Helvetica").unwrap().as_ptr(), ptr::null_mut());

        /* add a new page object. */
        let mut page = HPDF_AddPage (pdf);

        HPDF_Page_SetWidth (page, 650.0);
        HPDF_Page_SetHeight (page, 500.0);

        let dst = HPDF_Page_CreateDestination (page);
        HPDF_Destination_SetXYZ (dst, 0.0, HPDF_Page_GetHeight (page), 1.0);
        HPDF_SetOpenAction(pdf, dst);

        HPDF_Page_BeginText (page);
        HPDF_Page_SetFontAndSize (page, font, 20.0);
        HPDF_Page_MoveTextPos (page, 220.0, HPDF_Page_GetHeight (page) - 70.0);
        HPDF_Page_ShowText (page, CString::new("JpegDemo").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        HPDF_Page_SetFontAndSize (page, font, 12.0);

        draw_image (pdf, "rgb.jpg", 70.0, HPDF_Page_GetHeight (page) - 410.0,
                    "24bit color image");
        draw_image (pdf, "gray.jpg", 340.0, HPDF_Page_GetHeight (page) - 410.0,
                    "8bit grayscale image");

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }
}

