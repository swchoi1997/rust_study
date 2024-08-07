
#[derive(Debug)]
struct Rectangle{
    width   : u32,
    height  : u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self{
            width: size,
            height: size
        }
    }
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn half_size(&self) -> u32{
        return self.area() / 2;

    }
}

fn main() {
    let rect: Rectangle = Rectangle{
        width: 32,
        height: 2,
    };

    let size: u32 = rect.area();
    let half_size: u32 = rect.half_size();

    dbg!(size);
    dbg!(half_size);

    let square: Rectangle = Rectangle::square(10);

    dbg!(&square);

}