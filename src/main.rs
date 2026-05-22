mod moving_average3d;
use moving_average3d::MovingAverage3D;

fn main() {

    println!("Program Start");

    let mut mov_ave = MovingAverage3D::new(10);

    for i in 1..100{

        let data:[f32; 3] = [i as f32, (i*2) as f32, (i*3) as f32];
        mov_ave.update(data);
    }

    let result: [f32; 3];
    result = mov_ave.getdata();

    println!("result {} {} {}", result[0], result[1], result[2]);
}