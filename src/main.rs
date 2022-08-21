use duckvoxel::run;
use pollster::block_on;

fn main() {
    pollster::block_on(run());
}