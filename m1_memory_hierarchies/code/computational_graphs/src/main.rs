use computational_graphs::run;

fn main() {
    pollster::block_on(run());
}
