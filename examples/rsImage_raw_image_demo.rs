//Using Image and libharu to render various images in pdf


extern crate libharu_sys;
extern crate libc;
extern crate image;


use std::ptr;
use std::ffi::CString;
use libharu_sys::*;
use std::fs::File;
use std::path::Path;



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


        let img = image::open(&Path::new("examples/images/PNG_trans_1.png")).unwrap();
        println!("{:?}", img.color());
        let img = img.to_rgba();
        let mut img_buff = Vec::<u8>::new();
        for pixel in img.pixels(){
            let rgba = pixel;
            let alpha = (rgba[3] as f32) / 255.0;
            img_buff.push( (rgba[0] as f32 * alpha + 255.0 * (1.0 - alpha)) as u8); 
            img_buff.push( (rgba[1] as f32 * alpha + 255.0 * (1.0 - alpha)) as u8); 
            img_buff.push( (rgba[2] as f32 * alpha + 255.0 * (1.0 - alpha)) as u8); 
        }

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


        HPDF_SetCompressionMode (pdf, HPDF_COMP_ALL);

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

        image = HPDF_LoadRawImageFromMem (pdf, img_buff.as_ptr(), img.width(), img.height(),
                    HPDF_ColorSpace::HPDF_CS_DEVICE_RGB, 8);//1);

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


