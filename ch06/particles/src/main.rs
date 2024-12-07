use piston_window::*;
use rand::prelude::ThreadRng;
use rand::{Rng, rng};
use std::alloc::{GlobalAlloc, Layout, System};
use std::time::Instant;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let start = Instant::now();
    let ptr = unsafe { System.alloc(layout) };
    let lapse = Instant::now() - start;
    let size = layout.size();

    eprintln!("{}\t{}", size, lapse.as_nanos());
    ptr
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    unsafe {
      System.dealloc(ptr, layout);
    }
  }
}

struct World {
  current_turn: u64,
  particles: Vec<Box<Particle>>,
  height: f64,
  width: f64,
  rng: ThreadRng,
}

struct Particle {
  height: f64,
  width: f64,
  position: graphics::math::Vec2d<f64>,
  velocity: graphics::math::Vec2d<f64>,
  acceleration: graphics::math::Vec2d<f64>,
  color: [f32; 4],
}

impl Particle {
  fn new(world: &World) -> Particle {
    let mut rng = rng();
    let x = rng.random_range(0.0..=world.width);
    let y = world.height;
    let x_velocity = 0.0;
    let y_velocity = rng.random_range(-2.0..0.0);
    let x_acceleration = 0.0;
    let y_acceleration = rng.random_range(0.0..0.15);

    Particle {
      height: 4.0,
      width: 4.0,
      position: [x, y].into(),
      velocity: [x_velocity, y_velocity].into(),
      acceleration: [x_acceleration, y_acceleration].into(),
      color: [1.0, 1.0, 1.0, 0.99],
    }
  }

  fn update(&mut self) {
    self.velocity = graphics::math::add(self.velocity, self.acceleration);
    self.position = graphics::math::add(self.position, self.velocity);
    self.acceleration = graphics::math::mul_scalar(self.acceleration, 0.7);
    self.color[3] *= 0.995;
  }
}

impl World {
  fn new(width: f64, height: f64) -> World {
    World {
      particles: Vec::<Box<Particle>>::new(),
      current_turn: 0,
      rng: rng(),
      height,
      width,
    }
  }

  fn add_shapes(&mut self, n: i32) {
    for _ in 0..n.abs() {
      let particle = Particle::new(&self);
      let boxed_particle = Box::new(particle);
      self.particles.push(boxed_particle);
    }
  }

  fn remove_shapes(&mut self, n: i32) {
    for _ in 0..n.abs() {
      let mut to_delete = None;
      let particle_iter = self.particles.iter().enumerate();
      for (i, particle) in particle_iter {
        if particle.color[3] < 0.02 {
          to_delete = Some(i);
        }
        break;
      }

      self.particles.remove(match to_delete {
        Some(i) => i,
        None => 0,
      });
    }
  }

  fn update(&mut self) {
    let n = self.rng.random_range(-3..=3);
    if n > 0 {
      self.add_shapes(n);
    } else {
      self.remove_shapes(n);
    }

    self.particles.shrink_to_fit();

    for shape in &mut self.particles {
      shape.update();
    }
    self.current_turn += 1;
  }
}

fn main() {
  let (width, height) = (1280.0, 960.0);
  let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
    .exit_on_esc(true)
    .build()
    .expect("Could not create window.");

  let mut world = World::new(width, height);
  world.add_shapes(1000);

  while let Some(event) = window.next() {
    world.update();
    window.draw_2d(&event, |ctx, renderer, _device| {
      clear([0.15, 0.17, 0.17, 0.9], renderer);

      for s in &mut world.particles {
        let size = [s.position[0], s.position[1], s.width, s.height];
        rectangle(s.color, size, ctx.transform, renderer);
      }
    });
  }
}
