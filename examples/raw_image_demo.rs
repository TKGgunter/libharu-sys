/*
 * << Haru Free PDF Library 2.0.0 >> -- raw_image_demo.c
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



const RAW_IMAGE_DATA: [HPDF_BYTE; 128] = [
    0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xff, 0xfc,
    0xff, 0xff, 0xff, 0xf8, 0xff, 0xff, 0xff, 0xf0,
    0xf3, 0xf3, 0xff, 0xe0, 0xf3, 0xf3, 0xff, 0xc0,
    0xf3, 0xf3, 0xff, 0x80, 0xf3, 0x33, 0xff, 0x00,
    0xf3, 0x33, 0xfe, 0x00, 0xf3, 0x33, 0xfc, 0x00,
    0xf8, 0x07, 0xf8, 0x00, 0xf8, 0x07, 0xf0, 0x00,
    0xfc, 0xcf, 0xe0, 0x00, 0xfc, 0xcf, 0xc0, 0x00,
    0xff, 0xff, 0x80, 0x00, 0xff, 0xff, 0x00, 0x00,
    0xff, 0xfe, 0x00, 0x00, 0xff, 0xfc, 0x00, 0x00,
    0xff, 0xf8, 0x0f, 0xe0, 0xff, 0xf0, 0x0f, 0xe0,
    0xff, 0xe0, 0x0c, 0x30, 0xff, 0xc0, 0x0c, 0x30,
    0xff, 0x80, 0x0f, 0xe0, 0xff, 0x00, 0x0f, 0xe0,
    0xfe, 0x00, 0x0c, 0x30, 0xfc, 0x00, 0x0c, 0x30,
    0xf8, 0x00, 0x0f, 0xe0, 0xf0, 0x00, 0x0f, 0xe0,
    0xe0, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00,
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

fn main () -> ()
{
    unsafe{
        let mut page: HPDF_Page;
        let fname = CString::new("TEST.pdf").unwrap();
        let mut image: HPDF_Image;

        let mut x: HPDF_REAL;
        let mut y: HPDF_REAL;


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf==ptr::null_mut()) {
            println!("error: cannot create PdfDoc object\n");
            return ();
        }


        println!("Did compression work? {}", HPDF_SetCompressionMode (pdf, HPDF_COMP_ALL));

        /* create default-font */
        let font = HPDF_GetFont (pdf, CString::new("Helvetica").unwrap().as_ptr(), ptr::null_mut());

        /* add a new page object. */
        page = HPDF_AddPage (pdf);

        HPDF_Page_SetWidth (page, 172.0);
        HPDF_Page_SetHeight (page, 80.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_SetFontAndSize (page, font, 20.0);
        HPDF_Page_MoveTextPos (page, 220.0, HPDF_Page_GetHeight (page) - 70.0);
        HPDF_Page_ShowText (page, CString::new("RawImageDemo").unwrap().as_ptr());
        HPDF_Page_EndText (page);

        /* load RGB raw-image file. */
        image = HPDF_LoadRawImageFromFile (pdf, CString::new("examples/rawimage/32_32_rgb.dat").unwrap().as_ptr(),
                32, 32, HPDF_ColorSpace::HPDF_CS_DEVICE_RGB);

        x = 20.0;
        y = 20.0;

        /* Draw image to the canvas. (normal-mode with actual size.)*/
        HPDF_Page_DrawImage (page, image, x, y, 32.0, 32.0);

        /* load GrayScale raw-image file. */
        image = HPDF_LoadRawImageFromFile (pdf, CString::new("examples/rawimage/32_32_gray.dat").unwrap().as_ptr(),
                32, 32, HPDF_ColorSpace::HPDF_CS_DEVICE_GRAY);

        x = 70.0;
        y = 20.0;

        /* Draw image to the canvas. (normal-mode with actual size.)*/
        HPDF_Page_DrawImage (page, image, x, y, 32.0, 32.0);

        /* load GrayScale raw-image (1bit) file from memory. */
        //HPDF_Image_LoadRaw1BitImageFromMem 
        //The function above is what a the function calls when you pass it CS_DEVICE_GRAY, 1 bit_per_component.
        //This doesn't seem to work. I would not use it if possible.
        //You can check for your self by looking at the bits from the first element numericly and
        //visually
        println!("{:b}", RAW_IMAGE_DATA[0]);
        image = HPDF_LoadRawImageFromMem (pdf, RAW_IMAGE_DATA.as_ptr(), 8, 1,//32, 32,
                    HPDF_ColorSpace::HPDF_CS_DEVICE_GRAY, 1);//1);

        x = 120.0;
        y = 20.0;

        /* Draw image to the canvas. (normal-mode with actual size.)*/
        HPDF_Page_DrawImage (page, image, x, y, 32.0, 32.0);

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);

    }
    return ();
}


