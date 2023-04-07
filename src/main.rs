use ray::RayTuple;

fn main() {
    chapter_one_cannon();
}

//CHAPTER ONE Cannon Exercise
//projectile has a position point and a velocity vector
//environment has a gravity vector and a wind vector

//the tick function accepts an environment and projectile, it updates the position based on the projectile\ts velocity,
//and also applies the velocity vectors to the projectiles velocity each tick

//the program initializes the projectile and velocity and runs the tick function until the projectile
//hits the ground (y pos <= 0)
//report the projectile's position after each tick
//report the number of ticks run until the projectile hits the ground
fn chapter_one_cannon() {
    fn tick(p: (RayTuple, RayTuple), e: (RayTuple, RayTuple)) -> (RayTuple, RayTuple) {
        let new_position = p.0 + p.1;
        let new_velocity = p.1 + e.0 + e.1;
        (new_position, new_velocity)
    }

    let mut p = (
        RayTuple::point(0.0, 1.0, 0.0), //initial point, y=1, x/z = 0
        RayTuple::vector(1.0, 1.0, 0.0).normalize(), //the starting velocity is x=1, y=1 with a magnitude of sqrt(2)
    );

    let e = (
        RayTuple::vector(0.0, -0.1, 0.0),  //gravity
        RayTuple::vector(-0.01, 0.0, 0.0), //wind
    );

    let mut tick_count = 1;

    println!("t\tx\ty\tz");

    while p.0.y > 0.0 {
        println!("{tick_count}\t{}\t{}\t{}", p.0.x, p.0.y, p.0.z);
        p = tick(p, e);
        tick_count += 1;
    }

    println!("{tick_count}\t{}\t{}\t{}", p.0.x, p.0.y, p.0.z);
}
