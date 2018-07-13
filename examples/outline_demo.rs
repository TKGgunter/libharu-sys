/*
 * << Haru Free PDF Library 2.0.0 >> -- outline_demo.c
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




fn print_page  ( page: HPDF_Page, page_num: i32)
{
    unsafe{

        HPDF_Page_SetWidth (page, 800.0);
        HPDF_Page_SetHeight (page, 800.0);

        HPDF_Page_BeginText (page);
        HPDF_Page_MoveTextPos (page, 30.0, 740.0);

        let buf = cstring!(format!("Page:{}", page_num));

        HPDF_Page_ShowText (page, buf.as_ptr());
        HPDF_Page_EndText (page);
    }
}

fn main()
{
    unsafe{
        let mut page = Vec::new();// HPDF_Page page[4];
        let mut outline = Vec::new(); //HPDF_Outline outline[4];
        let fname = cstring!("TEST.pdf");


        let pdf = HPDF_New (error_handler, ptr::null_mut());
        if (pdf == ptr::null_mut()) {
            println! ("error: cannot create PdfDoc object\n");
            return ;
        }


        /* create default-font */
        let font = HPDF_GetFont (pdf, cstring!("Helvetica").as_ptr(), ptr::null_mut());

        /* Set page mode to use outlines. */
        HPDF_SetPageMode(pdf, HPDF_PageMode::HPDF_PAGE_MODE_USE_OUTLINE);

        /* Add 3 pages to the document. */
        page.push( HPDF_AddPage (pdf) );
        HPDF_Page_SetFontAndSize (page[0], font, 30.0);
        print_page(page[0], 1);

        page.push(  HPDF_AddPage (pdf) );
        HPDF_Page_SetFontAndSize (page[1], font, 30.0);
        print_page(page[1], 2);

        page.push( HPDF_AddPage (pdf) );
        HPDF_Page_SetFontAndSize (page[2], font, 30.0);
        print_page(page[2], 3);

        /* create outline root. */
        let root = HPDF_CreateOutline (pdf, ptr::null_mut(), cstring!("OutlineRoot").as_ptr(), ptr::null_mut());
        HPDF_Outline_SetOpened (root, HPDF_TRUE);

        outline.push(HPDF_CreateOutline (pdf, root, cstring!("page1").as_ptr(), ptr::null_mut()));
        outline.push(HPDF_CreateOutline (pdf, root, cstring!("page2").as_ptr(), ptr::null_mut()));

        /* create outline with test which is ISO8859-2 encoding */
        //According to the rust compiler this is not valid utf8
        //outline.push(HPDF_CreateOutline (pdf, root, cstirng!("ISO8859-2 text XXXXXXX").as_ptr(),
        //                HPDF_GetEncoder (pdf, cstring!("ISO8859-2").as_ptr())));
        outline.push(HPDF_CreateOutline (pdf, root, cstring!("page3").as_ptr(),
                        HPDF_GetEncoder (pdf, cstring!("ISO8859-2").as_ptr())));

        /* create destination objects on each pages
         * and link it to outline items.
         */
        let mut dst = HPDF_Page_CreateDestination (page[0]);
        HPDF_Destination_SetXYZ(dst, 0.0, HPDF_Page_GetHeight(page[0]), 1.0);
        HPDF_Outline_SetDestination(outline[0], dst);
      //  HPDF_Catalog_SetOpenAction(dst);

        dst = HPDF_Page_CreateDestination (page[1]);
        HPDF_Destination_SetXYZ(dst, 0.0, HPDF_Page_GetHeight(page[1]), 1.0);
        HPDF_Outline_SetDestination(outline[1], dst);

        dst = HPDF_Page_CreateDestination (page[2]);
        HPDF_Destination_SetXYZ(dst, 0.0, HPDF_Page_GetHeight(page[2]), 1.0);
        HPDF_Outline_SetDestination(outline[2], dst);

        /* save the document to a file */
        HPDF_SaveToFile (pdf, fname.as_ptr());

        /* clean up */
        HPDF_Free (pdf);
    }
    return ;
}
