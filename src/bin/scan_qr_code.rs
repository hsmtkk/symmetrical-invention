fn main(){
    let img = image::open("sample/qr-code.png").expect("open image").to_luma8();
    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();
    let (meta, content) = grids[0].decode().expect("decode");
    println!("{:?}", meta);
    println!("{}", content);
}
