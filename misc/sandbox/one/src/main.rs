fn main() {
  one::lib();
  two::lib();
  three::lib();
  one::inner::one();
  three::inner::three();
  three::pack::inner::three();
  three::packf::inner::three();

  let mut rng = three::packf::inner::Range::to(10);
  for i in rng.iter() {
    rng.stop += i;
    println!("Current: {i}");
  }
  println!("ALL: {:?}", Vec::from_iter(rng.iter()));
}
