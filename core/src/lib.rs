// NOTE: The following is a basic end to end example

#[cfg(test)]
mod tests {
    use mopro_core::middleware::circom::serialization::SerializableInputs;
    use mopro_core::middleware::gpu_explorations::{arkworksPippenger, trapdoorTechZprizeMsm, BenchmarkResult};

    const INSTANCE_SIZE: u32 = 16;
    const NUM_INSTANCE: u32 = 10;
    const UTILSPATH: &str = "core/benchmarks/vectors";
    const BENCHMARKSPATH: &str = "core/benchmarks/results";

    #[test]
    fn test_arkworks_pippenger() {
        let utils_path = format!("{}/{}x{}", &UTILSPATH, INSTANCE_SIZE, NUM_INSTANCE);
        let result = run_benchmark(INSTANCE_SIZE, NUM_INSTANCE, &utils_path).unwrap();
        println!("Benchmark result: {:#?}", result);
    }

    #[test]
    fn test_run_multi_benchmarks() {
        let output_path = format!("{}/{}_benchmark.txt", &BENCHMARKSPATH, "arkworks_pippenger");
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)");

        let instance_size = vec![8, 12, 16, 18, 20];
        let num_instance = vec![5, 10];
        for size in &instance_size {
            for num in &num_instance {
                let utils_path = format!("{}/{}x{}", &UTILSPATH, *size, *num);
                let result = run_benchmark(*size, *num, &utils_path).unwrap();
                println!("{}x{} result: {:#?}", *size, *num, result);
                writeln!(
                    output_file,
                    "{},{},{}",
                    result.instance_size, result.num_instance, result.avg_processing_time
                );
            }
        }
    }
}