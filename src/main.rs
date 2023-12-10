use std::any::Any;
use exif::{Error, Field};

fn main() {
    // for path in &["IMG_2241.JPG"] {
    //     let file = std::fs::File::open(path).expect("이미지 열기 실패");
    //     let mut buffer = std::io::BufReader::new(&file);
    //     let exifreader = exif::Reader::new();
    //     let exif = exifreader.read_from_container(&mut buffer).expect("exif 조회 실패");
    //
    //     for f in exif.fields() {
    //         println!("{} : {} -> {}", f.tag, f.ifd_num, f.display_value().with_unit(&exif));
    //     }
    // }

    match imghdr::from_file("Canon_40D.jpg") {
        Ok(Some(imghdr::Type::Png)) => println!("PNG"),
        Ok(Some(imghdr::Type::Jpeg)) => println!("JPG"),
        Err(e) => println!("Image type check fail : {}", e),
        _ => {}
    }

    println!("----------------------------------------------------");
    let file = std::fs::File::open("Canon_40D.jpg").expect("이미지 열기 실패");
    let mut buffer = std::io::BufReader::new(&file);
    let reader = exif::Reader::new();
    // let reader = exif::Reader::new(&mut std::io::BufReader::new(&file)).unwrap();
    let exif = reader.read_from_container(&mut buffer).expect("exif 조회 실패");
    let exist_field = exif.get_field(exif::Tag::UserComment, exif::In::PRIMARY);


    buffer = std::io::BufReader::new(&file);

    let exif_attr = exif::get_exif_attr_from_jpeg(&mut buffer);

    println!("check is little endian : {}", exif.little_endian());

    match exif_attr {
        Ok(v) => {
            println!("exif_attr : {}", String::from_utf8(v).unwrap());
        }
        Err(e) => {
            println!("exif error : {}", e);
        }
    }

    match exist_field {
        None => {
            println!("UserComment field 존재하지 않음");
            std::process::exit(1);
        }
        Some(v) => {
            println!("UserComment 존재함");
        }
    }

    for field in exif.fields() {
        // println!("tag : {}", field.display_value());
        if field.tag == exif::Tag::UserComment {
            let user_comment = field.value.display_as(field.tag).to_string();

            println!("User comment : {}", user_comment);
        }
    }

    // add tag test
    // let input = std::fs::read("IMG_2241.JPG").expect("파일 읽기 실패");
    // let output = std::fs::File::create("out.jpg").expect("파일 생성 실패");
    // let mut jpg = Jpeg::from_bytes(input.into()).unwrap();
    // let comment = Bytes::from("Hello");
    // let comment_segment = JpegSegment::new_with_contents(markers::COM, comment);

    // jpg.segments_mut().insert(1, comment_segment);
    // jpg.encoder().write_to(output);
}
