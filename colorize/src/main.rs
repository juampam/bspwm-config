use image::{GenericImageView, Rgba};
fn main(){
    let img = image::open("/root/.config/wallpapers/submarinetrip.jpg").expect("err");
    let width = img.width();
    let height = img.height();

    for y in 0..height{
        for x in 0..width{
            let pixel = img.get_pixel(x,y);
            let color = pixel.0;
            let color_hex = format!("{:02X}{:02X}{:02X}", color[0], color[1], color[2]);
            let pixel_number = y * width + x;
            println!("#{}",color_hex);
        }
    }
}
