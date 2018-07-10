extern crate pkg_config;
extern crate gcc;

use std::path::Path;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&root).join("libharu");

    println!("cargo:include={}", root.join("include").into_os_string().into_string().unwrap());
    println!("cargo:src={}", root.join("lib").into_os_string().into_string().unwrap());

    match pkg_config::find_library("haru") {
        Ok(_) => return,
        Err(..) => {}
    };


    gcc::Build::new()
                .file("libharu/src/hpdf_3dmeasure.c")
                .file("libharu/src/hpdf_annotation.c")
                .file("libharu/src/hpdf_array.c")
                .file("libharu/src/hpdf_binary.c")
                .file("libharu/src/hpdf_boolean.c")
                .file("libharu/src/hpdf_catalog.c")
                .file("libharu/src/hpdf_destination.c")
                .file("libharu/src/hpdf_dict.c")
                .file("libharu/src/hpdf_doc.c")
                .file("libharu/src/hpdf_doc_png.c")
                .file("libharu/src/hpdf_encoder.c")
                .file("libharu/src/hpdf_encoder_cns.c")
                .file("libharu/src/hpdf_encoder_cnt.c")
                .file("libharu/src/hpdf_encoder_jp.c")
                .file("libharu/src/hpdf_encoder_kr.c")
                .file("libharu/src/hpdf_encoder_utf.c")
                .file("libharu/src/hpdf_encrypt.c")
                .file("libharu/src/hpdf_encryptdict.c")
                .file("libharu/src/hpdf_error.c")
                .file("libharu/src/hpdf_exdata.c")
                .file("libharu/src/hpdf_ext_gstate.c")
                .file("libharu/src/hpdf_font.c")
                .file("libharu/src/hpdf_font_cid.c")
                .file("libharu/src/hpdf_fontdef_base14.c")
                .file("libharu/src/hpdf_fontdef.c")
                .file("libharu/src/hpdf_fontdef_cid.c")
                .file("libharu/src/hpdf_fontdef_cns.c")
                .file("libharu/src/hpdf_fontdef_cnt.c")
                .file("libharu/src/hpdf_fontdef_jp.c")
                .file("libharu/src/hpdf_fontdef_kr.c")
                .file("libharu/src/hpdf_fontdef_tt.c")
                .file("libharu/src/hpdf_fontdef_type1.c")
                .file("libharu/src/hpdf_font_tt.c")
                .file("libharu/src/hpdf_font_type1.c")
                .file("libharu/src/hpdf_gstate.c")
                .file("libharu/src/hpdf_image.c")
                .file("libharu/src/hpdf_image_ccitt.c")
                .file("libharu/src/hpdf_image_png.c")
                .file("libharu/src/hpdf_info.c")
                .file("libharu/src/hpdf_list.c")
                .file("libharu/src/hpdf_mmgr.c")
                .file("libharu/src/hpdf_name.c")
                .file("libharu/src/hpdf_namedict.c")
                .file("libharu/src/hpdf_null.c")
                .file("libharu/src/hpdf_number.c")
                .file("libharu/src/hpdf_objects.c")
                .file("libharu/src/hpdf_outline.c")
                .file("libharu/src/hpdf_page_label.c")
                .file("libharu/src/hpdf_page_operator.c")
                .file("libharu/src/hpdf_pages.c")
                .file("libharu/src/hpdf_pdfa.c")
                .file("libharu/src/hpdf_real.c")
                .file("libharu/src/hpdf_streams.c")
                .file("libharu/src/hpdf_string.c")
                .file("libharu/src/hpdf_string.c")
                .file("libharu/src/hpdf_u3d.c")
                .file("libharu/src/hpdf_utils.c")
                .file("libharu/src/hpdf_xref.c")
                .define("_USRDLL", None)
                .define("LIBHARU_EXPORTS", None)
                .include(&root.join("include"))
                .include(&root.join("lib"))
                //.include(&ogg_inc)
                .compile("libharu.a");
}
