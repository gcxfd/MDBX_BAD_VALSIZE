use blake3::guts::{parent_cv, ChunkState, CHUNK_LEN};

fn test1() {
  let mut hasher = blake3::Hasher::new();
  let mut buf = [0; CHUNK_LEN];

  buf[0] = 'a' as u8;
  hasher.update(&buf);
  let chunk0_cv = ChunkState::new(0).update(&buf).finalize(false);

  buf[0] = 'b' as u8;
  hasher.update(&buf);
  let chunk1_cv = ChunkState::new(1).update(&buf).finalize(false);

  buf[0] = 'c' as u8;
  hasher.update(&buf);
  let chunk2_cv = ChunkState::new(2).update(&buf).finalize(false);

  buf[0] = 'd' as u8;
  hasher.update(&buf);
  let chunk3_cv = ChunkState::new(3).update(&buf).finalize(false);

  hasher.update(b".");
  let end = ChunkState::new(4).update(b".").finalize(false);

  let parent1 = parent_cv(&chunk0_cv, &chunk1_cv, false);
  let parent2 = parent_cv(&chunk2_cv, &chunk3_cv, false);
  let parent3 = parent_cv(&parent1, &parent2, false);
  let root = parent_cv(&parent3, &end, true);

  println!("{}", hasher.finalize());
  println!("{}", root);
  println!("{}", root == hasher.finalize());
}

fn test2() {
  let mut hasher = blake3::Hasher::new();
  let mut buf = [0; CHUNK_LEN];

  buf[0] = 'a' as u8;
  hasher.update(&buf);
  let chunk0_cv = ChunkState::new(0).update(&buf).finalize(false);

  buf[0] = 'b' as u8;
  hasher.update(&buf);
  let chunk1_cv = ChunkState::new(1).update(&buf).finalize(false);

  buf[0] = 'c' as u8;
  hasher.update(&buf);
  let chunk2_cv = ChunkState::new(2).update(&buf).finalize(false);

  hasher.update(b".");
  let end = ChunkState::new(3).update(b".").finalize(false);

  let parent1 = parent_cv(&chunk0_cv, &chunk1_cv, false);
  let parent2 = parent_cv(&chunk2_cv, &end, false);
  let root = parent_cv(&parent1, &parent2, true);

  println!("{}", hasher.finalize());
  println!("{}", root);
  println!("{}", root == hasher.finalize());
}

fn main() {
  test1();
  test2();
}
