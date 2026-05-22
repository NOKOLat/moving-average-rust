use std::collections::VecDeque;

pub struct MovingAverage3D {
    window_size: usize,

    x: VecDeque<f32>,
    y: VecDeque<f32>,
    z: VecDeque<f32>,

    sum_x: f32,
    sum_y: f32,
    sum_z: f32,

    result_x: f32,
    result_y: f32,
    result_z: f32,
}

impl MovingAverage3D {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "window size must be greater than 0");

        Self {
            window_size: size,
            x: VecDeque::with_capacity(size),
            y: VecDeque::with_capacity(size),
            z: VecDeque::with_capacity(size),

            sum_x: 0.0,
            sum_y: 0.0,
            sum_z: 0.0,

            result_x: 0.0,
            result_y: 0.0,
            result_z: 0.0,
        }
    }

    pub fn update(&mut self, data: [f32; 3]) {
        if self.x.len() == self.window_size {
            self.sum_x -= self.x.pop_front().unwrap();
            self.sum_y -= self.y.pop_front().unwrap();
            self.sum_z -= self.z.pop_front().unwrap();
        }

        self.x.push_back(data[0]);
        self.y.push_back(data[1]);
        self.z.push_back(data[2]);

        self.sum_x += data[0];
        self.sum_y += data[1];
        self.sum_z += data[2];

        let data_count = self.x.len() as f32;

        self.result_x = self.sum_x / data_count;
        self.result_y = self.sum_y / data_count;
        self.result_z = self.sum_z / data_count;
    }

    pub fn getdata(&self) -> [f32; 3] {
        let data: [f32; 3] = [self.result_x, self.result_y, self.result_z];
        data
    }
}

#[cfg(test)]
mod tests {
    use super::MovingAverage3D;
    use std::{
        fs::{self, File},
        io::Write,
        path::{Path, PathBuf},
    };

    fn assert_array_near(actual: [f32; 3], expected: [f32; 3]) {
        for index in 0..3 {
            assert!(
                (actual[index] - expected[index]).abs() < f32::EPSILON,
                "index {index}: actual {}, expected {}",
                actual[index],
                expected[index]
            );
        }
    }

    fn test_output_path(file_name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test_output")
            .join(file_name)
    }

    fn write_results_csv(path: &Path, rows: &[(usize, [f32; 3], [f32; 3])]) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;
        writeln!(
            file,
            "step,input_x,input_y,input_z,average_x,average_y,average_z"
        )?;

        for (step, input, average) in rows {
            writeln!(
                file,
                "{},{},{},{},{},{},{}",
                step, input[0], input[1], input[2], average[0], average[1], average[2]
            )?;
        }

        Ok(())
    }

    #[test]
    fn first_n_updates_are_divided_by_available_data_count() -> std::io::Result<()> {
        let mut moving_average = MovingAverage3D::new(3);
        let inputs = [[3.0, 6.0, 9.0], [6.0, 9.0, 12.0], [9.0, 12.0, 15.0]];
        let expected_results = [[3.0, 6.0, 9.0], [4.5, 7.5, 10.5], [6.0, 9.0, 12.0]];
        let mut csv_rows = Vec::new();

        for (index, input) in inputs.iter().enumerate() {
            moving_average.update(*input);
            let result = moving_average.getdata();

            assert_array_near(result, expected_results[index]);
            csv_rows.push((index + 1, *input, result));
        }

        let csv_path = test_output_path("first_n_updates.csv");
        write_results_csv(&csv_path, &csv_rows)?;

        let csv = fs::read_to_string(csv_path)?;
        assert_eq!(
            csv,
            concat!(
                "step,input_x,input_y,input_z,average_x,average_y,average_z\n",
                "1,3,6,9,3,6,9\n",
                "2,6,9,12,4.5,7.5,10.5\n",
                "3,9,12,15,6,9,12\n",
            )
        );

        Ok(())
    }

    #[test]
    fn after_n_updates_oldest_value_is_removed() {
        let mut moving_average = MovingAverage3D::new(3);

        moving_average.update([3.0, 6.0, 9.0]);
        moving_average.update([6.0, 9.0, 12.0]);
        moving_average.update([9.0, 12.0, 15.0]);
        moving_average.update([12.0, 15.0, 18.0]);

        assert_array_near(moving_average.getdata(), [9.0, 12.0, 15.0]);
    }

    #[test]
    #[should_panic(expected = "window size must be greater than 0")]
    fn window_size_must_be_greater_than_zero() {
        MovingAverage3D::new(0);
    }
}
