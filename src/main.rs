fn main() -> anyhow::Result<()> {
    //Ok(algo::percolation::monte_carlo()?)
    algo::collinear_points::run_collinear_points("./projects/collinear/rs1423.txt".to_string())
}
